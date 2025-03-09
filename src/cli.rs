use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the project directory (defaults to current directory)
    #[arg(short, long)]
    pub path: Option<String>,
}

/// Parse command line arguments
pub fn parse_args() -> Args {
    Args::parse()
}

/// Changes the current working directory to the specified path.
///
/// # Arguments
///
/// * `path` - The path to change to
///
/// # Errors
///
/// Returns an error if the directory change fails
pub fn change_directory(path: &str) -> Result<()> {
    std::env::set_current_dir(path)
        .with_context(|| format!("Failed to change directory to {}", path))
}
