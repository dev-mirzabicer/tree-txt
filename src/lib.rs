//! # Tree-TXT
//!
//! **Interactive file selector and codebase exporter** for generating beautiful, 
//! formatted text files from your project structure.
//!
//! Tree-TXT provides both a command-line interface and programmatic API for:
//! - Interactive file selection with a terminal UI
//! - Batch file processing via configuration files  
//! - Customizable output formatting
//! - Project-specific selection state management
//!
//! ## Quick Start
//!
//! ```rust
//! use tree_txt::{OutputGenerator, OutputFormat, FileSelector};
//! use std::path::Path;
//!
//! // Generate output programmatically
//! let generator = OutputGenerator::new();
//! let config = OutputFormat::default();
//! let files = vec![Path::new("src/main.rs").to_path_buf()];
//! 
//! generator.generate_with_config(
//!     Path::new("."),
//!     &files,
//!     "output.txt",
//!     &config,
//! )?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Features
//!
//! - **Interactive Selection**: Terminal-based file browser with tree navigation
//! - **Flexible Configuration**: TOML config files and CLI arguments
//! - **Beautiful Output**: Formatted exports with headers and structure
//! - **State Management**: Remembers selections per project directory
//! - **Security**: Path traversal protection and input validation
//! - **Performance**: Optimized for large codebases
//!
//! ## Safety and Security
//!
//! Tree-TXT includes comprehensive security measures:
//! - All file operations are constrained to the specified base directory
//! - Input validation prevents malicious path manipulation
//! - Memory-safe Rust implementation with proper error handling
//! - No arbitrary code execution or unsafe operations

pub mod config;
pub mod error;
pub mod file_selector;
pub mod output_generator;
pub mod state_manager;

// Re-export main types for convenience
pub use config::{Config, OutputFormat};
pub use error::{TreeTxtError, Result};
pub use file_selector::{FileSelector, FileItem};
pub use output_generator::OutputGenerator;
pub use state_manager::{StateManager, ProjectState, GlobalState};