//! Redis Cloud REST API client
//!
//! This module provides a client for interacting with Redis Cloud's REST API,
//! enabling subscription management, database operations, and monitoring.
//!
//! # Examples
//!
//! ## Creating a Client
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = CloudConfig {
//!     api_key: "your-api-key".to_string(),
//!     api_secret_key: "your-secret-key".to_string(),
//!     api_url: "https://api.redislabs.com/v1".to_string(),
//! };
//!
//! let client = CloudClient::new(config)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Managing Subscriptions
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudSubscriptionHandler, CreateSubscriptionRequest};
//!
//! # async fn example(client: CloudClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CloudSubscriptionHandler::new(client);
//!
//! // List all subscriptions
//! let subscriptions = handler.list().await?;
//! for sub in subscriptions {
//!     println!("Subscription: {} ({})", sub.name, sub.id);
//! }
//!
//! // Create a new subscription
//! let request = CreateSubscriptionRequest {
//!     name: "production".to_string(),
//!     payment_method_id: 123,
//!     memory_storage: "ram".to_string(),
//!     cloud_provider: vec![/* provider config */],
//!     // ... other fields
//! };
//!
//! let new_sub = handler.create(request).await?;
//! println!("Created subscription: {}", new_sub.id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Database Operations
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudDatabaseHandler, CreateDatabaseRequest};
//!
//! # async fn example(client: CloudClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CloudDatabaseHandler::new(client);
//! let subscription_id = 12345;
//!
//! // List databases in a subscription
//! let databases = handler.list(subscription_id).await?;
//! for db in databases {
//!     println!("Database: {} at {}:{}", db.name, db.public_endpoint, db.port);
//! }
//!
//! // Create a new database
//! let request = CreateDatabaseRequest {
//!     name: "cache-db".to_string(),
//!     memory_limit_in_gb: 1.0,
//!     modules: vec!["RedisJSON".to_string()],
//!     data_persistence: "aof-every-1-second".to_string(),
//!     replication: true,
//!     // ... other fields
//! };
//!
//! let new_db = handler.create(subscription_id, request).await?;
//! println!("Created database: {}", new_db.database_id);
//!
//! // Get database metrics
//! let metrics = handler.get_metrics(subscription_id, new_db.database_id, None, None).await?;
//! println!("Ops/sec: {:?}", metrics);
//! # Ok(())
//! # }
//! ```
//!
//! ## Backup Management
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudBackupHandler, CreateBackupRequest};
//!
//! # async fn example(client: CloudClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CloudBackupHandler::new(client);
//! let subscription_id = 12345;
//! let database_id = 67890;
//!
//! // List backups
//! let backups = handler.list(subscription_id, database_id).await?;
//! for backup in backups {
//!     println!("Backup: {} ({})", backup.backup_id, backup.status);
//! }
//!
//! // Create a backup
//! let request = CreateBackupRequest {
//!     description: Some("Pre-deployment backup".to_string()),
//! };
//!
//! let new_backup = handler.create(subscription_id, database_id, request).await?;
//! println!("Created backup: {}", new_backup.backup_id);
//!
//! // Restore from backup
//! handler.restore(subscription_id, database_id, new_backup.backup_id).await?;
//! println!("Restore initiated");
//! # Ok(())
//! # }
//! ```
//!
//! ## ACL Management
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudAclHandler};
//!
//! # async fn example(client: CloudClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CloudAclHandler::new(client);
//! let subscription_id = 12345;
//! let database_id = 67890;
//!
//! // Get database ACLs
//! let acls = handler.get_database_acls(subscription_id, database_id).await?;
//! for acl in acls {
//!     println!("ACL User: {}", acl.username);
//! }
//!
//! // List ACL users
//! let users = handler.list_acl_users().await?;
//! for user in users {
//!     println!("User: {} - Rules: {:?}", user.name, user.rules);
//! }
//!
//! // List ACL roles
//! let roles = handler.list_acl_roles().await?;
//! for role in roles {
//!     println!("Role: {} - Permissions: {:?}", role.name, role.permissions);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## VPC Peering
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudPeeringHandler, CreatePeeringRequest};
//!
//! # async fn example(client: CloudClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CloudPeeringHandler::new(client);
//! let subscription_id = 12345;
//!
//! // List peerings
//! let peerings = handler.list(subscription_id).await?;
//! for peering in peerings {
//!     println!("Peering: {} ({})", peering.peering_id, peering.status);
//! }
//!
//! // Create VPC peering
//! let request = CreatePeeringRequest {
//!     aws_account_id: "123456789012".to_string(),
//!     vpc_id: "vpc-12345".to_string(),
//!     vpc_cidr: "10.0.0.0/16".to_string(),
//!     region: "us-east-1".to_string(),
//! };
//!
//! let new_peering = handler.create(subscription_id, request).await?;
//! println!("Created peering: {}", new_peering.peering_id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Cloud Provider Regions
//!
//! ```ignore
//! use redis_cloud::{CloudClient, CloudRegionHandler};
//!
//! # async fn example(client: CloudClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CloudRegionHandler::new(client);
//!
//! // List AWS regions
//! let aws_regions = handler.list("AWS").await?;
//! for region in aws_regions {
//!     println!("AWS Region: {} - {}", region.name, region.display_name);
//! }
//!
//! // List GCP regions
//! let gcp_regions = handler.list("GCP").await?;
//! for region in gcp_regions {
//!     println!("GCP Region: {} - {}", region.name, region.display_name);
//! }
//! # Ok(())
//! # }
//! ```

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
