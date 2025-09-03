//! VPC Peering management for Redis Cloud
//!
//! ## Overview
//! - Create VPC peering connections
//! - Manage peering configurations
//! - Query peering status

use crate::client::CloudClient;
use crate::error::Result;
use crate::types::CloudProvider;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Peering information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peering {
    #[serde(rename = "peeringId")]
    pub peering_id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<CloudProvider>,

    #[serde(rename = "regionId", skip_serializing_if = "Option::is_none")]
    pub region_id: Option<u32>,

    #[serde(rename = "regionName", skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,

    #[serde(rename = "vpcId", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    #[serde(rename = "vpcCidr", skip_serializing_if = "Option::is_none")]
    pub vpc_cidr: Option<String>,

    #[serde(rename = "awsAccountId", skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,

    #[serde(rename = "gcpProjectId", skip_serializing_if = "Option::is_none")]
    pub gcp_project_id: Option<String>,

    #[serde(
        rename = "azureSubscriptionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_subscription_id: Option<String>,

    #[serde(rename = "azureTenantId", skip_serializing_if = "Option::is_none")]
    pub azure_tenant_id: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create peering request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreatePeeringRequest {
    #[serde(rename = "regionId")]
    pub region_id: u32,

    // AWS peering
    #[serde(rename = "awsAccountId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub aws_account_id: Option<String>,

    #[serde(rename = "vpcId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub vpc_id: Option<String>,

    #[serde(rename = "vpcCidr", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub vpc_cidr: Option<String>,

    // GCP peering
    #[serde(rename = "gcpProjectId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub gcp_project_id: Option<String>,

    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub network_name: Option<String>,

    // Azure peering
    #[serde(
        rename = "azureSubscriptionId",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into, strip_option))]
    pub azure_subscription_id: Option<String>,

    #[serde(rename = "azureTenantId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub azure_tenant_id: Option<String>,

    #[serde(rename = "resourceGroupName", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub resource_group_name: Option<String>,

    #[serde(rename = "vnetName", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub vnet_name: Option<String>,
}

/// Update peering request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdatePeeringRequest {
    // AWS peering
    #[serde(rename = "vpcCidr", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub vpc_cidr: Option<String>,
}

/// Peering handler
pub struct PeeringHandler {
    client: CloudClient,
}

impl PeeringHandler {
    pub fn new(client: CloudClient) -> Self {
        PeeringHandler { client }
    }

    /// List all peerings for a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Vec<Peering>> {
        self.client
            .get(&format!("/subscriptions/{}/peerings", subscription_id))
            .await
    }

    /// Get a specific peering
    pub async fn get(&self, subscription_id: u32, peering_id: u32) -> Result<Peering> {
        self.client
            .get(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await
    }

    /// Create a new peering
    pub async fn create(
        &self,
        subscription_id: u32,
        request: CreatePeeringRequest,
    ) -> Result<Peering> {
        self.client
            .post(
                &format!("/subscriptions/{}/peerings", subscription_id),
                &request,
            )
            .await
    }

    /// Update a peering
    pub async fn update(
        &self,
        subscription_id: u32,
        peering_id: u32,
        request: UpdatePeeringRequest,
    ) -> Result<Peering> {
        self.client
            .put(
                &format!("/subscriptions/{}/peerings/{}", subscription_id, peering_id),
                &request,
            )
            .await
    }

    /// Delete a peering
    pub async fn delete(&self, subscription_id: u32, peering_id: u32) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await
    }

    /// List all regional peerings
    pub async fn list_regional(&self, subscription_id: u32) -> Result<Vec<Peering>> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/peerings",
                subscription_id
            ))
            .await
    }

    /// Get a specific regional peering
    pub async fn get_regional(&self, subscription_id: u32, peering_id: u32) -> Result<Peering> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/peerings/{}",
                subscription_id, peering_id
            ))
            .await
    }

    /// Delete a regional peering
    pub async fn delete_regional(&self, subscription_id: u32, peering_id: u32) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/regions/peerings/{}",
                subscription_id, peering_id
            ))
            .await
    }
}
