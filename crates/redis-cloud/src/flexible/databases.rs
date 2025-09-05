//! Database management operations for Pro subscriptions
//!
//! This module provides comprehensive database management functionality for Redis Cloud
//! Pro subscriptions, including creation, configuration, backup, import/export, and
//! monitoring capabilities.
//!
//! # Overview
//!
//! Pro databases offer the full range of Redis Cloud features including high availability,
//! auto-scaling, clustering, modules, and advanced data persistence options. They can be
//! deployed across multiple cloud providers and regions.
//!
//! # Key Features
//!
//! - **Database Lifecycle**: Create, update, delete, and manage databases
//! - **Backup & Restore**: Automated and on-demand backup operations
//! - **Import/Export**: Import data from RDB files or other Redis instances
//! - **Modules**: Support for RedisJSON, RediSearch, RedisGraph, RedisTimeSeries, RedisBloom
//! - **High Availability**: Replication, auto-failover, and clustering support
//! - **Monitoring**: Metrics, alerts, and performance insights
//! - **Security**: TLS, password protection, and ACL support
//!
//! # Database Configuration Options
//!
//! - Memory limits from 250MB to 500GB+
//! - Support for Redis OSS Cluster API
//! - Data persistence: AOF, snapshot, or both
//! - Data eviction policies
//! - Replication and clustering
//! - Custom Redis versions
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, DatabaseHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = DatabaseHandler::new(client);
//!
//! // List all databases in a subscription (subscription ID 123)
//! let databases = handler.get_subscription_databases(123, None, None).await?;
//!
//! // Get specific database details
//! let database = handler.get_subscription_database_by_id(123, 456).await?;
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
pub struct AccountSubscriptionDatabases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

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

/// Active-Active database flush request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrdbFlushRequest {
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

/// Database certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCertificate {
    /// An X.509 PEM (base64) encoded server certificate with new line characters replaced by '\n'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_certificate_pem_string: Option<String>,

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

/// BdbVersionUpgradeStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BdbVersionUpgradeStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_redis_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_status: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Active-Active database update local properties request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrdbUpdatePropertiesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// Optional. Updated database name. Database name is limited to 40 characters or less and must include only letters, digits, and hyphens ('-'). It must start with a letter and end with a letter or digit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional. When 'false': Creates a deployment plan and deploys it, updating any resources required by the plan. When 'true': creates a read-only deployment plan and does not update any resources. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// Optional. Total memory in GB, including replication and other overhead. You cannot set both datasetSizeInGb and totalMemoryInGb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    /// Optional. The maximum amount of data in the dataset for this database in GB. You cannot set both datasetSizeInGb and totalMemoryInGb. If ‘replication’ is 'true', the database’s total memory will be twice as large as the datasetSizeInGb.If ‘replication’ is false, the database’s total memory will be the datasetSizeInGb value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    /// Optional. Support Redis [OSS Cluster API](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#oss-cluster-api). Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    /// Optional. If set to 'true', the database will use the external endpoint for OSS Cluster API. This setting blocks the database's private endpoint. Can only be set if 'supportOSSClusterAPI' is 'true'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    /// Optional. A public key client TLS/SSL certificate with new line characters replaced with '\n'. If specified, mTLS authentication will be required to authenticate user connections if it is not already required. If set to an empty string, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ssl_certificate: Option<String>,

    /// Optional. A list of client TLS/SSL certificates. If specified, mTLS authentication will be required to authenticate user connections. If set to an empty list, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tls_certificates: Option<Vec<DatabaseCertificateSpec>>,

    /// Optional. When 'true', requires TLS authentication for all connections - mTLS with valid clientTlsCertificates, regular TLS when clientTlsCertificates is not provided. If enableTls is set to 'false' while mTLS is required, it will remove the mTLS requirement and erase previously provided clientTlsCertificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,

    /// Optional. Type and rate of data persistence in all regions that don't set local 'dataPersistence'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_data_persistence: Option<String>,

    /// Optional. Changes the password used to access the database in all regions that don't set a local 'password'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_password: Option<String>,

    /// Optional. List of source IP addresses or subnet masks to allow in all regions that don't set local 'sourceIp' settings. If set, Redis clients will be able to connect to this database only from within the specified source IP addresses ranges. Example: ['192.168.10.0/32', '192.168.12.0/24']
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_source_ip: Option<Vec<String>>,

    /// Optional. Redis database alert settings in all regions that don't set local 'alerts'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_alerts: Option<Vec<DatabaseAlertSpec>>,

    /// Optional. A list of regions and local settings to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<LocalRegionProperties>>,

    /// Optional. Data eviction policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

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

