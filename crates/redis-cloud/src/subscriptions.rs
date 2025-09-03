//! Subscription management for Redis Cloud Pro
//!
//! ## Overview
//! - Create and manage Pro subscriptions
//! - Configure subscription settings
//! - Manage subscription regions and networks
//! - Query subscription pricing

use crate::client::CloudClient;
use crate::error::Result;
use crate::types::{
    CloudProvider, Link, MemoryStorage, PaymentMethodType, Pricing, Region, SubscriptionStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Subscription information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: u32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,

    #[serde(rename = "paymentMethodId", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<u32>,

    #[serde(rename = "paymentMethodType", skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodType>,

    #[serde(rename = "memoryStorage", skip_serializing_if = "Option::is_none")]
    pub memory_storage: Option<MemoryStorage>,

    #[serde(rename = "numberOfDatabases", skip_serializing_if = "Option::is_none")]
    pub number_of_databases: Option<u32>,

    #[serde(
        rename = "persistentStorageEncryptionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_storage_encryption_type: Option<String>,

    #[serde(
        rename = "deletionGracePeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_grace_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create subscription request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateSubscriptionRequest {
    #[builder(setter(into))]
    pub name: String,

    #[serde(rename = "paymentMethodId")]
    pub payment_method_id: u32,

    #[serde(rename = "cloudProviders")]
    pub cloud_providers: Vec<CloudProviderConfig>,

    #[serde(rename = "memoryStorage", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub memory_storage: Option<MemoryStorage>,

    #[serde(
        rename = "persistentStorageEncryptionType",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into, strip_option))]
    pub persistent_storage_encryption_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub databases: Option<Vec<CreateDatabaseInSubscription>>,
}

/// Cloud provider configuration
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CloudProviderConfig {
    pub provider: CloudProvider,

    #[serde(rename = "cloudAccountId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cloud_account_id: Option<u32>,

    pub regions: Vec<RegionConfig>,
}

/// Region configuration
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct RegionConfig {
    #[builder(setter(into))]
    pub region: String,

    #[serde(
        rename = "multipleAvailabilityZones",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub multiple_availability_zones: Option<bool>,

    #[serde(
        rename = "preferredAvailabilityZones",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub preferred_availability_zones: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub networking: Option<NetworkingConfig>,
}

/// Networking configuration
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct NetworkingConfig {
    #[serde(rename = "deploymentCIDR")]
    #[builder(setter(into))]
    pub deployment_cidr: String,

    #[serde(rename = "vpcId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub vpc_id: Option<String>,
}

/// Database creation within subscription
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateDatabaseInSubscription {
    #[builder(setter(into))]
    pub name: String,

    #[serde(rename = "memoryLimitInGb")]
    pub memory_limit_in_gb: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub modules: Option<Vec<String>>,
}

/// Update subscription request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[serde(rename = "paymentMethodId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_method_id: Option<u32>,
}

/// Subscription handler
pub struct SubscriptionHandler {
    client: CloudClient,
}

impl SubscriptionHandler {
    pub fn new(client: CloudClient) -> Self {
        SubscriptionHandler { client }
    }

    /// List all subscriptions
    pub async fn list(&self) -> Result<Vec<Subscription>> {
        self.client.get("/subscriptions").await
    }

    /// Get a specific subscription
    pub async fn get(&self, subscription_id: u32) -> Result<Subscription> {
        self.client
            .get(&format!("/subscriptions/{}", subscription_id))
            .await
    }

    /// Create a new subscription
    pub async fn create(&self, request: CreateSubscriptionRequest) -> Result<Subscription> {
        self.client.post("/subscriptions", &request).await
    }

    /// Update a subscription
    pub async fn update(
        &self,
        subscription_id: u32,
        request: UpdateSubscriptionRequest,
    ) -> Result<Subscription> {
        self.client
            .put(&format!("/subscriptions/{}", subscription_id), &request)
            .await
    }

    /// Delete a subscription
    pub async fn delete(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/subscriptions/{}", subscription_id))
            .await
    }

    /// Get subscription pricing
    pub async fn pricing(&self, subscription_id: u32) -> Result<Pricing> {
        self.client
            .get(&format!("/subscriptions/{}/pricing", subscription_id))
            .await
    }

    /// List subscription regions
    pub async fn regions(&self, subscription_id: u32) -> Result<Vec<Region>> {
        self.client
            .get(&format!("/subscriptions/{}/regions", subscription_id))
            .await
    }

    /// Add regions to subscription
    pub async fn add_regions(
        &self,
        subscription_id: u32,
        regions: Vec<RegionConfig>,
    ) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/regions", subscription_id),
                &regions,
            )
            .await
    }

    /// Remove region from subscription
    pub async fn remove_region(&self, subscription_id: u32, region_id: u32) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/regions/{}",
                subscription_id, region_id
            ))
            .await
    }

    /// Get Redis versions available for subscription
    pub async fn redis_versions(&self) -> Result<Vec<String>> {
        self.client.get("/subscriptions/redis-versions").await
    }
}
