//! Database management operations for Essentials (Fixed) subscriptions
//!
//! This module provides database management functionality for Redis Cloud Essentials
//! (formerly Fixed) subscriptions, which offer a simplified, cost-effective option
//! for smaller workloads with predictable capacity requirements.
//!
//! # Overview
//!
//! Essentials databases are pre-configured Redis instances with fixed memory allocations
//! and simplified pricing. They're ideal for development, testing, and production
//! workloads that don't require auto-scaling or advanced clustering features.
//!
//! # Key Features
//!
//! - **Fixed Capacity**: Pre-defined memory sizes from 250MB to 12GB
//! - **Simple Pricing**: Predictable monthly costs
//! - **Essential Features**: Replication, persistence, and backup support
//! - **Module Support**: Limited module availability based on plan
//! - **Quick Setup**: Simplified configuration for faster deployment
//!
//! # Differences from Pro Databases
//!
//! - Fixed memory allocations (no auto-scaling)
//! - Limited to single-region deployments
//! - Simplified module selection
//! - No clustering support
//! - Predictable pricing model
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, FixedDatabaseHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = FixedDatabaseHandler::new(client);
//!
//! // Example: List databases in a fixed subscription (ID 123)
//! let databases = handler.get_fixed_subscription_databases(123, None, None).await?;
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

/// RedisLabs Account Subscription Databases information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFixedSubscriptionDatabases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database import request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedDatabaseImportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// Type of storage from which to import the database RDB file or Redis data.
    pub source_type: String,

    /// One or more paths to source data files or Redis databases, as appropriate to specified source type.
    pub import_from_uri: Vec<String>,

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

