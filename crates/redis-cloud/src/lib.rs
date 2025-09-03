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
//! ### Typed vs Raw API
//!
//! This client offers typed handlers for common operations as well as raw helpers when you
//! need full control over request/response payloads:
//!
//! - Prefer typed handlers (e.g., `CloudDatabaseHandler`) for structured, ergonomic access.
//! - Use raw helpers for passthroughs: `get_raw`, `post_raw`, `put_raw`, `patch_raw`, `delete_raw`.
//!
//! ```rust,no_run
//! use redis_cloud::CloudClient;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! // Raw call example
//! let created = client.post_raw("/subscriptions", json!({ "name": "example" })).await?;
//! println!("{}", created);
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
//!     "memoryLimitInGb": 1.0,
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
//! use redis_cloud::{CloudClient, CloudApiKeyHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let keys = CloudApiKeyHandler::new(client.clone());
//! let all = keys.list().await?; // Vec<ApiKey>
//! if let Some(first) = all.first() {
//!     let detailed = keys.get(first.id).await?;
//!     let _usage = keys.get_usage(detailed.id, "7d").await?;
//! }
//!
//! let created = keys
//!     .create(&json!({ "name": "ci-bot" }))
//!     .await?;
//! let _updated = keys
//!     .update(created.id, &json!({ "name": "ci-bot", "status": "disabled" }))
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
//!
//! Environment variables commonly used with this client:
//! - `REDIS_CLOUD_API_KEY`
//! - `REDIS_CLOUD_API_SECRET`
//! - Optional: set a custom base URL via the builder for non‑prod/test environments (defaults to `https://api.redislabs.com/v1`).

pub mod client;

#[cfg(test)]
mod lib_tests;

// Re-export client types
pub use client::{CloudClient, CloudClientBuilder};

// Types module for shared models
pub mod types;

// Handler modules will be added incrementally as we implement them from the spec
// Each module will contain the handler struct, models, and associated methods

// Re-export error types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloudError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Bad Request (400): {message}")]
    BadRequest { message: String },

    #[error("Authentication failed (401): {message}")]
    AuthenticationFailed { message: String },

    #[error("Forbidden (403): {message}")]
    Forbidden { message: String },

    #[error("Not Found (404): {message}")]
    NotFound { message: String },

    #[error("Precondition Failed (412): Feature flag for this flow is off")]
    PreconditionFailed,

    #[error("Internal Server Error (500): {message}")]
    InternalServerError { message: String },

    #[error("Service Unavailable (503): {message}")]
    ServiceUnavailable { message: String },

    #[error("API error ({code}): {message}")]
    ApiError { code: u16, message: String },

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CloudError>;
