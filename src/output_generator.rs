use anyhow::Result;
use crate::config::OutputFormat;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct OutputGenerator {
    // Configuration options can be added here later
}

impl OutputGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(
        &self,
        base_path: &Path,
        selected_files: &[PathBuf],
        output_file: &str,
    ) -> Result<()> {
        let config = OutputFormat::default();
        self.generate_with_config(base_path, selected_files, output_file, &config)
    }

    pub fn generate_with_config(
        &self,
        base_path: &Path,
        selected_files: &[PathBuf],
        output_file: &str,
        config: &OutputFormat,
    ) -> Result<()> {
        let mut content = String::new();

        // Add header
        content.push_str(&self.generate_header(base_path, selected_files));

        // Add directory tree if requested
        if config.include_tree {
            content.push_str(&self.generate_tree(base_path, selected_files)?);
        }

        // Add file contents if requested
        if config.include_file_contents {
            content.push_str(&self.generate_file_contents(base_path, selected_files, config)?);
        }

        // Write to file
        fs::write(output_file, content)?;

        Ok(())
    }

    fn generate_header(&self, base_path: &Path, selected_files: &[PathBuf]) -> String {
        let mut header = String::new();
        
        header.push_str("# Codebase Export\n");
        header.push_str(&format!("Generated on: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        header.push_str(&format!("Base directory: {}\n", base_path.display()));
        header.push_str(&format!("Total files: {}\n\n", selected_files.len()));

        let separator = "═".repeat(80);
        header.push_str(&format!("{}\n", separator));
        header.push_str("## DIRECTORY STRUCTURE\n");
        header.push_str(&format!("{}\n\n", separator));

        header
    }

    fn generate_tree(&self, base_path: &Path, selected_files: &[PathBuf]) -> Result<String> {
        let mut tree_content = String::new();
        let mut paths_set: BTreeSet<PathBuf> = BTreeSet::new();

        // Collect all paths including parent directories
        for file_path in selected_files {
            let relative_path = file_path.strip_prefix(base_path)
                .unwrap_or(file_path);
            
            paths_set.insert(relative_path.to_path_buf());
            
            // Add all parent directories
            let mut current = relative_path;
            while let Some(parent) = current.parent() {
                if parent == Path::new("") {
                    break;
                }
                paths_set.insert(parent.to_path_buf());
                current = parent;
            }
        }

        // Generate tree structure
        tree_content.push_str(&format!("{}/\n", base_path.file_name().unwrap_or_default().to_string_lossy()));
        
        let mut sorted_paths: Vec<&PathBuf> = paths_set.iter().collect();
        sorted_paths.sort();

        for path in sorted_paths {
            let depth = path.components().count();
            let indent = "    ".repeat(depth);
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            
            if selected_files.iter().any(|f| f.strip_prefix(base_path).unwrap_or(f) == path) {
                tree_content.push_str(&format!("{}├── {} ✓\n", indent, name));
            } else if base_path.join(path).is_dir() {
                tree_content.push_str(&format!("{}├── {}/\n", indent, name));
            }
        }

        tree_content.push('\n');
        Ok(tree_content)
    }

    fn generate_file_contents(&self, base_path: &Path, selected_files: &[PathBuf], config: &OutputFormat) -> Result<String> {
        let mut content = String::new();
        
        let separator = "═".repeat(80);
        content.push_str(&format!("{}\n", separator));
        content.push_str("## FILE CONTENTS\n");
        content.push_str(&format!("{}\n\n", separator));

        let mut sorted_files = selected_files.to_vec();
        sorted_files.sort();

        for (index, file_path) in sorted_files.iter().enumerate() {
            if index > 0 {
                content.push('\n');
            }

            let relative_path = file_path.strip_prefix(base_path)
                .unwrap_or(file_path);

            // File header
            let file_separator = "─".repeat(60);
            content.push_str(&format!("{}\n", file_separator));
            content.push_str(&format!("File: {}\n", relative_path.display()));
            content.push_str(&format!("{}\n\n", file_separator));

            // File contents
            match fs::read_to_string(file_path) {
                Ok(file_content) => {
                    if file_content.trim().is_empty() {
                        content.push_str("(empty file)\n");
                    } else {
                        // Add content with or without line numbers based on config
                        if config.include_line_numbers {
                            for (line_num, line) in file_content.lines().enumerate() {
                                content.push_str(&format!("{:4} | {}\n", line_num + 1, line));
                            }
                        } else {
                            content.push_str(&file_content);
                            if !file_content.ends_with('\n') {
                                content.push('\n');
                            }
                        }
                    }
                }
                Err(e) => {
                    content.push_str(&format!("Error reading file: {}\n", e));
                }
            }
        }

        Ok(content)
    }
}

impl Default for OutputGenerator {
    fn default() -> Self {
        Self::new()
    }
}