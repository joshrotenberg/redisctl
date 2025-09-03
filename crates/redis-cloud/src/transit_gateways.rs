//! Transit Gateway management for Redis Cloud
//!
//! ## Overview
//! - Configure AWS Transit Gateway attachments
//! - Manage Transit Gateway invitations

use crate::client::CloudClient;

/// Transit Gateway handler
pub struct TransitGatewayHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl TransitGatewayHandler {
    pub fn new(client: CloudClient) -> Self {
        TransitGatewayHandler { client }
    }
}
