//! Private Service Connect management for Redis Cloud
//!
//! ## Overview
//! - Configure Private Service Connect endpoints
//! - Manage GCP private connectivity

use crate::client::CloudClient;

/// Private Service Connect handler
pub struct PrivateServiceConnectHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl PrivateServiceConnectHandler {
    pub fn new(client: CloudClient) -> Self {
        PrivateServiceConnectHandler { client }
    }
}
