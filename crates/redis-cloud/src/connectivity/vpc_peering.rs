//! VPC Peering connectivity operations
//!
//! Manages VPC peering connections between Redis Cloud VPCs and customer VPCs
//! for both standard and Active-Active subscriptions.

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// VPC peering creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcPeeringCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Base VPC peering creation request (for backward compatibility)
pub type VpcPeeringCreateBaseRequest = VpcPeeringCreateRequest;

/// VPC peering update request for AWS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcPeeringUpdateAwsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// VPC Peering ID to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_id: Option<i32>,

    /// Optional. VPC CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cidr: Option<String>,

    /// Optional. List of VPC CIDRs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cidrs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// VPC peering update request (generic)
pub type VpcPeeringUpdateRequest = VpcPeeringUpdateAwsRequest;

/// Task state update response
pub use crate::types::TaskStateUpdate;

/// VPC Peering handler
pub struct VpcPeeringHandler {
    client: CloudClient,
}

impl VpcPeeringHandler {
    /// Create a new VPC peering handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    // ========================================================================
    // Standard VPC Peering
    // ========================================================================

    /// Get VPC peering for subscription
    pub async fn get(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!("/subscriptions/{}/peerings", subscription_id))
            .await
    }

    /// Create VPC peering
    pub async fn create(
        &self,
        subscription_id: i32,
        request: &VpcPeeringCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/peerings", subscription_id),
                request,
            )
            .await
    }

    /// Delete VPC peering
    pub async fn delete(&self, subscription_id: i32, peering_id: i32) -> Result<serde_json::Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Update VPC peering
    pub async fn update(
        &self,
        subscription_id: i32,
        peering_id: i32,
        request: &VpcPeeringCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!("/subscriptions/{}/peerings/{}", subscription_id, peering_id),
                request,
            )
            .await
    }

    // ========================================================================
    // Active-Active VPC Peering
    // ========================================================================

    /// Get Active-Active VPC peerings
    pub async fn get_active_active(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!("/subscriptions/{}/peerings", subscription_id))
            .await
    }

    /// Create Active-Active VPC peering
    pub async fn create_active_active(
        &self,
        subscription_id: i32,
        request: &VpcPeeringCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/peerings", subscription_id),
                request,
            )
            .await
    }

    /// Delete Active-Active VPC peering
    pub async fn delete_active_active(
        &self,
        subscription_id: i32,
        peering_id: i32,
    ) -> Result<serde_json::Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Update Active-Active VPC peering
    pub async fn update_active_active(
        &self,
        subscription_id: i32,
        peering_id: i32,
        request: &VpcPeeringCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!("/subscriptions/{}/peerings/{}", subscription_id, peering_id),
                request,
            )
            .await
    }
}
