//! Error types for redisctl
//!
//! Defines structured error types using thiserror for better error handling and user experience.

#![allow(dead_code)] // Foundation code - will be used in future PRs

use thiserror::Error;

/// Main error type for the redisctl application
#[derive(Error, Debug)]
pub enum RedisCtlError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Profile '{name}' not found")]
    ProfileNotFound { name: String },

    #[error("Profile '{name}' is type '{actual_type}' but command requires '{expected_type}'")]
    ProfileTypeMismatch {
        name: String,
        actual_type: String,
        expected_type: String,
    },

    #[error("No profile configured. Use 'redisctl profile set' to configure a profile.")]
    NoProfileConfigured,

    #[error("Missing credentials for profile '{name}'")]
    MissingCredentials { name: String },

    #[error("Authentication failed: {message}")]
    AuthenticationFailed { message: String },

    #[error("API error: {message}")]
    ApiError { message: String },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("Command not supported for deployment type '{deployment_type}'")]
    UnsupportedDeploymentType { deployment_type: String },
    #[error("File error for '{path}': {message}")]
    FileError { path: String, message: String },

    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    #[error("Timeout: {message}")]
    Timeout { message: String },

    #[error("Output formatting error: {message}")]
    OutputError { message: String },
}

/// Result type for redisctl operations
pub type Result<T> = std::result::Result<T, RedisCtlError>;

impl From<redis_cloud::CloudError> for RedisCtlError {
    fn from(err: redis_cloud::CloudError) -> Self {
        match err {
            redis_cloud::CloudError::AuthenticationFailed { message } => {
                RedisCtlError::AuthenticationFailed { message }
            }
            redis_cloud::CloudError::ConnectionError(message) => {
                RedisCtlError::ConnectionError { message }
            }
            _ => RedisCtlError::ApiError {
                message: err.to_string(),
            },
        }
    }
}

impl From<redis_enterprise::RestError> for RedisCtlError {
    fn from(err: redis_enterprise::RestError) -> Self {
        match err {
            redis_enterprise::RestError::AuthenticationFailed => {
                RedisCtlError::AuthenticationFailed {
                    message: "Authentication failed".to_string(),
                }
            }
            redis_enterprise::RestError::RequestFailed(reqwest_err) => {
                RedisCtlError::ConnectionError {
                    message: reqwest_err.to_string(),
                }
            }
            _ => RedisCtlError::ApiError {
                message: err.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for RedisCtlError {
    fn from(err: serde_json::Error) -> Self {
        RedisCtlError::OutputError {
            message: format!("JSON error: {}", err),
        }
    }
}

impl From<std::io::Error> for RedisCtlError {
    fn from(err: std::io::Error) -> Self {
        RedisCtlError::OutputError {
            message: format!("IO error: {}", err),
        }
    }
}

impl From<anyhow::Error> for RedisCtlError {
    fn from(err: anyhow::Error) -> Self {
        RedisCtlError::Config(err.to_string())
    }
}
