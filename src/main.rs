// Remove duplicated files found in a reference directory in a directory
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2024  Manuel Amersdorfer

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{ArgAction, Parser};
use data_encoding::HEXLOWER;
use log::{error, info, warn};
use log::{Level, LevelFilter, Metadata, Record};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

/// Remove duplicated files in the reference directory that are found in the root directory tree.
#[derive(Parser)]
#[clap(author="Manuel Amersdorfer", version)]
struct Cli {
    /// Reference directory path
    reference_dir: PathBuf,
    /// Root directory path
    root_dir: PathBuf,
    /// Perform a dry-run without removing any file
    #[clap(long, short, action(ArgAction::SetTrue))]
    dry_run: bool,

}

/// Hash a file and return its sha256 hash value
fn sha256sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(&path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 4098];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Check if the path is a subdirectory of the reference path
fn is_subdirectory(entry: &PathBuf, reference: &PathBuf) -> bool {
    entry
        .to_str()
        .unwrap()
        .starts_with(reference.to_str().unwrap())
}

/// Check if directory entry is a file
fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

/// Checks if the string equals the empty hash
fn is_empty_hash(hash: &str) -> bool {
    hash.eq("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    let _ = log::set_logger(&CONSOLE_LOGGER);
    log::set_max_level(LevelFilter::Info);

    // Parse command line arguments
    let args = Cli::parse();
    let root_dir = match Path::new(&args.root_dir).canonicalize() {
        Ok(dir) => dir,
        Err(err) => {
            error!("Error checking root path: {}", err);
            return Err(err.into());
        }
    };
    let reference_dir = match Path::new(&args.reference_dir).canonicalize() {
        Ok(dir) => dir,
        Err(err) => {
            error!("Error checking reference path: {}", err);
            return Err(err.into());
        }
    };
    if root_dir.is_dir() {
        info!("Root directory: {}", root_dir.to_str().unwrap());
    } else {
        warn!(
            "Root path {} should be a directory",
            root_dir.to_str().unwrap()
        );
    }
    if reference_dir.is_dir() {
        info!("Reference directory: {}", reference_dir.to_str().unwrap());
    } else {
        warn!(
            "Reference path {} should be a directory",
            reference_dir.to_str().unwrap()
        );
    }

    // Calculate list of hashes for the root directory tree
    let root_dirs: Vec<DirEntry> = WalkDir::new(root_dir.clone())
        .into_iter()
        .filter_entry(|e| !is_subdirectory(&e.clone().into_path(), &reference_dir))
        .filter_map(|v| v.ok())
        .collect();
    let root_files: Vec<DirEntry> = root_dirs.into_iter().filter(|e| is_file(e)).collect();

    let root_pairs: Vec<(String, String)> = root_files
        .into_iter()
        .map(|e| {
            (
                sha256sum(e.path()).unwrap_or(String::new()),
                (fs::canonicalize(e.path().to_str().unwrap_or(""))
                    .unwrap()
                    .to_str()
                    .to_owned())
                .unwrap()
                .to_string(),
            )
        })
        .filter(|pair| !is_empty_hash(pair.0.as_str()))
        .collect();

    // Calculate list of hashes for the reference directory tree
    let reference_dirs: Vec<DirEntry> = WalkDir::new(reference_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();
    let reference_files: Vec<DirEntry> =
        reference_dirs.into_iter().filter(|e| is_file(e)).collect();

    let reference_pairs: Vec<(String, String)> = reference_files
        .into_iter()
        .map(|e| {
            (
                sha256sum(e.path()).unwrap_or(String::new()),
                (fs::canonicalize(e.path().to_str().unwrap_or(""))
                    .unwrap()
                    .to_str()
                    .to_owned())
                .unwrap()
                .to_string(),
            )
        })
        .filter(|pair| !is_empty_hash(pair.0.as_str()))
        .collect();

    // Find duplicates
    println!("Check for duplicates");
    let reference_hashes: Vec<String> = reference_pairs.into_iter().map(|p| p.0).collect();
    let duplicate_pairs: Vec<(String, String)> = root_pairs
        .into_iter()
        .filter(|pair| reference_hashes.contains(&pair.0))
        .collect();

    if duplicate_pairs.len() == 0 {
        println!("No duplicates found")
    }

    duplicate_pairs
        .into_iter()
        .for_each(|s| println!("{} {}", s.0, s.1));

    if !args.dry_run {
        info!("Removing files...");
        todo!("Removing files...");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::sha256sum;

    #[test]
    fn file_exists() {
        let path: &str = "test/test.txt";
        let result = sha256sum(Path::new(path)).unwrap();
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn file_does_not_exists() {
        let path: &str = "test/test2.txt";
        let result = sha256sum(Path::new(path));
        assert!(result.is_err());
    }
}
