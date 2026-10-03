// list.rs - Handles the "juv list" command.
// This shows all Java versions that are installed on this system.

use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::jdk::manager::get_project_jdk_dir;

/// Run the list command.
/// This prints all Java versions installed across all projects.
pub async fn run() -> Result<()> {
    // Look for all projects that have a .juv/jdk folder.
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let mut versions: Vec<String> = Vec::new();

    // Search for .juv/jdk folders in the home directory and /tmp.
    let search_dirs = vec![home, std::path::PathBuf::from("/tmp")];
    for dir in search_dirs {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let project_dir = entry.path();
                let jdk_dir = project_dir.join(".juv").join("jdk");
                if jdk_dir.exists() {
                    // Find all jdk-* folders inside .juv/jdk.
                    if let Ok(jdk_entries) = fs::read_dir(&jdk_dir) {
                        for jdk_entry in jdk_entries.flatten() {
                            if let Some(name) = jdk_entry.file_name().to_str() {
                                if name.starts_with("jdk-") {
                                    versions.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort versions so they appear in order.
    versions.sort();
    versions.dedup();

    if versions.is_empty() {
        println!("{} No Java versions installed yet.", "→".cyan());
        println!("{} Run 'juv init <project>' to install one.", "→".cyan());
    } else {
        println!("{} Installed Java versions:", "→".cyan());
        for version in versions {
            println!("  {} {}", "•".green(), version);
        }
    }

    Ok(())
}
