//! Transit Gateway operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayAttachment {
    pub id: u32,
    pub name: String,
    pub status: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayAttachmentCreateRequest {
    pub name: String,
    pub transit_gateway_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayInvitation {
    pub id: u32,
    pub status: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// Handler for Cloud Transit Gateway operations
pub struct CloudTransitGatewayHandler {
    client: CloudClient,
}

impl CloudTransitGatewayHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudTransitGatewayHandler { client }
    }

    /// List all transit gateways for a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Vec<TransitGatewayAttachment>> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/transitGateways",
                subscription_id
            ))
            .await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("transitGateways") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else if let Some(arr) = v.get("attachments") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get transit gateway attachment details
    pub async fn get_attachment(
        &self,
        subscription_id: u32,
        tgw_id: &str,
    ) -> Result<TransitGatewayAttachment> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/transitGateways/{}/attachment",
                subscription_id, tgw_id
            ))
            .await?;
        if let Some(obj) = v.get("attachment") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create transit gateway attachment
    pub async fn create_attachment(
        &self,
        subscription_id: u32,
        tgw_id: &str,
        attachment: TransitGatewayAttachmentCreateRequest,
    ) -> Result<serde_json::Value> {
        let v: serde_json::Value = self
            .client
            .post(
                &format!(
                    "/subscriptions/{}/transitGateways/{}/attachment",
                    subscription_id, tgw_id
                ),
                &attachment,
            )
            .await?;
        Ok(v)
    }

    /// Delete transit gateway attachment
    pub async fn delete_attachment(&self, subscription_id: u32, tgw_id: &str) -> Result<()> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/transitGateways/{}/attachment",
                subscription_id, tgw_id
            ))
            .await
    }

    /// List transit gateway invitations
    pub async fn list_invitations(
        &self,
        subscription_id: u32,
    ) -> Result<Vec<TransitGatewayInvitation>> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/transitGateways/invitations",
                subscription_id
            ))
            .await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("invitations") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Accept transit gateway invitation
    pub async fn accept_invitation(
        &self,
        subscription_id: u32,
        invitation_id: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/transitGateways/invitations/{}/accept",
                    subscription_id, invitation_id
                ),
                &Value::Null,
            )
            .await
    }

    /// Reject transit gateway invitation
    pub async fn reject_invitation(
        &self,
        subscription_id: u32,
        invitation_id: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/transitGateways/invitations/{}/reject",
                    subscription_id, invitation_id
                ),
                &Value::Null,
            )
            .await
    }

    /// List regional transit gateways
    pub async fn list_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
    ) -> Result<Vec<TransitGatewayAttachment>> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways",
                subscription_id, region_id
            ))
            .await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("transitGateways") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else if let Some(arr) = v.get("attachments") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get regional transit gateway attachment
    pub async fn get_regional_attachment(
        &self,
        subscription_id: u32,
        region_id: &str,
        tgw_id: &str,
    ) -> Result<TransitGatewayAttachment> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                subscription_id, region_id, tgw_id
            ))
            .await?;
        if let Some(obj) = v.get("attachment") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create regional transit gateway attachment
    pub async fn create_regional_attachment(
        &self,
        subscription_id: u32,
        region_id: &str,
        tgw_id: &str,
        attachment: TransitGatewayAttachmentCreateRequest,
    ) -> Result<serde_json::Value> {
        let v: serde_json::Value = self
            .client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                    subscription_id, region_id, tgw_id
                ),
                &attachment,
            )
            .await?;
        Ok(v)
    }

    /// Delete regional transit gateway attachment
    pub async fn delete_regional_attachment(
        &self,
        subscription_id: u32,
        region_id: &str,
        tgw_id: &str,
    ) -> Result<()> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                subscription_id, region_id, tgw_id
            ))
            .await
    }

    /// List regional transit gateway invitations
    pub async fn list_regional_invitations(
        &self,
        subscription_id: u32,
        region_id: &str,
    ) -> Result<Vec<TransitGatewayInvitation>> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/invitations",
                subscription_id, region_id
            ))
            .await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("invitations") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Accept regional transit gateway invitation
    pub async fn accept_regional_invitation(
        &self,
        subscription_id: u32,
        region_id: &str,
        invitation_id: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/invitations/{}/accept",
                    subscription_id, region_id, invitation_id
                ),
                &Value::Null,
            )
            .await
    }

    /// Reject regional transit gateway invitation
    pub async fn reject_regional_invitation(
        &self,
        subscription_id: u32,
        region_id: &str,
        invitation_id: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/invitations/{}/reject",
                    subscription_id, region_id, invitation_id
                ),
                &Value::Null,
            )
            .await
    }
}
