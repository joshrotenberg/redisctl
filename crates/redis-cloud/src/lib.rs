//! Redis Cloud REST API Client
//!
//! A comprehensive Rust client for the Redis Cloud REST API, providing full access to
//! subscription management, database operations, billing, monitoring, and advanced features
//! like VPC peering, SSO/SAML, and Private Service Connect.
//!
//! ## Features
//!
//! - **Subscription Management**: Create, update, delete subscriptions across AWS, GCP, Azure
//! - **Database Operations**: Full CRUD operations, backups, imports, metrics
//! - **Advanced Networking**: VPC peering, Transit Gateway, Private Service Connect
//! - **Security & Access**: ACLs, SSO/SAML integration, API key management
//! - **Monitoring & Billing**: Comprehensive metrics, logs, billing and payment management
//! - **Enterprise Features**: Active-Active databases (CRDB), fixed/essentials plans
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudDatabaseHandler};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with API credentials
//!     let client = CloudClient::builder()
//!         .api_key("your-api-key")
//!         .api_secret("your-api-secret")
//!         .build()?;
//!
//!     // List all databases  
//!     let db_handler = CloudDatabaseHandler::new(client.clone());
//!     let databases = db_handler.list(123).await?;
//!     println!("Found {} databases", databases.as_array().unwrap_or(&vec![]).len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Core Usage Patterns
//!
//! ### Client Creation
//!
//! The client uses a builder pattern for flexible configuration:
//!
//! ```rust,no_run
//! use redis_cloud::CloudClient;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Basic client with default settings
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! // Custom configuration
//! let client2 = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .base_url("https://api.redislabs.com/v1".to_string())
//!     .timeout(std::time::Duration::from_secs(60))
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Working with Subscriptions
//!
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudSubscriptionHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let sub_handler = CloudSubscriptionHandler::new(client.clone());
//!
//! // List subscriptions
//! let subscriptions = sub_handler.list().await?;
//!
//! // Create a new subscription using raw API
//! let new_subscription = json!({
//!     "name": "my-redis-subscription",
//!     "provider": "AWS",
//!     "region": "us-east-1",
//!     "plan": "cache.m5.large"
//! });
//! let created = client.post_raw("/subscriptions", new_subscription).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Database Management
//!
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudDatabaseHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let db_handler = CloudDatabaseHandler::new(client.clone());
//!
//! // Create database using raw API
//! let database_config = json!({
//!     "name": "my-database",
//!     "memory_limit_in_gb": 1.0,
//!     "support_oss_cluster_api": false,
//!     "replication": true
//! });
//! let database = client.post_raw("/subscriptions/123/databases", database_config).await?;
//!
//! // Get database info
//! let db_info = db_handler.get(123, 456).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Advanced Features
//!
//! #### VPC Peering
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudPeeringHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let peering_handler = CloudPeeringHandler::new(client.clone());
//!
//! let peering_request = json!({
//!     "aws_account_id": "123456789012",
//!     "vpc_id": "vpc-12345678",
//!     "vpc_cidr": "10.0.0.0/16",
//!     "region": "us-east-1"
//! });
//! let peering = client.post_raw("/subscriptions/123/peerings", peering_request).await?;
//! # Ok(())
//! # }
//! ```
//!
//! #### SSO/SAML Management
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudSsoHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let sso_handler = CloudSsoHandler::new(client.clone());
//!
//! // Configure SSO using raw API
//! let sso_config = json!({
//!     "enabled": true,
//!     "auto_provision": true
//! });
//! let config = client.put_raw("/sso", sso_config).await?;
//! # Ok(())
//! # }
//! ```
//!
//! #### API Keys (Typed)
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudApiKeyHandler, ApiKeyRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let keys = CloudApiKeyHandler::new(client.clone());
//! let all = keys.list_typed().await?; // Vec<ApiKey>
//! if let Some(first) = all.first() {
//!     let detailed = keys.get_typed(first.id).await?;
//!     let _usage = keys.get_usage_typed(detailed.id, "7d").await?;
//! }
//!
//! let created = keys
//!     .create_typed(&ApiKeyRequest { name: "ci-bot".into(), status: None })
//!     .await?;
//! let _updated = keys
//!     .update_typed(created.id, &ApiKeyRequest { name: "ci-bot".into(), status: Some("disabled".into()) })
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The client provides comprehensive error handling for different failure scenarios:
//!
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudError, CloudDatabaseHandler};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build().unwrap();
//!
//! let db_handler = CloudDatabaseHandler::new(client.clone());
//!
//! match db_handler.get(123, 456).await {
//!     Ok(database) => println!("Database: {:?}", database),
//!     Err(CloudError::ApiError { code: 404, .. }) => {
//!         println!("Database not found");
//!     },
//!     Err(CloudError::AuthenticationFailed) => {
//!         println!("Invalid API credentials");
//!     },
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # }
//! ```
//!
//! ## Handler Overview
//!
//! The client provides specialized handlers for different API domains:
//!
//! | Handler | Purpose | Key Operations |
//! |---------|---------|----------------|
//! | [`CloudSubscriptionHandler`] | Subscription management | create, list, update, delete, pricing |
//! | [`CloudDatabaseHandler`] | Database operations | create, backup, import, metrics, resize |
//! | [`CloudAccountHandler`] | Account information | info, users, payment methods |
//! | [`CloudUserHandler`] | User management | create, update, delete, invite |
//! | [`CloudBillingHandler`] | Billing & payments | invoices, payment methods, usage reports |
//! | [`CloudBackupHandler`] | Database backups | create, restore, list, delete |
//! | [`CloudAclHandler`] | Access control | users, roles, Redis rules |
//! | [`CloudPeeringHandler`] | VPC peering | create, delete, list peering connections |
//! | [`CloudSsoHandler`] | SSO/SAML | configure, test, user/group mappings |
//! | [`CloudMetricsHandler`] | Monitoring | database and subscription metrics |
//! | [`CloudLogsHandler`] | Audit trails | system, database, and session logs |
//! | [`CloudTaskHandler`] | Async operations | track long-running operations |
//!
//! ## Authentication
//!
//! Redis Cloud uses API key authentication with two required headers:
//! - `x-api-key`: Your API key
//! - `x-api-secret-key`: Your API secret
//!
//! These credentials can be obtained from the Redis Cloud console under Account Settings > API Keys.

