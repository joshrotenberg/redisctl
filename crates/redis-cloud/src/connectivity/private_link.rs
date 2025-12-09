//! AWS PrivateLink connectivity operations
//!
//! This module provides AWS PrivateLink connectivity functionality for Redis Cloud,
//! enabling secure, private connections from AWS VPCs to Redis Cloud databases.
//!
//! # Overview
//!
//! AWS PrivateLink allows you to connect to Redis Cloud from your AWS VPC without
//! traversing the public internet. This provides enhanced security and potentially
//! lower latency.
//!
//! # Features
//!
//! - **PrivateLink Management**: Create and retrieve PrivateLink configurations
//! - **Principal Management**: Control which AWS principals can access the service
//! - **Endpoint Scripts**: Get scripts to create endpoints in your AWS account
//! - **Active-Active Support**: PrivateLink for CRDB (Active-Active) databases
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, PrivateLinkHandler};
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = PrivateLinkHandler::new(client);
//!
//! // Create a PrivateLink
//! let request = json!({
//!     "shareName": "my-redis-share",
//!     "principal": "123456789012",
//!     "type": "aws_account",
//!     "alias": "Production Account"
//! });
//! let result = handler.create(123, request).await?;
//!
//! // Get PrivateLink configuration
//! let config = handler.get(123).await?;
//! # Ok(())
//! # }
//! ```

use crate::{CloudClient, Result};
use serde_json::Value;

/// AWS PrivateLink handler
///
/// Manages AWS PrivateLink connectivity for Redis Cloud subscriptions.
pub struct PrivateLinkHandler {
    client: CloudClient,
}

impl PrivateLinkHandler {
    /// Create a new PrivateLink handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get PrivateLink configuration
    ///
    /// Gets the AWS PrivateLink configuration for a subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/private-link
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    ///
    /// # Returns
    ///
    /// Returns the PrivateLink configuration as JSON
    pub async fn get(&self, subscription_id: i32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/private-link", subscription_id))
            .await
    }

    /// Create a PrivateLink
    ///
    /// Creates a new AWS PrivateLink configuration for a subscription.
    ///
    /// POST /subscriptions/{subscriptionId}/private-link
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `request` - PrivateLink creation request (shareName, principal, type required)
    ///
    /// # Returns
    ///
    /// Returns a task response that can be tracked for completion
    pub async fn create(&self, subscription_id: i32, request: Value) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/private-link", subscription_id),
                &request,
            )
            .await
    }

    /// Add principals to PrivateLink
    ///
    /// Adds AWS principals (accounts, IAM roles, etc.) that can access the PrivateLink.
    ///
    /// POST /subscriptions/{subscriptionId}/private-link/principals
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `request` - Principal to add (principal required, type/alias optional)
    ///
    /// # Returns
    ///
    /// Returns the updated principal configuration
    pub async fn add_principals(&self, subscription_id: i32, request: Value) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/private-link/principals", subscription_id),
                &request,
            )
            .await
    }

    /// Remove principals from PrivateLink
    ///
    /// Removes AWS principals from the PrivateLink access list.
    ///
    /// DELETE /subscriptions/{subscriptionId}/private-link/principals
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `request` - Principal to remove (principal, type, alias)
    ///
    /// # Returns
    ///
    /// Returns confirmation of deletion
    pub async fn remove_principals(&self, subscription_id: i32, request: Value) -> Result<Value> {
        self.client
            .delete_with_body(
                &format!("/subscriptions/{}/private-link/principals", subscription_id),
                request,
            )
            .await
    }

    /// Get endpoint creation script
    ///
    /// Gets a script to create the VPC endpoint in your AWS account.
    ///
    /// GET /subscriptions/{subscriptionId}/private-link/endpoint-script
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    ///
    /// # Returns
    ///
    /// Returns the endpoint creation script
    pub async fn get_endpoint_script(&self, subscription_id: i32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-link/endpoint-script",
                subscription_id
            ))
            .await
    }

    /// Delete PrivateLink
    ///
    /// Deletes the AWS PrivateLink configuration for a subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/private-link
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    ///
    /// # Returns
    ///
    /// Returns task information for tracking the deletion
    pub async fn delete(&self, subscription_id: i32) -> Result<Value> {
        self.client
            .delete_raw(&format!("/subscriptions/{}/private-link", subscription_id))
            .await
    }

    /// Get Active-Active PrivateLink configuration
    ///
    /// Gets the AWS PrivateLink configuration for an Active-Active (CRDB) subscription region.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/private-link
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `region_id` - The region ID
    ///
    /// # Returns
    ///
    /// Returns the PrivateLink configuration for the region
    pub async fn get_active_active(&self, subscription_id: i32, region_id: i32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-link",
                subscription_id, region_id
            ))
            .await
    }

    /// Create Active-Active PrivateLink
    ///
    /// Creates a new AWS PrivateLink for an Active-Active (CRDB) subscription region.
    ///
    /// POST /subscriptions/{subscriptionId}/regions/{regionId}/private-link
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `region_id` - The region ID
    /// * `request` - PrivateLink creation request
    ///
    /// # Returns
    ///
    /// Returns a task response
    pub async fn create_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        request: Value,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/private-link",
                    subscription_id, region_id
                ),
                &request,
            )
            .await
    }

    /// Add principals to Active-Active PrivateLink
    ///
    /// Adds AWS principals to an Active-Active PrivateLink.
    ///
    /// POST /subscriptions/{subscriptionId}/regions/{regionId}/private-link/principals
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `region_id` - The region ID
    /// * `request` - Principal to add
    ///
    /// # Returns
    ///
    /// Returns the updated configuration
    pub async fn add_principals_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        request: Value,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/private-link/principals",
                    subscription_id, region_id
                ),
                &request,
            )
            .await
    }

    /// Remove principals from Active-Active PrivateLink
    ///
    /// Removes AWS principals from an Active-Active PrivateLink.
    ///
    /// DELETE /subscriptions/{subscriptionId}/regions/{regionId}/private-link/principals
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `region_id` - The region ID
    /// * `request` - Principal to remove
    ///
    /// # Returns
    ///
    /// Returns confirmation of deletion
    pub async fn remove_principals_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
        request: Value,
    ) -> Result<Value> {
        self.client
            .delete_with_body(
                &format!(
                    "/subscriptions/{}/regions/{}/private-link/principals",
                    subscription_id, region_id
                ),
                request,
            )
            .await
    }

    /// Get Active-Active endpoint creation script
    ///
    /// Gets a script to create the VPC endpoint for an Active-Active region.
    ///
    /// GET /subscriptions/{subscriptionId}/regions/{regionId}/private-link/endpoint-script
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - The subscription ID
    /// * `region_id` - The region ID
    ///
    /// # Returns
    ///
    /// Returns the endpoint creation script
    pub async fn get_endpoint_script_active_active(
        &self,
        subscription_id: i32,
        region_id: i32,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-link/endpoint-script",
                subscription_id, region_id
            ))
            .await
    }
}
