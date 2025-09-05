//! Fixed plan subscriptions and databases
//!
//! Fixed plans are pre-configured Redis Cloud offerings with set resources and pricing,
//! as opposed to flexible (pay-as-you-go) plans.

pub mod databases;
pub mod subscriptions;

// Re-export handlers for convenience
pub use databases::FixedDatabaseHandler;
pub use subscriptions::FixedSubscriptionHandler;
