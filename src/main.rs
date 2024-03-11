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

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn is_subdirectory(entry: &PathBuf, reference: &PathBuf) -> bool {
    entry
        .to_str()
        .unwrap()
        .starts_with(reference.to_str().unwrap())
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

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

    let dir_list: Vec<DirEntry> = WalkDir::new(root_dir.clone())
        .into_iter()
        .filter_entry(|e| !is_subdirectory(&e.clone().into_path(), &reference_dir))
        .filter_map(|v| v.ok())
        .collect();
    let file_list: Vec<DirEntry> = dir_list.into_iter().filter(|e| is_file(e)).collect();

    let hash_list: Vec<(String, String)> = file_list
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
    // hash_list
    //     .into_iter()
    //     .for_each(|s| println!("{} {}", s.0, s.1));

    let reference_dir_list: Vec<DirEntry> = WalkDir::new(reference_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();
    let reference_file_list: Vec<DirEntry> = reference_dir_list
        .into_iter()
        .filter(|e| is_file(e))
        .collect();

    let reference_hash_list: Vec<(String, String)> = reference_file_list
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
    let reference_hashes: Vec<String> = reference_hash_list.into_iter().map(|p| p.0).collect();
    let duplicates_list : Vec<(String, String)> = hash_list
        .into_iter()
        .filter(|pair| reference_hashes.contains(&pair.0))
        .collect();

    if duplicates_list.len() == 0 {
        println!("No duplicates found")
    }

    duplicates_list
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
