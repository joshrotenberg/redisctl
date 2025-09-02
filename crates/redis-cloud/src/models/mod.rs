//! Cloud API data models
//!
//! This module contains all data structures used for Redis Cloud API requests and responses.
//! Models are organized by functional area and include both request/response types and
//! configuration structures.
//!
//! # Module Organization
//!
//! - [`account`] - Account information, users, and payment method models
//! - [`acl`] - ACL users, roles, and Redis rules models
//! - [`backup`] - Database backup and restore operation models
//! - [`billing`] - Billing information, invoices, and payment models  
//! - [`database`] - Database configuration, status, and operational models
//! - [`metrics`] - Performance metrics, measurements, and monitoring models
//! - [`peering`] - VPC peering connection and networking models
//! - [`subscription`] - Subscription management and cloud provider models
//! - [`users`] - User management models
//!
//! # Common Patterns
//!
//! Most models follow consistent patterns:
//! - Request models: `Create*Request`, `Update*Request` for API inputs
//! - Response models: Plain struct names like `CloudDatabase`, `CloudSubscription`
//! - Configuration models: `*Config` for nested configuration objects
//!
//! All models implement `Serialize` and `Deserialize` for JSON handling and many
//! include `Debug`, `Clone`, and other useful derives.

pub mod account;
pub mod acl;
pub mod backup;
pub mod billing;
pub mod database;
pub mod logs;
pub mod metrics;
pub mod peering;
pub mod subscription;
pub mod users;

// Re-export all models
pub use account::*;
pub use acl::*;
pub use backup::*;
pub use billing::*;
pub use database::*;
pub use logs::*;
pub use metrics::*;
pub use peering::*;
pub use subscription::*;
pub use users::*;
