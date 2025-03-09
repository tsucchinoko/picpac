use anyhow::{Context, Result};
use std::{
    path::Path,
    process::{Command, Stdio},
};

/// Represents the supported package managers
#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Npm,
    Pnpm,
}

impl PackageManager {
    /// Returns the command string for this package manager
    pub fn command(&self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
        }
    }
}

/// Determines which package manager to use based on lock files.
///
/// Returns `PackageManager::Pnpm` if pnpm-lock.yaml exists, otherwise `PackageManager::Npm`.
pub fn determine_package_manager() -> PackageManager {
    if Path::new("pnpm-lock.yaml").exists() {
        PackageManager::Pnpm
    } else {
        PackageManager::Npm
    }
}

/// Runs the selected script with the appropriate package manager.
///
/// # Arguments
///
/// * `package_manager` - The package manager to use
/// * `script_name` - The name of the script to run
///
/// # Errors
///
/// Returns an error if the script execution fails to start
pub fn run_script(package_manager: PackageManager, script_name: &str) -> Result<()> {
    let cmd = package_manager.command();
    println!("Running: {} run {}", cmd, script_name);

    let status = Command::new(cmd)
        .args(["run", script_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to execute {} run {}", cmd, script_name))?;

    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
