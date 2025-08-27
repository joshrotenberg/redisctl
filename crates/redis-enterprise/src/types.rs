//! Specialized types for Redis Enterprise API objects

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Database group - represents a group of databases sharing a memory pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BdbGroup {
    pub uid: u32,
    pub memory_size: u64,
    pub members: Vec<u32>,
}

/// Database connections auditing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConnsAuditingConfig {
    pub audit_protocol: String,
    pub audit_address: String,
    pub audit_port: u16,
    pub audit_reconnect_interval: Option<u32>,
    pub audit_reconnect_max_attempts: Option<u32>,
}

/// Cluster check result for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub cluster_test_result: Option<String>,
    pub nodes: Option<Vec<NodeCheckResult>>,
}

/// Node check result for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCheckResult {
    pub node_uid: u32,
    pub status: String,
    pub checks: Option<Vec<Value>>,
}

/// Services configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfiguration {
    pub cm_server: Option<ServiceConfig>,
    pub crdb_coordinator: Option<ServiceConfig>,
    pub crdb_worker: Option<ServiceConfig>,
    pub mdns_server: Option<ServiceConfig>,
    pub pdns_server: Option<ServiceConfig>,
    pub redis_server: Option<ServiceConfig>,
    pub saslauthd: Option<ServiceConfig>,
    pub stats_archiver: Option<ServiceConfig>,
}

/// Individual service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub enabled: bool,
    pub port: Option<u16>,
    pub settings: Option<Value>,
}

/// OCSP (Online Certificate Status Protocol) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcspConfig {
    pub ocsp_url: String,
    pub ocsp_response_timeout_seconds: Option<u32>,
    pub query_frequency_seconds: Option<u32>,
    pub recovery_frequency_seconds: Option<u32>,
    pub recovery_max_tries: Option<u32>,
    pub responder_cert: Option<String>,
}

/// OCSP status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcspStatus {
    pub cert_name: String,
    pub ocsp_status: String,
    pub produced_at: Option<String>,
    pub responder_url: Option<String>,
    pub this_update: Option<String>,
    pub next_update: Option<String>,
}

/// JWT authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAuthorize {
    pub jwks_uri: String,
}

/// State machine status (for long-running operations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    pub id: String,
    pub name: String,
    pub state: String,
    pub status: String,
    pub progress: Option<f64>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub error: Option<String>,
}

/// Module metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub name: String,
    pub version: String,
    pub semantic_version: Option<String>,
    pub min_redis_version: Option<String>,
    pub min_redis_pack_version: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub command_line_args: Option<String>,
    pub config_command: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub is_bundled: Option<bool>,
    pub license: Option<String>,
    pub os_list: Option<Vec<String>>,
    pub sha256: Option<String>,
    pub uid: Option<String>,
    pub architecture_list: Option<Vec<String>>,
    pub author: Option<String>,
}

/// Database command statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbCommand {
    pub command: String,
    pub count: u64,
}

/// Action v2 - enhanced action tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionV2 {
    pub action_uid: String,
    pub name: String,
    pub action_type: String,
    pub creation_time: u64,
    pub progress: f64,
    pub status: String,
    pub additional_info: Option<ActionAdditionalInfo>,
}

/// Additional info for ActionV2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionAdditionalInfo {
    pub description: Option<String>,
    pub error: Option<String>,
    pub object_type: Option<String>,
    pub object_uid: Option<String>,
}

// Types are already public, no need for re-export
