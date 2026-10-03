// config.rs - Handles reading and writing project configuration files.
// Each project has a juv.toml file that records its settings.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// The project configuration structure.
/// This stores all settings for a project.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    /// The name of the project.
    pub project_name: String,
    /// The Java version this project uses.
    pub java_version: String,
    /// The build tool used (gradle or maven).
    pub build_tool: String,
    /// The path to the local Java installation.
    pub jdk_path: String,
}

/// Read the project configuration from the current folder.
///
/// # Returns
/// The project configuration.
pub fn read_project_config() -> Result<ProjectConfig> {
    // Look for the config file in the current folder.
    let config_path = Path::new(".juv").join("juv.toml");

    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "No project found. Run 'juv init <name>' to create one."
        ));
    }

    // Read and parse the config file.
    let content = fs::read_to_string(&config_path)
        .context("Failed to read project config. The file may be corrupted.")?;

    let config: ProjectConfig =
        toml::from_str(&content).context("Failed to parse project config.")?;

    Ok(config)
}

/// Create a new project configuration file.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
/// * `java_version` - The Java version to use.
/// * `jdk_path` - The path to the local Java installation.
pub fn create_project_config(
    project_path: &Path,
    java_version: &str,
    jdk_path: &str,
) -> Result<()> {
    // Create the .juv folder inside the project.
    let juv_dir = project_path.join(".juv");
    fs::create_dir_all(&juv_dir)?;

    // Detect which build tool to use based on existing files.
    let build_tool = if project_path.join("pom.xml").exists() {
        "maven"
    } else if project_path.join("build.gradle").exists()
        || project_path.join("build.gradle.kts").exists()
    {
        "gradle"
    } else {
        "maven"
    };

    // Create the config.
    let config = ProjectConfig {
        project_name: project_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        java_version: java_version.to_string(),
        build_tool: build_tool.to_string(),
        jdk_path: jdk_path.to_string(),
    };

    // Write the config file.
    let config_path = juv_dir.join("juv.toml");
    let content = toml::to_string(&config).context("Failed to serialize config.")?;

    fs::write(&config_path, content)
        .with_context(|| format!("Failed to write config to '{}'", config_path.display()))?;

    Ok(())
}

impl ProjectConfig {
    /// Save the config back to the project folder.
    /// Uses current directory since we are inside the project.
    pub fn save(&self) -> Result<()> {
        let config_path = Path::new(".").join(".juv").join("juv.toml");

        let content = toml::to_string(self).context("Failed to serialize config.")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config to '{}'", config_path.display()))?;

        Ok(())
    }
}
