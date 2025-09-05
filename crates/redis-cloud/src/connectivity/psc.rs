//! Private Service Connect (PSC) operations
//!
//! Manages Google Cloud Private Service Connect endpoints for secure connectivity
//! to Redis Cloud databases without traversing the public internet.

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};

/// Private Service Connect endpoint update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PscEndpointUpdateRequest {
    pub subscription_id: i32,
    pub psc_service_id: i32,
    pub endpoint_id: i32,

    /// Google Cloud project ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_project_id: Option<String>,

    /// Name of the Google Cloud VPC that hosts your application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_name: Option<String>,

    /// Name of your VPC's subnet of IP address ranges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_subnet_name: Option<String>,

    /// Prefix used to create PSC endpoints in the consumer application VPC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_connection_name: Option<String>,
}

/// Task state update response
pub use crate::types::TaskStateUpdate;

/// Private Service Connect handler
pub struct PscHandler {
    client: CloudClient,
}

impl PscHandler {
    /// Create a new PSC handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    // ========================================================================
    // Standard PSC Operations
    // ========================================================================

    /// Delete Private Service Connect service
    pub async fn delete_service(&self, subscription_id: i32) -> Result<serde_json::Value> {
        let _response = self
            .client
            .delete(&format!(
                "/subscriptions/{}/private-service-connect",
                subscription_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Get Private Service Connect service
    pub async fn get_service(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect",
                subscription_id
            ))
            .await
    }

    /// Create Private Service Connect service
    pub async fn create_service(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/private-service-connect", subscription_id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Get Private Service Connect endpoints
    pub async fn get_endpoints(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/endpoints",
                subscription_id
            ))
            .await
    }

    /// Create Private Service Connect endpoint
    pub async fn create_endpoint(
        &self,
        subscription_id: i32,
        request: &PscEndpointUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/private-service-connect/endpoints",
                    subscription_id
                ),
                request,
            )
            .await
    }

    /// Delete Private Service Connect endpoint
    pub async fn delete_endpoint(
        &self,
        subscription_id: i32,
        endpoint_id: i32,
    ) -> Result<serde_json::Value> {
        let _response = self
            .client
            .delete(&format!(
                "/subscriptions/{}/private-service-connect/endpoints/{}",
                subscription_id, endpoint_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Update Private Service Connect endpoint
    pub async fn update_endpoint(
        &self,
        subscription_id: i32,
        endpoint_id: i32,
        request: &PscEndpointUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/private-service-connect/endpoints/{}",
                    subscription_id, endpoint_id
                ),
                request,
            )
            .await
    }

    /// Get PSC endpoint creation script
    pub async fn get_endpoint_creation_script(
        &self,
        subscription_id: i32,
        endpoint_id: i32,
    ) -> Result<String> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/endpoints/{}/creationScripts",
                subscription_id, endpoint_id
            ))
            .await
    }

    /// Get PSC endpoint deletion script
    pub async fn get_endpoint_deletion_script(
        &self,
        subscription_id: i32,
        endpoint_id: i32,
    ) -> Result<String> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/endpoints/{}/deletionScripts",
                subscription_id, endpoint_id
            ))
            .await
    }

    // ========================================================================
    // Active-Active PSC Operations
    // ========================================================================

    /// Delete Active-Active PSC service
    pub async fn delete_service_active_active(
        &self,
        subscription_id: i32,
    ) -> Result<serde_json::Value> {
        let _response = self
            .client
            .delete(&format!(
                "/subscriptions/{}/regions/private-service-connect",
                subscription_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Get Active-Active PSC service
    pub async fn get_service_active_active(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/private-service-connect",
                subscription_id
            ))
            .await
    }

    /// Create Active-Active PSC service
    pub async fn create_service_active_active(
        &self,
        subscription_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/private-service-connect",
                    subscription_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Get Active-Active PSC endpoints
    pub async fn get_endpoints_active_active(
        &self,
        subscription_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/private-service-connect/endpoints",
                subscription_id
            ))
            .await
    }

    /// Create Active-Active PSC endpoint
    pub async fn create_endpoint_active_active(
        &self,
        subscription_id: i32,
        request: &PscEndpointUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/private-service-connect/endpoints",
                    subscription_id
                ),
                request,
            )
            .await
    }

    /// Delete Active-Active PSC endpoint
    pub async fn delete_endpoint_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        endpoint_id: i32,
    ) -> Result<serde_json::Value> {
        let _response = self
            .client
            .delete(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/endpoints/{}",
                subscription_id, region_id, endpoint_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Update Active-Active PSC endpoint
    pub async fn update_endpoint_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        endpoint_id: i32,
        request: &PscEndpointUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}",
                    subscription_id, region_id, subscription_id, endpoint_id
                ),
                request,
            )
            .await
    }

    /// Get Active-Active PSC endpoint creation script
    pub async fn get_endpoint_creation_script_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<String> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/creationScripts",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get Active-Active PSC endpoint deletion script
    pub async fn get_endpoint_deletion_script_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<String> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/deletionScripts",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await
    }
}
