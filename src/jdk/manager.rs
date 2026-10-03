// manager.rs - Handles finding and managing Java installations inside a project.
// Each project has its own JDK inside .juv/jdk/.

use std::path::PathBuf;

/// Get the folder where the JDK is stored inside a project.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
///
/// # Returns
/// The path to the JDK folder inside the project.
pub fn get_project_jdk_dir(project_path: &str) -> PathBuf {
    PathBuf::from(project_path).join(".juv").join("jdk")
}

/// Get the path to the Java binary inside a project.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
///
/// # Returns
/// The path to the java executable.
pub fn get_project_java_binary(project_path: &str) -> PathBuf {
    get_project_jdk_dir(project_path)
        .join("bin")
        .join("java")
}

/// Check if a project has a JDK installed.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
///
/// # Returns
/// True if the project has a JDK, false otherwise.
pub fn has_project_jdk(project_path: &str) -> bool {
    get_project_java_binary(project_path).exists()
}
