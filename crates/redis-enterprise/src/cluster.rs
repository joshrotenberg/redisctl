//! Cluster management commands for Redis Enterprise
//!
//! Overview
//! - Read: info, settings, topology, license, nodes
//! - Actions: reset, recover, join node, policy and services configuration
//! - Certificates and LDAP helpers
//! - Alert detail queries

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Response from cluster action operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterActionResponse {
    /// The action UID for tracking async operations
    pub action_uid: String,
    /// Description of the action
    pub description: Option<String>,
    /// Additional fields from the response
    #[serde(flatten)]
    pub extra: Value,
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNode {
    pub id: u32,
    pub address: String,
    pub status: String,
    pub role: Option<String>,
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,
    pub cpu_cores: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster information from the REST API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub name: String,
    pub version: Option<String>,
    pub license_expired: Option<bool>,
    pub nodes: Option<Vec<u32>>,
    pub databases: Option<Vec<u32>>,
    pub status: Option<String>,
    pub email_alerts: Option<bool>,
    pub rack_aware: Option<bool>,

    // Stats
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,
    pub total_shards: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster-wide settings configuration (57 fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSettings {
    /// Automatic recovery on shard failure
    pub auto_recovery: Option<bool>,

    /// Automatic migration of shards from overbooked nodes
    pub automatic_node_offload: Option<bool>,

    /// BigStore migration thresholds
    pub bigstore_migrate_node_threshold: Option<u32>,
    pub bigstore_migrate_node_threshold_p: Option<u32>,
    pub bigstore_provision_node_threshold: Option<u32>,
    pub bigstore_provision_node_threshold_p: Option<u32>,

    /// Default BigStore version
    pub default_bigstore_version: Option<u32>,

    /// Data internode encryption
    pub data_internode_encryption: Option<bool>,

    /// Database connections auditing
    pub db_conns_auditing: Option<bool>,

    /// Default concurrent restore actions
    pub default_concurrent_restore_actions: Option<u32>,

    /// Default fork evict RAM
    pub default_fork_evict_ram: Option<bool>,

    /// Default proxy policies
    pub default_non_sharded_proxy_policy: Option<String>,
    pub default_sharded_proxy_policy: Option<String>,

    /// OSS cluster defaults
    pub default_oss_cluster: Option<bool>,
    pub default_oss_sharding: Option<bool>,

    /// Default Redis version for new databases
    pub default_provisioned_redis_version: Option<String>,

    /// Recovery settings
    pub default_recovery_wait_time: Option<u32>,

    /// Shards placement strategy
    pub default_shards_placement: Option<String>,

    /// Tracking table settings
    pub default_tracking_table_max_keys_policy: Option<String>,

    /// Additional cluster-wide settings
    pub email_alerts: Option<bool>,
    pub endpoint_rebind_enabled: Option<bool>,
    pub failure_detection_sensitivity: Option<String>,
    pub gossip_envoy_admin_port: Option<u32>,
    pub gossip_envoy_port: Option<u32>,
    pub gossip_envoy_proxy_mode: Option<bool>,
    pub hot_spare: Option<bool>,
    pub max_saved_events_per_type: Option<u32>,
    pub max_simultaneous_backups: Option<u32>,
    pub parallel_shards_upgrade: Option<u32>,
    pub persistent_node_removal: Option<bool>,
    pub rack_aware: Option<bool>,
    pub redis_migrate_node_threshold: Option<String>,
    pub redis_migrate_node_threshold_p: Option<u32>,
    pub redis_provision_node_threshold: Option<String>,
    pub redis_provision_node_threshold_p: Option<u32>,
    pub redis_upgrade_policy: Option<String>,
    pub resp3_default: Option<bool>,
    pub show_internals: Option<bool>,
    pub slave_threads_when_master: Option<bool>,
    pub use_empty_shard_backups: Option<bool>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Bootstrap request for creating a new cluster
#[derive(Debug, Serialize, TypedBuilder)]
pub struct BootstrapRequest {
    #[builder(setter(into))]
    pub action: String,
    pub cluster: ClusterBootstrapInfo,
    pub credentials: BootstrapCredentials,
}

/// Cluster information for bootstrap
#[derive(Debug, Serialize, TypedBuilder)]
pub struct ClusterBootstrapInfo {
    #[builder(setter(into))]
    pub name: String,
}

/// Credentials for bootstrap
#[derive(Debug, Serialize, TypedBuilder)]
pub struct BootstrapCredentials {
    #[builder(setter(into))]
    pub username: String,
    #[builder(setter(into))]
    pub password: String,
}

/// Cluster handler for executing cluster commands
pub struct ClusterHandler {
    client: RestClient,
}

impl ClusterHandler {
    pub fn new(client: RestClient) -> Self {
        ClusterHandler { client }
    }

    /// Get cluster information (CLUSTER.INFO)
    pub async fn info(&self) -> Result<ClusterInfo> {
        self.client.get("/v1/cluster").await
    }

    /// Bootstrap a new cluster (CLUSTER.BOOTSTRAP)
    pub async fn bootstrap(&self, request: BootstrapRequest) -> Result<Value> {
        // The bootstrap endpoint returns empty response on success
        // Note: Despite docs saying /v1/bootstrap, the actual endpoint is /v1/bootstrap/create_cluster
        self.client
            .post_bootstrap("/v1/bootstrap/create_cluster", &request)
            .await
    }

    /// Update cluster configuration (CLUSTER.UPDATE)
    pub async fn update(&self, updates: Value) -> Result<Value> {
        self.client.put("/v1/cluster", &updates).await
    }

    /// Get cluster stats (CLUSTER.STATS)
    pub async fn stats(&self) -> Result<Value> {
        self.client.get("/v1/cluster/stats").await
    }

    /// Get cluster nodes (CLUSTER.NODES)
    pub async fn nodes(&self) -> Result<Vec<NodeInfo>> {
        self.client.get("/v1/nodes").await
    }

    /// Get cluster license (CLUSTER.LICENSE)
    pub async fn license(&self) -> Result<LicenseInfo> {
        self.client.get("/v1/license").await
    }

    /// Join node to cluster (CLUSTER.JOIN)
    pub async fn join_node(
        &self,
        node_address: &str,
        username: &str,
        password: &str,
    ) -> Result<Value> {
        let body = serde_json::json!({
            "action": "join_cluster",
            "cluster": {
                "nodes": [node_address]
            },
            "credentials": {
                "username": username,
                "password": password
            }
        });
        self.client.post("/v1/bootstrap/join", &body).await
    }

    /// Remove node from cluster (CLUSTER.REMOVE_NODE)
    pub async fn remove_node(&self, node_uid: u32) -> Result<Value> {
        self.client
            .delete(&format!("/v1/nodes/{}", node_uid))
            .await?;
        Ok(serde_json::json!({"message": format!("Node {} removed", node_uid)}))
    }

    /// Reset cluster to factory defaults (CLUSTER.RESET) - DANGEROUS - typed version
    pub async fn reset(&self) -> Result<ClusterActionResponse> {
        self.client
            .post("/v1/cluster/actions/reset", &serde_json::json!({}))
            .await
    }

    // raw variant removed: use reset()

    /// Recover cluster from failure (CLUSTER.RECOVER) - typed version
    pub async fn recover(&self) -> Result<ClusterActionResponse> {
        self.client
            .post("/v1/cluster/actions/recover", &serde_json::json!({}))
            .await
    }

    // raw variant removed: use recover()

    /// Get cluster settings (CLUSTER.SETTINGS)
    pub async fn settings(&self) -> Result<Value> {
        self.client.get("/v1/cluster/settings").await
    }

    /// Get cluster topology (CLUSTER.TOPOLOGY)
    pub async fn topology(&self) -> Result<Value> {
        self.client.get("/v1/cluster/topology").await
    }

    /// List available cluster actions - GET /v1/cluster/actions
    pub async fn actions(&self) -> Result<Value> {
        self.client.get("/v1/cluster/actions").await
    }

    /// Get a specific cluster action details - GET /v1/cluster/actions/{action}
    pub async fn action_detail(&self, action: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/cluster/actions/{}", action))
            .await
    }

    /// Execute a specific cluster action - POST /v1/cluster/actions/{action}
    pub async fn action_execute(&self, action: &str, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/cluster/actions/{}", action), &body)
            .await
    }

    /// Delete a specific cluster action - DELETE /v1/cluster/actions/{action}
    pub async fn action_delete(&self, action: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/cluster/actions/{}", action))
            .await
    }

    /// Get auditing DB connections - GET /v1/cluster/auditing/db_conns
    pub async fn auditing_db_conns(&self) -> Result<Value> {
        self.client.get("/v1/cluster/auditing/db_conns").await
    }

    /// Update auditing DB connections - PUT /v1/cluster/auditing/db_conns
    pub async fn auditing_db_conns_update(&self, cfg: Value) -> Result<Value> {
        self.client.put("/v1/cluster/auditing/db_conns", &cfg).await
    }

    /// Delete auditing DB connections - DELETE /v1/cluster/auditing/db_conns
    pub async fn auditing_db_conns_delete(&self) -> Result<()> {
        self.client.delete("/v1/cluster/auditing/db_conns").await
    }

    /// List cluster certificates - GET /v1/cluster/certificates
    pub async fn certificates(&self) -> Result<Value> {
        self.client.get("/v1/cluster/certificates").await
    }

    /// Delete a certificate - DELETE /v1/cluster/certificates/{uid}
    pub async fn certificate_delete(&self, uid: u32) -> Result<()> {
        self.client
            .delete(&format!("/v1/cluster/certificates/{}", uid))
            .await
    }

    /// Rotate certificates - POST /v1/cluster/certificates/rotate
    pub async fn certificates_rotate(&self) -> Result<Value> {
        self.client
            .post("/v1/cluster/certificates/rotate", &serde_json::json!({}))
            .await
    }

    /// Update certificate bundle - PUT /v1/cluster/update_cert
    pub async fn update_cert(&self, body: Value) -> Result<Value> {
        self.client.put("/v1/cluster/update_cert", &body).await
    }

    /// Delete LDAP configuration - DELETE /v1/cluster/ldap
    pub async fn ldap_delete(&self) -> Result<()> {
        self.client.delete("/v1/cluster/ldap").await
    }

    /// Get cluster module capabilities - GET /v1/cluster/module-capabilities
    pub async fn module_capabilities(&self) -> Result<Value> {
        self.client.get("/v1/cluster/module-capabilities").await
    }

    /// Get cluster policy - GET /v1/cluster/policy
    pub async fn policy(&self) -> Result<Value> {
        self.client.get("/v1/cluster/policy").await
    }

    /// Update cluster policy - PUT /v1/cluster/policy
    pub async fn policy_update(&self, policy: Value) -> Result<Value> {
        self.client.put("/v1/cluster/policy", &policy).await
    }

    /// Restore default cluster policy - PUT /v1/cluster/policy/restore_default
    pub async fn policy_restore_default(&self) -> Result<Value> {
        self.client
            .put("/v1/cluster/policy/restore_default", &serde_json::json!({}))
            .await
    }

    /// Get services configuration - GET /v1/cluster/services_configuration
    pub async fn services_configuration(&self) -> Result<Value> {
        self.client.get("/v1/cluster/services_configuration").await
    }

    /// Update services configuration - PUT /v1/cluster/services_configuration
    pub async fn services_configuration_update(&self, cfg: Value) -> Result<Value> {
        self.client
            .put("/v1/cluster/services_configuration", &cfg)
            .await
    }

    /// Get witness disk info - GET /v1/cluster/witness_disk
    pub async fn witness_disk(&self) -> Result<Value> {
        self.client.get("/v1/cluster/witness_disk").await
    }

    /// Get specific cluster alert detail - GET /v1/cluster/alerts/{alert}
    pub async fn alert_detail(&self, alert: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/cluster/alerts/{}", alert))
            .await
    }
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub uid: u32,
    pub address: String,
    pub status: String,
    pub role: Option<String>,
    pub shards: Option<Vec<u32>>,
    pub total_memory: Option<u64>,
    pub used_memory: Option<u64>,

    #[serde(flatten)]
    pub extra: Value,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_type: Option<String>,
    pub expired: Option<bool>,
    pub expiration_date: Option<String>,
    pub shards_limit: Option<u32>,
    pub features: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}