/// Optional. Changes Remote backup configuration details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseBackupConfig {
    /// Optional. Determine if backup should be active. Default: null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Required when active is 'true'. Defines the interval between backups. Format: 'every-x-hours', where x is one of 24, 12, 6, 4, 2, or 1. Example: "every-4-hours"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_interval: Option<String>,

    /// Optional. Hour when the backup starts. Available only for "every-12-hours" and "every-24-hours" backup intervals. Specified as an hour in 24-hour UTC time. Example: "14:00" is 2 PM UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_utc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_backup_time_utc: Option<String>,

    /// Required when active is 'true'. Type of storage to host backup files. Can be "aws-s3", "google-blob-storage", "azure-blob-storage", or "ftp". See [Set up backup storage locations](https://redis.io/docs/latest/operate/rc/databases/back-up-data/#set-up-backup-storage-locations) to learn how to set up backup storage locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_storage_type: Option<String>,

    /// Required when active is 'true'. Path to the backup storage location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_path: Option<String>,

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

/// Database backup request message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseBackupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// Required for Active-Active databases. Name of the cloud provider region to back up. When backing up an Active-Active database, you must back up each region separately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,

    /// Optional. Manually backs up data to this location, instead of the set 'remoteBackup' location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adhoc_backup_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

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

/// Request structure for creating a new Pro database
///
/// Contains all configuration options for creating a database in a Pro subscription,
/// including memory settings, replication, persistence, modules, and networking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// Optional. When 'false': Creates a deployment plan and deploys it, creating any resources required by the plan. When 'true': creates a read-only deployment plan and does not create any resources. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// Name of the database. Database name is limited to 40 characters or less and must include only letters, digits, and hyphens ('-'). It must start with a letter and end with a letter or digit.
    pub name: String,

    /// Optional. Database protocol. Only set to 'memcached' if you have a legacy application. Default: 'redis'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Optional. TCP port on which the database is available (10000-19999). Generated automatically if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Optional. Total memory in GB, including replication and other overhead. You cannot set both datasetSizeInGb and totalMemoryInGb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    /// Optional. The maximum amount of data in the dataset for this database in GB. You cannot set both datasetSizeInGb and totalMemoryInGb. If ‘replication’ is 'true', the database’s total memory will be twice as large as the datasetSizeInGb. If ‘replication’ is false, the database’s total memory will be the datasetSizeInGb value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    /// Optional. If specified, redisVersion defines the Redis database version. If omitted, the Redis version will be set to the default version (available in 'GET /subscriptions/redis-versions')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

    /// Optional. Redis Serialization Protocol version. Must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    /// Optional. Support [OSS Cluster API](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#oss-cluster-api). Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    /// Optional. If set to 'true', the database will use the external endpoint for OSS Cluster API. This setting blocks the database's private endpoint. Can only be set if 'supportOSSClusterAPI' is 'true'. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    /// Optional. Type and rate of data persistence in persistent storage. Default: 'none'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    /// Optional. Data eviction policy. Default: 'volatile-lru'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<String>,

    /// Optional. Sets database replication. Default: 'true'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    /// Optional. This database will be a replica of the specified Redis databases provided as one or more URI(s). Example: 'redis://user:password@host:port'. If the URI provided is a Redis Cloud database, only host and port should be provided. Example: ['redis://endpoint1:6379', 'redis://endpoint2:6380'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_of: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica: Option<ReplicaOfSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_measurement: Option<DatabaseThroughputSpec>,

    /// Optional. Expected throughput per region for an Active-Active database. Default: 1000 read and write ops/sec for each region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_throughput_measurement: Option<Vec<LocalThroughput>>,

    /// Optional. Relevant only to ram-and-flash (also known as Auto Tiering) subscriptions. Estimated average size in bytes of the items stored in the database. Default: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_item_size_in_bytes: Option<i64>,

    /// Optional. The path to a backup storage location. If specified, the database will back up every 24 hours to this location, and you can manually back up the database to this location at any time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_backup_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_backup: Option<DatabaseBackupConfig>,

    /// Optional. List of source IP addresses or subnet masks to allow. If specified, Redis clients will be able to connect to this database only from within the specified source IP addresses ranges. Example: '['192.168.10.0/32', '192.168.12.0/24']'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<Vec<String>>,

    /// Optional. A public key client TLS/SSL certificate with new line characters replaced with '\n'. If specified, mTLS authentication will be required to authenticate user connections. Default: 'null'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ssl_certificate: Option<String>,

    /// Optional. A list of client TLS/SSL certificates. If specified, mTLS authentication will be required to authenticate user connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tls_certificates: Option<Vec<DatabaseCertificateSpec>>,

    /// Optional. When 'true', requires TLS authentication for all connections - mTLS with valid clientTlsCertificates, regular TLS when clientTlsCertificates is not provided. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,

    /// Optional. Password to access the database. If not set, a random 32-character alphanumeric password will be automatically generated. Can only be set if 'protocol' is 'redis'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Optional. Memcached (SASL) Username to access the database. If not set, the username will be set to a 'mc-' prefix followed by a random 5 character long alphanumeric. Can only be set if 'protocol' is 'memcached'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_username: Option<String>,

    /// Optional. Memcached (SASL) Password to access the database. If not set, a random 32 character long alphanumeric password will be automatically generated. Can only be set if 'protocol' is 'memcached'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_password: Option<String>,

    /// Optional. Redis database alert details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<DatabaseAlertSpec>>,

    /// Optional. Redis advanced capabilities (also known as modules) to be provisioned in the database. Use GET /database-modules to get a list of available advanced capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<DatabaseModuleSpec>>,

    /// Optional. Database [Hashing policy](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#manage-the-hashing-policy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Optional. The query performance factor adds extra compute power specifically for search and query databases. You can increase your queries per second by the selected factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_performance_factor: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Database import request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseImportRequest {
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

