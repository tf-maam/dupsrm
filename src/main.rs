use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, BufReader, Read};
use sha2::{Sha256, Digest};
use data_encoding::HEXLOWER;
use log::{info, error};
use log::{Metadata, Level, Record, LevelFilter};


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
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 { break }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(HEXLOWER.encode(digest.as_ref()))
}

fn main() {
    let _ = log::set_logger(&CONSOLE_LOGGER);
    log::set_max_level(LevelFilter::Info);
    
    let path : &str = "test.txt";
    match sha256sum(Path::new(path)){
        Err(err) => error!("{}: {}", err, path),
        Ok(hash_str) => info!("{} {}", hash_str, path)
    }
}
