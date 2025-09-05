//! Subscription management for Pro (Flexible) plans
//!
//! This module provides comprehensive management of Redis Cloud Pro subscriptions,
//! which offer flexible, scalable Redis deployments with advanced features like
//! auto-scaling, multi-region support, and Active-Active configurations.
//!
//! # Overview
//!
//! Pro subscriptions are Redis Cloud's most flexible offering, supporting everything
//! from small development instances to large-scale production deployments with
//! automatic scaling, clustering, and global distribution.
//!
//! # Key Features
//!
//! - **Flexible Scaling**: Auto-scaling based on usage patterns
//! - **Multi-Region**: Deploy across multiple regions and cloud providers
//! - **Active-Active**: Global database replication with local reads/writes
//! - **Advanced Networking**: VPC peering, Transit Gateway, Private endpoints
//! - **Maintenance Windows**: Configurable maintenance scheduling
//! - **CIDR Management**: IP allowlist and security group configuration
//! - **Custom Pricing**: Usage-based pricing with detailed cost tracking
//!
//! # Subscription Types
//!
//! - **Single-Region**: Standard deployment in one region
//! - **Multi-Region**: Replicated across multiple regions
//! - **Active-Active**: CRDB with conflict-free replicated data types
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, SubscriptionHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = SubscriptionHandler::new(client);
//!
//! // List all Pro subscriptions
//! let subscriptions = handler.get_all_subscriptions().await?;
//!
//! // Get subscription details (subscription ID 123)
//! let subscription = handler.get_subscription_by_id(123).await?;
//!
//! // Manage maintenance windows
//! let windows = handler.get_subscription_maintenance_windows(123).await?;
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