/// Upgrades the specified Pro database to a later Redis version.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUpgradeRedisVersionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    /// The target Redis version the database will be upgraded to. Use GET /subscriptions/redis-versions to get a list of available Redis versions.
    pub target_redis_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

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

/// Optional. A list of regions and local settings to update.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalRegionProperties {
    /// Required. Name of the region to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_backup: Option<DatabaseBackupConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_throughput_measurement: Option<LocalThroughput>,

    /// Optional. Type and rate of data persistence for this region. If set, 'globalDataPersistence' will not apply to this region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    /// Optional. Changes the password used to access the database in this region. If set, 'globalPassword' will not apply to this region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Optional. List of source IP addresses or subnet masks to allow in this region. If set, Redis clients will be able to connect to the database in this region only from within the specified source IP addresses ranges, and 'globalSourceIp' will not apply to this region. Example: ['192.168.10.0/32', '192.168.12.0/24']
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<Vec<String>>,

    /// Optional. Redis database alert settings for this region. If set, 'glboalAlerts' will not apply to this region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<DatabaseAlertSpec>>,

    /// Optional. Redis Serialization Protocol version for this region. Must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

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

/// Database update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i32>,

    /// Optional. When 'false': Creates a deployment plan and deploys it, updating any resources required by the plan. When 'true': creates a read-only deployment plan and does not update any resources. Default: 'false'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// Optional. Updated database name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional. Total memory in GB, including replication and other overhead. You cannot set both datasetSizeInGb and totalMemoryInGb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    /// Optional. The maximum amount of data in the dataset for this database in GB. You cannot set both datasetSizeInGb and totalMemoryInGb. If ‘replication’ is 'true', the database’s total memory will be twice as large as the datasetSizeInGb.If ‘replication’ is false, the database’s total memory will be the datasetSizeInGb value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_size_in_gb: Option<f64>,

    /// Optional. Redis Serialization Protocol version. Must be compatible with Redis version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_measurement: Option<DatabaseThroughputSpec>,

    /// Optional. Type and rate of data persistence in persistent storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,

    /// Optional. Data eviction policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<String>,

    /// Optional. Turns database replication on or off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    /// Optional. Hashing policy Regex rules. Used only if 'shardingType' is 'custom-regex-rules'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_rules: Option<Vec<String>>,

    /// Optional. This database will be a replica of the specified Redis databases provided as one or more URI(s). Example: 'redis://user:password@host:port'. If the URI provided is a Redis Cloud database, only host and port should be provided. Example: ['redis://endpoint1:6379', 'redis://endpoint2:6380'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_of: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica: Option<ReplicaOfSpec>,

    /// Optional. Support Redis [OSS Cluster API](https://redis.io/docs/latest/operate/rc/databases/configuration/clustering/#oss-cluster-api).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,

    /// Optional. If set to 'true', the database will use the external endpoint for OSS Cluster API. This setting blocks the database's private endpoint. Can only be set if 'supportOSSClusterAPI' is 'true'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    /// Optional. Changes the password used to access the database with the 'default' user. Can only be set if 'protocol' is 'redis'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Optional. Changes the Memcached (SASL) username to access the database. Can only be set if 'protocol' is 'memcached'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_username: Option<String>,

    /// Optional. Changes the Memcached (SASL) password to access the database. Can only be set if 'protocol' is 'memcached'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_password: Option<String>,

    /// Optional. List of source IP addresses or subnet masks to allow. If specified, Redis clients will be able to connect to this database only from within the specified source IP addresses ranges. Example: '['192.168.10.0/32', '192.168.12.0/24']'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<Vec<String>>,

    /// Optional. A public key client TLS/SSL certificate with new line characters replaced with '\n'. If specified, mTLS authentication will be required to authenticate user connections if it is not already required. If set to an empty string, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ssl_certificate: Option<String>,

    /// Optional. A list of client TLS/SSL certificates. If specified, mTLS authentication will be required to authenticate user connections. If set to an empty list, TLS client certificates will be removed and mTLS will not be required. TLS connection may still apply, depending on the value of 'enableTls'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tls_certificates: Option<Vec<DatabaseCertificateSpec>>,

    /// Optional. When 'true', requires TLS authentication for all connections - mTLS with valid clientTlsCertificates, regular TLS when clientTlsCertificates is not provided. If enableTls is set to 'false' while mTLS is required, it will remove the mTLS requirement and erase previously provided clientTlsCertificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,

    /// Optional. When 'true', allows connecting to the database with the 'default' user. When 'false', only defined access control users can connect to the database. Can only be set if 'protocol' is 'redis'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_user: Option<bool>,

    /// Optional. Changes the backup location path. If specified, the database will back up every 24 hours to this location, and you can manually back up the database to this location at any time. If set to an empty string, the backup path will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_backup_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_backup: Option<DatabaseBackupConfig>,

    /// Optional. Changes Redis database alert details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<DatabaseAlertSpec>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Optional. Changes the query performance factor. The query performance factor adds extra compute power specifically for search and query databases. You can increase your queries per second by the selected factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_performance_factor: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Handler for Pro database operations
