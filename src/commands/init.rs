// init.rs - Handles the "juv init" command.
// This creates a new project folder, downloads the requested Java version,
// and sets up the project to use that Java version.

use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::jdk::download::download_jdk;
use crate::jdk::install::install_jdk;
use crate::jdk::manager::get_project_jdk_dir;
use crate::project::config::create_project_config;

/// Run the init command.
/// This creates a new project with a local Java installation.
///
/// # Arguments
/// * `name` - The name of the project folder to create.
/// * `java_version` - The Java version to install (e.g., "17", "21").
pub async fn run(name: &str, java_version: &str) -> Result<()> {
    println!("{} Creating project '{}'...", "→".cyan(), name);

    // Create the project folder.
    let project_path = Path::new(name);
    if project_path.exists() {
        return Err(anyhow::anyhow!(
            "Folder '{}' already exists. Choose a different name.",
            name
        ));
    }
    fs::create_dir_all(project_path)
        .with_context(|| format!("Failed to create folder '{}'", name))?;

    // Create a standard project structure.
    println!("{} Setting up project structure...", "→".cyan());
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("src/main"))?;
    fs::create_dir_all(project_path.join("src/main/java"))?;
    fs::create_dir_all(project_path.join("src/test"))?;
    fs::create_dir_all(project_path.join("src/test/java"))?;

    // Create the .juv folder inside the project.
    let juv_dir = project_path.join(".juv");
    fs::create_dir_all(&juv_dir)?;

    // Download and install the requested Java version inside the project.
    println!(
        "{} Downloading Java {}...",
        "→".cyan(),
        java_version
    );
    let jdk_dir = get_project_jdk_dir(name);
    let downloaded_path = download_jdk(java_version, jdk_dir.to_str().unwrap()).await?;

    println!("{} Installing Java {}...", "→".cyan(), java_version);
    let installed_path = install_jdk(&downloaded_path, jdk_dir.to_str().unwrap(), java_version)?;

    // Create the project config file that records which Java version to use.
    create_project_config(project_path, java_version, &installed_path)?;

    println!(
        "{} Project '{}' created with Java {}!",
        "✓".green().bold(),
        name,
        java_version
    );
    println!(
        "{} Run 'cd {}' and 'juv run' to start.",
        "→".cyan(),
        name
    );

    Ok(())
}
