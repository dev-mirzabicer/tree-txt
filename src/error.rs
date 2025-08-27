use std::fmt;

#[derive(Debug)]
pub enum TreeTxtError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    TomlSer(toml::ser::Error),
    InvalidPath(String),
    NoFilesSelected,
    PermissionDenied(String),
    ConfigError(String),
}

impl fmt::Display for TreeTxtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Toml(err) => write!(f, "TOML parsing error: {err}"),
            Self::TomlSer(err) => write!(f, "TOML serialization error: {err}"),
            Self::InvalidPath(path) => write!(f, "Invalid path: {path}"),
            Self::NoFilesSelected => write!(f, "No files were selected for export"),
            Self::PermissionDenied(path) => write!(f, "Permission denied accessing: {path}"),
            Self::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for TreeTxtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Toml(err) => Some(err),
            Self::TomlSer(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TreeTxtError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => Self::PermissionDenied(err.to_string()),
            _ => Self::Io(err),
        }
    }
}

impl From<toml::de::Error> for TreeTxtError {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err)
    }
}

impl From<toml::ser::Error> for TreeTxtError {
    fn from(err: toml::ser::Error) -> Self {
        Self::TomlSer(err)
    }
}

pub type Result<T> = std::result::Result<T, TreeTxtError>;