/// Database tag update request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseTagUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Database tag value
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// DynamicEndpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicEndpoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// Database tag key.
    pub key: String,

    /// Database tag value.
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database tags update request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseTagsUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// List of database tags.
    pub tags: Vec<Tag>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. This database will be a replica of the specified Redis databases, provided as a list of objects with endpoint and certificate details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSyncSourceSpec {
    /// Redis URI of a source database. Example format: 'redis://user:password@host:port'. If the URI provided is a Redis Cloud database, only host and port should be provided. Example: 'redis://endpoint1:6379'.
    pub endpoint: String,

    /// Defines if encryption should be used to connect to the sync source. If not set the source is a Redis Cloud database, it will automatically detect if the source uses encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<bool>,

    /// TLS/SSL certificate chain of the sync source. If not set and the source is a Redis Cloud database, it will automatically detect the certificate to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_cert: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. A list of client TLS/SSL certificates. If specified, mTLS authentication will be required to authenticate user connections. If set to an empty list, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCertificateSpec {
    /// Client certificate public key in PEM format, with new line characters replaced with '\n'.
    pub public_certificate_pem_string: String,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database slowlog entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSlowLogEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseTagCreateRequest {
    /// Database tag key.
    pub key: String,

    /// Database tag value.
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Essentials database backup request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedDatabaseBackupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// Optional. Manually backs up data to this location, instead of the set 'periodicBackupPath' location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adhoc_backup_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

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

/// Optional. Changes Replica Of (also known as Active-Passive) configuration details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaOfSpec {
    /// Optional. This database will be a replica of the specified Redis databases, provided as a list of objects with endpoint and certificate details.
    pub sync_sources: Vec<DatabaseSyncSourceSpec>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. Changes Redis database alert details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseAlertSpec {
    /// Alert type. Available options depend on Plan type. See [Configure alerts](https://redis.io/docs/latest/operate/rc/databases/monitor-performance/#configure-metric-alerts) for more information.
    pub name: String,

    /// Value over which an alert will be sent. Default values and range depend on the alert type. See [Configure alerts](https://redis.io/docs/latest/operate/rc/databases/monitor-performance/#configure-metric-alerts) for more information.
    pub value: i32,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis list of database tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// FixedDatabase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedDatabase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version_compliance: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_memory_limit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_dataset_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_measurement_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_used_in_mb: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_monthly_usage_in_byte: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_storage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_flex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_endpoints: Option<DynamicEndpoints>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// DatabaseSlowLogEntries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSlowLogEntries {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<DatabaseSlowLogEntry>>,

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

/// Essentials database definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedDatabaseCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Name of the database. Database name is limited to 40 characters or less and must include only letters, digits, and hyphens ('-'). It must start with a letter and end with a letter or digit.
    pub name: String,

    /// Optional. Database protocol. Use 'stack' to get all of Redis' advanced capabilities. Only use 'redis' for Pay-as-you-go or Redis Flex subscriptions. Default: 'stack' for most subscriptions, 'redis' for Redis Flex subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// (Pay-as-you-go subscriptions only) Optional. Total memory in GB, including replication and other overhead. You cannot set both datasetSizeInGb and totalMemoryInGb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    /// (Pay-as-you-go subscriptions only) Optional. The maximum amount of data in the dataset for this database in GB. You cannot set both datasetSizeInGb and totalMemoryInGb. If ‘replication’ is 'true', the database’s total memory will be twice as large as the datasetSizeInGb. If ‘replication’ is false, the database’s total memory will be the datasetSizeInGb value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    /// (Pay-as-you-go subscriptions only) Optional. Support Redis [OSS Cluster API](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#oss-cluster-api). Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    /// Optional. If specified, redisVersion defines the Redis database version. If omitted, the Redis version will be set to the default version.  (available in 'GET /fixed/redis-versions')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

    /// Optional. Redis Serialization Protocol version. Must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    /// (Pay-as-you-go subscriptions only) Optional. If set to 'true', the database will use the external endpoint for OSS Cluster API. This setting blocks the database's private endpoint. Can only be set if 'supportOSSClusterAPI' is 'true'. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    /// (Pay-as-you-go subscriptions only) Optional. Distributes database data to different cloud instances. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_database_clustering: Option<bool>,

    /// (Pay-as-you-go subscriptions only) Optional. Specifies the number of master shards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<i32>,

    /// Optional. Type and rate of data persistence in persistent storage. Use GET /fixed/plans/{planId} to see if your plan supports data persistence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    /// Optional. Data eviction policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<String>,

    /// Optional. Sets database replication. Use GET /fixed/plans/{planId} to see if your plan supports database replication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    /// Optional. The path to a backup storage location. If specified, the database will back up every 24 hours to this location, and you can manually back up the database to this location at any time. Use GET /fixed/plans/{planId} to see if your plan supports database backups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_backup_path: Option<String>,

    /// Optional. List of source IP addresses or subnet masks to allow. If specified, Redis clients will be able to connect to this database only from within the specified source IP addresses ranges. Use GET /fixed/plans/{planId} to see how many CIDR allow rules your plan supports. Example: '['192.168.10.0/32', '192.168.12.0/24']'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,

    /// (Pay-as-you-go subscriptions only) Optional. Hashing policy Regex rules. Used only if 'enableDatabaseClustering' is set to 'true' and .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_rules: Option<Vec<String>>,

    /// Optional. This database will be a replica of the specified Redis databases provided as one or more URI(s). Example: 'redis://user:password@host:port'. If the URI provided is a Redis Cloud database, only host and port should be provided. Example: ['redis://endpoint1:6379', 'redis://endpoint2:6380'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_of: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica: Option<ReplicaOfSpec>,

    /// Optional. A public key client TLS/SSL certificate with new line characters replaced with '\n'. If specified, mTLS authentication will be required to authenticate user connections. Default: 'null'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ssl_certificate: Option<String>,

    /// Optional. A list of client TLS/SSL certificates. If specified, mTLS authentication will be required to authenticate user connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tls_certificates: Option<Vec<DatabaseCertificateSpec>>,

    /// Optional. When 'true', requires TLS authentication for all connections - mTLS with valid clientTlsCertificates, regular TLS when clientTlsCertificates is not provided. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,

    /// Optional. Password to access the database. If not set, a random 32-character alphanumeric password will be automatically generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Optional. Redis database alert details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<DatabaseAlertSpec>>,

    /// Optional. Redis advanced capabilities (also known as modules) to be provisioned in the database. Use GET /database-modules to get a list of available advanced capabilities. Can only be set if 'protocol' is 'redis'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<DatabaseModuleSpec>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Essentials database update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedDatabaseUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// Optional. Updated database name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// (Pay-as-you-go subscriptions only) Optional. Total memory in GB, including replication and other overhead. You cannot set both datasetSizeInGb and totalMemoryInGb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    /// (Pay-as-you-go subscriptions only) Optional. The maximum amount of data in the dataset for this database in GB. You cannot set both datasetSizeInGb and totalMemoryInGb. If ‘replication’ is 'true', the database’s total memory will be twice as large as the datasetSizeInGb. If ‘replication’ is false, the database’s total memory will be the datasetSizeInGb value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    /// (Pay-as-you-go subscriptions only) Optional. Support Redis [OSS Cluster API](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#oss-cluster-api).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    /// Optional. Redis Serialization Protocol version. Must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    /// (Pay-as-you-go subscriptions only) Optional. If set to 'true', the database will use the external endpoint for OSS Cluster API. This setting blocks the database's private endpoint. Can only be set if 'supportOSSClusterAPI' is 'true'. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    /// (Pay-as-you-go subscriptions only) Optional. Distributes database data to different cloud instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_database_clustering: Option<bool>,

    /// (Pay-as-you-go subscriptions only) Optional. Changes the number of master shards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<i32>,

    /// Optional. Type and rate of data persistence in persistent storage. Use GET /fixed/plans/{planId} to see if your plan supports data persistence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    /// Optional. Turns database replication on or off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<String>,

    /// Optional. Sets database replication. Use GET /fixed/plans/{planId} to see if your plan supports database replication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    /// Optional. Changes the backup location path. If specified, the database will back up every 24 hours to this location, and you can manually back up the database to this location at any time. Use GET /fixed/plans/{planId} to see if your plan supports database backups. If set to an empty string, the backup path will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_backup_path: Option<String>,

    /// Optional. List of source IP addresses or subnet masks to allow. If specified, Redis clients will be able to connect to this database only from within the specified source IP addresses ranges. Example: '['192.168.10.0/32', '192.168.12.0/24']'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,

    /// Optional. This database will be a replica of the specified Redis databases provided as one or more URI (sample format: 'redis://user:password@host:port)'. If the URI provided is Redis Cloud instance, only host and port should be provided (using the format: ['redis://endpoint1:6379', 'redis://endpoint2:6380'] ).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_of: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica: Option<ReplicaOfSpec>,

    /// (Pay-as-you-go subscriptions only) Optional. Hashing policy Regex rules. Used only if 'shardingType' is 'custom-regex-rules'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_rules: Option<Vec<String>>,

    /// Optional. A public key client TLS/SSL certificate with new line characters replaced with '\n'. If specified, mTLS authentication will be required to authenticate user connections if it is not already required. If set to an empty string, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ssl_certificate: Option<String>,

    /// Optional. A list of client TLS/SSL certificates. If specified, mTLS authentication will be required to authenticate user connections. If set to an empty list, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tls_certificates: Option<Vec<DatabaseCertificateSpec>>,

    /// Optional. When 'true', requires TLS authentication for all connections - mTLS with valid clientTlsCertificates, regular TLS when clientTlsCertificates is not provided. If enableTls is set to 'false' while mTLS is required, it will remove the mTLS requirement and erase previously provided clientTlsCertificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,

    /// Optional. Changes the password used to access the database with the 'default' user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Optional. When 'true', allows connecting to the database with the 'default' user. When 'false', only defined access control users can connect to the database.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_user: Option<bool>,

    /// Optional. Changes Redis database alert details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<DatabaseAlertSpec>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Handler for Essentials database operations
///
/// Manages fixed-capacity databases with simplified configuration
/// and predictable pricing for Redis Cloud Essentials subscriptions.
pub struct FixedDatabasesHandler {
    client: CloudClient,
}

impl FixedDatabasesHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get all databases in an Essentials subscription
    /// Gets a list of all databases in the specified Essentials subscription.
    ///
    /// GET /fixed/subscriptions/{subscriptionId}/databases
    pub async fn get_fixed_subscription_databases(
        &self,
        subscription_id: i32,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AccountFixedSubscriptionDatabases> {
        let mut query = Vec::new();
        if let Some(v) = offset {
            query.push(format!("offset={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases{}",
                subscription_id, query_string
            ))
            .await
    }

    /// Create Essentials database
    /// Creates a new database in the specified Essentials subscription.
    ///
    /// POST /fixed/subscriptions/{subscriptionId}/databases
    pub async fn create_fixed_database(
        &self,
        subscription_id: i32,
        request: &FixedDatabaseCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/fixed/subscriptions/{}/databases", subscription_id),
                request,
            )
            .await
    }

    /// Delete Essentials database
    /// Deletes a database from an Essentials subscription.
    ///
    /// DELETE /fixed/subscriptions/{subscriptionId}/databases/{databaseId}
    pub async fn delete_fixed_database_by_id_1(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/fixed/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single Essentials database
    /// Gets details and settings of a single database in an Essentials subscription.
    ///
    /// GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}
    pub async fn get_subscription_database_by_id_1(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<FixedDatabase> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Update Essentials database
    /// Updates the specified Essentials database.
    ///
    /// PUT /fixed/subscriptions/{subscriptionId}/databases/{databaseId}
    pub async fn fixed_database_by_id(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &FixedDatabaseUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/fixed/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Backup Essentials database status
    /// Information on the latest database backup status identified by Essentials subscription Id and Essentials database Id
    ///
    /// GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/backup
    pub async fn get_database_backup_status_1(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}/backup",
                subscription_id, database_id
            ))
            .await
    }

    /// Back up Essentials database
    /// Manually back up the specified Essentials database to a backup path. By default, backups will be stored in the 'periodicBackupPath' location for this database.
    ///
    /// POST /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/backup
    pub async fn backup_database_1(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &FixedDatabaseBackupRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/fixed/subscriptions/{}/databases/{}/backup",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Get Essentials database import status
    /// Gets information on the latest import attempt for this Essentials database.
    ///
    /// GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/import
    pub async fn get_database_import_status_1(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}/import",
                subscription_id, database_id
            ))
            .await
    }

    /// Import data to an Essentials database
    /// Imports data from an RDB file or from a different Redis database into this Essentials database. WARNING: Importing data into a database removes all existing data from the database.
    ///
    /// POST /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/import
    pub async fn import_database_1(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &FixedDatabaseImportRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/fixed/subscriptions/{}/databases/{}/import",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Get Essentials database slow-log by database id
    /// Get slow-log for a specific database identified by Essentials subscription Id and database Id
    ///
    /// GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/slow-log
    pub async fn get_slow_log_1(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<DatabaseSlowLogEntries> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}/slow-log",
                subscription_id, database_id
            ))
            .await
    }

    /// Get database tags
    /// Gets a list of all database tags.
    ///
    /// GET /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags
    pub async fn get_tags_1(&self, subscription_id: i32, database_id: i32) -> Result<CloudTags> {
        self.client
            .get(&format!(
                "/fixed/subscriptions/{}/databases/{}/tags",
                subscription_id, database_id
            ))
            .await
    }

    /// Add a database tag
    /// Adds a single database tag to a database.
    ///
    /// POST /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags
    pub async fn create_tag_1(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseTagCreateRequest,
    ) -> Result<CloudTag> {
        self.client
            .post(
                &format!(
                    "/fixed/subscriptions/{}/databases/{}/tags",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Overwrite database tags
    /// Overwrites all tags on the database.
    ///
    /// PUT /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags
    pub async fn update_tags_1(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseTagsUpdateRequest,
    ) -> Result<CloudTags> {
        self.client
            .put(
                &format!(
                    "/fixed/subscriptions/{}/databases/{}/tags",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Delete database tag
    /// Removes the specified tag from the database.
    ///
    /// DELETE /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags/{tagKey}
    pub async fn delete_tag_1(
        &self,
        subscription_id: i32,
        database_id: i32,
        tag_key: String,
    ) -> Result<HashMap<String, Value>> {
        let response = self
            .client
            .delete_raw(&format!(
                "/fixed/subscriptions/{}/databases/{}/tags/{}",
                subscription_id, database_id, tag_key
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update database tag value
    /// Updates the value of the specified database tag.
    ///
    /// PUT /fixed/subscriptions/{subscriptionId}/databases/{databaseId}/tags/{tagKey}
    pub async fn update_tag_1(
        &self,
        subscription_id: i32,
        database_id: i32,
        tag_key: String,
        request: &DatabaseTagUpdateRequest,
    ) -> Result<CloudTag> {
        self.client
            .put(
                &format!(
                    "/fixed/subscriptions/{}/databases/{}/tags/{}",
                    subscription_id, database_id, tag_key
                ),
                request,
            )
            .await
    }
}
