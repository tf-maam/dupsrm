use blake::{self, Blake};
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
pub fn sha256sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
    // Ok(HEXLOWER.decode(HEXLOWER.encode(digest.as_ref())).unwrap())
}

/// Hash a file and return its SHA3-256 hash value
pub fn sha3_256sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
}

/// Hash a file and return its SHA1 hash value
pub fn sha1sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
}

/// Hash a file and return its MD5 hash value
pub fn md5sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
}

/// Hash a file and return its Whirlpool hash value
pub fn whirlpool_sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
}

/// Hash a file and return its BLAKE-256 hash value
pub fn blake256_sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
}

/// Hash a file and return its RIPEMD-160 hash value
pub fn ripemd160_sum(path: &Path) -> Result<Vec<u8>, io::Error> {
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
    Ok(digest.to_vec())
}

/// Checks if the string equals the empty hash
pub fn is_empty_hash(hash: &Vec<u8>, algorithm: &HashAlgorithm) -> bool {
    match algorithm {
        // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        HashAlgorithm::SHA2_256 => {
            hash.to_owned()
                == vec![
                    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99,
                    0x6f, 0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95,
                    0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
                ]
        }
        // a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a
        HashAlgorithm::SHA3_256 => {
            hash.to_owned()
                == vec![
                    0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0,
                    0x61, 0xd6, 0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8,
                    0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a,
                ]
        }
        // da39a3ee5e6b4b0d3255bfef95601890afd80709
        HashAlgorithm::SHA1 => {
            hash.to_owned()
                == vec![
                    0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d, 0x32, 0x55, 0xbf, 0xef, 0x95,
                    0x60, 0x18, 0x90, 0xaf, 0xd8, 0x07, 0x09,
                ]
        }
        // d41d8cd98f00b204e9800998ecf8427e
        HashAlgorithm::MD5 => {
            hash.to_owned()
                == vec![
                    0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec,
                    0xf8, 0x42, 0x7e,
                ]
        }
        // 19fa61d75522a4669b44e39c1d2e1726c530232130d407f89afee0964997f7a73e83be698b288febcf88e3e03c4f0757ea8964e59b63d93708b138cc42a66eb3
        HashAlgorithm::WHIRLPOOL => {
            hash.to_owned()
                == vec![
                    0x19, 0xfa, 0x61, 0xd7, 0x55, 0x22, 0xa4, 0x66, 0x9b, 0x44, 0xe3, 0x9c, 0x1d,
                    0x2e, 0x17, 0x26, 0xc5, 0x30, 0x23, 0x21, 0x30, 0xd4, 0x07, 0xf8, 0x9a, 0xfe,
                    0xe0, 0x96, 0x49, 0x97, 0xf7, 0xa7, 0x3e, 0x83, 0xbe, 0x69, 0x8b, 0x28, 0x8f,
                    0xeb, 0xcf, 0x88, 0xe3, 0xe0, 0x3c, 0x4f, 0x07, 0x57, 0xea, 0x89, 0x64, 0xe5,
                    0x9b, 0x63, 0xd9, 0x37, 0x08, 0xb1, 0x38, 0xcc, 0x42, 0xa6, 0x6e, 0xb3,
                ]
        }
        // 9c1185a5c5e9fc54612808977ee8f548b2258d31
        HashAlgorithm::RIPEMD160 => {
            hash.to_owned()
                == vec![
                    0x9c, 0x11, 0x85, 0xa5, 0xc5, 0xe9, 0xfc, 0x54, 0x61, 0x28, 0x08, 0x97, 0x7e,
                    0xe8, 0xf5, 0x48, 0xb2, 0x25, 0x8d, 0x31,
                ]
        }
        // 716f6e863f744b9ac22c97ec7b76ea5f5908bc5b2f67c61510bfc4751384ea7a
        HashAlgorithm::BLAKE256 => {
            hash.to_owned()
                == vec![
                    0x71, 0x6f, 0x6e, 0x86, 0x3f, 0x74, 0x4b, 0x9a, 0xc2, 0x2c, 0x97, 0xec, 0x7b,
                    0x76, 0xea, 0x5f, 0x59, 0x08, 0xbc, 0x5b, 0x2f, 0x67, 0xc6, 0x15, 0x10, 0xbf,
                    0xc4, 0x75, 0x13, 0x84, 0xea, 0x7a,
                ]
        }
    }
}
