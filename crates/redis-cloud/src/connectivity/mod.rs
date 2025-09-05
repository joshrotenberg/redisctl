//! Network connectivity and peering operations for Pro subscriptions
//!
//! This module manages advanced networking features for Redis Cloud Pro subscriptions,
//! including VPC peering, AWS Transit Gateway attachments, GCP Private Service Connect,
//! and other cloud-native networking integrations.
//!
//! # Supported Connectivity Types
//!
//! - **VPC Peering**: Direct peering between Redis Cloud VPC and your VPC
//! - **Transit Gateway**: AWS Transit Gateway attachments for hub-and-spoke topologies
//! - **Private Service Connect**: GCP Private Service Connect for private endpoints
//!
//! # Module Organization
//!
//! The connectivity features are split into three specialized modules:
//! - `vpc_peering` - VPC peering operations for AWS, GCP, and Azure
//! - `psc` - Google Cloud Private Service Connect endpoints
//! - `transit_gateway` - AWS Transit Gateway attachments

pub mod psc;
pub mod transit_gateway;
pub mod vpc_peering;

// Re-export handlers for convenience
pub use psc::PscHandler;
pub use transit_gateway::TransitGatewayHandler;
pub use vpc_peering::VpcPeeringHandler;

// Re-export types used by handlers
pub use psc::PscEndpointUpdateRequest;
pub use transit_gateway::{Cidr, TgwAttachmentRequest, TgwUpdateCidrsRequest};
pub use vpc_peering::{
    VpcPeeringCreateBaseRequest, VpcPeeringCreateRequest, VpcPeeringUpdateAwsRequest,
    VpcPeeringUpdateRequest,
};

// For backward compatibility, provide a unified handler
use crate::CloudClient;

/// Unified connectivity handler - provides backward compatibility
///
/// Consider using the specific handlers directly:
/// - `VpcPeeringHandler` for VPC peering operations
/// - `PscHandler` for Private Service Connect operations
/// - `TransitGatewayHandler` for Transit Gateway operations
pub struct ConnectivityHandler {
    pub vpc_peering: VpcPeeringHandler,
    pub psc: PscHandler,
    pub transit_gateway: TransitGatewayHandler,
}

impl ConnectivityHandler {
    pub fn new(client: CloudClient) -> Self {
        Self {
            vpc_peering: VpcPeeringHandler::new(client.clone()),
            psc: PscHandler::new(client.clone()),
            transit_gateway: TransitGatewayHandler::new(client),
        }
    }

    // VPC Peering delegation methods
    pub async fn get_vpc_peering(
        &self,
        subscription_id: i32,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        self.vpc_peering.get(subscription_id).await
    }

    pub async fn create_vpc_peering(
        &self,
        subscription_id: i32,
        request: &VpcPeeringCreateRequest,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        self.vpc_peering.create(subscription_id, request).await
    }

    pub async fn delete_vpc_peering(
        &self,
        subscription_id: i32,
        peering_id: i32,
    ) -> crate::Result<serde_json::Value> {
        self.vpc_peering.delete(subscription_id, peering_id).await
    }

    pub async fn update_vpc_peering(
        &self,
        subscription_id: i32,
        peering_id: i32,
        request: &VpcPeeringUpdateAwsRequest,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        // VpcPeeringUpdateAwsRequest can be used as VpcPeeringCreateRequest for the update
        let create_request = VpcPeeringCreateRequest {
            provider: None,
            command_type: None,
            extra: serde_json::json!(request),
        };
        self.vpc_peering
            .update(subscription_id, peering_id, &create_request)
            .await
    }

    // PSC delegation methods
    pub async fn get_psc_service(
        &self,
        subscription_id: i32,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        self.psc.get_service(subscription_id).await
    }

    pub async fn create_psc_service(
        &self,
        subscription_id: i32,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        self.psc.create_service(subscription_id).await
    }

    pub async fn delete_psc_service(
        &self,
        subscription_id: i32,
    ) -> crate::Result<serde_json::Value> {
        self.psc.delete_service(subscription_id).await
    }

    pub async fn create_psc_endpoint(
        &self,
        subscription_id: i32,
        request: &PscEndpointUpdateRequest,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        self.psc.create_endpoint(subscription_id, request).await
    }

    // Transit Gateway delegation methods
    pub async fn get_tgws(
        &self,
        subscription_id: i32,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        self.transit_gateway.get_attachments(subscription_id).await
    }

    pub async fn create_tgw_attachment(
        &self,
        subscription_id: i32,
        tgw_id: &str,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        let request = TgwAttachmentRequest {
            tgw_id: Some(tgw_id.to_string()),
            aws_account_id: None,
            cidrs: None,
            extra: serde_json::Value::Object(serde_json::Map::new()),
        };
        self.transit_gateway
            .create_attachment(subscription_id, &request)
            .await
    }

    pub async fn delete_tgw_attachment(
        &self,
        subscription_id: i32,
        attachment_id: i32,
    ) -> crate::Result<serde_json::Value> {
        self.transit_gateway
            .delete_attachment(subscription_id, attachment_id.to_string())
            .await
    }

    pub async fn update_tgw_cidrs(
        &self,
        subscription_id: i32,
        attachment_id: &str,
        request: &TgwUpdateCidrsRequest,
    ) -> crate::Result<crate::types::TaskStateUpdate> {
        // Convert TgwUpdateCidrsRequest to TgwAttachmentRequest
        let attachment_request = TgwAttachmentRequest {
            aws_account_id: None,
            tgw_id: None,
            cidrs: request.cidrs.as_ref().map(|cidrs| {
                cidrs
                    .iter()
                    .filter_map(|c| c.cidr_address.clone())
                    .collect()
            }),
            extra: serde_json::Value::Object(serde_json::Map::new()),
        };
        self.transit_gateway
            .update_attachment_cidrs(
                subscription_id,
                attachment_id.to_string(),
                &attachment_request,
            )
            .await
    }
}
