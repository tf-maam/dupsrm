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
use dupsrm::hasher::{is_empty_hash, sha256sum};
use dupsrm::logger::CONSOLE_LOGGER;
use dupsrm::path::{is_file, is_subdirectory};
use env_logger::Env;
use log::Level;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
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
