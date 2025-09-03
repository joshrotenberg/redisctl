//! Error types for Redis Cloud API operations

use thiserror::Error;

/// Result type for Redis Cloud operations
pub type Result<T> = std::result::Result<T, CloudError>;

/// Redis Cloud API error types
#[derive(Error, Debug)]
pub enum CloudError {
    /// HTTP request failed
    #[error("Request failed: {0}")]
    RequestFailed(String),

    /// API returned an error response
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Invalid input provided
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Network or HTTP error
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Environment variable error
    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),

    /// Generic error
    #[error("{0}")]
    Other(String),
}
