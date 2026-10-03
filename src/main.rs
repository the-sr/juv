// main.rs - Entry point for the juv command line tool.
// This file sets up the command line interface and routes user input to the correct action.

mod commands;
mod jdk;
mod project;
mod utils;

use clap::{Parser, Subcommand};
use colored::Colorize;

/// The main command line structure.
/// This defines what the user can type after "juv".
#[derive(Parser)]
#[command(name = "juv")]
#[command(about = "A tool to manage Java versions and project dependencies")]
#[command(version = "0.1.0")]
struct Cli {
    /// The action the user wants to perform.
    #[command(subcommand)]
    command: Commands,
}

/// All available actions the user can perform.
#[derive(Subcommand)]
enum Commands {
    /// Create a new project with a local Java installation.
    Init {
        /// The name of the project folder to create.
        name: String,
        /// The Java version to install (default: 17).
        #[arg(short, long, default_value = "17")]
        java: String,
    },
    /// Add a dependency to the project.
    Add {
        /// The name of the dependency to add.
        dependency: String,
    },
    /// Run the project using the local Java installation.
    Run {
        /// The command to run (default: ./gradlew bootRun).
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Create a lock file that records exact dependency versions.
    Lock,
    /// List all Java versions installed on this system.
    List,
    /// Set the Java version for the current project.
    Use {
        /// The Java version to use (e.g., 17, 21).
        version: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Route the user's input to the correct action.
    let result = match cli.command {
        Commands::Init { name, java } => {
            commands::init::run(&name, &java).await
        }
        Commands::Add { dependency } => {
            commands::add::run(&dependency).await
        }
        Commands::Run { args } => {
            commands::run::run(&args).await
        }
        Commands::Lock => {
            commands::lock::run().await
        }
        Commands::List => {
            commands::list::run().await
        }
        Commands::Use { version } => {
            commands::use_jdk::run(&version).await
        }
    };

    // If something went wrong, print the error and exit.
    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
