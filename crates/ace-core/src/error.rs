//! Error types for the ACE core module.

use thiserror::Error;

/// Result type for core operations.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Errors that can occur in core consensus and agreement operations.
#[derive(Error, Debug)]
pub enum CoreError {
    /// Invalid input data.
    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },

    /// No annotations provided.
    #[error("No annotations provided")]
    NoAnnotations,

    /// Computation error.
    #[error("Computation error: {0}")]
    ComputationError(String),

    /// Numerical error (e.g., NaN, overflow).
    #[error("Numerical error: {0}")]
    NumericalError(String),
}
