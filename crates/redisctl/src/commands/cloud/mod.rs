//! Cloud command implementations
//!
//! This module contains all cloud-specific command handlers organized into submodules:
//! - `account`: Account management commands
//! - `subscription`: Subscription management commands
//! - `user`: User management commands
//! - `database`: Database management commands
//! - `utils`: Shared utilities and helper functions

pub mod account;
pub mod database;
pub mod database_impl;
pub mod subscription;
pub mod user;
pub mod utils;

// Re-export all handler functions for backward compatibility
#[allow(unused_imports)]
pub use account::handle_account_command;
#[allow(unused_imports)]
pub use database::handle_database_command;
#[allow(unused_imports)]
pub use subscription::handle_subscription_command;
#[allow(unused_imports)]
pub use user::handle_user_command;
