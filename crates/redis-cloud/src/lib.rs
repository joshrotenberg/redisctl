//! Redis Cloud REST API client
//!
//! This module provides a client for interacting with Redis Cloud's REST API,
//! enabling subscription management, database operations, and monitoring.

pub mod client;
pub mod handlers;
pub mod models;

#[cfg(test)]
mod lib_tests;

// Re-export from the new structure
pub use client::{CloudClient, CloudConfig};

// Re-export handlers explicitly
pub use handlers::{
    CloudAccountHandler, CloudAccountsHandler, CloudAclHandler, CloudApiKeysHandler,
    CloudBackupHandler, CloudBillingHandler, CloudCrdbHandler, CloudDatabaseHandler,
    CloudFixedHandler, CloudLogsHandler, CloudMetricsHandler, CloudPeeringHandler,
    CloudPrivateServiceConnectHandler, CloudRegionHandler, CloudSsoHandler,
    CloudSubscriptionHandler, CloudTasksHandler, CloudTransitGatewayHandler, CloudUsersHandler,
};

// Re-export models explicitly
pub use models::{
    // Account models
    AccountKey,
    CloudAccount,
    // Backup models
    CloudBackup,
    // Database models
    CloudDatabase,
    // Metrics models
    CloudMetrics,
    // Peering models
    CloudPeering,
    // Subscription models
    CloudProvider,
    CloudProviderConfig,
    CloudRegion,
    CloudRegionConfig,
    CloudSubscription,
    CreateBackupRequest,
    CreateDatabaseRequest,
    CreatePeeringRequest,
    CreateSubscriptionRequest,
    Measurement,
    MetricValue,
    ThroughputMeasurement,
    UpdateDatabaseRequest,
    UpdateSubscriptionRequest,
};

// Re-export error types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloudError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("API error ({code}): {message}")]
    ApiError { code: u16, message: String },

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CloudError>;
