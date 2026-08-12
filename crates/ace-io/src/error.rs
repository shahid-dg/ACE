//! Error types for the ACE I/O module.

use thiserror::Error;

/// Result type for I/O operations.
pub type Result<T> = std::result::Result<T, IoError>;

/// Errors that can occur during I/O operations.
#[derive(Error, Debug)]
pub enum IoError {
    /// File not found.
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV parsing error.
    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    /// JSON parsing error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Invalid data format.
    #[error("Invalid format: {reason}")]
    InvalidFormat { reason: String },
}
