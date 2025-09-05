//! Network connectivity and peering operations for Pro subscriptions
//!
//! This module manages advanced networking features for Redis Cloud Pro subscriptions,
//! including VPC peering, AWS Transit Gateway attachments, GCP Private Service Connect,
//! and other cloud-native networking integrations.
//!
//! # Overview
//!
//! Connectivity features enable secure, private network connections between your
//! Redis Cloud databases and your application infrastructure. This eliminates the
//! need for public internet routing and provides lower latency.
//!
//! # Supported Connectivity Types
//!
//! - **VPC Peering**: Direct peering between Redis Cloud VPC and your VPC
//! - **Transit Gateway**: AWS Transit Gateway attachments for hub-and-spoke topologies
//! - **Private Service Connect**: GCP Private Service Connect for private endpoints
//! - **PrivateLink**: AWS PrivateLink endpoints (coming soon)
//!
//! # Key Features
//!
//! - **VPC Peering Management**: Create, update, and delete VPC peering connections
//! - **Transit Gateway Attachments**: Manage AWS TGW attachments and CIDR blocks
//! - **Private Service Connect**: Configure GCP PSC endpoints and service attachments
//! - **Multi-region Support**: Handle connectivity across different cloud regions
//! - **Status Monitoring**: Track connection status and health
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, ConnectivityHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = ConnectivityHandler::new(client);
//!
//! // Get VPC peering for a subscription (subscription ID 123)
//! let peerings = handler.get_vpc_peering(123).await?;
//!
//! // Get Transit Gateway attachments
//! let tgw_attachments = handler.get_tgws(123).await?;
//! # Ok(())
//! # }
//! ```

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Models
// ============================================================================

