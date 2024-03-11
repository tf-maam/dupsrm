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


use data_encoding::HEXLOWER;
use log::{error, info};
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

fn main() {
    let _ = log::set_logger(&CONSOLE_LOGGER);
    log::set_max_level(LevelFilter::Info);

    let path: &str = "test.txt";
    match sha256sum(Path::new(path)) {
        Err(err) => error!("{}: {}", err, path),
        Ok(hash_str) => info!("{} {}", hash_str, path),
    }

    let root_dir = Path::new(".").canonicalize().unwrap();
    let reference_dir = Path::new("target").canonicalize().unwrap();
    println!("{}", reference_dir.to_str().unwrap());
    assert!(reference_dir
        .to_str()
        .unwrap()
        .starts_with(root_dir.to_str().unwrap()));
    assert!(!root_dir
        .to_str()
        .unwrap()
        .starts_with(reference_dir.to_str().unwrap()));
    assert!(reference_dir.is_dir() == true);
    assert!(is_subdirectory(&reference_dir, &root_dir));

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
        .filter(|pair | !is_empty_hash(pair.0.as_str()))
        .collect();

    let reference_dirs: Vec<DirEntry> = WalkDir::new(reference_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();
    let reference_files: Vec<DirEntry> = reference_dirs
        .into_iter()
        .filter(|e| is_file(e))
        .collect();

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
        .filter(|pair | !is_empty_hash(pair.0.as_str()))
        .collect();

    println!("Check for duplicates");
    let reference_hashes: Vec<String> = reference_pairs.into_iter().map(|p| p.0).collect();
    let duplicate_pairs : Vec<(String, String)> = root_pairs
        .into_iter()
        .filter(|pair| reference_hashes.contains(&pair.0))
        .collect();

    if duplicate_pairs.len() == 0 {
        println!("No duplicates found")
    }

    duplicate_pairs
        .into_iter()
        .for_each(|s| println!("{} {}", s.0, s.1));
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
