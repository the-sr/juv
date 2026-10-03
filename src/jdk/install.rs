// install.rs - Handles extracting and setting up downloaded Java Development Kits.
// This module takes the downloaded archive and makes it ready to use.

use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

/// Install a downloaded Java Development Kit.
/// This extracts the archive and renames the folder to a standard name.
///
/// # Arguments
/// * `archive_path` - Path to the downloaded archive file.
/// * `install_dir` - The folder where Java versions are stored.
/// * `version` - The Java version being installed.
///
/// # Returns
/// The path to the installed Java folder.
pub fn install_jdk(archive_path: &str, install_dir: &str, version: &str) -> Result<String> {
    println!("{} Extracting Java {}...", "→".cyan(), version);

    // Create the install folder if it does not exist.
    fs::create_dir_all(install_dir)?;

    // Open the archive file.
    let file = fs::File::open(archive_path)
        .with_context(|| format!("Failed to open archive '{}'", archive_path))?;

    // Extract the tar.gz archive.
    let temp_dir = Path::new(install_dir).join(format!("temp-jdk-{}", version));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // Decompress the gzip file.
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    archive
        .unpack(&temp_dir)
        .context("Failed to extract the archive. The download may be corrupted.")?;

    // Find the extracted folder (it usually has a name like "jdk-17.0.9+9").
    let extracted_folder = find_extracted_folder(&temp_dir)?;

    // Rename to a standard name.
    let final_path = Path::new(install_dir).join(format!("jdk-{}", version));
    if final_path.exists() {
        fs::remove_dir_all(&final_path)?;
    }
    fs::rename(&extracted_folder, &final_path)?;

    // Clean up the temporary folder.
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    // Remove the downloaded archive to save space.
    fs::remove_file(archive_path).ok();

    println!(
        "{} Java {} installed to {}",
        "✓".green().bold(),
        version,
        final_path.display()
    );

    Ok(final_path.to_string_lossy().to_string())
}

/// Find the main folder inside an extracted archive.
/// Archives usually contain one top-level folder with the Java files.
fn find_extracted_folder(temp_dir: &Path) -> Result<std::path::PathBuf> {
    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            return Ok(entry.path());
        }
    }
    Err(anyhow::anyhow!(
        "Could not find the Java folder inside the archive."
    ))
}
