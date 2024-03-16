// Remove duplicated files found in a reference directory in a directory
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (Ctests::) 2024  Manuel Amersdorfer

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

use clap::Parser;
use dupsrm::cli::Cli;
use dupsrm::error::ArgumentError;
use dupsrm::hasher::{
    blake256_sum, is_empty_hash, md5sum, ripemd160_sum, sha1sum, sha256sum, sha3_256sum,
    whirlpool_sum, HashAlgorithm,
};
use dupsrm::logger::CONSOLE_LOGGER;
use dupsrm::path::{is_file, is_subdirectory};
use env_logger::Env;
use log::Level;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

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
    if reference_dir.eq(&root_dir) {
        error!("Reference directory must not be identical to root directory");
        return Err(ArgumentError::new(
            "Reference directory must not be identical to root directory",
        ));
    }

    // Formulate regex
    match &args.regex {
        Some(str) => info!("regex: \'{}\'", str),
        None => info!("regex: \"\""),
    }

    let regex: Option<Regex> = args
        .regex
        .map(|re_str| Regex::new(re_str.as_str()).unwrap());

    // Choose hash function
    let hash_sum = match args.hash_algorithm {
        HashAlgorithm::SHA2_256 => |path: &Path| sha256sum(path),
        HashAlgorithm::SHA3_256 => |path: &Path| sha3_256sum(path),
        HashAlgorithm::SHA1 => |path: &Path| sha1sum(path),
        HashAlgorithm::MD5 => |path: &Path| md5sum(path),
        HashAlgorithm::WHIRLPOOL => |path: &Path| whirlpool_sum(path),
        HashAlgorithm::RIPEMD160 => |path: &Path| ripemd160_sum(path),
        HashAlgorithm::BLAKE256 => |path: &Path| blake256_sum(path),
    };

    // Calculate list of hashes for the root directory tree
    let root_dirs: Vec<DirEntry> = WalkDir::new(root_dir.clone())
        .into_iter()
        .filter_entry(|e| !is_subdirectory(&e.clone().into_path(), &reference_dir))
        .filter_map(|v| v.ok())
        .collect();
    let root_files: Vec<DirEntry> = root_dirs.into_par_iter().filter(is_file).collect();

    let root_pairs: Vec<(Vec<u8>, PathBuf)> = root_files
        .into_par_iter()
        .map(|e| {
            (
                hash_sum(e.path()).unwrap(),
                fs::canonicalize(e.path()).unwrap(),
            )
        })
        .filter(|pair| !is_empty_hash(&pair.0, &args.hash_algorithm))
        .collect();

    // Calculate list of hashes for the reference directory tree
    let reference_dirs: Vec<DirEntry> = WalkDir::new(reference_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();
    let reference_files: Vec<DirEntry> = reference_dirs.into_par_iter().filter(is_file).collect();

    let reference_files: Vec<DirEntry> = reference_files
        .into_par_iter()
        .filter(|path| match &regex {
            Some(re) => re.is_match(path.path().to_str().unwrap_or("")),
            None => true,
        })
        .collect();

    let reference_pairs: Vec<(Vec<u8>, PathBuf)> = reference_files
        .into_par_iter()
        .map(|e| {
            (
                hash_sum(e.path()).unwrap(),
                fs::canonicalize(e.path()).unwrap(),
            )
        })
        .filter(|pair| !is_empty_hash(&pair.0, &args.hash_algorithm))
        .collect();

    // Find duplicates
    debug!("Check for duplicates");
    let root_hashes: Vec<Vec<u8>> = root_pairs.into_par_iter().map(|p| p.0).collect();
    let mut duplicate_pairs: Vec<(Vec<u8>, PathBuf)> = reference_pairs
        .into_par_iter()
        .filter(|pair| root_hashes.contains(&pair.0))
        .collect();
    duplicate_pairs.sort_by(|a, b| a.1.cmp(&b.1));

    if duplicate_pairs.is_empty() {
        info!("No duplicates found");
        return Ok(());
    }

    if !args.dry_run {
        duplicate_pairs
            .par_iter()
            .for_each(|pair| match fs::remove_file(&pair.1) {
                Ok(()) => info!("Removed file {}", pair.1.to_str().unwrap()),
                Err(err) => error!("Removing file {} failed: {}", pair.1.to_str().unwrap(), err),
            });
    } else {
        duplicate_pairs
            .into_par_iter()
            .for_each(|s| info!("Found {}", s.1.to_str().unwrap()));
    }

    Ok(())
}