/// Subscription update request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSubscriptionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Subscription update request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Optional. Updated subscription name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional. The payment method ID you'd like to use for this subscription. Must be a valid payment method ID for this account. Use GET /payment-methods to get all payment methods for your account. This value is optional if ‘paymentMethod’ is ‘marketplace’, but required if 'paymentMethod' is 'credit-card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,

    /// Optional. The payment method for the subscription. If set to ‘credit-card’ , ‘paymentMethodId’ must be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Cloud provider, region, and networking details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSpec {
    /// Optional. Cloud provider. Default: 'AWS'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Optional. Cloud account identifier. Default: Redis internal cloud account (Cloud Account ID = 1). Use GET /cloud-accounts to list all available cloud accounts. Note: A subscription on Google Cloud can be created only with Redis internal cloud account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_account_id: Option<i32>,

    /// The cloud provider region or list of regions (Active-Active only) and networking details.
    pub regions: Vec<SubscriptionRegionSpec>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Object representing a customer managed key (CMK), along with the region it is associated to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerManagedKey {
    /// Required. Resource name of the customer managed key as defined by the cloud provider.
    pub resource_name: String,

    /// Name of region to for the customer managed key as defined by the cloud provider. Required for active-active subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. Expected read and write throughput for this region.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalThroughput {
    /// Specify one of the selected cloud provider regions for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Write operations for this region per second. Default: 1000 ops/sec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_operations_per_second: Option<i64>,

    /// Read operations for this region per second. Default: 1000 ops/sec
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_operations_per_second: Option<i64>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// List of databases in the subscription with local throughput details. Default: 1000 read and write ops/sec for each database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrdbRegionSpec {
    /// Database name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_throughput_measurement: Option<LocalThroughput>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Subscription update request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionUpdateCMKRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Optional. The grace period for deleting the subscription. If not set, will default to immediate deletion grace period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period: Option<String>,

    /// The customer managed keys (CMK) to use for this subscription. If is active-active subscription, must set a key for each region.
    pub customer_managed_keys: Vec<CustomerManagedKey>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// SubscriptionPricings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPricings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing: Option<Vec<SubscriptionPricing>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. Throughput measurement method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseThroughputSpec {
    /// Throughput measurement method. Use 'operations-per-second' for all new databases.
    pub by: String,

    /// Throughput value in the selected measurement method.
    pub value: i64,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. Redis advanced capabilities (also known as modules) to be provisioned in the database. Use GET /database-modules to get a list of available advanced capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseModuleSpec {
    /// Redis advanced capability name. Use GET /database-modules for a list of available capabilities.
    pub name: String,

    /// Optional. Redis advanced capability parameters. Use GET /database-modules to get the available capabilities and their parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Value>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Update Pro subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CidrAllowlistUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// List of CIDR values. Example: ['10.1.1.0/32']
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ips: Option<Vec<String>>,

    /// List of AWS Security group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// SubscriptionMaintenanceWindowsSpec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMaintenanceWindowsSpec {
    /// Maintenance window mode: either 'manual' or 'automatic'. Must provide 'windows' if manual.
    pub mode: String,

    /// Maintenance window timeframes if mode is set to 'manual'. Up to 7 maintenance windows can be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<MaintenanceWindowSpec>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// MaintenanceWindowSkipStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceWindowSkipStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_skips: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_skip_end: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// List of active-active subscription regions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveSubscriptionRegions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// SubscriptionPricing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_measurement: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Request structure for creating a new Pro subscription
///
/// Defines configuration for flexible subscriptions including cloud providers,
/// regions, deployment type, and initial database specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCreateRequest {
    /// Optional. New subscription name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional. When 'false': Creates a deployment plan and deploys it, creating any resources required by the plan. When 'true': creates a read-only deployment plan and does not create any resources. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// Optional. When 'single-region' or not set: Creates a single region subscription. When 'active-active': creates an Active-Active (multi-region) subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,

    /// Optional. The payment method for the subscription. If set to ‘credit-card’, ‘paymentMethodId’ must be defined. Default: 'credit-card'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// Optional. A valid payment method ID for this account. Use GET /payment-methods to get a list of all payment methods for your account. This value is optional if ‘paymentMethod’ is ‘marketplace’, but required for all other account types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,

    /// Optional. Memory storage preference: either 'ram' or a combination of 'ram-and-flash' (also known as Auto Tiering). Default: 'ram'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_storage: Option<String>,

    /// Optional. Persistent storage encryption secures data-at-rest for database persistence. You can use 'cloud-provider-managed-key' or 'customer-managed-key'.  Default: 'cloud-provider-managed-key'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_storage_encryption_type: Option<String>,

    /// Cloud provider, region, and networking details.
    pub cloud_providers: Vec<SubscriptionSpec>,

    /// One or more database specification(s) to create in this subscription.
    pub databases: Vec<SubscriptionDatabaseSpec>,

    /// Optional. Defines the Redis version of the databases created in this specific request. It doesn't determine future databases associated with this subscription. If not set, databases will use the default Redis version. This field is deprecated and will be removed in a future API version - use the database-level redisVersion property instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

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

/// Configuration regarding customer managed persistent storage encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerManagedKeyAccessDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_service_account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_predefined_roles: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_custom_permissions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_iam_role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_key_policy_statements: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_options: Option<Vec<String>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// One or more database specification(s) to create in this subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionDatabaseSpec {
    /// Name of the database. Database name is limited to 40 characters or less and must include only letters, digits, and hyphens ('-'). It must start with a letter and end with a letter or digit.
    pub name: String,

    /// Optional. Database protocol. Only set to 'memcached' if you have a legacy application. Default: 'redis'
    pub protocol: String,

    /// Optional. Total memory in GB, including replication and other overhead. You cannot set both datasetSizeInGb and totalMemoryInGb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    /// Optional. The maximum amount of data in the dataset for this database in GB. You cannot set both datasetSizeInGb and totalMemoryInGb. If ‘replication’ is 'true', the database’s total memory will be twice as large as the datasetSizeInGb.If ‘replication’ is false, the database’s total memory will be the datasetSizeInGb value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    /// Optional. Support Redis [OSS Cluster API](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#oss-cluster-api). Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    /// Optional. Type and rate of data persistence in persistent storage. Default: 'none'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    /// Optional. Databases replication. Default: 'true'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_measurement: Option<DatabaseThroughputSpec>,

    /// Optional. Expected throughput per region for an Active-Active database. Default: 1000 read and write ops/sec for each region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_throughput_measurement: Option<Vec<LocalThroughput>>,

    /// Optional. Redis advanced capabilities (also known as modules) to be provisioned in the database. Use GET /database-modules to get a list of available advanced capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<DatabaseModuleSpec>>,

    /// Optional. Number of databases that will be created with these settings. Default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,

    /// Optional. Relevant only to ram-and-flash (also known as Auto Tiering) subscriptions. Estimated average size in bytes of the items stored in the database. Default: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_item_size_in_bytes: Option<i64>,

    /// Optional. Redis Serialization Protocol version. Must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    /// Optional. If specified, redisVersion defines the Redis database version. If omitted, the Redis version will be set to the default version (available in 'GET /subscriptions/redis-versions')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

    /// Optional. Database [Hashing policy](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#manage-the-hashing-policy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_type: Option<String>,

    /// Optional. The query performance factor adds extra compute power specifically for search and query databases. You can increase your queries per second by the selected factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_performance_factor: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. Cloud networking details, per region. Required if creating an Active-Active subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRegionNetworkingSpec {
    /// Optional. Deployment CIDR mask. Must be a valid CIDR format with a range of 256 IP addresses. Default for single-region subscriptions: If using Redis internal cloud account, 192.168.0.0/24
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_cidr: Option<String>,

    /// Optional. Enter a VPC identifier that exists in the hosted AWS account. Creates a new VPC if not set. VPC Identifier must be in a valid format (for example: 'vpc-0125be68a4625884ad') and must exist within the hosting account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// Optional. Enter a list of subnets identifiers that exists in the hosted AWS account. Subnet Identifier must exist within the hosting account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,

    /// Optional. Enter a security group identifier that exists in the hosted AWS account. Security group Identifier must be in a valid format (for example: 'sg-0125be68a4625884ad') and must exist within the hosting account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,

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

/// MaintenanceWindow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceWindow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_hours: Option<i32>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs Subscription information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_storage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_databases: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_storage_encryption_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_access_details: Option<CustomerManagedKeyAccessDetails>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Maintenance window timeframes if mode is set to 'manual'. Up to 7 maintenance windows can be provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceWindowSpec {
    /// Starting hour of the maintenance window. Can be between '0' (12 AM in the deployment region's local time) and '23' (11 PM in the deployment region's local time).
    pub start_hour: i32,

    /// The duration of the maintenance window in hours. Can be between 4-24 hours (or 8-24 hours if using 'ram-and-flash').
    pub duration_in_hours: i32,

    /// Days where this maintenance window applies. Can contain one or more of: "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", or "Sunday".
    pub days: Vec<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs list of subscriptions in current account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSubscriptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Active active region creation request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveRegionCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Name of region to add as defined by the cloud provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Optional. Enter a VPC identifier that exists in the hosted AWS account. Creates a new VPC if not set. VPC Identifier must be in a valid format and must exist within the hosting account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    /// Deployment CIDR mask. Must be a valid CIDR format with a range of 256 IP addresses.
    pub deployment_cidr: String,

    /// Optional. When 'false': Creates a deployment plan and deploys it, creating any resources required by the plan. When 'true': creates a read-only deployment plan, and does not create any resources. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// List of databases in the subscription with local throughput details. Default: 1000 read and write ops/sec for each database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<CrdbRegionSpec>>,

    /// Optional. RESP version must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    /// Optional. Resource name of the customer managed key as defined by the cloud provider for customer managed subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_resource_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

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

/// Active active region deletion request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveActiveRegionDeleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// The names of the regions to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<ActiveActiveRegionToDelete>>,

    /// Optional. When 'false': Creates a deployment plan and deploys it, deleting any resources required by the plan. When 'true': creates a read-only deployment plan and does not delete or modify any resources. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// The names of the regions to delete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveActiveRegionToDelete {
    /// Name of the cloud provider region to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

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

/// The cloud provider region or list of regions (Active-Active only) and networking details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRegionSpec {
    /// Deployment region as defined by the cloud provider.
    pub region: String,

    /// Optional. Support deployment on multiple availability zones within the selected region. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_availability_zones: Option<bool>,

    /// Optional. List the zone ID(s) for your preferred availability zone(s) for the cloud provider and region. If ‘multipleAvailabilityZones’ is set to 'true', you must list three availability zones. Otherwise, list one availability zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<SubscriptionRegionNetworkingSpec>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// SubscriptionMaintenanceWindows
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMaintenanceWindows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<MaintenanceWindow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_status: Option<MaintenanceWindowSkipStatus>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Handler for Pro subscription operations
///
/// Manages flexible subscriptions with auto-scaling, multi-region support,
/// Active-Active configurations, and advanced networking features.
pub struct SubscriptionHandler {
    client: CloudClient,
}

impl SubscriptionHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get Pro subscriptions
    /// Gets a list of all Pro subscriptions in the current account.
    ///
    /// GET /subscriptions
    pub async fn get_all_subscriptions(&self) -> Result<AccountSubscriptions> {
        self.client.get("/subscriptions").await
    }

    /// Create Pro subscription
    /// Creates a new Redis Cloud Pro subscription.
    ///
    /// POST /subscriptions
    pub async fn create_subscription(
        &self,
        request: &SubscriptionCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client.post("/subscriptions", request).await
    }

    /// Get available Redis database versions
    /// Gets a list of all available Redis database versions for Pro subscriptions.
    ///
    /// GET /subscriptions/redis-versions
    pub async fn get_redis_versions(&self, subscription_id: Option<i32>) -> Result<RedisVersions> {
        let mut query = Vec::new();
        if let Some(v) = subscription_id {
            query.push(format!("subscriptionId={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!("/subscriptions/redis-versions{}", query_string))
            .await
    }

    /// Delete Pro subscription
    /// Delete the specified Pro subscription. All databases in the subscription must be deleted before deleting it.
    ///
    /// DELETE /subscriptions/{subscriptionId}
    pub async fn delete_subscription_by_id(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/subscriptions/{}", subscription_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single Pro subscription
    /// Gets information on the specified Pro subscription.
    ///
    /// GET /subscriptions/{subscriptionId}
    pub async fn get_subscription_by_id(&self, subscription_id: i32) -> Result<Subscription> {
        self.client
            .get(&format!("/subscriptions/{}", subscription_id))
            .await
    }

    /// Update Pro subscription
    /// Updates the specified Pro subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}
    pub async fn update_subscription(
        &self,
        subscription_id: i32,
        request: &BaseSubscriptionUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/subscriptions/{}", subscription_id), request)
            .await
    }

    /// Get Pro subscription CIDR allowlist
    /// (Self-hosted AWS subscriptions only) Gets a Pro subscription's CIDR allowlist.
    ///
    /// GET /subscriptions/{subscriptionId}/cidr
    pub async fn get_cidr_allowlist(&self, subscription_id: i32) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!("/subscriptions/{}/cidr", subscription_id))
            .await
    }

    /// Update Pro subscription CIDR allowlist
    /// (Self-hosted AWS subscriptions only) Updates a Pro subscription's CIDR allowlist.
    ///
    /// PUT /subscriptions/{subscriptionId}/cidr
    pub async fn update_subscription_cidr_allowlist(
        &self,
        subscription_id: i32,
        request: &CidrAllowlistUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/subscriptions/{}/cidr", subscription_id), request)
            .await
    }

    /// Get Pro subscription maintenance windows
    /// Gets maintenance windows for the specified Pro subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/maintenance-windows
    pub async fn get_subscription_maintenance_windows(
        &self,
        subscription_id: i32,
    ) -> Result<SubscriptionMaintenanceWindows> {
        self.client
            .get(&format!(
                "/subscriptions/{}/maintenance-windows",
                subscription_id
            ))
            .await
    }

    /// Update Pro subscription maintenance windows
    /// Updates maintenance windows for the specified Pro subscription.
    ///
    /// PUT /subscriptions/{subscriptionId}/maintenance-windows
    pub async fn update_subscription_maintenance_windows(
        &self,
        subscription_id: i32,
        request: &SubscriptionMaintenanceWindowsSpec,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!("/subscriptions/{}/maintenance-windows", subscription_id),
                request,
            )
            .await
    }

    /// Get Pro subscription pricing
    /// Gets pricing details for the specified Pro subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/pricing
    pub async fn get_subscription_pricing(
        &self,
        subscription_id: i32,
    ) -> Result<SubscriptionPricings> {
        self.client
            .get(&format!("/subscriptions/{}/pricing", subscription_id))
            .await
    }

    /// Delete regions from an Active-Active subscription
    /// (Active-Active subscriptions only) Deletes one or more regions from the specified Active-Active subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/regions
    pub async fn delete_regions_from_active_active_subscription(
        &self,
        subscription_id: i32,
        request: &ActiveActiveRegionDeleteRequest,
    ) -> Result<TaskStateUpdate> {
        // TODO: DELETE with body not yet supported by client
        let _ = request; // Suppress unused variable warning
        let response = self
            .client
            .delete_raw(&format!("/subscriptions/{}/regions", subscription_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get regions in an Active-Active subscription
    /// (Active-Active subscriptions only) Gets a list of regions in the specified Active-Active subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/regions
    pub async fn get_regions_from_active_active_subscription(
        &self,
        subscription_id: i32,
    ) -> Result<ActiveActiveSubscriptionRegions> {
        self.client
            .get(&format!("/subscriptions/{}/regions", subscription_id))
            .await
    }

    /// Add region to Active-Active subscription
    /// Adds a new region to an Active-Active subscription.
    ///
    /// POST /subscriptions/{subscriptionId}/regions
    pub async fn add_new_region_to_active_active_subscription(
        &self,
        subscription_id: i32,
        request: &ActiveActiveRegionCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/regions", subscription_id),
                request,
            )
            .await
    }
}
