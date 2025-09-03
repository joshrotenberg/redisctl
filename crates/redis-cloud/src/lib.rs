//! Redis Cloud API client library
//!
//! This crate provides a comprehensive Rust client for the Redis Cloud REST API.
//!
//! ## Overview
//!
//! The library is organized into logical modules that correspond to API resource types:
//!
//! - **Account** - Current account details and operations
//! - **ACL** - Role-based access control (users, roles, Redis rules)
//! - **Cloud Accounts** - AWS/GCP/Azure cloud account management
//! - **Subscriptions** - Pro and Essentials subscription management
//! - **Databases** - Database creation, configuration and management
//! - **Connectivity** - VPC peering, Transit Gateway, Private Service Connect
//! - **Tasks** - Asynchronous operation tracking
//! - **Users** - Account user management
//!
//! ## Authentication
//!
//! The Redis Cloud API uses API key authentication:
//!
//! ```no_run
//! use redis_cloud::client::CloudClient;
//!
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret_key("your-secret-key")
//!     .build();
//! ```
//!
//! ## Example Usage
//!
//! ```no_run
//! use redis_cloud::client::CloudClient;
//! use redis_cloud::subscriptions::SubscriptionHandler;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("key")
//!     .api_secret_key("secret")
//!     .build();
//!
//! // List all subscriptions
//! let handler = SubscriptionHandler::new(client);
//! let subscriptions = handler.list().await?;
//!
//! for sub in subscriptions {
//!     println!("Subscription: {} ({})", sub.name, sub.id);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Pro vs Essentials
//!
//! Redis Cloud offers two tiers with different APIs:
//!
//! - **Pro** (`/subscriptions`) - Full-featured subscriptions with advanced options
//! - **Essentials** (`/fixed/subscriptions`) - Simplified fixed plans
//!
//! The library provides separate handlers for each tier while sharing common types.

pub mod client;
pub mod error;
pub mod types;

// Core resource modules
pub mod account;
pub mod acl;
pub mod cloud_accounts;
pub mod databases;
pub mod subscriptions;
pub mod tasks;
pub mod users;

// Connectivity modules
pub mod cidr;
pub mod maintenance_windows;
pub mod peerings;
pub mod private_service_connect;
pub mod transit_gateways;

// Essentials (fixed) modules
pub mod fixed_databases;
pub mod fixed_plans;
pub mod fixed_subscriptions;

// Utility modules
pub mod logs;
pub mod payment_methods;
pub mod regions;

pub use client::CloudClient;
pub use error::{CloudError, Result};
