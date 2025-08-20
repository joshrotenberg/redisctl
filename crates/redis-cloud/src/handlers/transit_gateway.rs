//! Transit Gateway operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud Transit Gateway operations
pub struct CloudTransitGatewayHandler {
    client: CloudClient,
}

impl CloudTransitGatewayHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudTransitGatewayHandler { client }
    }

    /// List all transit gateways for a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/transitGateways",
                subscription_id
            ))
            .await
    }

    /// Get transit gateway attachment details
    pub async fn get_attachment(&self, subscription_id: u32, tgw_id: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/transitGateways/{}/attachment",
                subscription_id, tgw_id
            ))
            .await
    }

    /// Create transit gateway attachment
    pub async fn create_attachment(
        &self,
        subscription_id: u32,
        tgw_id: &str,
        attachment: Value,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/transitGateways/{}/attachment",
                    subscription_id, tgw_id
                ),
                &attachment,
            )
            .await
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
    pub async fn list_invitations(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/transitGateways/invitations",
                subscription_id
            ))
            .await
    }

    /// Accept transit gateway invitation
    pub async fn accept_invitation(
        &self,
        subscription_id: u32,
        invitation_id: &str,
    ) -> Result<Value> {
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
    ) -> Result<Value> {
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
    pub async fn list_regional(&self, subscription_id: u32, region_id: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways",
                subscription_id, region_id
            ))
            .await
    }

    /// Get regional transit gateway attachment
    pub async fn get_regional_attachment(
        &self,
        subscription_id: u32,
        region_id: &str,
        tgw_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                subscription_id, region_id, tgw_id
            ))
            .await
    }

    /// Create regional transit gateway attachment
    pub async fn create_regional_attachment(
        &self,
        subscription_id: u32,
        region_id: &str,
        tgw_id: &str,
        attachment: Value,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                    subscription_id, region_id, tgw_id
                ),
                &attachment,
            )
            .await
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
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/invitations",
                subscription_id, region_id
            ))
            .await
    }

    /// Accept regional transit gateway invitation
    pub async fn accept_regional_invitation(
        &self,
        subscription_id: u32,
        region_id: &str,
        invitation_id: &str,
    ) -> Result<Value> {
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
    ) -> Result<Value> {
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
