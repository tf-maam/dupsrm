use std::path::Path;
use walkdir::DirEntry;

/// Check if the path is a subdirectory of the reference path
pub fn is_subdirectory(entry: &Path, reference: &Path) -> bool {
    entry
        .to_str()
        .unwrap()
        .starts_with(reference.to_str().unwrap())
}

/// Check if directory entry is a file
pub fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}
