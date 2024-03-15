pub use clap::{ArgAction, Parser};
use std::path::PathBuf;

/// Remove duplicated files in the reference directory that are found in the root directory tree.
#[derive(Parser)]
#[clap(author = "Manuel Amersdorfer", version)]
pub struct Cli {
    /// Reference directory path
    pub reference_dir: PathBuf,
    /// Root directory path
    pub root_dir: PathBuf,
    /// Perform a dry-run without removing any file
    #[clap(long, short, action(ArgAction::SetTrue))]
    pub dry_run: bool,
    /// Regular expression filtering files in reference directories
    #[clap(long, short)]
    pub regex: Option<String>,
}