///
/// Manages database lifecycle, configuration, backup/restore, import/export,
/// and monitoring for Redis Cloud Pro subscriptions.
pub struct DatabaseHandler {
    client: CloudClient,
}

impl DatabaseHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get all databases in a Pro subscription
    /// Gets a list of all databases in the specified Pro subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/databases
    pub async fn get_subscription_databases(
        &self,
        subscription_id: i32,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AccountSubscriptionDatabases> {
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
                "/subscriptions/{}/databases{}",
                subscription_id, query_string
            ))
            .await
    }

    /// Create Pro database in existing subscription
    /// Creates a new database in an existing Pro subscription.
    ///
    /// POST /subscriptions/{subscriptionId}/databases
    pub async fn create_database(
        &self,
        subscription_id: i32,
        request: &DatabaseCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!("/subscriptions/{}/databases", subscription_id),
                request,
            )
            .await
    }

    /// Delete Pro database
    /// Deletes a database from a Pro subscription.
    ///
    /// DELETE /subscriptions/{subscriptionId}/databases/{databaseId}
    pub async fn delete_database_by_id(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single Pro database
    /// Gets details and settings of a single database in a Pro subscription.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}
    pub async fn get_subscription_database_by_id(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<Database> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Update Pro database
    /// Updates an existing Pro database.
    ///
    /// PUT /subscriptions/{subscriptionId}/databases/{databaseId}
    pub async fn update_database(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Get Pro database backup status
    /// Gets information on the latest backup attempt for this Pro database.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}/backup
    pub async fn get_database_backup_status(
        &self,
        subscription_id: i32,
        database_id: i32,
        region_name: Option<String>,
    ) -> Result<TaskStateUpdate> {
        let mut query = Vec::new();
        if let Some(v) = region_name {
            query.push(format!("regionName={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/backup{}",
                subscription_id, database_id, query_string
            ))
            .await
    }

    /// Back up Pro database
    /// Manually back up the specified Pro database to a backup path. By default, backups will be stored in the 'remoteBackup' location for this database.
    ///
    /// POST /subscriptions/{subscriptionId}/databases/{databaseId}/backup
    pub async fn backup_database(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseBackupRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/backup",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Get Pro database TLS certificate
    /// Gets the X.509 PEM (base64) encoded server certificate for TLS connection to the database. Requires 'enableTLS' to be 'true' for the database.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}/certificate
    pub async fn get_subscription_database_certificate(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<DatabaseCertificate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/certificate",
                subscription_id, database_id
            ))
            .await
    }

    /// Flush Pro database
    /// Deletes all data from the specified Pro database.
    ///
    /// PUT /subscriptions/{subscriptionId}/databases/{databaseId}/flush
    pub async fn flush_crdb(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &CrdbFlushRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/flush",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Get Pro database import status
    /// Gets information on the latest import attempt for this Pro database.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}/import
    pub async fn get_database_import_status(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<TaskStateUpdate> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/import",
                subscription_id, database_id
            ))
            .await
    }

    /// Import data to a Pro database
    /// Imports data from an RDB file or from a different Redis database into this Pro database. WARNING: Importing data into a database removes all existing data from the database.
    ///
    /// POST /subscriptions/{subscriptionId}/databases/{databaseId}/import
    pub async fn import_database(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseImportRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/import",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Update Active-Active database
    /// (Active-Active databases only) Updates database properties for an Active-Active database.
    ///
    /// PUT /subscriptions/{subscriptionId}/databases/{databaseId}/regions
    pub async fn update_crdb_local_properties(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &CrdbUpdatePropertiesRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/regions",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Get database slowlog
    /// Gets the slowlog for a specific database.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}/slow-log
    pub async fn get_slow_log(
        &self,
        subscription_id: i32,
        database_id: i32,
        region_name: Option<String>,
    ) -> Result<DatabaseSlowLogEntries> {
        let mut query = Vec::new();
        if let Some(v) = region_name {
            query.push(format!("regionName={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/slow-log{}",
                subscription_id, database_id, query_string
            ))
            .await
    }

    /// Get database tags
    /// Gets a list of all database tags.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}/tags
    pub async fn get_tags(&self, subscription_id: i32, database_id: i32) -> Result<CloudTags> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/tags",
                subscription_id, database_id
            ))
            .await
    }

    /// Add a database tag
    /// Adds a single database tag to a database.
    ///
    /// POST /subscriptions/{subscriptionId}/databases/{databaseId}/tags
    pub async fn create_tag(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseTagCreateRequest,
    ) -> Result<CloudTag> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/tags",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Overwrite database tags
    /// Overwrites all tags on the database.
    ///
    /// PUT /subscriptions/{subscriptionId}/databases/{databaseId}/tags
    pub async fn update_tags(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseTagsUpdateRequest,
    ) -> Result<CloudTags> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/tags",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }

    /// Delete database tag
    /// Removes the specified tag from the database.
    ///
    /// DELETE /subscriptions/{subscriptionId}/databases/{databaseId}/tags/{tagKey}
    pub async fn delete_tag(
        &self,
        subscription_id: i32,
        database_id: i32,
        tag_key: String,
    ) -> Result<HashMap<String, Value>> {
        let response = self
            .client
            .delete_raw(&format!(
                "/subscriptions/{}/databases/{}/tags/{}",
                subscription_id, database_id, tag_key
            ))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update database tag value
    /// Updates the value of the specified database tag.
    ///
    /// PUT /subscriptions/{subscriptionId}/databases/{databaseId}/tags/{tagKey}
    pub async fn update_tag(
        &self,
        subscription_id: i32,
        database_id: i32,
        tag_key: String,
        request: &DatabaseTagUpdateRequest,
    ) -> Result<CloudTag> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/tags/{}",
                    subscription_id, database_id, tag_key
                ),
                request,
            )
            .await
    }

    /// Get Pro database version upgrade status
    /// Gets information on the latest upgrade attempt for this Pro database.
    ///
    /// GET /subscriptions/{subscriptionId}/databases/{databaseId}/upgrade
    pub async fn get_database_redis_version_upgrade_status(
        &self,
        subscription_id: i32,
        database_id: i32,
    ) -> Result<BdbVersionUpgradeStatus> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/upgrade",
                subscription_id, database_id
            ))
            .await
    }

    /// Upgrade Pro database version
    ///
    /// POST /subscriptions/{subscriptionId}/databases/{databaseId}/upgrade
    pub async fn upgrade_database_redis_version(
        &self,
        subscription_id: i32,
        database_id: i32,
        request: &DatabaseUpgradeRedisVersionRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/upgrade",
                    subscription_id, database_id
                ),
                request,
            )
            .await
    }
}