/// Vpc peering creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcPeeringCreateBaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Private Service Connect endpoint update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PscEndpointUpdateRequest {
    pub subscription_id: i32,

    pub psc_service_id: i32,

    pub endpoint_id: i32,

    /// Google Cloud project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_project_id: Option<String>,

    /// Name of the Google Cloud VPC that hosts your application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_name: Option<String>,

    /// Name of your VPC's subnet of IP address ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_subnet_name: Option<String>,

    /// Prefix used to create PSC endpoints in the consumer application VPC. Endpoint names appear in Google Cloud as endpoint name prefix + endpoint number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_connection_name: Option<String>,

    /// Action to perform on the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// ProcessorResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_resource_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Active-Active VPC peering creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveVpcPeeringCreateBaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Name of region to create a VPC peering from.
    pub source_region: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Private Service Connect endpoint create request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActivePscEndpointCreateRequest {
    pub subscription_id: i32,

    pub psc_service_id: i32,

    /// Deployment region id as defined by cloud provider
    pub region_id: i32,

    /// Google Cloud project ID.
    pub gcp_project_id: String,

    /// Name of the Google Cloud VPC that hosts your application.
    pub gcp_vpc_name: String,

    /// Name of your VPC's subnet of IP address ranges.
    pub gcp_vpc_subnet_name: String,

    /// Prefix used to create PSC endpoints in the consumer application VPC. Endpoint names appear in Google Cloud as endpoint name prefix + endpoint number.
    pub endpoint_connection_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Active-Active VPC peering update request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveVpcPeeringUpdateAwsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// VPC Peering id to update.
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

/// VPC peering creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveVpcPeeringCreateGcpRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Name of region to create a VPC peering from.
    pub source_region: String,

    /// VPC project ID.
    pub vpc_project_uid: String,

    /// VPC network name.
    pub vpc_network_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// VPC peering creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveVpcPeeringCreateAwsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Name of region to create a VPC peering from.
    pub source_region: String,

    /// Name of region to create a VPC peering to.
    pub destination_region: String,

    /// AWS Account ID.
    pub aws_account_id: String,

    /// VPC ID.
    pub vpc_id: String,

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

/// Active active Transit Gateway update attachment cidr/s request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveTgwUpdateCidrsRequest {
    /// Optional. List of transit gateway attachment CIDRs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<Cidr>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Private Service Connect endpoint update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActivePscEndpointUpdateRequest {
    pub subscription_id: i32,

    pub psc_service_id: i32,

    pub endpoint_id: i32,

    /// Deployment region id as defined by cloud provider
    pub region_id: i32,

    /// Google Cloud project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_project_id: Option<String>,

    /// Name of the Google Cloud VPC that hosts your application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_name: Option<String>,

    /// Name of your VPC's subnet of IP address ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_subnet_name: Option<String>,

    /// Prefix used to create PSC endpoints in the consumer application VPC. Endpoint names appear in Google Cloud as endpoint name prefix + endpoint number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_connection_name: Option<String>,

    /// Action to perform on the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// VPC peering update request message
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

/// VPC peering creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcPeeringCreateAwsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Deployment region as defined by the cloud provider.
    pub region: String,

    /// AWS Account ID.
    pub aws_account_id: String,

    /// VPC ID.
    pub vpc_id: String,

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

/// Private Service Connect endpoint create request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PscEndpointCreateRequest {
    pub subscription_id: i32,

    pub psc_service_id: i32,

    /// Google Cloud project ID.
    pub gcp_project_id: String,

    /// Name of the Google Cloud VPC that hosts your application.
    pub gcp_vpc_name: String,

    /// Name of your VPC's subnet of IP address ranges.
    pub gcp_vpc_subnet_name: String,

    /// Prefix used to create PSC endpoints in the consumer application VPC. Endpoint names appear in Google Cloud as endpoint name prefix + endpoint number.
    pub endpoint_connection_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. List of transit gateway attachment CIDRs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cidr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_address: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Vpc peering creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcPeeringCreateGcpRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// VPC project ID.
    pub vpc_project_uid: String,

    /// VPC network name.
    pub vpc_network_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Transit Gateway update attachment cidr/s request message
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

/// TaskStateUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ProcessorResponse>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Handler for network connectivity operations
///
/// Manages VPC peering, Transit Gateway attachments, Private Service Connect,
/// and other advanced networking features for Pro subscriptions.
pub struct ConnectivityHandler {
    client: CloudClient,
}

impl ConnectivityHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get VPC peering details
    /// Gets VPC peering details for the specified subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/peerings
    pub async fn get_vpc_peering(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!("/subscriptions/{}/peerings", subscription_id))
            .await
    }

    /// Create VPC peering
    /// Sets up VPC peering for the specified subscription. Ensure your cloud provider is also set up for VPC Peering with Redis Cloud. See [VPC Peering](https://docs.redis.com/latest/rc/security/vpc-peering) to learn how to set up VPC Peering with AWS and Google Cloud.
    ///
    /// POST /subscriptions/{subscriptionId}/peerings
    pub async fn create_vpc_peering(
        &self,
        subscription_id: i32,
        request: &VpcPeeringCreateBaseRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/peerings", subscription_id),
                request,
            )
            .await
    }

    /// Delete VPC peering
    /// Deletes the specified VPC peering.
    ///
    /// DELETE /subscriptions/{subscriptionId}/peerings/{peeringId}
    pub async fn delete_vpc_peering(
        &self,
        subscription_id: i32,
        peering_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/peerings/{}",
                subscription_id, peering_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update VPC peering
    /// Updates VPC peering for the specified subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/peerings/{peeringId}
    pub async fn update_vpc_peering(
        &self,
        subscription_id: i32,
        peering_id: i32,
        request: &VpcPeeringUpdateAwsRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!("/subscriptions/{}/peerings/{}", subscription_id, peering_id),
                request,
            )
            .await
    }

    /// Remove Private Service Connect for a subscription
    /// Deletes Private Service Connect for a subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/private-service-connect
    pub async fn delete_psc_service(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/private-service-connect",
                subscription_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get Private Service Connect
    /// Gets Private Service Connect details for a subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/private-service-connect
    pub async fn get_psc_service(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect",
                subscription_id
            ))
            .await
    }

    /// Set up Private Service Connect for a subscription
    /// Sets up Google Cloud Private Service Connect for an existing subscription hosted on Google Cloud.
    ///
    /// POST /subscriptions/{subscriptionId}/private-service-connect
    pub async fn create_psc_service(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/private-service-connect", subscription_id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Get Private Service Connect endpoints
    /// Gets endpoint details for the specified Private Service Connect.
    ///
    /// GET /subscriptions/{subscriptionId}/private-service-connect/{pscServiceId}
    pub async fn get_psc_service_endpoints(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}",
                subscription_id, psc_service_id
            ))
            .await
    }

    /// Create a Private Service Connect endpoint
    /// Creates a new Private Service Connect endpoint.
    ///
    /// POST /subscriptions/{subscriptionId}/private-service-connect/{pscServiceId}
    pub async fn create_psc_service_endpoint(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        request: &PscEndpointCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/private-service-connect/{}",
                    subscription_id, psc_service_id
                ),
                request,
            )
            .await
    }

    /// Delete a Private Service Connect endpoint
    /// Deletes the specified Private Service Connect endpoint.
    ///
    /// DELETE /subscriptions/{subscriptionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}
    pub async fn delete_psc_service_endpoint(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update a Private Service Connect endpoint
    /// Updates the specified Private Service Connect endpoint.
    ///
    /// PUT /subscriptions/{subscriptionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}
    pub async fn update_psc_service_endpoint(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
        request: &PscEndpointUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/private-service-connect/{}/endpoints/{}",
                    subscription_id, psc_service_id, endpoint_id
                ),
                request,
            )
            .await
    }

    /// Get Private Service Connect endpoint creation script
    /// Gets the gcloud script to create the specified Private Service Connect endpoint on Google Cloud.
    ///
    /// GET /subscriptions/{subscriptionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}/creationScripts
    pub async fn get_psc_service_endpoint_creation_script(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}/creationScripts",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get Private Service Connect endpoint deletion script
    /// Gets the gcloud script to delete the specified Private Service Connect endpoint on Google Cloud.
    ///
    /// GET /subscriptions/{subscriptionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}/deletionScripts
    pub async fn get_psc_service_endpoint_deletion_script(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}/deletionScripts",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get Active-Active VPC peering details
    /// (Active-Active subscriptions only) Gets VPC peering details for an Active-Active subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/peerings
    pub async fn get_active_active_vpc_peerings(
        &self,
        subscription_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/peerings",
                subscription_id
            ))
            .await
    }

    /// Create Active-Active VPC peering
    /// (Active-Active subscriptions only) Sets up VPC peering for an Active-Active subscription. Ensure your cloud provider is also set up for VPC Peering with Redis Cloud. See [VPC Peering](https://docs.redis.com/latest/rc/security/vpc-peering) to learn how to set up VPC Peering with AWS and Google Cloud.
    ///
    /// POST /subscriptions/{subscriptionId}/regions/peerings
    pub async fn create_active_active_vpc_peering(
        &self,
        subscription_id: i32,
        request: &ActiveActiveVpcPeeringCreateBaseRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/regions/peerings", subscription_id),
                request,
            )
            .await
    }

    /// Delete Active-Active VPC peering
    /// (Active-Active subscriptions only) Deletes VPC peering for an Active-Active subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/regions/peerings/{peeringId}
    pub async fn delete_active_active_vpc_peering(
        &self,
        subscription_id: i32,
        peering_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/regions/peerings/{}",
                subscription_id, peering_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update Active-Active VPC peering
    /// (Active-Active subscriptions only) Updates VPC peering for Active-Active subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/regions/peerings/{peeringId}
    pub async fn update_active_active_vpc_peering(
        &self,
        subscription_id: i32,
        peering_id: i32,
        request: &ActiveActiveVpcPeeringUpdateAwsRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/peerings/{}",
                    subscription_id, peering_id
                ),
                request,
            )
            .await
    }

    /// Remove Private Service Connect for a single region
    /// (Active-Active subscriptions only) Deletes a Private Service Connect for a single region in an existing Active-Active subscription hosted on Google Cloud.
    ///
    /// DELETE /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect
    pub async fn delete_active_active_psc_service(
        &self,
        subscription_id: i32,
        region_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect",
                subscription_id, region_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get Private Service Connect for a single region
    /// (Active-Active subscriptions only) Gets Private Service Connect details for a single region in an Active-Active subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect
    pub async fn get_active_active_psc_service(
        &self,
        subscription_id: i32,
        region_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect",
                subscription_id, region_id
            ))
            .await
    }

    /// Set up a single region Private Service Connect
    /// (Active-Active subscriptions only) Sets up Google Cloud Private Service Connect for a single region in an existing Active-Active subscription hosted on Google Cloud.
    ///
    /// POST /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect
    pub async fn create_active_active_psc_service(
        &self,
        subscription_id: i32,
        region_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect",
                    subscription_id, region_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Get Private Service Connect endpoints for a single region
    /// (Active-Active subscriptions only) Gets endpoint details for the specified Private Service Connect in a single region in an Active-Active subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect/{pscServiceId}
    pub async fn get_active_active_psc_service_endpoints(
        &self,
        subscription_id: i32,
        region_id: i32,
        psc_service_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}",
                subscription_id, region_id, psc_service_id
            ))
            .await
    }

    /// Create an Private Service Connect endpoint for a single region
    /// (Active-Active subscriptions only) Creates a new Private Service Connect endpoint for a single region in an Active-Active subscription.
    ///
    /// POST /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect/{pscServiceId}
    pub async fn create_active_active_psc_service_endpoint(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        region_id: i32,
        request: &ActiveActivePscEndpointCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect/{}",
                    subscription_id, psc_service_id, region_id
                ),
                request,
            )
            .await
    }

    /// Delete a Private Service Connect endpoint for a single region
    /// (Active-Active subscriptions only) Deletes the specified Private Service Connect endpoint for a single region in an Active-Active subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}
    pub async fn delete_active_active_psc_service_endpoint(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        region_id: i32,
        endpoint_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}",
                subscription_id, psc_service_id, region_id, endpoint_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update a Private Service Connect endpoint for a single region
    /// (Active-Active subscriptions only) Updates a Private Service Connect endpoint for a single region in an Active-Active subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}
    pub async fn update_active_active_psc_service_endpoint(
        &self,
        subscription_id: i32,
        psc_service_id: i32,
        region_id: i32,
        endpoint_id: i32,
        request: &ActiveActivePscEndpointUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}",
                    subscription_id, psc_service_id, region_id, endpoint_id
                ),
                request,
            )
            .await
    }

    /// Get Private Service Connect endpoint creation script for a single region
    /// (Active-Active subscriptions only) Gets the gcloud script to create the specified Private Service Connect endpoint on Google Cloud.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}/creationScripts
    pub async fn get_active_active_psc_service_endpoint_creation_script(
        &self,
        subscription_id: i32,
        region_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client.get(&format!("/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/creationScripts", subscription_id, region_id, psc_service_id, endpoint_id)).await
    }

    /// Get Private Service Connect endpoint deletion script for a single region
    /// (Active-Active subscriptions only) Gets the gcloud script to delete the specified Private Service Connect endpoint on Google Cloud.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/private-service-connect/{pscServiceId}/endpoints/{endpointId}/deletionScripts
    pub async fn get_active_active_psc_service_endpoint_deletion_script(
        &self,
        subscription_id: i32,
        region_id: i32,
        psc_service_id: i32,
        endpoint_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client.get(&format!("/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/deletionScripts", subscription_id, region_id, psc_service_id, endpoint_id)).await
    }

    /// Get transit gateways for a single region
    /// (Active-Active subscriptions only) Gets all AWS transit gateway details for the specified region in an Active-Active subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways
    pub async fn get_active_active_tgws(
        &self,
        subscription_id: i32,
        region_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways",
                subscription_id, region_id
            ))
            .await
    }

    /// Get transit gateway invitations for a single region
    /// (Active-Active subscriptions only) Gets AWS transit gateway invitations for the specified region in an Active-Active subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways/invitations
    pub async fn get_active_active_tgw_shared_invitations(
        &self,
        subscription_id: i32,
        region_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/invitations",
                subscription_id, region_id
            ))
            .await
    }

    /// Accept a transit gateway resource share for a specific region
    /// (Active-Active subscriptions only) Accepts the specified AWS transit gateway resource share for one region in an Active-Active subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways/invitations/{tgwInvitationId}/accept
    pub async fn accept_active_active_tgw_resource_share(
        &self,
        subscription_id: i32,
        region_id: i32,
        tgw_invitation_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/invitations/{}/accept",
                    subscription_id, region_id, tgw_invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reject transit gateway resource share for a specific region
    /// (Active-Active subscriptions only) Rejects the specified AWS transit gateway resource share for one region in an Active-Active subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways/invitations/{tgwInvitationId}/reject
    pub async fn reject_active_active_tgw_resource_share(
        &self,
        subscription_id: i32,
        region_id: i32,
        tgw_invitation_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/invitations/{}/reject",
                    subscription_id, region_id, tgw_invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete a transit gateway attachment for a specific region
    /// (Active-Active subscriptions only) Deletes the specified AWS transit gateway attachment for one region in an Active-Active subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways/{TgwId}/attachment
    pub async fn delete_active_active_tgw_attachment(
        &self,
        subscription_id: i32,
        region_id: i32,
        tgw_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                subscription_id, region_id, tgw_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Create a transit gateway attachment for a specific region
    /// (Active-Active subscriptions only) Creates an AWS transit gateway attachment for one region in an Active-Active subscription.
    ///
    /// POST /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways/{TgwId}/attachment
    pub async fn create_active_active_tgw_attachment(
        &self,
        subscription_id: i32,
        region_id: i32,
        tgw_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                    subscription_id, region_id, tgw_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Update a specific region transit gateway attachment
    /// (Active-Active subscriptions only) Updates the specified AWS transit gateway attachment for one region in an Active-Active subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/regions/{regionId}/transitGateways/{TgwId}/attachment
    pub async fn update_active_active_tgw_attachment_cidrs(
        &self,
        subscription_id: i32,
        region_id: i32,
        tgw_id: i32,
        request: &ActiveActiveTgwUpdateCidrsRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/transitGateways/{}/attachment",
                    subscription_id, region_id, tgw_id
                ),
                request,
            )
            .await
    }

    /// Get transit gateways for a subscription
    /// Gets all AWS transit gateway details for the specified subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/transitGateways
    pub async fn get_tgws(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/transitGateways",
                subscription_id
            ))
            .await
    }

    /// Get transit gateway invitations for a subscription
    /// Gets all AWS transit gateway invitations for the specified subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/transitGateways/invitations
    pub async fn get_tgw_shared_invitations(
        &self,
        subscription_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/transitGateways/invitations",
                subscription_id
            ))
            .await
    }

    /// Accept a transit gateway resource share
    /// Accepts the specified AWS transit gateway resource share.
    ///
    /// PUT /subscriptions/{subscriptionId}/transitGateways/invitations/{tgwInvitationId}/accept
    pub async fn accept_tgw_resource_share(
        &self,
        subscription_id: i32,
        tgw_invitation_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/transitGateways/invitations/{}/accept",
                    subscription_id, tgw_invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reject a transit gateway resource share
    /// Rejects the specified AWS transit gateway resource share.
    ///
    /// PUT /subscriptions/{subscriptionId}/transitGateways/invitations/{tgwInvitationId}/reject
    pub async fn reject_tgw_resource_share(
        &self,
        subscription_id: i32,
        tgw_invitation_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/transitGateways/invitations/{}/reject",
                    subscription_id, tgw_invitation_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete a transit gateway attachment
    /// Deletes the specified AWS transit gateway attachment.
    ///
    /// DELETE /subscriptions/{subscriptionId}/transitGateways/{TgwId}/attachment
    pub async fn delete_tgw_attachment(
        &self,
        subscription_id: i32,
        tgw_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/transitGateways/{}/attachment",
                subscription_id, tgw_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Create a transit gateway attachment
    /// Creates an AWS transit gateway attachment.
    ///
    /// POST /subscriptions/{subscriptionId}/transitGateways/{TgwId}/attachment
    pub async fn create_tgw_attachment(
        &self,
        subscription_id: i32,
        tgw_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/transitGateways/{}/attachment",
                    subscription_id, tgw_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Update a transit gateway attachment
    /// Updates the specified AWS transit gateway attachment.
    ///
    /// PUT /subscriptions/{subscriptionId}/transitGateways/{TgwId}/attachment
    pub async fn update_tgw_attachment_cidrs(
        &self,
        subscription_id: i32,
        tgw_id: i32,
        request: &TgwUpdateCidrsRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/transitGateways/{}/attachment",
                    subscription_id, tgw_id
                ),
                request,
            )
            .await
    }
}
