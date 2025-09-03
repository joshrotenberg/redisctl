//! Service configuration for Redis Enterprise
//!
//! Overview
//! - List/get/update services
//! - Start/stop/restart, and retrieve service status

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub service_id: String,
    pub name: String,
    pub service_type: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uids: Option<Vec<u32>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Service configuration request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ServiceConfigRequest {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub node_uids: Option<Vec<u32>>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub service_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_statuses: Option<Vec<NodeServiceStatus>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Node service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeServiceStatus {
    pub node_uid: u32,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Services handler
pub struct ServicesHandler {
    client: RestClient,
}

impl ServicesHandler {
    pub fn new(client: RestClient) -> Self {
        ServicesHandler { client }
    }

    /// List all services
    pub async fn list(&self) -> Result<Vec<Service>> {
        self.client.get("/v1/services").await
    }

    /// Get specific service
    pub async fn get(&self, service_id: &str) -> Result<Service> {
        self.client
            .get(&format!("/v1/services/{}", service_id))
            .await
    }

    /// Update service configuration
    pub async fn update(&self, service_id: &str, request: ServiceConfigRequest) -> Result<Service> {
        self.client
            .put(&format!("/v1/services/{}", service_id), &request)
            .await
    }

    /// Get service status
    pub async fn status(&self, service_id: &str) -> Result<ServiceStatus> {
        self.client
            .get(&format!("/v1/services/{}/status", service_id))
            .await
    }

    /// Restart service
    pub async fn restart(&self, service_id: &str) -> Result<ServiceStatus> {
        self.client
            .post(
                &format!("/v1/services/{}/restart", service_id),
                &Value::Null,
            )
            .await
    }

    /// Stop service
    pub async fn stop(&self, service_id: &str) -> Result<ServiceStatus> {
        self.client
            .post(&format!("/v1/services/{}/stop", service_id), &Value::Null)
            .await
    }

    /// Start service
    pub async fn start(&self, service_id: &str) -> Result<ServiceStatus> {
        self.client
            .post(&format!("/v1/services/{}/start", service_id), &Value::Null)
            .await
    }

    /// Create a service - POST /v1/services
    pub async fn create(&self, body: Value) -> Result<Service> {
        self.client.post("/v1/services", &body).await
    }
}
