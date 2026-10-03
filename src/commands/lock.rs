// lock.rs - Handles the "juv lock" command.
// This creates a lock file that records the exact versions of all dependencies.
// The lock file makes sure everyone on the project uses the same versions.

use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::project::config::read_project_config;

/// Run the lock command.
/// This generates a lock file with exact dependency versions.
pub async fn run() -> Result<()> {
    println!("{} Generating lock file...", "→".cyan());

    // Read the project config.
    let config = read_project_config()?;
    let project_path = Path::new(".");

    // Find the build file.
    let build_file = if project_path.join("pom.xml").exists() {
        "pom.xml"
    } else if project_path.join("build.gradle").exists() {
        "build.gradle"
    } else {
        return Err(anyhow::anyhow!(
            "No build file found. Run 'juv init' first."
        ));
    };

    // Read the build file content.
    let build_content = fs::read_to_string(project_path.join(build_file))
        .with_context(|| format!("Failed to read {}", build_file))?;

    // Generate a simple lock file.
    // In a real tool, this would resolve exact versions from the build tool.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let lock_content = format!(
        "# juv lock file\n# This file records the exact versions of all dependencies.\n# Generated on: {} (unix timestamp)\n\n[project]\nname = \"{}\"\njava_version = \"{}\"\nbuild_tool = \"{}\"\n\n[dependencies]\n{}\n",
        now,
        config.project_name,
        config.java_version,
        config.build_tool,
        extract_dependencies(&build_content)
    );

    // Write the lock file.
    fs::write(project_path.join("juv.lock"), lock_content)
        .context("Failed to write juv.lock")?;

    println!(
        "{} Lock file created: {}",
        "✓".green().bold(),
        project_path.join("juv.lock").display()
    );

    Ok(())
}

/// Extract dependency lines from the build file content.
fn extract_dependencies(content: &str) -> String {
    content
        .lines()
        .filter(|line| {
            line.contains("implementation")
                || line.contains("testImplementation")
                || line.contains("<dependency>")
                || line.contains("<groupId>")
                || line.contains("<artifactId>")
        })
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
