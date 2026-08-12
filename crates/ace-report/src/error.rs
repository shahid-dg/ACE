//! Error types for the ACE report generation module.

use thiserror::Error;

/// Result type for report operations.
pub type Result<T> = std::result::Result<T, ReportError>;

/// Errors that can occur during report generation.
#[derive(Error, Debug)]
pub enum ReportError {
    /// Template rendering error.
    #[error("Template error: {0}")]
    TemplateError(String),

    /// File writing error.
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),

    /// Asset embedding error.
    #[error("Asset error: {reason}")]
    AssetError { reason: String },

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Invalid report configuration.
    #[error("Invalid configuration: {reason}")]
    InvalidConfiguration { reason: String },
}
