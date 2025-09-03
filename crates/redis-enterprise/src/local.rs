//! Local node operations and health checks
//!
//! ## Overview
//! - Query local node status
//! - Perform health checks
//! - Manage local services

use crate::client::RestClient;
use crate::error::Result;
use serde_json::Value;

pub struct LocalHandler {
    client: RestClient,
}

impl LocalHandler {
    pub fn new(client: RestClient) -> Self {
        LocalHandler { client }
    }

    /// Master healthcheck for local node - GET /v1/local/node/master_healthcheck
    pub async fn master_healthcheck(&self) -> Result<Value> {
        self.client.get("/v1/local/node/master_healthcheck").await
    }

    /// List local services - GET /v1/local/services
    pub async fn services(&self) -> Result<Value> {
        self.client.get("/v1/local/services").await
    }

    /// Create/update local services - POST /v1/local/services
    pub async fn services_update(&self, body: Value) -> Result<Value> {
        self.client.post("/v1/local/services", &body).await
    }
}
