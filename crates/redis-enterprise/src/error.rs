//! Error types for REST API operations

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RestError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("API error: {message} (code: {code})")]
    ApiError { code: u16, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Not connected to REST API")]
    NotConnected,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Server error: {0}")]
    ServerError(String),
}

impl RestError {
    /// Check if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, RestError::NotFound)
            || matches!(self, RestError::ApiError { code, .. } if *code == 404)
    }

    /// Check if this is an authentication error
    pub fn is_unauthorized(&self) -> bool {
        matches!(self, RestError::Unauthorized)
            || matches!(self, RestError::AuthenticationFailed)
            || matches!(self, RestError::ApiError { code, .. } if *code == 401)
    }

    /// Check if this is a server error
    pub fn is_server_error(&self) -> bool {
        matches!(self, RestError::ServerError(_))
            || matches!(self, RestError::ApiError { code, .. } if *code >= 500)
    }
}

pub type Result<T> = std::result::Result<T, RestError>;
