# 🌳 Tree-TXT

[![Crates.io](https://img.shields.io/crates/v/tree-txt.svg)](https://crates.io/crates/tree-txt)
[![Documentation](https://docs.rs/tree-txt/badge.svg)](https://docs.rs/tree-txt)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)

**Interactive file selector and codebase exporter** - Generate beautiful, formatted text files from your project structure with an intuitive terminal interface.

Perfect for creating documentation, sharing code snippets, preparing AI training data, or generating comprehensive project overviews.

---

## ✨ Features

- 🎯 **Interactive File Selection** - Beautiful terminal UI with tree navigation
- 📁 **Smart Directory Traversal** - Expand/collapse directories with visual indicators  
- ✅ **Visual Selection Feedback** - Clear indicators for selected files and directories
- 💾 **Project Memory** - Remembers your selections per project directory
- ⚙️ **Flexible Configuration** - Config files, CLI flags, and customizable output formats
- 🎨 **Beautiful Output** - Clean, formatted text files with headers and structure
- 🔒 **Secure** - Path traversal protection and comprehensive input validation
- ⚡ **Fast** - Optimized for large codebases with efficient file handling
- 🔄 **Batch Operations** - Select entire directories or individual files
- 📝 **Multiple Export Formats** - With/without line numbers, tree structure, content

## 🚀 Quick Start

### Installation

#### 🦀 Cargo (Recommended - All Platforms)
```bash
cargo install tree-txt
```

#### 🍺 Homebrew (macOS & Linux)
```bash
# Add our custom tap
brew tap dev-mirzabicer/tree-txt

# Install tree-txt
brew install tree-txt
```

#### 🐧 Linux Package Managers

**Arch Linux (AUR):**
```bash
# Using yay (recommended)
yay -S tree-txt

# Or using paru
paru -S tree-txt

# Manual installation
git clone https://aur.archlinux.org/tree-txt.git
cd tree-txt && makepkg -si
```

**Ubuntu/Debian:**
```bash
# Method 1: Download .deb package
wget https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-amd64.deb
sudo dpkg -i tree-txt-v0.1.0-amd64.deb

# Method 2: PPA (coming soon)
# sudo add-apt-repository ppa:dev-mirzabicer/tree-txt
# sudo apt update && sudo apt install tree-txt
```

**Fedora/RHEL/CentOS:**
```bash
# Download and install RPM
wget https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-x86_64.rpm
sudo rpm -i tree-txt-v0.1.0-x86_64.rpm

# Or using dnf
sudo dnf install tree-txt-v0.1.0-x86_64.rpm
```

#### 📦 Direct Download (All Platforms)

**Linux:**
```bash
# x86_64
wget https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
tar xzf tree-txt-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
sudo cp tree-txt-v0.1.0-x86_64-unknown-linux-gnu/tree-txt /usr/local/bin/

# ARM64
wget https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-aarch64-unknown-linux-gnu.tar.gz
```

**macOS:**
```bash
# Intel Macs
wget https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-x86_64-apple-darwin.tar.gz
tar xzf tree-txt-v0.1.0-x86_64-apple-darwin.tar.gz
cp tree-txt-v0.1.0-x86_64-apple-darwin/tree-txt /usr/local/bin/

# Apple Silicon (M1/M2/M3)
wget https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-aarch64-apple-darwin.tar.gz
```

**Windows:**
1. Download: [tree-txt-v0.1.0-x86_64-pc-windows-msvc.zip](https://github.com/dev-mirzabicer/tree-txt/releases/latest/download/tree-txt-v0.1.0-x86_64-pc-windows-msvc.zip)
2. Extract the ZIP file
3. Add `tree-txt.exe` to your PATH or run directly

#### 🔨 From Source
```bash
# Clone repository
git clone https://github.com/dev-mirzabicer/tree-txt.git
cd tree-txt

# Install with cargo
cargo install --path .

# Or build manually
cargo build --release
# Binary will be in target/release/tree-txt
```

#### 📦 Coming Soon
- **Snap:** `snap install tree-txt`
- **Flatpak:** `flatpak install tree-txt` 
- **Chocolatey (Windows):** `choco install tree-txt`
- **winget (Windows):** `winget install tree-txt`

### Basic Usage

1. Navigate to your project directory:
```bash
cd /path/to/your/project
```

2. Launch the interactive selector:
```bash
tree-txt
```

3. Use the intuitive controls:
   - **Arrow keys** or **j/k** - Navigate up/down
   - **→** or **l** - Expand directory
   - **←** - Collapse directory  
   - **Space** - Select/deselect file or entire directory
   - **Enter** - Confirm selections and generate output
   - **Ctrl+A** - Select all visible files
   - **Ctrl+D** - Deselect all files
   - **Ctrl+H** - Toggle hidden files
   - **Q** - Quit without generating

4. Your formatted codebase will be saved as `codebase.txt`!

## 📖 Usage Examples

### Interactive Mode (Default)
```bash
# Launch interactive file selector
tree-txt

# Specify custom output file
tree-txt -o my-project-export.txt

# Include line numbers in output
tree-txt --line-numbers

# Export only file list (no content)
tree-txt --no-content

# Export without directory tree
tree-txt --no-tree
```

### Configuration File Mode
Create a `tree-txt.toml` configuration file:

```toml
# List of files to include (relative to current directory)
files = [
    "src/main.rs",
    "src/lib.rs", 
    "Cargo.toml",
    "README.md"
]

# Output format options
[output_format]
include_line_numbers = false
include_tree = true
include_file_contents = true
file_separator = "═══════════════════════════════════════════════════════════"
```

Then run:
```bash
tree-txt -c tree-txt.toml -o configured-export.txt
```

### Sample Output

```
# Codebase Export
Generated on: 2024-12-27 15:30:45 UTC
Base directory: /Users/dev/my-rust-project
Total files: 12

═══════════════════════════════════════════════════════════
## DIRECTORY STRUCTURE
═══════════════════════════════════════════════════════════

my-rust-project/
    ├── src/
    │   ├── main.rs ✓
    │   ├── lib.rs ✓
    │   └── utils.rs ✓
    ├── Cargo.toml ✓
    └── README.md ✓

═══════════════════════════════════════════════════════════
## FILE CONTENTS  
═══════════════════════════════════════════════════════════

────────────────────────────────────────────────────────────
File: src/main.rs
────────────────────────────────────────────────────────────

fn main() {
    println!("Hello, world!");
}

────────────────────────────────────────────────────────────
File: Cargo.toml
────────────────────────────────────────────────────────────

[package]
name = "my-rust-project"
version = "0.1.0"
edition = "2021"
```

## ⚙️ Configuration

### CLI Options

| Option | Short | Description |
|--------|-------|-------------|
| `--config <FILE>` | `-c` | Use configuration file instead of interactive selection |
| `--output <FILE>` | `-o` | Output file name (default: `codebase.txt`) |
| `--line-numbers` | `-l` | Include line numbers in file contents |
| `--no-tree` | | Skip directory tree generation |
| `--no-content` | | Only show file list, not contents |
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version |

### Configuration File Format

The configuration file uses TOML format with the following structure:

```toml
# Required: List of files to include
files = [
    "src/main.rs",
    "README.md",
    "Cargo.toml"
]

# Optional: Output format customization
[output_format]
include_line_numbers = false      # Add line numbers to file contents
include_tree = true               # Include directory tree structure  
include_file_contents = true      # Include actual file contents
file_separator = "═══════════════" # Customize section separators
```

### Project State Management

Tree-TXT automatically remembers your file selections per project directory. State files are stored in:

- **macOS**: `~/Library/Application Support/tree-txt/state.toml`
- **Linux**: `~/.config/tree-txt/state.toml`  
- **Windows**: `%APPDATA%\tree-txt\state.toml`

## 🎨 Interactive Interface

### Navigation Controls

- **↑↓** or **j/k** - Move selection up/down
- **→** or **l** - Expand directory (show contents)
- **←** - Collapse directory (hide contents)  
- **Space** - Toggle selection for files or entire directories
- **Enter** - Confirm selections and generate export
- **Q** - Quit without saving

### Bulk Operations

- **Ctrl+A** - Select all visible files
- **Ctrl+D** - Deselect all files
- **Ctrl+H** - Toggle display of hidden files (starting with `.`)

### Visual Indicators

- 📁 **Directories** - Cyan color with expand/collapse arrows (▶/▼)
- ✅ **Selected Files** - Green color with checkmark (✓)
- 📄 **Unselected Files** - White color  
- **Tree Structure** - Proper indentation showing file hierarchy

## 🔧 Advanced Usage

### Large Codebases

Tree-TXT is optimized for large projects:

- Efficient memory usage with lazy directory loading
- Fast file system operations with proper error handling
- Scalable tree rendering for thousands of files
- Progress indication for lengthy operations

### Security Features

- **Path Traversal Protection** - Cannot access files outside project directory
- **Input Validation** - Comprehensive validation of all inputs
- **Safe File Operations** - Proper error handling for permissions and I/O
- **Memory Safety** - Built with Rust's memory safety guarantees

### Integration Examples

**Generate documentation for AI tools:**
```bash
tree-txt --line-numbers -o ai-training-data.txt
```

**Create project snapshot for sharing:**
```bash  
tree-txt --no-line-numbers -o project-snapshot.txt
```

**Export specific modules only:**
```bash
tree-txt -c module-config.toml -o module-export.txt
```

## 🐛 Troubleshooting

### Common Issues

**"Permission denied" errors:**
- Ensure you have read permissions for the target directory
- Run with appropriate permissions or choose a different directory
- Check that files haven't been locked by other applications

**"No files selected" error:**
- Make sure to select at least one file using Space in interactive mode
- Verify your configuration file contains valid file paths
- Check that specified files actually exist

**Interface not displaying correctly:**
- Ensure your terminal supports Unicode characters and colors
- Try a different terminal emulator if issues persist  
- Check terminal size (minimum 80x24 recommended)

**Large directory performance:**
- Use Ctrl+H to hide unnecessary hidden files
- Consider using configuration files for very large projects
- Close expanded directories you don't need

### Getting Help

1. Check this README for common solutions
2. Run `tree-txt --help` for usage information
3. Open an issue on [GitHub](https://github.com/dev-mirzabicer/tree-txt/issues)
4. Check existing issues for similar problems

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/dev-mirzabicer/tree-txt.git
   cd tree-txt
   ```

2. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

5. **Run locally:**
   ```bash
   cargo run
   ```

### Code Quality

We maintain high code quality standards:

- **Linting**: `cargo clippy -- -D warnings`
- **Formatting**: `cargo fmt --check`
- **Security**: `cargo audit`
- **Documentation**: `cargo doc`

### Contribution Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b amazing-feature`)
3. Make your changes with tests and documentation
4. Run the full test suite and quality checks
5. Commit with clear, descriptive messages
6. Push to your fork and submit a pull request

### Areas for Contribution

- 🔍 Additional export formats (JSON, XML, Markdown)
- 🎨 Theme and color customization options  
- 🔌 Plugin system for custom file processors
- 📱 Cross-platform packaging (Homebrew, APT, etc.)
- 🌐 Internationalization and localization
- ⚡ Performance optimizations
- 📚 Documentation improvements
- 🧪 Test coverage expansion

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Ratatui](https://ratatui.rs/) for the beautiful terminal interface
- Inspired by the need for better codebase documentation tools
- Thanks to the Rust community for excellent crates and tooling

## 📊 Project Stats

- **Language**: Rust 🦀
- **Dependencies**: Minimal and carefully selected
- **Security**: Comprehensive audit and validation  
- **Performance**: Optimized for large codebases
- **Cross-platform**: Windows, macOS, Linux

---

**Happy coding! 🚀**

*Generate beautiful codebase exports with Tree-TXT*
