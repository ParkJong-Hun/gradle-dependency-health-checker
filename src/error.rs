/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use std::fmt;

/// Common error type for the application
#[derive(Debug)]
pub enum AnalysisError {
    /// IO errors (file not found, permission denied, etc.)
    Io(std::io::Error),
    /// Regex compilation or matching errors
    Regex(regex::Error),
    /// TOML parsing errors for version catalogs
    TomlParsing(toml::de::Error),
    /// Validation errors for CLI arguments
    Validation(String),
    /// General parsing errors
    Parsing(String),
    /// File system errors
    FileSystem(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::Io(err) => write!(f, "IO error: {}", err),
            AnalysisError::Regex(err) => write!(f, "Regex error: {}", err),
            AnalysisError::TomlParsing(err) => write!(f, "TOML parsing error: {}", err),
            AnalysisError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AnalysisError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            AnalysisError::FileSystem(msg) => write!(f, "File system error: {}", msg),
        }
    }
}

impl std::error::Error for AnalysisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnalysisError::Io(err) => Some(err),
            AnalysisError::Regex(err) => Some(err),
            AnalysisError::TomlParsing(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AnalysisError {
    fn from(err: std::io::Error) -> Self {
        AnalysisError::Io(err)
    }
}

impl From<regex::Error> for AnalysisError {
    fn from(err: regex::Error) -> Self {
        AnalysisError::Regex(err)
    }
}

impl From<toml::de::Error> for AnalysisError {
    fn from(err: toml::de::Error) -> Self {
        AnalysisError::TomlParsing(err)
    }
}

impl From<walkdir::Error> for AnalysisError {
    fn from(err: walkdir::Error) -> Self {
        AnalysisError::FileSystem(format!("Directory traversal error: {}", err))
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AnalysisError>;