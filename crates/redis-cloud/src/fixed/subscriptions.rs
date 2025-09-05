//! Subscription management for Essentials (Fixed) plans
//!
//! This module manages Redis Cloud Essentials subscriptions, which provide
//! simplified, fixed-capacity Redis deployments with predictable pricing.
//! Essentials subscriptions are ideal for smaller, stable workloads.
//!
//! # Overview
//!
//! Essentials subscriptions offer a streamlined experience with pre-defined
//! plans that include specific memory allocations, regions, and feature sets.
//! Unlike Pro subscriptions, they don't support auto-scaling or multi-region
//! deployments.
//!
//! # Key Features
//!
//! - **Fixed Plans**: Pre-defined subscription plans with set resources
//! - **Simple Management**: Create, update, and delete subscriptions
//! - **Plan Discovery**: Browse available plans by region and size
//! - **Redis Versions**: Access supported Redis versions for the subscription
//! - **Cost Predictability**: Fixed monthly pricing based on plan selection
//!
//! # Plan Structure
//!
//! Essentials plans are defined by:
//! - Memory size (250MB to 12GB)
//! - Cloud provider and region
//! - Included features and modules
//! - Fixed monthly price
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, FixedSubscriptionHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = FixedSubscriptionHandler::new(client);
//!
//! // List available plans
//! let plans = handler.list_plans(None, None).await?;
//!
//! // Get all fixed subscriptions
//! let subscriptions = handler.list().await?;
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

/// RedisVersions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisVersions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_versions: Option<Vec<RedisVersion>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis list of Essentials subscriptions plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedSubscriptionsPlans {
    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

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

/// Essentials subscription update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedSubscriptionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Optional. Updated subscription name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional. An Essentials plan ID. The plan describes the dataset size, cloud provider and region, and available database configuration options. Use GET /fixed/plans/subscriptions/{subscriptionId} to get a list of compatible options for the specified subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i32>,

    /// Optional. The payment method for the subscription. If set to ‘credit-card’ , ‘paymentMethodId’ must be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// Optional. The payment method ID you'd like to use for this subscription. Must be a valid payment method ID for this account. Use GET /payment-methods to get a list of payment methods for your account. This value is optional if ‘paymentMethod’ is ‘marketplace’, but required if 'paymentMethod' is 'credit-card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis Essentials subscription plan information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedSubscriptionsPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_measurement_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_databases: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_throughput: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bandwidth_gb: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_rules: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_data_persistence: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_flex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_instant_and_daily_backups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_replication: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_clustering: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_ssl: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_support: Option<String>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Essentials subscription create request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedSubscriptionCreateRequest {
    /// New Essentials subscription name.
    pub name: String,

    /// An Essentials plan ID. The plan describes the dataset size, cloud provider and region, and available database configuration options. Use GET /fixed/plans to get a list of available options.
    pub plan_id: i32,

    /// Optional. The payment method for the subscription. If set to ‘credit-card’, ‘paymentMethodId’ must be defined. Default: 'credit-card'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// Optional. A valid payment method ID for this account. Use GET /payment-methods to get a list of all payment methods for your account. This value is optional if ‘paymentMethod’ is ‘marketplace’, but required for all other account types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis list of Essentials subscriptions in current account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedSubscriptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisVersion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eol_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis Essentials Subscription information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_measurement_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_databases: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_allow_rules: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_data_persistence: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_instant_and_daily_backups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_replication: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_clustering: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_support: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_status: Option<String>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

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

/// Handler for Essentials subscription operations
///
/// Manages fixed-capacity subscriptions with pre-defined plans,
/// simplified pricing, and streamlined configuration options.
pub struct FixedSubscriptionHandler {
    client: CloudClient,
}

