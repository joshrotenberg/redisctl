//! Common types used across the Redis Cloud API
//!
//! This module contains shared types and enumerations used by multiple API endpoints.
//! Types specific to a single resource are defined in their respective modules.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Cloud provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
}

/// Subscription status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SubscriptionStatus {
    Active,
    Inactive,
    Error,
    Deleting,
    Pending,
}

/// Database status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DatabaseStatus {
    Active,
    Inactive,
    Creating,
    Deleting,
    Error,
    Updating,
    Pending,
}

/// Memory storage types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MemoryStorage {
    Ram,
    RamAndFlash,
}

/// Payment method types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaymentMethodType {
    CreditCard,
    Marketplace,
}

/// Data persistence options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataPersistence {
    None,
    AofEveryWrite,
    AofEverySecond,
    SnapshotEveryHour,
    SnapshotEvery6Hours,
    SnapshotEvery12Hours,
}

/// Eviction policy options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvictionPolicy {
    NoEviction,
    AllkeysLru,
    AllkeysLfu,
    AllkeysRandom,
    VolatileLru,
    VolatileLfu,
    VolatileTtl,
    VolatileRandom,
}

/// Protocol types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    Redis,
    Memcached,
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Initialized,
    InProgress,
    Completed,
    Failed,
}

/// Link object for API navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    pub href: String,
    pub rel: String,
}

/// Alert settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// Module capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

/// Tag for resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// Pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_hour_price: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_hour_price: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_gb_hour_price: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_gb_hour_price: Option<f64>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: u32,
    pub name: String,
    pub provider: CloudProvider,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkInfo>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_cidr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}
