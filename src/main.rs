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
use env_logger::Env;
use log::{debug, error, info, warn};
use log::{Level, Metadata, Record};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
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
#[clap(author = "Manuel Amersdorfer", version)]
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
    env_logger::Builder::from_env(Env::default().default_filter_or(Level::Info.as_str()))
        .format_timestamp(None)
        .init();
    let _ = log::set_logger(&CONSOLE_LOGGER);

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
    let root_files: Vec<DirEntry> = root_dirs.into_par_iter().filter(|e| is_file(e)).collect();

    let root_pairs: Vec<(String, String)> = root_files
        .into_par_iter()
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
    let reference_files: Vec<DirEntry> = reference_dirs
        .into_par_iter()
        .filter(|e| is_file(e))
        .collect();

    let reference_pairs: Vec<(String, String)> = reference_files
        .into_par_iter()
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
    debug!("Check for duplicates");
    let root_hashes: Vec<String> = root_pairs.into_par_iter().map(|p| p.0).collect();
    let mut duplicate_pairs: Vec<(String, String)> = reference_pairs
        .into_par_iter()
        .filter(|pair| root_hashes.contains(&pair.0))
        .collect();
    duplicate_pairs.sort_by(|a, b| a.1.cmp(&b.1));

    if duplicate_pairs.len() == 0 {
        info!("No duplicates found");
        return Ok(());
    }

    if !args.dry_run {
        duplicate_pairs
            .par_iter()
            .for_each(|pair| match fs::remove_file(&pair.1) {
                Ok(()) => info!("Removed file {}", pair.1),
                Err(err) => error!("Removing file {} failed: {}", pair.1, err),
            });
    } else {
        duplicate_pairs
            .into_par_iter()
            .for_each(|s| info!("Found {}", s.1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*;
    use std::process::Command; // Used for writing assertions
    use std::fs;
    use std::{
        io::Write,
        path::{Path, PathBuf},
    };


    pub struct CliTestCase {
        pub root_dir_path: PathBuf,
        pub reference_dir_path: PathBuf,
        pub file_path_1: PathBuf,
        pub file_path_2: PathBuf,
    }

    trait CliTest {
        fn new() -> Self;
        fn startup(&self);
        fn teardown(&self);
    }

    impl CliTest for CliTestCase {
        fn new() -> Self {
            let root_dir_path = PathBuf::from("./test/test_root/");
            let reference_dir_path = PathBuf::from("./test/test_reference/");
            fs::create_dir(&reference_dir_path).unwrap_or(());
            let k = 6;
            let file_path_1: PathBuf = reference_dir_path.clone().join(format!("file_test_{}.txt", k));
            let k = 9;
            let file_path_2: PathBuf = reference_dir_path.join(format!("file_test_{}.txt", k));

            CliTestCase {
                root_dir_path: root_dir_path,
                reference_dir_path: reference_dir_path,
                file_path_1: file_path_1,
                file_path_2: file_path_2,
            }
        }

        fn startup(&self) {
            // Create file structure
            fs::create_dir(&self.root_dir_path).unwrap_or(());
            for i in 0..10 {
                let dir_path: PathBuf = self.root_dir_path.join(format!("dir_{}/", i)).to_owned();
                fs::create_dir(&dir_path).unwrap_or(());
                for j in 0..10 {
                    let file_path: PathBuf = dir_path.join(format!("file_{}.txt", j));
                    let mut file = fs::File::create(&file_path).unwrap();
                    let _ = file.write_all(format!("test {} {}", i, j).as_bytes());
                }
            }
            fs::create_dir(&self.reference_dir_path).unwrap_or(());
            let i = 5;
            let j = 2;
            let mut file = fs::File::create(&self.file_path_1).unwrap();
            let _ = file.write_all(format!("test {} {}", i, j).as_bytes());
            let i = 50;
            let j = 200;
            let mut file = fs::File::create(&self.file_path_2).unwrap();
            let _ = file.write_all(format!("test {} {}", i, j).as_bytes());
        }

        fn teardown(&self) {
            // Cleanup
            std::fs::remove_dir_all(&self.root_dir_path).unwrap_or(());
            std::fs::remove_dir_all(&self.reference_dir_path).unwrap_or(());
            assert!(!self.root_dir_path.exists());
            assert!(!self.reference_dir_path.exists());
        }
    }

    #[test]
    fn empty_file_exists() {
        let path: &str = "test/test_empty.txt";
        let result = sha256sum(Path::new(path)).unwrap();
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn file_exists() {
        let path: &str = "test/test.txt";
        let result = sha256sum(Path::new(path)).unwrap();
        assert_eq!(
            result,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn file_does_not_exists() {
        let path: &str = "test/test2.txt";
        let result = sha256sum(Path::new(path));
        assert!(result.is_err());
    }

    #[test]
    // #[ignore]
    #[serial]
    fn dry_run() {
        let test_case = CliTestCase::new();
        test_case.startup();

        // Check prerequisites
        assert!(test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());
        assert!(test_case.root_dir_path.exists());
        assert!(test_case.reference_dir_path.exists());

        // Execute program
        let mut cmd = match Command::cargo_bin("dupsrm") {
            Err(err) => panic!("{}", err),
            Ok(cmd) => cmd,
        };
        cmd.arg(&test_case.reference_dir_path)
            .arg(&test_case.root_dir_path)
            .arg("-d");
        cmd.assert().success();

        // Check results
        assert!(test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());

        test_case.teardown();
    }

    #[test]
    #[serial]
    fn duplicates_removed() {
        let test_case = CliTestCase::new();
        test_case.startup();

        // Check prerequisites
        assert!(test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());
        assert!(test_case.root_dir_path.exists());
        assert!(test_case.reference_dir_path.exists());

        // Execute program
        let mut cmd = match Command::cargo_bin("dupsrm") {
            Err(err) => panic!("{}", err),
            Ok(cmd) => cmd,
        };
        cmd.arg(&test_case.reference_dir_path).arg(&test_case.root_dir_path);
        cmd.assert().success();

        // Check results
        assert!(!test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());

        test_case.teardown();
    }

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("dupsrm")?;

        cmd.arg("./dsdfgdf").arg(".");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
    }
}
