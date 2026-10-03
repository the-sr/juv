// download.rs - Handles downloading Java Development Kits from the internet.
// This module fetches JDK files from the Adoptium API (a free Java distribution).

use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Download a Java Development Kit of the specified version.
///
/// # Arguments
/// * `version` - The Java version to download (e.g., "17", "21").
/// * `download_dir` - The folder where the JDK will be saved.
///
/// # Returns
/// The path to the downloaded file.
pub async fn download_jdk(version: &str, download_dir: &str) -> Result<String> {
    // Build the API URL for the Adoptium service.
    // This service provides free Java distributions for Linux, macOS, and Windows.
    let url = format!(
        "https://api.adoptium.net/v3/binary/latest/{}/ga/linux/x64/jdk/hotspot/normal/eclipse",
        version
    );

    println!("{} Fetching download link...", "→".cyan());

    // Ask the Adoptium API for the download link.
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to connect to the download server. Check your internet connection.")?;

    // Check if the server returned an error.
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Could not find Java {}. Try a different version (e.g., 11, 17, 21).",
            version
        ));
    }

    // Get the final download URL from the redirect.
    let download_url = response.url().to_string();

    // Download the file with a progress bar.
    println!("{} Downloading Java {}...", "→".cyan(), version);

    let mut response = client
        .get(&download_url)
        .send()
        .await
        .context("Failed to start the download.")?;

    // Get the total file size for the progress bar.
    let total_size = response.content_length().unwrap_or(0);

    // Create the download folder if it does not exist.
    std::fs::create_dir_all(download_dir)?;

    // Create the output file.
    let file_name = format!("jdk-{}.tar.gz", version);
    let file_path = Path::new(download_dir).join(&file_name);
    let mut file = File::create(&file_path)
        .with_context(|| format!("Failed to create file '{}'", file_path.display()))?;

    // Set up a progress bar.
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    // Download the file in chunks and write to disk.
    let mut downloaded: u64 = 0;
    while let Some(chunk) = response.chunk().await.context("Download failed.")? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    Ok(file_path.to_string_lossy().to_string())
}
