//! # Tree-TXT CLI Application
//!
//! Interactive file selector and codebase exporter with terminal user interface.
//!
//! This binary provides a command-line interface for Tree-TXT functionality,
//! including interactive file selection and batch processing via configuration files.

use anyhow::Result;
use clap::{Arg, Command};
use std::env;
use std::path::Path;

mod config;
mod error;
mod file_selector;
mod output_generator;
mod state_manager;

use config::Config;
use error::TreeTxtError;
use file_selector::FileSelector;
use output_generator::OutputGenerator;
use state_manager::StateManager;

/// Main entry point for the Tree-TXT CLI application.
///
/// Parses command-line arguments and either launches the interactive file selector
/// or processes files according to a configuration file.
///
/// # Examples
///
/// Run interactively:
/// ```bash
/// tree-txt
/// ```
///
/// Use configuration file:
/// ```bash  
/// tree-txt -c config.toml -o output.txt
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Current directory is not accessible
/// - Configuration file is invalid or not found
/// - No files are selected for export
/// - Output directory is not writable
/// - File generation fails
fn main() -> Result<()> {
    let matches = Command::new("tree-txt")
        .version("0.1.0")
        .author("Tree-TXT")
        .about("Generate pretty-printed codebase text files from selected files")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Use configuration file instead of interactive selection"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file name (default: codebase.txt)"),
        )
        .arg(
            Arg::new("line_numbers")
                .short('l')
                .long("line-numbers")
                .action(clap::ArgAction::SetTrue)
                .help("Include line numbers in file contents"),
        )
        .arg(
            Arg::new("no_tree")
                .long("no-tree")
                .action(clap::ArgAction::SetTrue)
                .help("Skip directory tree generation"),
        )
        .arg(
            Arg::new("no_content")
                .long("no-content")
                .action(clap::ArgAction::SetTrue)
                .help("Only show file list, not contents"),
        )
        .get_matches();

    let current_dir = env::current_dir().map_err(|_| {
        anyhow::anyhow!(
            "Failed to get current directory. Please ensure you're in a valid directory."
        )
    })?;

    // Validate that current directory exists and is readable
    if !current_dir.exists() {
        return Err(anyhow::anyhow!("Current directory does not exist"));
    }

    if !current_dir.is_dir() {
        return Err(anyhow::anyhow!("Current path is not a directory"));
    }

    let mut state_manager = StateManager::new(&current_dir);

    let selected_files = if let Some(config_file) = matches.get_one::<String>("config") {
        // Validate config file exists and is readable
        if !std::path::Path::new(config_file).exists() {
            return Err(
                TreeTxtError::InvalidPath(format!("Config file not found: {config_file}")).into(),
            );
        }

        let config = Config::from_file(config_file).map_err(|e| {
            TreeTxtError::ConfigError(format!("Failed to parse config file '{config_file}': {e}"))
        })?;

        // Validate that files in config exist
        let mut valid_files = Vec::new();
        for file_path in config.files {
            if file_path.exists() {
                if file_path.is_file() {
                    valid_files.push(file_path);
                } else {
                    eprintln!("Warning: Skipping '{}' - not a file", file_path.display());
                }
            } else {
                eprintln!("Warning: File not found: '{}'", file_path.display());
            }
        }

        if valid_files.is_empty() {
            return Err(anyhow::anyhow!("No valid files found in config file"));
        }

        valid_files
    } else {
        let mut file_selector = FileSelector::new(&current_dir);

        // Load previous selections if they exist
        if let Ok(previous_selections) = state_manager.load_selections() {
            file_selector.set_selections(previous_selections);
        }

        let selections = file_selector
            .run_interactive()
            .map_err(|e| anyhow::anyhow!("File selection failed: {}", e))?;

        // Validate selections
        if selections.is_empty() {
            return Err(TreeTxtError::NoFilesSelected.into());
        }

        // Save selections for next time (ignore save errors - not critical)
        if let Err(e) = state_manager.save_selections(&selections) {
            eprintln!("Warning: Failed to save selections for next time: {e}");
        }

        selections
    };

    let output_file = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("codebase.txt");

    // Validate output file path
    let output_path = Path::new(output_file);
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(anyhow::anyhow!(
                "Output directory does not exist: {}",
                parent.display()
            ));
        }
        if parent.exists() && !parent.is_dir() {
            return Err(anyhow::anyhow!(
                "Output parent path is not a directory: {}",
                parent.display()
            ));
        }
    }

    // Check if output file already exists and warn user
    if output_path.exists() {
        eprintln!("Warning: Output file '{output_file}' already exists and will be overwritten");
    }

    // Create output configuration based on CLI args
    let output_config = config::OutputFormat {
        include_line_numbers: matches.get_flag("line_numbers"),
        include_tree: !matches.get_flag("no_tree"),
        include_file_contents: !matches.get_flag("no_content"),
        ..Default::default()
    };

    let output_generator = OutputGenerator::new();
    output_generator
        .generate_with_config(&current_dir, &selected_files, output_file, &output_config)
        .map_err(|e| anyhow::anyhow!("Failed to generate output file '{}': {}", output_file, e))?;

    println!("✅ Successfully generated codebase text file: {output_file}");
    Ok(())
}
