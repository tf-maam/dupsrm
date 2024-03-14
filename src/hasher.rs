
use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// Hash a file and return its sha256 hash value
pub fn sha256sum(path: &Path) -> Result<String, io::Error> {
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



/// Checks if the string equals the empty hash
pub fn is_empty_hash(hash: &str) -> bool {
    hash.eq("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
}