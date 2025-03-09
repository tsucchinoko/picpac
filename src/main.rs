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

fn main() -> Result<()> {
    let args: Args = Args::parse();

    // Change to the specified directory or use current directory
    if let Some(path) = &args.path {
        std::env::set_current_dir(path)
            .context(format!("Failed to change directory to {}", path))?;
    }

    // Check if package.json exists
    if !Path::new("package.json").exists() {
        eprintln!("Error: There is no package.json in the current directory");
        return Ok(());
    }

    // Determine package manager (npm or pnpm)
    let package_manager = if Path::new("pnpm-lock.yaml").exists() {
        "pnpm"
    } else {
        "npm"
    };

    // Read package.json
    let package_json = fs::read_to_string("package.json").context("Failed to read package.json")?;

    // Parse package.json
    let package_data: Value =
        serde_json::from_str(&package_json).context("Failed to parse package.json")?;

    // Extract scripts
    let scripts = match &package_data["scripts"] {
        Value::Object(scripts_obj) => {
            if scripts_obj.is_empty() {
                eprintln!("Error: There are no scripts in package.json");
                return Ok(());
            }

            scripts_obj
                .iter()
                .map(|(key, value)| format!("{} = {}", key, value.as_str().unwrap_or("")))
                .collect::<Vec<String>>()
        }
        _ => {
            eprintln!("Error: There are no scripts in package.json");
            return Ok(());
        }
    };

    // Use skim for fuzzy selection
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .reverse(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(std::io::Cursor::new(scripts.join("\n")));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    if selected_items.is_empty() {
        return Ok(());
    }

    // Extract the script name from the selected item
    let selected = selected_items[0].output();
    let script_name = selected.split(" = ").next().unwrap_or("");

    if script_name.is_empty() {
        return Ok(());
    }

    // Run the selected script
    println!("Running: {} run {}", package_manager, script_name);

    let status = Command::new(package_manager)
        .args(&["run", script_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context(format!(
            "Failed to execute {} run {}",
            package_manager, script_name
        ))?;

    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
