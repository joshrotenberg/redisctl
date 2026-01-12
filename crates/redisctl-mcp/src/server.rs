//! MCP server implementation for Redis Cloud and Enterprise

use std::sync::Arc;

use rmcp::{
    ErrorData as RmcpError, RoleServer, ServerHandler, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, schemars, service::RequestContext, tool,
    tool_handler, tool_router,
};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::cloud_tools::CloudTools;
use crate::enterprise_tools::EnterpriseTools;

/// Configuration for the MCP server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Profile name to use for credentials
    pub profile: Option<String>,
    /// Whether the server is in read-only mode
    pub read_only: bool,
}

/// MCP server for Redis Cloud and Enterprise management
///
/// This server exposes Redis Cloud and Enterprise operations as MCP tools
/// that can be invoked by AI systems.
#[derive(Clone)]
pub struct RedisCtlMcp {
    config: Arc<ServerConfig>,
    tool_router: ToolRouter<RedisCtlMcp>,
    cloud_tools: Arc<RwLock<Option<CloudTools>>>,
    enterprise_tools: Arc<RwLock<Option<EnterpriseTools>>>,
}

// Parameter structs for tools that need arguments
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SubscriptionIdParam {
    /// The subscription ID
    pub subscription_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseIdParam {
    /// The subscription ID
    pub subscription_id: i64,
    /// The database ID
    pub database_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TaskIdParam {
    /// The task ID
    pub task_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NodeIdParam {
    /// The node ID
    pub node_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EnterpriseDatabaseIdParam {
    /// The database ID (uid)
    pub database_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateEnterpriseDatabaseParam {
    /// Name for the new database
    pub name: String,
    /// Memory size in MB (default: 100)
    #[serde(default)]
    pub memory_size_mb: Option<u64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateEnterpriseDatabaseParam {
    /// The database ID (uid) to update
    pub database_id: i64,
    /// Memory size in bytes (optional)
    #[serde(default)]
    pub memory_size: Option<u64>,
    /// Replication enabled (optional)
    #[serde(default)]
    pub replication: Option<bool>,
    /// Data persistence setting: disabled, aof, snapshot, aof-and-snapshot (optional)
    #[serde(default)]
    pub data_persistence: Option<String>,
    /// Eviction policy (optional)
    #[serde(default)]
    pub eviction_policy: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExportDatabaseParam {
    /// The database ID (uid)
    pub database_id: i64,
    /// Export location (e.g., S3 URL, FTP URL, or local path)
    pub export_location: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ImportDatabaseParam {
    /// The database ID (uid)
    pub database_id: i64,
    /// Import location (e.g., S3 URL, FTP URL, or local path)
    pub import_location: String,
    /// Whether to flush the database before importing (default: false)
    #[serde(default)]
    pub flush_before_import: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RestoreDatabaseParam {
    /// The database ID (uid)
    pub database_id: i64,
    /// Specific backup UID to restore from (optional, uses latest if not specified)
    #[serde(default)]
    pub backup_uid: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateClusterParam {
    /// Cluster name (optional)
    #[serde(default)]
    pub name: Option<String>,
    /// Enable/disable email alerts (optional)
    #[serde(default)]
    pub email_alerts: Option<bool>,
    /// Enable/disable rack awareness (optional)
    #[serde(default)]
    pub rack_aware: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateNodeParam {
    /// The node ID (uid)
    pub node_id: i64,
    /// Whether the node accepts new shards (optional)
    #[serde(default)]
    pub accept_servers: Option<bool>,
    /// External IP addresses (optional)
    #[serde(default)]
    pub external_addr: Option<Vec<String>>,
    /// Rack ID where node is installed (optional)
    #[serde(default)]
    pub rack_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ShardIdParam {
    /// The shard UID (e.g., "1:1" for database 1, shard 1)
    pub shard_uid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AlertIdParam {
    /// The alert UID
    pub alert_uid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserIdParam {
    /// The user ID (uid)
    pub user_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateUserParam {
    /// User's email address (used as login)
    pub email: String,
    /// User's password
    pub password: String,
    /// User's role (e.g., "admin", "db_viewer", "db_member")
    pub role: String,
    /// User's display name (optional)
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RoleIdParam {
    /// The role ID (uid)
    pub role_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateRoleParam {
    /// Role name
    pub name: String,
    /// Management level (e.g., "admin", "db_viewer", "db_member") (optional)
    #[serde(default)]
    pub management: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AclIdParam {
    /// The Redis ACL ID (uid)
    pub acl_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateAclParam {
    /// ACL name
    pub name: String,
    /// ACL rules string (e.g., "+@all ~*")
    pub acl: String,
    /// Description (optional)
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleIdParam {
    /// The module UID (e.g., "bf" for RedisBloom)
    pub module_uid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CrdbGuidParam {
    /// The CRDB GUID (globally unique identifier)
    pub crdb_guid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateCrdbParam {
    /// The CRDB GUID
    pub crdb_guid: String,
    /// New memory size in bytes (optional)
    #[serde(default)]
    pub memory_size: Option<u64>,
    /// Enable/disable encryption (optional)
    #[serde(default)]
    pub encryption: Option<bool>,
    /// Data persistence setting (optional)
    #[serde(default)]
    pub data_persistence: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DebugInfoTaskIdParam {
    /// The debug info task ID
    pub task_id: String,
}

// JMESPath tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JpxFunctionsParam {
    /// Optional category filter (e.g., "String", "Math", "Array", "Datetime")
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JpxDescribeParam {
    /// Function name or alias to describe
    pub name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JpxEvaluateParam {
    /// JSON input to evaluate the expression against
    pub input: String,
    /// JMESPath expression to evaluate
    pub expression: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JpxValidateParam {
    /// JMESPath expression to validate
    pub expression: String,
}

impl RedisCtlMcp {
    /// Create a new MCP server instance
    pub fn new(profile: Option<&str>, read_only: bool) -> anyhow::Result<Self> {
        let config = Arc::new(ServerConfig {
            profile: profile.map(String::from),
            read_only,
        });

        info!(
            profile = ?config.profile,
            read_only = config.read_only,
            "Initializing RedisCtlMcp server"
        );

        Ok(Self {
            config,
            tool_router: Self::tool_router(),
            cloud_tools: Arc::new(RwLock::new(None)),
            enterprise_tools: Arc::new(RwLock::new(None)),
        })
    }

    /// Get server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Initialize Cloud tools lazily
    async fn get_cloud_tools(&self) -> Result<CloudTools, RmcpError> {
        let mut guard = self.cloud_tools.write().await;
        if guard.is_none() {
            debug!("Initializing Cloud tools");
            let tools = CloudTools::new(self.config.profile.as_deref())
                .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
            *guard = Some(tools);
        }
        Ok(guard.clone().unwrap())
    }

    /// Initialize Enterprise tools lazily
    async fn get_enterprise_tools(&self) -> Result<EnterpriseTools, RmcpError> {
        let mut guard = self.enterprise_tools.write().await;
        if guard.is_none() {
            debug!("Initializing Enterprise tools");
            let tools = EnterpriseTools::new(self.config.profile.as_deref())
                .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
            *guard = Some(tools);
        }
        Ok(guard.clone().unwrap())
    }
}

#[tool_router]
impl RedisCtlMcp {
    // =========================================================================
    // Cloud Tools - Read Only
    // =========================================================================

    #[tool(
        description = "Get Redis Cloud account information including account ID, name, and settings"
    )]
    async fn cloud_account_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_account_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_account().await
    }

    #[tool(description = "List all Redis Cloud subscriptions in the account")]
    async fn cloud_subscriptions_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_subscriptions_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_subscriptions().await
    }

    #[tool(description = "Get detailed information about a specific Redis Cloud subscription")]
    async fn cloud_subscription_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_subscription_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_subscription(params.subscription_id).await
    }

    #[tool(description = "List all databases in a specific Redis Cloud subscription")]
    async fn cloud_databases_list(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_databases_list"
        );
        let tools = self.get_cloud_tools().await?;
        tools.list_databases(params.subscription_id).await
    }

    #[tool(description = "Get detailed information about a specific Redis Cloud database")]
    async fn cloud_database_get(
        &self,
        Parameters(params): Parameters<DatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            database_id = params.database_id,
            "Tool called: cloud_database_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_database(params.subscription_id, params.database_id)
            .await
    }

    #[tool(description = "List recent async tasks in the Redis Cloud account")]
    async fn cloud_tasks_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_tasks_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_tasks().await
    }

    #[tool(description = "Get the status of a specific Redis Cloud async task")]
    async fn cloud_task_get(
        &self,
        Parameters(params): Parameters<TaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: cloud_task_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_task(&params.task_id).await
    }

    // =========================================================================
    // Enterprise Tools - Read Only
    // =========================================================================

    #[tool(
        description = "Get Redis Enterprise cluster information including name, version, and node count"
    )]
    async fn enterprise_cluster_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster().await
    }

    #[tool(description = "List all nodes in the Redis Enterprise cluster with their status")]
    async fn enterprise_nodes_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_nodes_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_nodes().await
    }

    #[tool(description = "Get detailed information about a specific Redis Enterprise node")]
    async fn enterprise_node_get(
        &self,
        Parameters(params): Parameters<NodeIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(node_id = params.node_id, "Tool called: enterprise_node_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_node(params.node_id).await
    }

    #[tool(description = "List all databases (BDBs) in the Redis Enterprise cluster")]
    async fn enterprise_databases_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_databases_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_databases().await
    }

    #[tool(
        description = "Get detailed information about a specific Redis Enterprise database (BDB)"
    )]
    async fn enterprise_database_get(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_get"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_database(params.database_id).await
    }

    #[tool(description = "Get performance statistics for a specific Redis Enterprise database")]
    async fn enterprise_database_stats(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_stats"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_database_stats(params.database_id).await
    }

    #[tool(description = "List all shards across all databases in the Redis Enterprise cluster")]
    async fn enterprise_shards_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_shards_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_shards().await
    }

    #[tool(description = "List active alerts in the Redis Enterprise cluster")]
    async fn enterprise_alerts_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_alerts_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_alerts().await
    }

    #[tool(description = "Get recent event logs from the Redis Enterprise cluster")]
    async fn enterprise_logs_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_logs_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_logs().await
    }

    #[tool(
        description = "Get Redis Enterprise license information including expiration and capacity"
    )]
    async fn enterprise_license_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_license_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_license().await
    }

    // =========================================================================
    // Enterprise Tools - Write Operations
    // =========================================================================

    #[tool(
        description = "Create a new Redis Enterprise database. Requires name, optionally memory_size_mb (default 100)."
    )]
    async fn enterprise_database_create(
        &self,
        Parameters(params): Parameters<CreateEnterpriseDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            name = %params.name,
            memory_size_mb = ?params.memory_size_mb,
            "Tool called: enterprise_database_create"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_database(&params.name, params.memory_size_mb)
            .await
    }

    #[tool(
        description = "Delete a Redis Enterprise database. This is a destructive operation that cannot be undone."
    )]
    async fn enterprise_database_delete(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_delete"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_database(params.database_id).await
    }

    #[tool(
        description = "Update a Redis Enterprise database configuration. Supports memory_size (bytes), replication, data_persistence, and eviction_policy."
    )]
    async fn enterprise_database_update(
        &self,
        Parameters(params): Parameters<UpdateEnterpriseDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_update"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(memory_size) = params.memory_size {
            updates.insert("memory_size".to_string(), serde_json::json!(memory_size));
        }
        if let Some(replication) = params.replication {
            updates.insert("replication".to_string(), serde_json::json!(replication));
        }
        if let Some(ref data_persistence) = params.data_persistence {
            updates.insert(
                "data_persistence".to_string(),
                serde_json::json!(data_persistence),
            );
        }
        if let Some(ref eviction_policy) = params.eviction_policy {
            updates.insert(
                "eviction_policy".to_string(),
                serde_json::json!(eviction_policy),
            );
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: memory_size, replication, data_persistence, eviction_policy",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_database(params.database_id, serde_json::Value::Object(updates))
            .await
    }

    #[tool(
        description = "Flush all data from a Redis Enterprise database. This is a destructive operation that cannot be undone."
    )]
    async fn enterprise_database_flush(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_flush"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.flush_database(params.database_id).await
    }

    #[tool(description = "Get performance metrics for a Redis Enterprise database")]
    async fn enterprise_database_metrics(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_metrics"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_database_metrics(params.database_id).await
    }

    #[tool(
        description = "Export a Redis Enterprise database to a specified location (S3, FTP, etc.)"
    )]
    async fn enterprise_database_export(
        &self,
        Parameters(params): Parameters<ExportDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            export_location = %params.export_location,
            "Tool called: enterprise_database_export"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .export_database(params.database_id, &params.export_location)
            .await
    }

    #[tool(description = "Import data into a Redis Enterprise database from a specified location")]
    async fn enterprise_database_import(
        &self,
        Parameters(params): Parameters<ImportDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            import_location = %params.import_location,
            flush_before_import = params.flush_before_import,
            "Tool called: enterprise_database_import"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .import_database(
                params.database_id,
                &params.import_location,
                params.flush_before_import,
            )
            .await
    }

    #[tool(description = "Trigger a backup of a Redis Enterprise database")]
    async fn enterprise_database_backup(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_backup"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.backup_database(params.database_id).await
    }

    #[tool(description = "Restore a Redis Enterprise database from a backup")]
    async fn enterprise_database_restore(
        &self,
        Parameters(params): Parameters<RestoreDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            backup_uid = ?params.backup_uid,
            "Tool called: enterprise_database_restore"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .restore_database(params.database_id, params.backup_uid.as_deref())
            .await
    }

    // =========================================================================
    // Enterprise Tools - Cluster Operations
    // =========================================================================

    #[tool(
        description = "Get Redis Enterprise cluster statistics including memory, CPU, and throughput metrics"
    )]
    async fn enterprise_cluster_stats(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_stats");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_stats().await
    }

    #[tool(description = "Get Redis Enterprise cluster settings and configuration")]
    async fn enterprise_cluster_settings(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_settings");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_settings().await
    }

    #[tool(
        description = "Get Redis Enterprise cluster topology showing nodes, shards, and their relationships"
    )]
    async fn enterprise_cluster_topology(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_topology");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_topology().await
    }

    #[tool(
        description = "Update Redis Enterprise cluster configuration. Supports name, email_alerts, and rack_aware settings."
    )]
    async fn enterprise_cluster_update(
        &self,
        Parameters(params): Parameters<UpdateClusterParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_update");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(ref name) = params.name {
            updates.insert("name".to_string(), serde_json::json!(name));
        }
        if let Some(email_alerts) = params.email_alerts {
            updates.insert("email_alerts".to_string(), serde_json::json!(email_alerts));
        }
        if let Some(rack_aware) = params.rack_aware {
            updates.insert("rack_aware".to_string(), serde_json::json!(rack_aware));
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: name, email_alerts, rack_aware",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_cluster(serde_json::Value::Object(updates))
            .await
    }

    // =========================================================================
    // Enterprise Tools - Node Operations
    // =========================================================================

    #[tool(
        description = "Get statistics for a specific Redis Enterprise node including CPU, memory, and network metrics"
    )]
    async fn enterprise_node_stats(
        &self,
        Parameters(params): Parameters<NodeIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            node_id = params.node_id,
            "Tool called: enterprise_node_stats"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_node_stats(params.node_id).await
    }

    #[tool(
        description = "Update a Redis Enterprise node configuration. Supports accept_servers, external_addr, and rack_id."
    )]
    async fn enterprise_node_update(
        &self,
        Parameters(params): Parameters<UpdateNodeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            node_id = params.node_id,
            "Tool called: enterprise_node_update"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(accept_servers) = params.accept_servers {
            updates.insert(
                "accept_servers".to_string(),
                serde_json::json!(accept_servers),
            );
        }
        if let Some(ref external_addr) = params.external_addr {
            updates.insert(
                "external_addr".to_string(),
                serde_json::json!(external_addr),
            );
        }
        if let Some(ref rack_id) = params.rack_id {
            updates.insert("rack_id".to_string(), serde_json::json!(rack_id));
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: accept_servers, external_addr, rack_id",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_node(params.node_id, serde_json::Value::Object(updates))
            .await
    }

    #[tool(
        description = "Remove a node from the Redis Enterprise cluster. This is a destructive operation."
    )]
    async fn enterprise_node_remove(
        &self,
        Parameters(params): Parameters<NodeIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            node_id = params.node_id,
            "Tool called: enterprise_node_remove"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.remove_node(params.node_id).await
    }

    // =========================================================================
    // Enterprise Tools - Shard Operations
    // =========================================================================

    #[tool(description = "Get detailed information about a specific Redis Enterprise shard")]
    async fn enterprise_shard_get(
        &self,
        Parameters(params): Parameters<ShardIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(shard_uid = %params.shard_uid, "Tool called: enterprise_shard_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_shard(&params.shard_uid).await
    }

    // =========================================================================
    // Enterprise Tools - Alert Operations
    // =========================================================================

    #[tool(description = "Get detailed information about a specific Redis Enterprise alert")]
    async fn enterprise_alert_get(
        &self,
        Parameters(params): Parameters<AlertIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(alert_uid = %params.alert_uid, "Tool called: enterprise_alert_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_alert(&params.alert_uid).await
    }

    // =========================================================================
    // Enterprise Tools - User Operations
    // =========================================================================

    #[tool(description = "List all users in the Redis Enterprise cluster")]
    async fn enterprise_users_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_users_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_users().await
    }

    #[tool(description = "Get detailed information about a specific Redis Enterprise user")]
    async fn enterprise_user_get(
        &self,
        Parameters(params): Parameters<UserIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(user_id = params.user_id, "Tool called: enterprise_user_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_user(params.user_id).await
    }

    #[tool(description = "Create a new user in the Redis Enterprise cluster")]
    async fn enterprise_user_create(
        &self,
        Parameters(params): Parameters<CreateUserParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(email = %params.email, "Tool called: enterprise_user_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_user(
                &params.email,
                &params.password,
                &params.role,
                params.name.as_deref(),
            )
            .await
    }

    #[tool(description = "Delete a user from the Redis Enterprise cluster")]
    async fn enterprise_user_delete(
        &self,
        Parameters(params): Parameters<UserIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            user_id = params.user_id,
            "Tool called: enterprise_user_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_user(params.user_id).await
    }

    // =========================================================================
    // Enterprise Tools - Role Operations
    // =========================================================================

    #[tool(description = "List all roles in the Redis Enterprise cluster")]
    async fn enterprise_roles_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_roles_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_roles().await
    }

    #[tool(description = "Get detailed information about a specific Redis Enterprise role")]
    async fn enterprise_role_get(
        &self,
        Parameters(params): Parameters<RoleIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(role_id = params.role_id, "Tool called: enterprise_role_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_role(params.role_id).await
    }

    #[tool(description = "Create a new role in the Redis Enterprise cluster")]
    async fn enterprise_role_create(
        &self,
        Parameters(params): Parameters<CreateRoleParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_role_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_role(&params.name, params.management.as_deref())
            .await
    }

    #[tool(description = "Delete a role from the Redis Enterprise cluster")]
    async fn enterprise_role_delete(
        &self,
        Parameters(params): Parameters<RoleIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            role_id = params.role_id,
            "Tool called: enterprise_role_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_role(params.role_id).await
    }

    // =========================================================================
    // Enterprise Tools - Redis ACL Operations
    // =========================================================================

    #[tool(description = "List all Redis ACLs in the Redis Enterprise cluster")]
    async fn enterprise_acls_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_acls_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_acls().await
    }

    #[tool(description = "Get detailed information about a specific Redis ACL")]
    async fn enterprise_acl_get(
        &self,
        Parameters(params): Parameters<AclIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(acl_id = params.acl_id, "Tool called: enterprise_acl_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_acl(params.acl_id).await
    }

    #[tool(description = "Create a new Redis ACL in the Redis Enterprise cluster")]
    async fn enterprise_acl_create(
        &self,
        Parameters(params): Parameters<CreateAclParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_acl_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_acl(&params.name, &params.acl, params.description.as_deref())
            .await
    }

    #[tool(description = "Delete a Redis ACL from the Redis Enterprise cluster")]
    async fn enterprise_acl_delete(
        &self,
        Parameters(params): Parameters<AclIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(acl_id = params.acl_id, "Tool called: enterprise_acl_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_acl(params.acl_id).await
    }

    // =========================================================================
    // Enterprise Tools - Module Operations
    // =========================================================================

    #[tool(description = "List all Redis modules available in the Redis Enterprise cluster")]
    async fn enterprise_modules_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_modules_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_modules().await
    }

    #[tool(description = "Get detailed information about a specific Redis module")]
    async fn enterprise_module_get(
        &self,
        Parameters(params): Parameters<ModuleIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(module_uid = %params.module_uid, "Tool called: enterprise_module_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_module(&params.module_uid).await
    }

    // =========================================================================
    // Enterprise Tools - CRDB (Active-Active) Operations
    // =========================================================================

    #[tool(description = "List all Active-Active (CRDB) databases in the Redis Enterprise cluster")]
    async fn enterprise_crdbs_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_crdbs_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_crdbs().await
    }

    #[tool(description = "Get detailed information about a specific Active-Active (CRDB) database")]
    async fn enterprise_crdb_get(
        &self,
        Parameters(params): Parameters<CrdbGuidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_crdb(&params.crdb_guid).await
    }

    #[tool(description = "Update an Active-Active (CRDB) database configuration")]
    async fn enterprise_crdb_update(
        &self,
        Parameters(params): Parameters<UpdateCrdbParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_update");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(memory_size) = params.memory_size {
            updates.insert("memory_size".to_string(), serde_json::json!(memory_size));
        }
        if let Some(encryption) = params.encryption {
            updates.insert("encryption".to_string(), serde_json::json!(encryption));
        }
        if let Some(ref data_persistence) = params.data_persistence {
            updates.insert(
                "data_persistence".to_string(),
                serde_json::json!(data_persistence),
            );
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: memory_size, encryption, data_persistence",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_crdb(&params.crdb_guid, serde_json::Value::Object(updates))
            .await
    }

    #[tool(
        description = "Delete an Active-Active (CRDB) database. This is a destructive operation."
    )]
    async fn enterprise_crdb_delete(
        &self,
        Parameters(params): Parameters<CrdbGuidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_crdb(&params.crdb_guid).await
    }

    // =========================================================================
    // Enterprise Tools - Debug Info / Support Operations
    // =========================================================================

    #[tool(description = "List debug info collection tasks in the Redis Enterprise cluster")]
    async fn enterprise_debuginfo_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_debuginfo_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_debuginfo().await
    }

    #[tool(description = "Get the status of a specific debug info collection task")]
    async fn enterprise_debuginfo_status(
        &self,
        Parameters(params): Parameters<DebugInfoTaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: enterprise_debuginfo_status");
        let tools = self.get_enterprise_tools().await?;
        tools.get_debuginfo_status(&params.task_id).await
    }

    // =========================================================================
    // JMESPath Tools - Query and Introspection
    // =========================================================================

    #[tool(
        description = "List available JMESPath function categories (String, Math, Array, Datetime, etc.)"
    )]
    async fn jpx_categories(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: jpx_categories");
        let categories = crate::jmespath::list_categories();
        let content = Content::text(serde_json::to_string_pretty(&categories).unwrap());
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "List available JMESPath functions. Optionally filter by category (e.g., 'String', 'Math', 'Array', 'Datetime'). Returns function names with descriptions and signatures."
    )]
    async fn jpx_functions(
        &self,
        Parameters(params): Parameters<JpxFunctionsParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(category = ?params.category, "Tool called: jpx_functions");
        let functions = crate::jmespath::list_functions(params.category.as_deref());
        let content = Content::text(serde_json::to_string_pretty(&functions).unwrap());
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(
        description = "Get detailed information about a specific JMESPath function including signature, description, and example usage. Supports function names and aliases."
    )]
    async fn jpx_describe(
        &self,
        Parameters(params): Parameters<JpxDescribeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: jpx_describe");
        match crate::jmespath::get_function(&params.name) {
            Some(func) => {
                let content = Content::text(serde_json::to_string_pretty(&func).unwrap());
                Ok(CallToolResult::success(vec![content]))
            }
            None => Err(RmcpError::invalid_request(
                format!("Function '{}' not found", params.name),
                None,
            )),
        }
    }

    #[tool(
        description = "Evaluate a JMESPath expression against JSON input. Use this to transform, filter, or extract data from JSON. Supports 335+ functions including string manipulation, math, dates, arrays, and more."
    )]
    async fn jpx_evaluate(
        &self,
        Parameters(params): Parameters<JpxEvaluateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(expression = %params.expression, "Tool called: jpx_evaluate");
        match crate::jmespath::evaluate(&params.input, &params.expression) {
            Ok(result) => {
                let content = Content::text(serde_json::to_string_pretty(&result).unwrap());
                Ok(CallToolResult::success(vec![content]))
            }
            Err(e) => Err(RmcpError::invalid_request(e, None)),
        }
    }

    #[tool(
        description = "Validate a JMESPath expression without executing it. Returns whether the expression is syntactically valid."
    )]
    async fn jpx_validate(
        &self,
        Parameters(params): Parameters<JpxValidateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(expression = %params.expression, "Tool called: jpx_validate");
        // Try to compile the expression to validate it
        let runtime = jmespath::Runtime::new();
        match runtime.compile(&params.expression) {
            Ok(_) => {
                let result = serde_json::json!({
                    "valid": true,
                    "expression": params.expression
                });
                let content = Content::text(serde_json::to_string_pretty(&result).unwrap());
                Ok(CallToolResult::success(vec![content]))
            }
            Err(e) => {
                let result = serde_json::json!({
                    "valid": false,
                    "expression": params.expression,
                    "error": e.to_string()
                });
                let content = Content::text(serde_json::to_string_pretty(&result).unwrap());
                Ok(CallToolResult::success(vec![content]))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for RedisCtlMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Redis Cloud and Enterprise management tools. \
                Use cloud_* tools for Redis Cloud operations and \
                enterprise_* tools for Redis Enterprise operations. \
                All tools are currently read-only."
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, RmcpError> {
        info!("MCP client connected, initializing session");
        Ok(self.get_info())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = RedisCtlMcp::new(None, true);
        assert!(server.is_ok());
        let server = server.unwrap();
        assert!(server.config().read_only);
        assert!(server.config().profile.is_none());
    }

    #[test]
    fn test_server_with_profile() {
        let server = RedisCtlMcp::new(Some("test-profile"), false);
        assert!(server.is_ok());
        let server = server.unwrap();
        assert!(!server.config().read_only);
        assert_eq!(server.config().profile, Some("test-profile".to_string()));
    }
}
