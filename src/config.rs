use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub files: Vec<PathBuf>,
    #[serde(default)]
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFormat {
    #[serde(default = "default_true")]
    pub include_tree: bool,
    #[serde(default = "default_true")]
    pub include_file_contents: bool,
    #[serde(default = "default_false")]
    pub include_line_numbers: bool,
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
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