impl FixedSubscriptionHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get Essentials plans
    /// Gets a list of Essentials plans. The plan describes the dataset size, cloud provider and region, and available database configuration options for an Essentials database.
    ///
    /// GET /fixed/plans
    pub async fn list_plans(
        &self,
        provider: Option<String>,
        redis_flex: Option<bool>,
    ) -> Result<FixedSubscriptionsPlans> {
        let mut query = Vec::new();
        if let Some(v) = provider {
            query.push(format!("provider={}", v));
        }
        if let Some(v) = redis_flex {
            query.push(format!("redisFlex={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!("/fixed/plans{}", query_string))
            .await
    }

    /// Get Essentials plans for a subscription
    /// Gets a list of compatible Essentials plans for the specified Essentials subscription.
    ///
    /// GET /fixed/plans/subscriptions/{subscriptionId}
    pub async fn get_plans_by_subscription_id(
        &self,
        subscription_id: i32,
    ) -> Result<FixedSubscriptionsPlans> {
        self.client
            .get(&format!("/fixed/plans/subscriptions/{}", subscription_id))
            .await
    }

    /// Get a single Essentials plan
    /// Gets information on the specified Essentials plan.
    ///
    /// GET /fixed/plans/{planId}
    pub async fn get_plan_by_id(&self, plan_id: i32) -> Result<FixedSubscriptionsPlan> {
        self.client.get(&format!("/fixed/plans/{}", plan_id)).await
    }

    /// Get available Redis database versions for specific Essentials subscription
    /// Gets a list of all available Redis database versions for a specific Essentials subscription.
    ///
    /// GET /fixed/redis-versions
    pub async fn get_redis_versions(&self, subscription_id: i32) -> Result<RedisVersions> {
        let mut query = Vec::new();
        query.push(format!("subscriptionId={}", subscription_id));
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!("/fixed/redis-versions{}", query_string))
            .await
    }

    /// Get Essentials subscriptions
    /// Gets a list of all Essentials subscriptions in the current account.
    ///
    /// GET /fixed/subscriptions
    pub async fn list(&self) -> Result<FixedSubscriptions> {
        self.client.get("/fixed/subscriptions").await
    }

    /// Create Essentials subscription
    /// Creates a new Essentials subscription.
    ///
    /// POST /fixed/subscriptions
    pub async fn create(
        &self,
        request: &FixedSubscriptionCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client.post("/fixed/subscriptions", request).await
    }

    /// Delete Essentials subscription
    /// Deletes the specified Essentials subscription. All databases in the subscription must be deleted before deleting it.
    ///
    /// DELETE /fixed/subscriptions/{subscriptionId}
    pub async fn delete_by_id(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/fixed/subscriptions/{}", subscription_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single Essentials subscription
    /// Gets information on the specified Essentials subscription.
    ///
    /// GET /fixed/subscriptions/{subscriptionId}
    pub async fn get_by_id(&self, subscription_id: i32) -> Result<FixedSubscription> {
        self.client
            .get(&format!("/fixed/subscriptions/{}", subscription_id))
            .await
    }

    /// Update Essentials subscription
    /// Updates the specified Essentials subscription.
    ///
    /// PUT /fixed/subscriptions/{subscriptionId}
    pub async fn update(
        &self,
        subscription_id: i32,
        request: &FixedSubscriptionUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!("/fixed/subscriptions/{}", subscription_id),
                request,
            )
            .await
    }

    // ========================================================================
    // Backward compatibility wrapper methods
    // ========================================================================

    /// Create fixed subscription (backward compatibility)
    pub async fn create_fixed_subscription(
        &self,
        request: &FixedSubscriptionCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.create(request).await
    }

    /// Get fixed subscription (backward compatibility)
    pub async fn get_fixed_subscription(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.get_by_id(subscription_id)
            .await
            .map(|sub| serde_json::from_value(serde_json::json!(sub)).unwrap())
    }

    /// Update fixed subscription (backward compatibility)
    pub async fn update_fixed_subscription(
        &self,
        subscription_id: i32,
        request: &FixedSubscriptionUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.update(subscription_id, request).await
    }

    /// Delete fixed subscription (backward compatibility)
    pub async fn delete_fixed_subscription(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.delete_by_id(subscription_id).await
    }
}
