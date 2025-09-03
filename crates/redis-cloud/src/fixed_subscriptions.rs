//! Subscription management for Redis Cloud Essentials
//!
//! ## Overview
//! - Manage Essentials tier subscriptions
//! - Configure fixed plan subscriptions

use crate::client::CloudClient;

/// Fixed subscription handler
pub struct FixedSubscriptionHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl FixedSubscriptionHandler {
    pub fn new(client: CloudClient) -> Self {
        FixedSubscriptionHandler { client }
    }
}
