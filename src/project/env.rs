// env.rs - Handles setting up the environment for running a project.
// This module prepares environment variables and paths so the project
// can find its local Java installation and dependency cache.

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Set up the environment variables for running a project.
/// This prepares JAVA_HOME, PATH, and dependency cache paths.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
/// * `jdk_path` - The path to the local Java installation.
///
/// # Returns
/// A map of environment variables to set.
pub fn setup_env(project_path: &Path, jdk_path: &str) -> Result<HashMap<String, String>> {
    let mut env = HashMap::new();

    // Set JAVA_HOME to the local Java installation.
    env.insert("JAVA_HOME".to_string(), jdk_path.to_string());

    // Add the Java bin folder to the front of PATH.
    let java_bin = Path::new(jdk_path).join("bin");
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", java_bin.display(), current_path);
    env.insert("PATH".to_string(), new_path);

    // Set project-local dependency cache.
    let cache_dir = project_path.join(".juv").join("cache");
    env.insert(
        "GRADLE_USER_HOME".to_string(),
        cache_dir.join("gradle").to_string_lossy().to_string(),
    );
    env.insert(
        "M2_HOME".to_string(),
        cache_dir.join("maven").to_string_lossy().to_string(),
    );
    env.insert(
        "MAVEN_OPTS".to_string(),
        format!(
            "-Dmaven.repo.local={}",
            cache_dir
                .join("maven")
                .join("repository")
                .display()
        ),
    );

    Ok(env)
}

/// Check if the project environment is set up correctly.
/// This verifies that the local Java installation exists.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
///
/// # Returns
/// True if the environment is ready, false otherwise.
pub fn is_env_ready(project_path: &Path) -> bool {
    let jdk_path = project_path.join(".juv").join("jdk");
    jdk_path.exists()
}
