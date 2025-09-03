//! Database management for Redis Cloud Pro
//!
//! ## Overview
//! - Create and manage Pro databases
//! - Configure database settings and modules
//! - Manage database backups and imports
//! - Query database metrics and slow log

use crate::client::CloudClient;
use crate::error::Result;
use crate::types::{
    AlertSettings, DataPersistence, DatabaseStatus, EvictionPolicy, Module, Protocol, Tag,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Database information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    #[serde(rename = "databaseId")]
    pub database_id: u32,

    pub name: String,

    #[serde(rename = "subscriptionId")]
    pub subscription_id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DatabaseStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,

    #[serde(rename = "memoryLimitInGb", skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,

    #[serde(rename = "memoryUsedInMb", skip_serializing_if = "Option::is_none")]
    pub memory_used_in_mb: Option<f64>,

    #[serde(rename = "memoryStorage", skip_serializing_if = "Option::is_none")]
    pub memory_storage: Option<String>,

    #[serde(
        rename = "supportOSSClusterApi",
        skip_serializing_if = "Option::is_none"
    )]
    pub support_oss_cluster_api: Option<bool>,

    #[serde(
        rename = "useExternalEndpointForOSSClusterApi",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    #[serde(rename = "dataPersistence", skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<DataPersistence>,

    #[serde(rename = "dataEvictionPolicy", skip_serializing_if = "Option::is_none")]
    pub data_eviction_policy: Option<EvictionPolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,

    #[serde(
        rename = "throughputMeasurement",
        skip_serializing_if = "Option::is_none"
    )]
    pub throughput_measurement: Option<ThroughputMeasurement>,

    #[serde(rename = "activatedOn", skip_serializing_if = "Option::is_none")]
    pub activated_on: Option<String>,

    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,

    #[serde(rename = "publicEndpoint", skip_serializing_if = "Option::is_none")]
    pub public_endpoint: Option<String>,

    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<Module>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<AlertSettings>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityConfig>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Throughput measurement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMeasurement {
    pub by: String,
    pub value: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(
        rename = "sslClientAuthentication",
        skip_serializing_if = "Option::is_none"
    )]
    pub ssl_client_authentication: Option<bool>,

    #[serde(
        rename = "tlsClientAuthentication",
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_client_authentication: Option<bool>,

    #[serde(rename = "sourceIps", skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,
}

/// Create database request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateDatabaseRequest {
    #[builder(setter(into))]
    pub name: String,

    #[serde(rename = "memoryLimitInGb")]
    pub memory_limit_in_gb: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub protocol: Option<Protocol>,

    #[serde(
        rename = "supportOSSClusterApi",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub support_oss_cluster_api: Option<bool>,

    #[serde(
        rename = "useExternalEndpointForOSSClusterApi",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    #[serde(rename = "dataPersistence", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub data_persistence: Option<DataPersistence>,

    #[serde(rename = "dataEvictionPolicy", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub data_eviction_policy: Option<EvictionPolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub replication: Option<bool>,

    #[serde(
        rename = "throughputMeasurement",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub throughput_measurement: Option<ThroughputMeasurement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub modules: Option<Vec<Module>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alerts: Option<Vec<AlertSettings>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub security: Option<SecurityConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,

    #[serde(rename = "enableTls", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_tls: Option<bool>,

    #[serde(
        rename = "clientSslCertificate",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into, strip_option))]
    pub client_ssl_certificate: Option<String>,

    #[serde(rename = "enableDefaultUser", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_default_user: Option<bool>,
}

/// Update database request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateDatabaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[serde(rename = "memoryLimitInGb", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub memory_limit_in_gb: Option<f64>,

    #[serde(
        rename = "supportOSSClusterApi",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub support_oss_cluster_api: Option<bool>,

    #[serde(
        rename = "useExternalEndpointForOSSClusterApi",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,

    #[serde(rename = "dataPersistence", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub data_persistence: Option<DataPersistence>,

    #[serde(rename = "dataEvictionPolicy", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub data_eviction_policy: Option<EvictionPolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub replication: Option<bool>,

    #[serde(
        rename = "throughputMeasurement",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(strip_option))]
    pub throughput_measurement: Option<ThroughputMeasurement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub modules: Option<Vec<Module>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alerts: Option<Vec<AlertSettings>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub security: Option<SecurityConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,

    #[serde(rename = "enableTls", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_tls: Option<bool>,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BackupConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub interval: Option<u32>,

    #[serde(rename = "timeUtc", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub time_utc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub destination: Option<BackupDestination>,
}

