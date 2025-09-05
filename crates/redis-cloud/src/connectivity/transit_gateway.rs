//! AWS Transit Gateway operations
//!
//! Manages AWS Transit Gateway attachments for hub-and-spoke network topologies,
//! enabling centralized connectivity management for Redis Cloud subscriptions.

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CIDR block definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cidr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_address: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Transit Gateway CIDRs update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TgwUpdateCidrsRequest {
    /// Optional. List of transit gateway attachment CIDRs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<Cidr>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Transit Gateway attachment request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TgwAttachmentRequest {
    /// AWS account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,

    /// Transit Gateway ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgw_id: Option<String>,

    /// CIDR blocks to route through the TGW
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,

    /// Additional fields
    #[serde(flatten)]
    pub extra: Value,
}

/// Task state update response
pub use crate::types::TaskStateUpdate;

/// Transit Gateway handler
pub struct TransitGatewayHandler {
    client: CloudClient,
}

impl TransitGatewayHandler {
    /// Create a new Transit Gateway handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    // ========================================================================
    // Standard Transit Gateway Operations
    // ========================================================================

    /// Get Transit Gateway attachments
    pub async fn get_attachments(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!("/subscriptions/{}/tgw", subscription_id))
            .await
    }

    /// Get Transit Gateway shared invitations
    pub async fn get_shared_invitations(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/tgw/shared-invitations",
                subscription_id
            ))
            .await
    }

    /// Accept Transit Gateway resource share
    pub async fn accept_resource_share(
        &self,
        subscription_id: i32,
        invitation_id: String,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/tgw/shared-invitations/{}/accept",
                    subscription_id, invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reject Transit Gateway resource share
    pub async fn reject_resource_share(
        &self,
        subscription_id: i32,
        invitation_id: String,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/tgw/shared-invitations/{}/reject",
                    subscription_id, invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete Transit Gateway attachment
    pub async fn delete_attachment(
        &self,
        subscription_id: i32,
        attachment_id: String,
    ) -> Result<serde_json::Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/tgw/attachments/{}",
                subscription_id, attachment_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Create Transit Gateway attachment
    pub async fn create_attachment(
        &self,
        subscription_id: i32,
        request: &TgwAttachmentRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/tgw/attachments", subscription_id),
                request,
            )
            .await
    }

    /// Update Transit Gateway attachment CIDRs
    pub async fn update_attachment_cidrs(
        &self,
        subscription_id: i32,
        attachment_id: String,
        request: &TgwAttachmentRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/tgw/attachments/{}/cidrs",
                    subscription_id, attachment_id
                ),
                request,
            )
            .await
    }

    // ========================================================================
    // Active-Active Transit Gateway Operations
    // ========================================================================

    /// Get Active-Active Transit Gateway attachments
    pub async fn get_attachments_active_active(
        &self,
        subscription_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!("/subscriptions/{}/regions/tgw", subscription_id))
            .await
    }

    /// Get Active-Active Transit Gateway shared invitations
    pub async fn get_shared_invitations_active_active(
        &self,
        subscription_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/tgw/shared-invitations",
                subscription_id
            ))
            .await
    }

    /// Accept Active-Active Transit Gateway resource share
    pub async fn accept_resource_share_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        invitation_id: String,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/tgw/shared-invitations/{}/accept",
                    subscription_id, region_id, invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reject Active-Active Transit Gateway resource share
    pub async fn reject_resource_share_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        invitation_id: String,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/tgw/shared-invitations/{}/reject",
                    subscription_id, region_id, invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete Active-Active Transit Gateway attachment
    pub async fn delete_attachment_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        attachment_id: String,
    ) -> Result<serde_json::Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/regions/{}/tgw/attachments/{}",
                subscription_id, region_id, attachment_id
            ))
            .await?;
        Ok(serde_json::Value::Null)
    }

    /// Create Active-Active Transit Gateway attachment
    pub async fn create_attachment_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        request: &TgwAttachmentRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/tgw/attachments",
                    subscription_id, region_id
                ),
                request,
            )
            .await
    }

    /// Update Active-Active Transit Gateway attachment CIDRs
    pub async fn update_attachment_cidrs_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        attachment_id: String,
        request: &TgwAttachmentRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/tgw/attachments/{}/cidrs",
                    subscription_id, region_id, attachment_id
                ),
                request,
            )
            .await
    }
}
