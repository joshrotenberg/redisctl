//! Flexible (pay-as-you-go) subscriptions and databases
//!
//! Flexible plans allow dynamic resource allocation with usage-based pricing,
//! as opposed to fixed plans with predetermined resources.

pub mod databases;
pub mod subscriptions;

// Re-export handlers for convenience
pub use databases::DatabaseHandler;
pub use subscriptions::SubscriptionHandler;
