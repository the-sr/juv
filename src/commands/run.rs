// run.rs - Handles the "juv run" command.
// If the project has a build file, run the build tool.
// If the project has no build file, run java with downloaded libraries.

use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::project::config::read_project_config;

/// Find all .java files in the project folder.
fn find_java_files(dir: &Path) -> Result<Vec<String>> {
    let mut java_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Skip the .juv folder.
            if path.file_name().and_then(|n| n.to_str()) != Some(".juv") {
                java_files.extend(find_java_files(&path)?);
            }
        } else if path.extension().and_then(|e| e.to_str()) == Some("java") {
            java_files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(java_files)
}

/// Run the run command.
/// This executes the project using the local Java installation.
///
/// # Arguments
/// * `args` - Extra arguments to pass to the build tool.
pub async fn run(args: &[String]) -> Result<()> {
    println!("{} Running project...", "→".cyan());

    // Read the project config to find the local Java installation.
    let config = read_project_config()?;

    // Find the Java binary inside the project's local installation.
    let jdk_base = Path::new(".").join(".juv").join("jdk");
    let java_home = jdk_base.join(format!("jdk-{}", config.java_version));
    let java_bin = java_home.join("bin").join("java");

    if !java_bin.exists() {
        return Err(anyhow::anyhow!(
            "Java not found in project. Run 'juv init' first."
        ));
    }

    // Check if the project has a build file.
    let has_build_file = Path::new("pom.xml").exists()
        || Path::new("build.gradle").exists()
        || Path::new("build.gradle.kts").exists();

    let mut cmd = if has_build_file {
        // Project has a build file, run the build tool.
        if config.build_tool == "maven" {
            let mut c = Command::new("mvn");
            c.arg("spring-boot:run");
            c
        } else {
            let mut c = Command::new("./gradlew");
            c.arg("bootRun");
            c
        }
    } else {
        // Project has no build file, compile and run java with downloaded libraries.
        let libs_dir = Path::new(".").join(".juv").join("libs");
        let classpath = if libs_dir.exists() {
            // Build classpath with all JARs in .juv/libs/.
            let jars: Vec<String> = fs::read_dir(&libs_dir)?
                .filter_map(|e| e.ok())
                .map(|e| e.path().to_string_lossy().to_string())
                .filter(|p| p.ends_with(".jar"))
                .collect();
            jars.join(":")
        } else {
            String::new()
        };

        // Find all .java files in the project.
        let java_files = find_java_files(Path::new("."))?;

        if java_files.is_empty() {
            return Err(anyhow::anyhow!(
                "No Java files found. Create a Main.java file first."
            ));
        }

        // Compile all .java files.
        println!("{} Compiling Java files...", "→".cyan());
        let mut javac = Command::new(java_home.join("bin").join("javac"));
        javac.arg("-cp");
        javac.arg(&classpath);
        javac.arg("-d");
        javac.arg(".juv/classes");
        for file in &java_files {
            javac.arg(file);
        }

        let compile_status = javac.status().context("Failed to compile Java files.")?;
        if !compile_status.success() {
            return Err(anyhow::anyhow!("Compilation failed."));
        }

        // Run the compiled code.
        let mut c = Command::new(&java_bin);
        c.arg("-cp");
        c.arg(format!("{}:.juv/classes", classpath));
        c.arg("Main");
        c
    };

    // Add any extra arguments the user provided.
    for arg in args {
        cmd.arg(arg);
    }

    // Set environment variables so the build tool uses the local Java.
    cmd.env("JAVA_HOME", &java_home);
    cmd.env(
        "PATH",
        format!(
            "{}:{}",
            java_home.join("bin").display(),
            std::env::var("PATH").unwrap_or_default()
        ),
    );

    // Run the command and wait for it to finish.
    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to run the project. Error: {}",
                e
            ));
        }
    };

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Project exited with code {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}
