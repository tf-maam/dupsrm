use blake::{self, Blake};
use data_encoding::HEXLOWER;
use md5::{self, Md5};
use ripemd::{self, Ripemd160};
use sha1::{self, Sha1};
use sha2::{Digest, Sha256};
use sha3::{self, Sha3_256};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use whirlpool::{self, Whirlpool};

/// Enumerates the hash algorithm
#[derive(Clone)]
pub enum HashAlgorithm {
    SHA2_256,  //< SHA256
    SHA3_256,  //< SHA3-256
    SHA1,      //< SHA1
    MD5,       //< MD5
    WHIRLPOOL, //< Whirlpool
    RIPEMD160, //< RIPEMD-160
    BLAKE256,  //< BLAKE-256
}

/// Hash a file and return its sha256 hash value
pub fn sha256sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
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
    digest.as_slice();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hash a file and return its SHA3-256 hash value
pub fn sha3_256sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let digest = {
        let mut hasher = Sha3_256::new();
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
    digest.as_slice();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hash a file and return its SHA1 hash value
pub fn sha1sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let digest = {
        let mut hasher = Sha1::new();
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
    digest.as_slice();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hash a file and return its MD5 hash value
pub fn md5sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let digest = {
        let mut hasher = Md5::new();
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
    digest.as_slice();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hash a file and return its Whirlpool hash value
pub fn whirlpool_sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let digest = {
        let mut hasher = Whirlpool::new();
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
    digest.as_slice();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hash a file and return its BLAKE-256 hash value
pub fn blake256_sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let mut hasher = Blake::new(256).unwrap();
    {
        let mut buffer = [0; 4098];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
    };
    let mut digest = [0; 32];
    hasher.finalise(&mut digest);
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Hash a file and return its RIPEMD-160 hash value
pub fn ripemd160_sum(path: &Path) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Err(err) => return Err(err),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let digest = {
        let mut hasher = Ripemd160::new();
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
    digest.as_slice();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

/// Checks if the string equals the empty hash
pub fn is_empty_hash(hash: &str, algorithm: &HashAlgorithm) -> bool {
    match algorithm {
        HashAlgorithm::SHA2_256 => hash.eq("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        HashAlgorithm::SHA3_256 =>hash.eq("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"),
        HashAlgorithm::SHA1 =>hash.eq("da39a3ee5e6b4b0d3255bfef95601890afd80709"),
        HashAlgorithm::MD5 =>hash.eq("d41d8cd98f00b204e9800998ecf8427e"),
        HashAlgorithm::WHIRLPOOL =>hash.eq("19fa61d75522a4669b44e39c1d2e1726c530232130d407f89afee0964997f7a73e83be698b288febcf88e3e03c4f0757ea8964e59b63d93708b138cc42a66eb3"),
        HashAlgorithm::RIPEMD160 =>hash.eq("9c1185a5c5e9fc54612808977ee8f548b2258d31"),
        HashAlgorithm::BLAKE256 =>hash.eq("716f6e863f744b9ac22c97ec7b76ea5f5908bc5b2f67c61510bfc4751384ea7a"),
    }
}
