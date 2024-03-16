pub use clap::{ArgAction, Parser, ValueEnum};
use clap::builder::PossibleValue;
use std::path::PathBuf;
use super::hasher::HashAlgorithm;


impl ValueEnum for HashAlgorithm {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::SHA2_256,
            Self::SHA3_256,
            Self::SHA1,
            Self::MD5,
            Self::WHIRLPOOL,
            Self::RIPEMD160,
            Self::BLAKE256,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::SHA2_256 => PossibleValue::new("SHA2-256"),
            Self::SHA3_256 => PossibleValue::new("SHA3-256"),
            Self::SHA1 => PossibleValue::new("SHA1"),
            Self::MD5 => PossibleValue::new("MD5"),
            Self::WHIRLPOOL => PossibleValue::new("WHIRLPOOL"),
            Self::RIPEMD160 => PossibleValue::new("RIPEMD-160"),
            Self::BLAKE256 => PossibleValue::new("BLAKE-256"),
        })
    }
}

/// Remove duplicated files in the reference directory that are found in the root directory tree.
#[derive(Parser)]
#[clap(author = "Manuel Amersdorfer", version)]
pub struct Cli {
    /// Reference directory path
    pub reference_dir: PathBuf,
    /// Root directory path
    pub root_dir: PathBuf,
    /// Perform a dry-run without removing any file
    #[clap(long, short='n', action(ArgAction::SetTrue))]
    pub dry_run: bool,
    /// Regular expression filtering files in reference directories
    #[clap(long, short)]
    pub regex: Option<String>,
    /// Hash algorithm
    #[clap(long, short='a', default_value="SHA2-256")]
    pub hash_algorithm: HashAlgorithm,
}
