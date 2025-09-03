//! Payment method management for Redis Cloud
//!
//! ## Overview
//! - Query payment methods
//! - Manage billing information

use crate::client::CloudClient;

/// Payment method handler
pub struct PaymentMethodHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl PaymentMethodHandler {
    pub fn new(client: CloudClient) -> Self {
        PaymentMethodHandler { client }
    }
}