pub mod client;
mod handlers;
pub mod models;

#[cfg(test)]
mod lib_tests;

// Re-export from the new structure
pub use client::{CloudClient, CloudClientBuilder};

// Re-export handlers from handlers::* directly
pub use handlers::account::CloudAccountHandler;
pub use handlers::acl::CloudAclHandler;
pub use handlers::api_keys::CloudApiKeyHandler;
pub use handlers::backup::CloudBackupHandler;
pub use handlers::billing::CloudBillingHandler;
pub use handlers::cloud_accounts::CloudAccountsHandler;
pub use handlers::crdb::CloudCrdbHandler;
pub use handlers::database::CloudDatabaseHandler;
pub use handlers::fixed::CloudFixedHandler;
pub use handlers::logs::CloudLogsHandler;
pub use handlers::metrics::CloudMetricsHandler;
pub use handlers::peering::CloudPeeringHandler;
pub use handlers::private_service_connect::CloudPrivateServiceConnectHandler;
pub use handlers::region::CloudRegionHandler;
pub use handlers::sso::CloudSsoHandler;
pub use handlers::subscription::CloudSubscriptionHandler;
pub use handlers::tasks::CloudTaskHandler;
pub use handlers::transit_gateway::CloudTransitGatewayHandler;
pub use handlers::users::CloudUserHandler;

// Re-export models explicitly
pub use models::{
    // Account models
    AccountKey,
    // API keys models
    ApiKey,
    ApiKeyAuditLogEntry,
    ApiKeyAuditLogsResponse,
    ApiKeyPermissions,
    ApiKeyRequest,
    ApiKeyUsagePoint,
    ApiKeyUsageResponse,
    ApiKeysResponse,
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
// Module shims to mirror redis-enterprise flat module style
// This allows imports like `redis_cloud::database::CloudDatabaseHandler` similar to
// `redis_enterprise::bdb::...`, while keeping internal organization under `handlers/`.
pub mod account { pub use crate::handlers::account::*; }
pub mod acl { pub use crate::handlers::acl::*; }
pub mod api_keys { pub use crate::handlers::api_keys::*; }
pub mod backup { pub use crate::handlers::backup::*; }
pub mod billing { pub use crate::handlers::billing::*; }
pub mod cloud_accounts { pub use crate::handlers::cloud_accounts::*; }
pub mod crdb { pub use crate::handlers::crdb::*; }
pub mod database { pub use crate::handlers::database::*; }
pub mod fixed { pub use crate::handlers::fixed::*; }
pub mod logs { pub use crate::handlers::logs::*; }
pub mod metrics { pub use crate::handlers::metrics::*; }
pub mod peering { pub use crate::handlers::peering::*; }
pub mod private_service_connect { pub use crate::handlers::private_service_connect::*; }
pub mod region { pub use crate::handlers::region::*; }
pub mod sso { pub use crate::handlers::sso::*; }
pub mod subscription { pub use crate::handlers::subscription::*; }
pub mod tasks { pub use crate::handlers::tasks::*; }
pub mod transit_gateway { pub use crate::handlers::transit_gateway::*; }
pub mod users { pub use crate::handlers::users::*; }

// Expose a `types` module that re-exports models, to mirror `redis-enterprise::types`.
pub mod types;
