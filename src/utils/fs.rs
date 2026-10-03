// fs.rs - Utility functions for working with files and folders.
// These are small helpers used across the project.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Check if a file or folder exists.
///
/// # Arguments
/// * `path` - The path to check.
///
/// # Returns
/// True if the path exists, false otherwise.
pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Create a folder and all its parent folders if they do not exist.
///
/// # Arguments
/// * `path` - The folder path to create.
pub fn create_dir_all(path: &str) -> Result<()> {
    fs::create_dir_all(path).with_context(|| format!("Failed to create folder '{}'", path))?;
    Ok(())
}

/// Remove a file or folder and everything inside it.
///
/// # Arguments
/// * `path` - The path to remove.
pub fn remove_all(path: &str) -> Result<()> {
    let p = Path::new(path);
    if p.is_dir() {
        fs::remove_dir_all(p).with_context(|| format!("Failed to remove folder '{}'", path))?;
    } else if p.exists() {
        fs::remove_file(p).with_context(|| format!("Failed to remove file '{}'", path))?;
    }
    Ok(())
}

/// Get the size of a file in bytes.
///
/// # Arguments
/// * `path` - The file path.
///
/// # Returns
/// The file size in bytes, or 0 if the file does not exist.
pub fn file_size(path: &str) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}
