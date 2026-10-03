// use_jdk.rs - Handles the "juv use" command.
// This changes which Java version the current project uses.

use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::jdk::download::download_jdk;
use crate::jdk::install::install_jdk;
use crate::jdk::manager::get_project_jdk_dir;
use crate::project::config::read_project_config;

/// Run the use command.
/// This switches the project to use a different Java version.
///
/// # Arguments
/// * `version` - The Java version to switch to (e.g., "17", "21").
pub async fn run(version: &str) -> Result<()> {
    println!("{} Switching to Java {}...", "→".cyan(), version);

    // Read the current project config.
    let mut config = read_project_config()?;

    // Get the project's JDK folder.
    let jdk_dir = get_project_jdk_dir(".");

    // Check if the requested version is already installed.
    let jdk_path = jdk_dir.join(format!("jdk-{}", version));
    if !jdk_path.exists() {
        // Download and install the requested version.
        println!("{} Downloading Java {}...", "→".cyan(), version);
        let downloaded_path = download_jdk(version, jdk_dir.to_str().unwrap()).await?;

        println!("{} Installing Java {}...", "→".cyan(), version);
        install_jdk(&downloaded_path, jdk_dir.to_str().unwrap(), version)?;
    }

    // Remove the old JDK folder.
    let old_jdk = jdk_dir.join(format!("jdk-{}", config.java_version));
    if old_jdk.exists() && old_jdk != jdk_path {
        fs::remove_dir_all(&old_jdk)?;
    }

    // Update the config file.
    config.java_version = version.to_string();
    config.jdk_path = jdk_path.to_string_lossy().to_string();
    config.save()?;

    println!(
        "{} Now using Java {} in project '{}'",
        "✓".green().bold(),
        version,
        config.project_name
    );

    Ok(())
}
