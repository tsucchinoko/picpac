use anyhow::{Context, Result};
use serde_json::Value;
use skim::prelude::*;
use std::fs;

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
pub fn get_scripts_from_package_json() -> Result<Vec<String>> {
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
pub fn select_script(scripts: &[String]) -> Result<String> {
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
