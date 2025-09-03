//! VPC Peering operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudPeering {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub peering_id: String,
    pub subscription_id: u32,
    pub provider_peering_id: Option<String>,
    pub aws_account_id: Option<String>,
    pub vpc_id: Option<String>,
    pub vpc_cidr: Option<String>,
    pub region: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePeeringRequest {
    pub subscription_id: u32,
    pub provider: String,
    pub aws_account_id: Option<String>,
    pub vpc_id: String,
    pub vpc_cidr: String,
    pub region: String,
}

/// Handler for Cloud peering operations
pub struct CloudPeeringHandler {
    client: CloudClient,
}

impl CloudPeeringHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudPeeringHandler { client }
    }

    /// List all peerings for a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Vec<CloudPeering>> {
        let response: Value = self
            .client
            .get(&format!("/subscriptions/{}/peerings", subscription_id))
            .await?;

        if let Some(peerings) = response.get("peerings") {
            serde_json::from_value(peerings.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Create a new peering
    pub async fn create(&self, request: CreatePeeringRequest) -> Result<CloudPeering> {
        self.client
            .post(
                &format!("/subscriptions/{}/peerings", request.subscription_id),
                &request,
            )
            .await
    }

    /// Get peering details
    pub async fn get(&self, subscription_id: u32, peering_id: &str) -> Result<CloudPeering> {
        self.client
            .get(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await
    }

    /// Delete peering
    pub async fn delete(&self, subscription_id: u32, peering_id: &str) -> Result<()> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await
    }
}