/// Backup destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupDestination {
    #[serde(rename = "type")]
    pub destination_type: String,

    pub uri: String,
}

/// Import configuration
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ImportConfig {
    #[serde(rename = "sourceType")]
    #[builder(setter(into))]
    pub source_type: String,

    #[serde(rename = "importFromUri")]
    #[builder(setter(into))]
    pub import_from_uri: String,
}

/// Database handler
pub struct DatabaseHandler {
    client: CloudClient,
}

impl DatabaseHandler {
    pub fn new(client: CloudClient) -> Self {
        DatabaseHandler { client }
    }

    /// List all databases in a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Vec<Database>> {
        self.client
            .get(&format!("/subscriptions/{}/databases", subscription_id))
            .await
    }

    /// Get a specific database
    pub async fn get(&self, subscription_id: u32, database_id: u32) -> Result<Database> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Create a new database
    pub async fn create(
        &self,
        subscription_id: u32,
        request: CreateDatabaseRequest,
    ) -> Result<Database> {
        self.client
            .post(
                &format!("/subscriptions/{}/databases", subscription_id),
                &request,
            )
            .await
    }

    /// Update a database
    pub async fn update(
        &self,
        subscription_id: u32,
        database_id: u32,
        request: UpdateDatabaseRequest,
    ) -> Result<Database> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    /// Delete a database
    pub async fn delete(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Flush database
    pub async fn flush(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .put_raw(
                &format!(
                    "/subscriptions/{}/databases/{}/flush",
                    subscription_id, database_id
                ),
                &Value::Null,
            )
            .await
    }

    /// Get database backup configuration
    pub async fn get_backup(&self, subscription_id: u32, database_id: u32) -> Result<BackupConfig> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/backup",
                subscription_id, database_id
            ))
            .await
    }

    /// Update database backup configuration
    pub async fn update_backup(
        &self,
        subscription_id: u32,
        database_id: u32,
        config: BackupConfig,
    ) -> Result<BackupConfig> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/backup",
                    subscription_id, database_id
                ),
                &config,
            )
            .await
    }

    /// Import data into database
    pub async fn import(
        &self,
        subscription_id: u32,
        database_id: u32,
        config: ImportConfig,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/import",
                    subscription_id, database_id
                ),
                &config,
            )
            .await
    }

    /// Get slow log entries
    pub async fn slow_log(
        &self,
        subscription_id: u32,
        database_id: u32,
        limit: Option<u32>,
    ) -> Result<Vec<Value>> {
        let path = match limit {
            Some(l) => format!(
                "/subscriptions/{}/databases/{}/slow-log?limit={}",
                subscription_id, database_id, l
            ),
            None => format!(
                "/subscriptions/{}/databases/{}/slow-log",
                subscription_id, database_id
            ),
        };
        self.client.get(&path).await
    }

    /// Get database tags
    pub async fn get_tags(&self, subscription_id: u32, database_id: u32) -> Result<Vec<Tag>> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/tags",
                subscription_id, database_id
            ))
            .await
    }

    /// Set database tags
    pub async fn set_tags(
        &self,
        subscription_id: u32,
        database_id: u32,
        tags: Vec<Tag>,
    ) -> Result<Vec<Tag>> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/tags",
                    subscription_id, database_id
                ),
                &tags,
            )
            .await
    }

    /// Add database tag
    pub async fn add_tag(&self, subscription_id: u32, database_id: u32, tag: Tag) -> Result<Tag> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/tags",
                    subscription_id, database_id
                ),
                &tag,
            )
            .await
    }

    /// Delete database tag
    pub async fn delete_tag(
        &self,
        subscription_id: u32,
        database_id: u32,
        tag_key: &str,
    ) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/databases/{}/tags/{}",
                subscription_id, database_id, tag_key
            ))
            .await
    }

    /// Get database certificate
    pub async fn get_certificate(&self, subscription_id: u32, database_id: u32) -> Result<String> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/certificate",
                subscription_id, database_id
            ))
            .await
    }

    /// Upgrade database Redis version
    pub async fn upgrade(
        &self,
        subscription_id: u32,
        database_id: u32,
        version: &str,
    ) -> Result<Value> {
        let body = serde_json::json!({"redisVersion": version});
        self.client
            .put_raw(
                &format!(
                    "/subscriptions/{}/databases/{}/upgrade",
                    subscription_id, database_id
                ),
                &body,
            )
            .await
    }
}
