mod cli;
mod package_manager;
mod scripts;

use anyhow::Result;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the project directory (defaults to current directory)
    #[arg(short, long)]
    path: Option<String>,
}

/// Main function that orchestrates the script selection and execution process.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to change directory
/// - Failed to read or parse package.json
/// - Failed to execute the selected script
fn main() -> Result<()> {
    let args = cli::parse_args();

    // Change to the specified directory or use current directory
    if let Some(path) = &args.path {
        cli::change_directory(path)?;
    }

    // Check if package.json exists
    if !Path::new("package.json").exists() {
        eprintln!("Error: There is no package.json in the current directory");
        return Ok(());
    }

    // Determine package manager and get scripts
    let package_manager = package_manager::determine_package_manager();
    let scripts = scripts::get_scripts_from_package_json()?;

    if scripts.is_empty() {
        eprintln!("Error: There are no scripts in package.json");
        return Ok(());
    }

    // Select script using fuzzy finder
    let script_name = scripts::select_script(&scripts)?;
    if script_name.is_empty() {
        return Ok(());
    }

    // Run the selected script
    package_manager::run_script(package_manager, &script_name)?;

    Ok(())
}
