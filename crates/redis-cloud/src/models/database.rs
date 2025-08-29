//! Database-related data models
//!
//! Contains data structures for Redis Cloud database operations including database
//! configuration, status information, and request/response models for database management.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Represents a Redis Cloud database instance
///
/// Contains all the configuration, status, and operational information for a database
/// deployed in Redis Cloud. This includes memory settings, persistence configuration,
/// replication status, and connection endpoints.
///
/// # Examples
///
/// ```rust,no_run
/// # use redis_cloud::{CloudClient, CloudDatabaseHandler};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = CloudClient::builder().api_key("key").api_secret("secret").build()?;
/// let db_handler = CloudDatabaseHandler::new(client);
/// let database = db_handler.get(123, 456).await?;
///
/// println!("Database: {}", database.name);
/// println!("Status: {}", database.status);
/// println!("Memory: {:.1} GB", database.memory_limit_in_gb);
///
/// if let Some(endpoint) = &database.public_endpoint {
///     println!("Connect to: {}", endpoint);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDatabase {
    /// Unique database identifier within the subscription
    pub db_id: u32,
    /// Human-readable database name
    pub name: String,
    /// Redis protocol version (e.g., "redis")
    pub protocol: String,
    /// Cloud provider hosting the database (AWS, GCP, Azure)
    pub provider: String,
    /// Cloud region where the database is deployed
    pub region: String,
    /// Current database status (active, pending, error, etc.)
    pub status: String,
    /// Maximum memory allocation in gigabytes
    pub memory_limit_in_gb: f64,
    /// Current memory usage in megabytes
    pub memory_used_in_mb: Option<f64>,
    /// Memory usage as a percentage (0-100)
    pub memory_usage: Option<f64>,
    /// Data persistence configuration (none, aof-every-1-sec, etc.)
    pub data_persistence: String,
    /// Whether replication is enabled for high availability
    pub replication: bool,
    /// Data eviction policy when memory limit is reached
    pub data_eviction: Option<String>,
    /// Throughput measurement configuration
    pub throughput_measurement: Option<ThroughputMeasurement>,
    /// ISO 8601 timestamp when database was activated
    pub activated_on: Option<String>,
    /// ISO 8601 timestamp of last modification
    pub last_modified: Option<String>,
    /// Public internet connection endpoint
    pub public_endpoint: Option<String>,
    /// VPC-private connection endpoint
    pub private_endpoint: Option<String>,

    /// Additional fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// Throughput measurement configuration for database performance monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMeasurement {
    /// Measurement method (e.g., "operations-per-second")
    pub by: String,
    /// Throughput limit value
    pub value: u32,
}

/// Request payload for creating a new database
///
/// Defines the configuration for a new Redis database including memory limits,
/// persistence settings, and optional features like cluster API support.
///
/// # Examples
///
/// ```rust,no_run
/// use redis_cloud::CreateDatabaseRequest;
///
/// let request = CreateDatabaseRequest::builder()
///     .name("production-cache")
///     .memory_limit_in_gb(5.0)
///     .data_persistence("aof-every-1-sec")
///     .replication(true)
///     .password("secure-password-123")
///     .support_oss_cluster_api(false)
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateDatabaseRequest {
    #[builder(setter(into))]
    pub name: String,
    pub memory_limit_in_gb: f64,
    #[builder(setter(into))]
    pub data_persistence: String,
    #[builder(default)]
    pub replication: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub data_eviction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub support_oss_cluster_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,
}

/// Update database request
///
/// All fields are optional - only provide the fields you want to update.
///
/// # Examples
///
/// ```rust,no_run
/// use redis_cloud::UpdateDatabaseRequest;
///
/// let request = UpdateDatabaseRequest::builder()
///     .memory_limit_in_gb(10.0)
///     .replication(true)
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct UpdateDatabaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub memory_limit_in_gb: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub data_persistence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub data_eviction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,
}
