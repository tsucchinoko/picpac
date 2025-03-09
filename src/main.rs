use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use skim::prelude::*;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

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
    let args: Args = Args::parse();

    // Change to the specified directory or use current directory
    if let Some(path) = &args.path {
        change_directory(path)?;
    }

    // Check if package.json exists
    if !Path::new("package.json").exists() {
        eprintln!("Error: There is no package.json in the current directory");
        return Ok(());
    }

    // Determine package manager and get scripts
    let package_manager = determine_package_manager();
    let scripts = get_scripts_from_package_json()?;

    if scripts.is_empty() {
        eprintln!("Error: There are no scripts in package.json");
        return Ok(());
    }

    // Select script using fuzzy finder
    let script_name = select_script(&scripts)?;
    if script_name.is_empty() {
        return Ok(());
    }

    // Run the selected script
    run_script(&package_manager, &script_name)?;

    Ok(())
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
fn change_directory(path: &str) -> Result<()> {
    std::env::set_current_dir(path)
        .with_context(|| format!("Failed to change directory to {}", path))
}

/// Determines which package manager to use based on lock files.
///
/// Returns "pnpm" if pnpm-lock.yaml exists, otherwise "npm".
fn determine_package_manager() -> String {
    if Path::new("pnpm-lock.yaml").exists() {
        String::from("pnpm")
    } else {
        String::from("npm")
    }
}

/// Extracts scripts from package.json file.
///
/// # Returns
///
/// A vector of strings in the format "script_name = script_command"
///
/// # Errors
///
/// Returns an error if:
/// - package.json cannot be read
/// - package.json cannot be parsed as valid JSON
fn get_scripts_from_package_json() -> Result<Vec<String>> {
    // Read package.json
    let package_json = fs::read_to_string("package.json").context("Failed to read package.json")?;

    // Parse package.json
    let package_data: Value =
        serde_json::from_str(&package_json).context("Failed to parse package.json")?;

    // Extract scripts
    match &package_data["scripts"] {
        Value::Object(scripts_obj) => {
            if scripts_obj.is_empty() {
                return Ok(Vec::new());
            }

            Ok(scripts_obj
                .iter()
                .map(|(key, value)| format!("{} = {}", key, value.as_str().unwrap_or("")))
                .collect())
        }
        _ => Ok(Vec::new()),
    }
}

/// Presents a fuzzy finder interface for script selection.
///
/// # Arguments
///
/// * `scripts` - A slice of strings containing script names and commands
///
/// # Returns
///
/// The selected script name or an empty string if no selection was made
///
/// # Errors
///
/// Returns an error if the fuzzy finder fails to run
fn select_script(scripts: &[String]) -> Result<String> {
    // Use skim for fuzzy selection
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .reverse(true)
        .build()
        .context("Failed to build skim options")?;

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(std::io::Cursor::new(scripts.join("\n")));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    if selected_items.is_empty() {
        return Ok(String::new());
    }

    // Extract the script name from the selected item
    let selected = selected_items[0].output();
    let script_name = selected.split('=').next().unwrap_or("").trim();

    Ok(script_name.to_string())
}

/// Runs the selected script with the appropriate package manager.
///
/// # Arguments
///
/// * `package_manager` - The package manager to use ("npm" or "pnpm")
/// * `script_name` - The name of the script to run
///
/// # Errors
///
/// Returns an error if the script execution fails to start
fn run_script(package_manager: &str, script_name: &str) -> Result<()> {
    println!("Running: {} run {}", package_manager, script_name);

    let status = Command::new(package_manager)
        .args(["run", script_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to execute {} run {}", package_manager, script_name))?;

    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
