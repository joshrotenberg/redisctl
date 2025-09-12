//! Service configuration and management
//!
//! ## Overview
//! - Configure cluster services
//! - Start/stop services
//! - Query service status

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Unique identifier for the service
    pub service_id: String,
    /// Human-readable name of the service
    pub name: String,
    /// Type of service (e.g., "mdns_server", "cm_server", "stats_archiver")
    pub service_type: String,
    /// Whether the service is enabled
    pub enabled: bool,
    /// Service-specific configuration parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    /// Current operational status of the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// List of node UIDs where this service is running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uids: Option<Vec<u32>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Service configuration request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ServiceConfigRequest {
    /// Whether to enable or disable the service
    pub enabled: bool,
    /// Service-specific configuration parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub config: Option<Value>,
    /// Specific nodes where the service should run (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub node_uids: Option<Vec<u32>>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Unique identifier for the service
    pub service_id: String,
    /// Overall status of the service (e.g., "running", "stopped", "error")
    pub status: String,
    /// Additional status message or error description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status of the service on individual nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_statuses: Option<Vec<NodeServiceStatus>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Node service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeServiceStatus {
    /// Node unique identifier where the service is running
    pub node_uid: u32,
    /// Service status on this specific node (e.g., "running", "stopped", "error")
    pub status: String,
    /// Node-specific status message or error description
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
