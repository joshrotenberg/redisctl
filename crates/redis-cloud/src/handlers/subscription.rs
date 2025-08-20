//! Subscription operations handler

use crate::{
    Result,
    client::CloudClient,
    models::{
        CloudDatabase, CloudSubscription, CreateSubscriptionRequest, UpdateSubscriptionRequest,
    },
};
use serde_json::Value;

/// Handler for Cloud subscription operations
pub struct CloudSubscriptionHandler {
    client: CloudClient,
}

impl CloudSubscriptionHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudSubscriptionHandler { client }
    }

    /// List all subscriptions
    pub async fn list(&self) -> Result<Vec<CloudSubscription>> {
        let response: Value = self.client.get("/subscriptions").await?;

        // The API returns { "subscriptions": [...] }
        if let Some(subs) = response.get("subscriptions") {
            serde_json::from_value(subs.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get subscription by ID
    pub async fn get(&self, subscription_id: u32) -> Result<CloudSubscription> {
        self.client
            .get(&format!("/subscriptions/{}", subscription_id))
            .await
    }

    /// Get subscription databases
    pub async fn databases(&self, subscription_id: u32) -> Result<Vec<CloudDatabase>> {
        let response: Value = self
            .client
            .get(&format!("/subscriptions/{}/databases", subscription_id))
            .await?;

        // The API returns { "subscription": {...}, "databases": [...] }
        if let Some(dbs) = response.get("databases") {
            serde_json::from_value(dbs.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Create a new subscription
    pub async fn create(&self, request: CreateSubscriptionRequest) -> Result<CloudSubscription> {
        self.client.post("/subscriptions", &request).await
    }

    /// Update subscription
    pub async fn update(
        &self,
        subscription_id: u32,
        request: UpdateSubscriptionRequest,
    ) -> Result<CloudSubscription> {
        self.client
            .put(&format!("/subscriptions/{}", subscription_id), &request)
            .await
    }

    /// Delete subscription
    pub async fn delete(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/subscriptions/{}", subscription_id))
            .await?;
        Ok(serde_json::json!({"message": format!("Subscription {} deleted", subscription_id)}))
    }

    /// Get subscription pricing
    pub async fn pricing(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/pricing", subscription_id))
            .await
    }

    /// Get available payment methods
    pub async fn payment_methods(&self) -> Result<Value> {
        self.client.get("/payment-methods").await
    }

    /// Get available cloud accounts
    pub async fn cloud_accounts(&self) -> Result<Value> {
        self.client.get("/cloud-accounts").await
    }
    
    /// List with result as Value for CLI filtering
    pub async fn list_raw(&self) -> Result<Value> {
        self.client.get("/subscriptions").await
    }
    
    /// Get as Value for CLI
    pub async fn get_raw(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}", subscription_id))
            .await
    }
    
    /// Create with Value for flexibility
    pub async fn create_raw(&self, request: Value) -> Result<Value> {
        self.client.post("/subscriptions", &request).await
    }
    
    /// Update with Value
    pub async fn update_raw(&self, subscription_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/subscriptions/{}", subscription_id), &request)
            .await
    }
    
    /// Get pricing
    pub async fn get_pricing(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/pricing", subscription_id))
            .await
    }
    
    /// Get CIDR whitelist
    pub async fn get_cidr_whitelist(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/cidr-whitelist", subscription_id))
            .await
    }
    
    /// Update CIDR whitelist
    pub async fn update_cidr_whitelist(&self, subscription_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/subscriptions/{}/cidr-whitelist", subscription_id), &request)
            .await
    }
    
    /// Get VPC peerings
    pub async fn get_vpc_peerings(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/peerings", subscription_id))
            .await
    }
}
