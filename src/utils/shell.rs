// shell.rs - Utility functions for running shell commands.
// These helpers run external programs and handle their output.

use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// Run a shell command and wait for it to finish.
///
/// # Arguments
/// * `command` - The command to run (e.g., "ls", "git").
/// * `args` - The arguments to pass to the command.
/// * `cwd` - The folder where the command should run.
///
/// # Returns
/// True if the command succeeded, false otherwise.
pub fn run_command(command: &str, args: &[&str], cwd: &str) -> Result<bool> {
    let output = Command::new(command)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| format!("Failed to run command '{}'", command))?;

    Ok(output.status.success())
}

/// Run a shell command and capture its output.
///
/// # Arguments
/// * `command` - The command to run.
/// * `args` - The arguments to pass to the command.
///
/// # Returns
/// The command output as a string.
pub fn run_command_output(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .with_context(|| format!("Failed to run command '{}'", command))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Command '{}' failed with code {}",
            command,
            output.status.code().unwrap_or(-1)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
