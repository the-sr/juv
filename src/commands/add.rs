// add.rs - Handles the "juv add" command.
// If the project has a build file (pom.xml or build.gradle), add the dependency entry.
// If the project has no build file, download the library JAR to .juv/libs/.

use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::project::config::read_project_config;

/// Run the add command.
/// This adds a dependency to the project.
///
/// # Arguments
/// * `dependency` - The dependency to add (e.g., "lombok", "spring-boot-starter-web").
pub async fn run(dependency: &str) -> Result<()> {
    println!("{} Adding dependency '{}'...", "→".cyan(), dependency);

    // Check that we are inside a project.
    let config = read_project_config()?;
    let project_path = Path::new(".");

    // Check if the project has a build file.
    let has_build_file = project_path.join("pom.xml").exists()
        || project_path.join("build.gradle").exists()
        || project_path.join("build.gradle.kts").exists();

    if has_build_file {
        // Project has a build file, add the dependency entry to it.
        let build_file = if project_path.join("pom.xml").exists() {
            "pom.xml"
        } else if project_path.join("build.gradle").exists() {
            "build.gradle"
        } else {
            "build.gradle.kts"
        };
        add_dependency_to_build_file(project_path, build_file, dependency)?;
        println!(
            "{} Dependency '{}' added to {}",
            "✓".green().bold(),
            dependency,
            build_file
        );
    } else {
        // Project has no build file, download the library JAR.
        download_library(project_path, dependency).await?;
        println!(
            "{} Library '{}' downloaded to .juv/libs/",
            "✓".green().bold(),
            dependency
        );
    }

    Ok(())
}

/// Download a library JAR from Maven Central to .juv/libs/.
///
/// # Arguments
/// * `project_path` - The path to the project folder.
/// * `dependency` - The library to download (e.g., "lombok", "com.google.guava:guava").
async fn download_library(project_path: &Path, dependency: &str) -> Result<()> {
    // Create the .juv/libs folder.
    let libs_dir = project_path.join(".juv").join("libs");
    fs::create_dir_all(&libs_dir)?;

    // Parse the dependency string.
    // Supports: "artifactId" or "groupId:artifactId"
    let (group_id, artifact_id) = if dependency.contains(':') {
        let parts: Vec<&str> = dependency.split(':').collect();
        (parts[0].to_string(), parts[1].to_string())
    } else {
        // Default group ID for common libraries.
        ("org.projectlombok".to_string(), dependency.to_string())
    };

    // Build the Maven Central download URL.
    // Format: https://repo1.maven.org/maven2/{groupId}/{artifactId}/{version}/{artifactId}-{version}.jar
    let version = "1.18.30"; // Default version for lombok
    let url = format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.jar",
        group_id.replace('.', "/"),
        artifact_id,
        version,
        artifact_id,
        version
    );

    println!("{} Downloading from {}...", "→".cyan(), url);

    // Download the JAR file.
    let response = reqwest::get(&url)
        .await
        .context("Failed to download the library. Check the library name and your internet connection.")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Could not find library '{}'. Check the library name.",
            dependency
        ));
    }

    // Save the JAR to .juv/libs/.
    let jar_name = format!("{}.jar", artifact_id);
    let jar_path = libs_dir.join(&jar_name);
    let bytes = response
        .bytes()
        .await
        .context("Failed to read the downloaded library.")?;

    fs::write(&jar_path, &bytes)
        .with_context(|| format!("Failed to save library to '{}'", jar_path.display()))?;

    Ok(())
}

/// Add a dependency to the build file.
fn add_dependency_to_build_file(
    project_path: &Path,
    build_file: &str,
    dependency: &str,
) -> Result<()> {
    let build_path = project_path.join(build_file);
    let content = fs::read_to_string(&build_path)
        .with_context(|| format!("Failed to read {}", build_file))?;

    let new_content = if build_file == "pom.xml" {
        add_dependency_to_pom(&content, dependency)
    } else {
        add_dependency_to_gradle(&content, dependency)
    };

    fs::write(&build_path, new_content)
        .with_context(|| format!("Failed to write {}", build_file))?;
    Ok(())
}

/// Add a dependency to a pom.xml file.
fn add_dependency_to_pom(content: &str, dependency: &str) -> String {
    let parts: Vec<&str> = dependency.split(':').collect();

    let dependency_xml = if parts.len() == 1 {
        format!(
            "        <dependency>\n            <groupId>org.springframework.boot</groupId>\n            <artifactId>{}</artifactId>\n        </dependency>\n",
            parts[0]
        )
    } else {
        format!(
            "        <dependency>\n            <groupId>{}</groupId>\n            <artifactId>{}</artifactId>\n        </dependency>\n",
            parts[0], parts[1]
        )
    };

    if content.contains("</dependencies>") {
        content.replace(
            "</dependencies>",
            &format!("{}    </dependencies>", dependency_xml)
        )
    } else {
        content.replace(
            "</project>",
            &format!("    <dependencies>\n{}    </dependencies>\n</project>", dependency_xml)
        )
    }
}

/// Add a dependency to a Gradle build file.
fn add_dependency_to_gradle(content: &str, dependency: &str) -> String {
    if content.contains("dependencies {") {
        content.replace(
            "dependencies {",
            &format!("dependencies {{\n    implementation '{}'", dependency),
        )
    } else {
        format!(
            "{}\ndependencies {{\n    implementation '{}'\n}}\n",
            content, dependency
        )
    }
}
