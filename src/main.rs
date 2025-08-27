use anyhow::Result;
use clap::{Arg, Command};
use std::env;

mod config;
mod file_selector;
mod output_generator;
mod state_manager;

use config::Config;
use file_selector::FileSelector;
use output_generator::OutputGenerator;
use state_manager::StateManager;

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
                .help("Use configuration file instead of interactive selection")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file name (default: codebase.txt)")
        )
        .arg(
            Arg::new("line_numbers")
                .short('l')
                .long("line-numbers")
                .action(clap::ArgAction::SetTrue)
                .help("Include line numbers in file contents")
        )
        .arg(
            Arg::new("no_tree")
                .long("no-tree")
                .action(clap::ArgAction::SetTrue)
                .help("Skip directory tree generation")
        )
        .arg(
            Arg::new("no_content")
                .long("no-content")
                .action(clap::ArgAction::SetTrue)
                .help("Only show file list, not contents")
        )
        .get_matches();

    let current_dir = env::current_dir()?;
    let mut state_manager = StateManager::new(&current_dir);
    
    let selected_files = if let Some(config_file) = matches.get_one::<String>("config") {
        let config = Config::from_file(config_file)?;
        config.files
    } else {
        let mut file_selector = FileSelector::new(&current_dir);
        
        // Load previous selections if they exist
        if let Ok(previous_selections) = state_manager.load_selections() {
            file_selector.set_selections(previous_selections);
        }
        
        let selections = file_selector.run_interactive()?;
        
        // Save selections for next time
        state_manager.save_selections(&selections)?;
        
        selections
    };

    let output_file = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("codebase.txt");

    // Create output configuration based on CLI args
    let mut output_config = config::OutputFormat::default();
    output_config.include_line_numbers = matches.get_flag("line_numbers");
    output_config.include_tree = !matches.get_flag("no_tree");
    output_config.include_file_contents = !matches.get_flag("no_content");

    let output_generator = OutputGenerator::new();
    output_generator.generate_with_config(&current_dir, &selected_files, output_file, &output_config)?;

    println!("Generated codebase text file: {}", output_file);
    Ok(())
}
