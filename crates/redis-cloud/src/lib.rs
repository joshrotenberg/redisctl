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
//! use redis_cloud::{CloudClient, DatabaseHandler};
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
//!     let db_handler = DatabaseHandler::new(client.clone());
//!     let databases = db_handler.get_subscription_databases(123, None, None).await?;
//!     println!("Found databases: {:?}", databases);
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
//! use redis_cloud::{CloudClient, SubscriptionHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let sub_handler = SubscriptionHandler::new(client.clone());
//!
//! // List subscriptions
//! let subscriptions = sub_handler.get_all_subscriptions().await?;
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
//! use redis_cloud::{CloudClient, DatabaseHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let db_handler = DatabaseHandler::new(client.clone());
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
//! let db_info = db_handler.get_subscription_database_by_id(123, 456).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Advanced Features
//!
//! #### VPC Peering
//! ```rust,no_run
//! use redis_cloud::{CloudClient, ConnectivityHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let peering_handler = ConnectivityHandler::new(client.clone());
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
//! use redis_cloud::{CloudClient, AccountHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let sso_handler = AccountHandler::new(client.clone());
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
//! use redis_cloud::{CloudClient, AccountHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build()?;
//!
//! let account = AccountHandler::new(client.clone());
//! let account_info = account.get_current_account().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The client provides comprehensive error handling for different failure scenarios:
//!
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudError, DatabaseHandler};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret("secret")
//!     .build().unwrap();
//!
//! let db_handler = DatabaseHandler::new(client.clone());
//!
//! match db_handler.get_subscription_database_by_id(123, 456).await {
//!     Ok(database) => println!("Database: {:?}", database),
//!     Err(CloudError::ApiError { code: 404, .. }) => {
//!         println!("Database not found");
//!     },
//!     Err(CloudError::AuthenticationFailed { message }) => {
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
//! | [`SubscriptionHandler`] | Pro subscriptions | create, list, update, delete, pricing |
//! | [`FixedSubscriptionHandler`] | Essentials subscriptions | fixed plans, create, update, delete |
//! | [`DatabaseHandler`] | Pro databases | create, backup, import, metrics, resize |
//! | [`FixedDatabaseHandler`] | Essentials databases | fixed capacity, backup, import |
//! | [`AccountHandler`] | Account management | info, API keys, payment methods, SSO |
//! | [`UserHandler`] | User management | create, update, delete, invite, roles |
//! | [`AclHandler`] | Access control | users, roles, Redis rules, database ACLs |
//! | [`ConnectivityHandler`] | Network connectivity | VPC peering, Transit Gateway, PSC |
//! | [`CloudAccountHandler`] | Cloud providers | AWS, GCP, Azure account integration |
//! | [`TaskHandler`] | Async operations | track long-running operations |
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

// Handler modules - each handles a specific API domain
pub mod account;
pub mod acl;
pub mod cloud_accounts;
pub mod connectivity;
pub mod fixed;
pub mod flexible;
pub mod tasks;
pub mod users;

// Re-export handlers with standard naming
pub use account::AccountHandler;
pub use acl::AclHandler;
pub use cloud_accounts::CloudAccountsHandler as CloudAccountHandler;

// Connectivity handlers
pub use connectivity::psc::PscHandler;
pub use connectivity::transit_gateway::TransitGatewayHandler;
pub use connectivity::vpc_peering::VpcPeeringHandler;
// Legacy connectivity export for backward compatibility
pub use connectivity::ConnectivityHandler;

// Fixed plan handlers
pub use fixed::databases::FixedDatabaseHandler;
pub use fixed::subscriptions::FixedSubscriptionHandler;
// Legacy exports for backward compatibility
pub use fixed::databases::FixedDatabaseHandler as FixedDatabasesHandler;
pub use fixed::subscriptions::FixedSubscriptionHandler as FixedSubscriptionsHandler;

// Flexible plan handlers (pay-as-you-go)
pub use flexible::databases::DatabaseHandler;
pub use flexible::subscriptions::SubscriptionHandler;
// Legacy exports for backward compatibility
pub use flexible::databases::DatabaseHandler as DatabasesHandler;
pub use flexible::subscriptions::SubscriptionHandler as SubscriptionsHandler;

pub use tasks::TasksHandler as TaskHandler;
pub use users::UsersHandler as UserHandler;

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
