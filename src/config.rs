//! # Configuration Management
//!
//! Provides configuration structures for Tree-TXT operations, supporting both
//! TOML configuration files and programmatic configuration.
//!
//! # Examples
//!
//! Create a configuration programmatically:
//! ```rust
//! use tree_txt::{Config, OutputFormat};
//! use std::path::PathBuf;
//!
//! let config = Config {
//!     files: vec![
//!         PathBuf::from("src/main.rs"),
//!         PathBuf::from("README.md"),
//!     ],
//!     output_format: OutputFormat {
//!         include_line_numbers: true,
//!         ..Default::default()
//!     },
//! };
//! ```
//!
//! Load from a TOML file:
//! ```rust
//! use tree_txt::Config;
//!
//! let config = Config::from_file("config.toml")?;
//! # Ok::<(), tree_txt::TreeTxtError>(())
//! ```

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure for Tree-TXT operations.
///
/// Contains file selection and output formatting configuration that can be
/// loaded from TOML files or created programmatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// List of files to include in the export (relative to base directory)
    pub files: Vec<PathBuf>,
    /// Output formatting configuration
    #[serde(default)]
    pub output_format: OutputFormat,
}

/// Configuration for output formatting and content inclusion.
///
/// Controls what elements are included in the generated output and how they
/// are formatted and presented.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFormat {
    /// Whether to include the directory tree structure
    #[serde(default = "default_true")]
    pub include_tree: bool,
    /// Whether to include the actual file contents
    #[serde(default = "default_true")]
    pub include_file_contents: bool,
    /// Whether to add line numbers to file contents
    #[serde(default = "default_false")]
    pub include_line_numbers: bool,
    /// Separator string used between sections
    #[serde(default = "default_separator")]
    pub file_separator: String,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self {
            include_tree: true,
            include_file_contents: true,
            include_line_numbers: false,
            file_separator: "═".repeat(80),
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_separator() -> String {
    "═".repeat(80)
}

impl Config {
    /// Loads configuration from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tree_txt::Config;
    ///
    /// let config = Config::from_file("tree-txt.toml")?;
    /// println!("Loaded {} files", config.files.len());
    /// # Ok::<(), tree_txt::TreeTxtError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The TOML content is invalid
    /// - File paths in the configuration are malformed
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
