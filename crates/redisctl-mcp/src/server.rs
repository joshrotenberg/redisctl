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

// Cloud-specific parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CloudProviderParam {
    /// Cloud provider filter (AWS, GCP, or Azure). Optional.
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateProSubscriptionParam {
    /// JSON payload for creating a Pro subscription. See Redis Cloud API docs for schema.
    pub request: serde_json::Value,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateEssentialsSubscriptionParam {
    /// Name for the new Essentials subscription
    pub name: String,
    /// Plan ID from cloud_essentials_plans_list
    pub plan_id: i64,
    /// Payment method ID (optional, use cloud_payment_methods_get to list available methods)
    #[serde(default)]
    pub payment_method_id: Option<i64>,
}

// Enterprise - LDAP Mapping parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LdapMappingIdParam {
    /// The LDAP mapping ID (uid)
    pub mapping_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateLdapMappingParam {
    /// Name for the LDAP mapping
    pub name: String,
    /// LDAP group distinguished name
    pub dn: String,
    /// Role identifier to map to
    pub role: String,
    /// Email address for alerts (optional)
    #[serde(default)]
    pub email: Option<String>,
}

// Enterprise - Job Scheduler parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobIdParam {
    /// The scheduled job ID
    pub job_id: String,
}

// Enterprise - Proxy parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProxyIdParam {
    /// The proxy ID (uid)
    pub proxy_id: i64,
}

// Enterprise - Endpoint parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EndpointIdParam {
    /// The endpoint ID (uid)
    pub endpoint_id: String,
}

// Enterprise - Diagnostics parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DiagnosticReportIdParam {
    /// The diagnostic report ID
    pub report_id: String,
}

// Cloud - Essentials Database parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EssentialsDatabaseIdParam {
    /// The Essentials subscription ID
    pub subscription_id: i64,
    /// The database ID
    pub database_id: i64,
}

// Cloud - VPC Peering parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VpcPeeringIdParam {
    /// The subscription ID
    pub subscription_id: i64,
    /// The VPC peering ID
    pub peering_id: i64,
}

// Cloud - Cloud Account parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CloudAccountIdParam {
    /// The cloud account ID
    pub account_id: i64,
}

// Enterprise - CRDB Task parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CrdbTaskIdParam {
    /// The CRDB task ID
    pub task_id: String,
}

// Cloud - Transit Gateway parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TransitGatewayAttachmentIdParam {
    /// The subscription ID
    pub subscription_id: i64,
    /// The Transit Gateway attachment ID
    pub attachment_id: String,
}

// Enterprise - BDB Groups parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BdbGroupIdParam {
    /// The BDB group UID
    pub uid: i64,
}

// Enterprise - DNS Suffix parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SuffixNameParam {
    /// The DNS suffix name
    pub name: String,
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
    // Cloud Tools - Account & Infrastructure
    // =========================================================================

    #[tool(description = "List all payment methods configured for your Redis Cloud account")]
    async fn cloud_payment_methods_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_payment_methods_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_payment_methods().await
    }

    #[tool(
        description = "List all available database modules (capabilities) supported in your account"
    )]
    async fn cloud_database_modules_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_database_modules_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_database_modules().await
    }

    #[tool(
        description = "Get available regions across cloud providers (AWS, GCP, Azure) for Pro subscriptions"
    )]
    async fn cloud_regions_get(
        &self,
        Parameters(params): Parameters<CloudProviderParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(provider = ?params.provider, "Tool called: cloud_regions_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_regions(params.provider.as_deref()).await
    }

    // =========================================================================
    // Cloud Tools - Pro Subscriptions (Write)
    // =========================================================================

    #[tool(
        description = "Create a new Pro subscription with advanced configuration options. Requires JSON payload with cloudProviders and databases arrays. Use cloud_regions_get to find available regions."
    )]
    async fn cloud_pro_subscription_create(
        &self,
        Parameters(params): Parameters<CreateProSubscriptionParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_pro_subscription_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.create_subscription(params.request).await
    }

    #[tool(
        description = "Delete a Pro subscription. All databases must be deleted first. This is a destructive operation."
    )]
    async fn cloud_pro_subscription_delete(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_pro_subscription_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.delete_subscription(params.subscription_id).await
    }

    // =========================================================================
    // Cloud Tools - Essentials Subscriptions
    // =========================================================================

    #[tool(description = "List all Essentials (fixed) subscriptions in the account")]
    async fn cloud_essentials_subscriptions_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_essentials_subscriptions_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_essentials_subscriptions().await
    }

    #[tool(description = "Get detailed information about a specific Essentials subscription")]
    async fn cloud_essentials_subscription_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_essentials_subscription_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_essentials_subscription(params.subscription_id)
            .await
    }

    #[tool(
        description = "Create a new Essentials subscription. Use cloud_essentials_plans_list to find available plans."
    )]
    async fn cloud_essentials_subscription_create(
        &self,
        Parameters(params): Parameters<CreateEssentialsSubscriptionParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            name = %params.name,
            plan_id = params.plan_id,
            "Tool called: cloud_essentials_subscription_create"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .create_essentials_subscription(&params.name, params.plan_id, params.payment_method_id)
            .await
    }

    #[tool(
        description = "Delete an Essentials subscription. This is a destructive operation that cannot be undone."
    )]
    async fn cloud_essentials_subscription_delete(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_essentials_subscription_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_essentials_subscription(params.subscription_id)
            .await
    }

    #[tool(
        description = "List available Essentials subscription plans with pricing. Optionally filter by cloud provider (AWS, GCP, Azure)."
    )]
    async fn cloud_essentials_plans_list(
        &self,
        Parameters(params): Parameters<CloudProviderParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(provider = ?params.provider, "Tool called: cloud_essentials_plans_list");
        let tools = self.get_cloud_tools().await?;
        tools
            .list_essentials_plans(params.provider.as_deref())
            .await
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
    // Enterprise Tools - LDAP Mapping Operations
    // =========================================================================

    #[tool(description = "List all LDAP mappings in the Redis Enterprise cluster")]
    async fn enterprise_ldap_mappings_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ldap_mappings_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_ldap_mappings().await
    }

    #[tool(description = "Get detailed information about a specific LDAP mapping")]
    async fn enterprise_ldap_mapping_get(
        &self,
        Parameters(params): Parameters<LdapMappingIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            mapping_id = params.mapping_id,
            "Tool called: enterprise_ldap_mapping_get"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_ldap_mapping(params.mapping_id as u64).await
    }

    #[tool(description = "Create a new LDAP mapping to map LDAP groups to Redis Enterprise roles")]
    async fn enterprise_ldap_mapping_create(
        &self,
        Parameters(params): Parameters<CreateLdapMappingParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_ldap_mapping_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_ldap_mapping(
                &params.name,
                &params.dn,
                &params.role,
                params.email.as_deref(),
            )
            .await
    }

    #[tool(description = "Delete an LDAP mapping from the Redis Enterprise cluster")]
    async fn enterprise_ldap_mapping_delete(
        &self,
        Parameters(params): Parameters<LdapMappingIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            mapping_id = params.mapping_id,
            "Tool called: enterprise_ldap_mapping_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_ldap_mapping(params.mapping_id as u64).await
    }

    // =========================================================================
    // Enterprise Tools - Job Scheduler Operations
    // =========================================================================

    #[tool(description = "List all scheduled jobs in the Redis Enterprise cluster")]
    async fn enterprise_jobs_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_jobs_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_jobs().await
    }

    #[tool(description = "Get detailed information about a specific scheduled job")]
    async fn enterprise_job_get(
        &self,
        Parameters(params): Parameters<JobIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(job_id = %params.job_id, "Tool called: enterprise_job_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_job(&params.job_id).await
    }

    #[tool(description = "Get execution history for a specific scheduled job")]
    async fn enterprise_job_history(
        &self,
        Parameters(params): Parameters<JobIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(job_id = %params.job_id, "Tool called: enterprise_job_history");
        let tools = self.get_enterprise_tools().await?;
        tools.get_job_history(&params.job_id).await
    }

    #[tool(description = "Trigger immediate execution of a scheduled job")]
    async fn enterprise_job_trigger(
        &self,
        Parameters(params): Parameters<JobIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(job_id = %params.job_id, "Tool called: enterprise_job_trigger");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.trigger_job(&params.job_id).await
    }

    // =========================================================================
    // Enterprise Tools - Proxy Operations
    // =========================================================================

    #[tool(description = "List all proxies in the Redis Enterprise cluster")]
    async fn enterprise_proxies_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_proxies_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_proxies().await
    }

    #[tool(description = "Get detailed information about a specific proxy")]
    async fn enterprise_proxy_get(
        &self,
        Parameters(params): Parameters<ProxyIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            proxy_id = params.proxy_id,
            "Tool called: enterprise_proxy_get"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_proxy(params.proxy_id as u64).await
    }

    #[tool(description = "Get statistics for a specific proxy")]
    async fn enterprise_proxy_stats(
        &self,
        Parameters(params): Parameters<ProxyIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            proxy_id = params.proxy_id,
            "Tool called: enterprise_proxy_stats"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_proxy_stats(params.proxy_id as u64).await
    }

    #[tool(description = "List proxies for a specific database")]
    async fn enterprise_proxies_by_database(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_proxies_by_database"
        );
        let tools = self.get_enterprise_tools().await?;
        tools
            .list_proxies_by_database(params.database_id as u64)
            .await
    }

    // =========================================================================
    // Enterprise Tools - Endpoint Operations
    // =========================================================================

    #[tool(description = "List all database endpoints in the Redis Enterprise cluster")]
    async fn enterprise_endpoints_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_endpoints_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_endpoints().await
    }

    #[tool(description = "Get detailed information about a specific endpoint")]
    async fn enterprise_endpoint_get(
        &self,
        Parameters(params): Parameters<EndpointIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(endpoint_id = %params.endpoint_id, "Tool called: enterprise_endpoint_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_endpoint(&params.endpoint_id).await
    }

    #[tool(description = "Get statistics for a specific endpoint")]
    async fn enterprise_endpoint_stats(
        &self,
        Parameters(params): Parameters<EndpointIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(endpoint_id = %params.endpoint_id, "Tool called: enterprise_endpoint_stats");
        let tools = self.get_enterprise_tools().await?;
        tools.get_endpoint_stats(&params.endpoint_id).await
    }

    #[tool(description = "List endpoints for a specific database")]
    async fn enterprise_endpoints_by_database(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_endpoints_by_database"
        );
        let tools = self.get_enterprise_tools().await?;
        tools
            .list_endpoints_by_database(params.database_id as u64)
            .await
    }

    // =========================================================================
    // Enterprise Tools - Diagnostics Operations
    // =========================================================================

    #[tool(description = "List available diagnostic checks in the Redis Enterprise cluster")]
    async fn enterprise_diagnostic_checks_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostic_checks_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_diagnostic_checks().await
    }

    #[tool(description = "List diagnostic reports in the Redis Enterprise cluster")]
    async fn enterprise_diagnostic_reports_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostic_reports_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_diagnostic_reports().await
    }

    #[tool(description = "Get a specific diagnostic report")]
    async fn enterprise_diagnostic_report_get(
        &self,
        Parameters(params): Parameters<DiagnosticReportIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(report_id = %params.report_id, "Tool called: enterprise_diagnostic_report_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_diagnostic_report(&params.report_id).await
    }

    #[tool(description = "Get the most recent diagnostic report")]
    async fn enterprise_diagnostic_report_last(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostic_report_last");
        let tools = self.get_enterprise_tools().await?;
        tools.get_last_diagnostic_report().await
    }

    #[tool(description = "Run diagnostics on the Redis Enterprise cluster")]
    async fn enterprise_diagnostics_run(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostics_run");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.run_diagnostics().await
    }

    // =========================================================================
    // Cloud Tools - Essentials Database Operations
    // =========================================================================

    #[tool(description = "List all databases in an Essentials subscription")]
    async fn cloud_essentials_databases_list(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_essentials_databases_list"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .list_essentials_databases(params.subscription_id)
            .await
    }

    #[tool(description = "Get detailed information about a specific Essentials database")]
    async fn cloud_essentials_database_get(
        &self,
        Parameters(params): Parameters<EssentialsDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            database_id = params.database_id,
            "Tool called: cloud_essentials_database_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_essentials_database(params.subscription_id, params.database_id)
            .await
    }

    #[tool(description = "Delete an Essentials database. This is a destructive operation.")]
    async fn cloud_essentials_database_delete(
        &self,
        Parameters(params): Parameters<EssentialsDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            database_id = params.database_id,
            "Tool called: cloud_essentials_database_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_essentials_database(params.subscription_id, params.database_id)
            .await
    }

    // =========================================================================
    // Cloud Tools - VPC Peering Operations
    // =========================================================================

    #[tool(description = "Get VPC peerings for a subscription")]
    async fn cloud_vpc_peerings_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_vpc_peerings_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_vpc_peerings(params.subscription_id).await
    }

    #[tool(description = "Delete a VPC peering. This is a destructive operation.")]
    async fn cloud_vpc_peering_delete(
        &self,
        Parameters(params): Parameters<VpcPeeringIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            peering_id = params.peering_id,
            "Tool called: cloud_vpc_peering_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_vpc_peering(params.subscription_id, params.peering_id)
            .await
    }

    // =========================================================================
    // Cloud Tools - Cloud Account Operations
    // =========================================================================

    #[tool(
        description = "List all cloud provider accounts (AWS, GCP, Azure) configured in your Redis Cloud account"
    )]
    async fn cloud_accounts_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_accounts_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_cloud_accounts().await
    }

    #[tool(description = "Get detailed information about a specific cloud provider account")]
    async fn cloud_account_get_by_id(
        &self,
        Parameters(params): Parameters<CloudAccountIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            account_id = params.account_id,
            "Tool called: cloud_account_get_by_id"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_cloud_account(params.account_id).await
    }

    #[tool(description = "Delete a cloud provider account. This is a destructive operation.")]
    async fn cloud_account_delete(
        &self,
        Parameters(params): Parameters<CloudAccountIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            account_id = params.account_id,
            "Tool called: cloud_account_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.delete_cloud_account(params.account_id).await
    }

    // =========================================================================
    // Enterprise Tools - CRDB Task Operations
    // =========================================================================

    #[tool(description = "List all Active-Active (CRDB) tasks in the Redis Enterprise cluster")]
    async fn enterprise_crdb_tasks_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_crdb_tasks_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_crdb_tasks().await
    }

    #[tool(description = "Get detailed information about a specific CRDB task")]
    async fn enterprise_crdb_task_get(
        &self,
        Parameters(params): Parameters<CrdbTaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: enterprise_crdb_task_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_crdb_task(&params.task_id).await
    }

    #[tool(description = "List CRDB tasks for a specific Active-Active database")]
    async fn enterprise_crdb_tasks_by_crdb(
        &self,
        Parameters(params): Parameters<CrdbGuidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_tasks_by_crdb");
        let tools = self.get_enterprise_tools().await?;
        tools.list_crdb_tasks_by_crdb(&params.crdb_guid).await
    }

    #[tool(description = "Cancel a CRDB task")]
    async fn enterprise_crdb_task_cancel(
        &self,
        Parameters(params): Parameters<CrdbTaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: enterprise_crdb_task_cancel");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.cancel_crdb_task(&params.task_id).await
    }

    // =========================================================================
    // Cloud Tools - Private Link Operations
    // =========================================================================

    #[tool(description = "Get AWS PrivateLink configuration for a subscription")]
    async fn cloud_private_link_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_private_link_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_private_link(params.subscription_id).await
    }

    #[tool(description = "Delete AWS PrivateLink configuration for a subscription")]
    async fn cloud_private_link_delete(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_private_link_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.delete_private_link(params.subscription_id).await
    }

    // =========================================================================
    // Cloud Tools - Transit Gateway Operations
    // =========================================================================

    #[tool(description = "Get AWS Transit Gateway attachments for a subscription")]
    async fn cloud_transit_gateway_attachments_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_transit_gateway_attachments_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_transit_gateway_attachments(params.subscription_id)
            .await
    }

    #[tool(description = "Delete an AWS Transit Gateway attachment")]
    async fn cloud_transit_gateway_attachment_delete(
        &self,
        Parameters(params): Parameters<TransitGatewayAttachmentIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            attachment_id = %params.attachment_id,
            "Tool called: cloud_transit_gateway_attachment_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_transit_gateway_attachment(params.subscription_id, &params.attachment_id)
            .await
    }

    // =========================================================================
    // Enterprise Tools - BDB Groups Operations
    // =========================================================================

    #[tool(description = "List all database groups in the Redis Enterprise cluster")]
    async fn enterprise_bdb_groups_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_bdb_groups_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_bdb_groups().await
    }

    #[tool(description = "Get detailed information about a specific database group")]
    async fn enterprise_bdb_group_get(
        &self,
        Parameters(params): Parameters<BdbGroupIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(uid = params.uid, "Tool called: enterprise_bdb_group_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_bdb_group(params.uid as u64).await
    }

    #[tool(description = "Delete a database group")]
    async fn enterprise_bdb_group_delete(
        &self,
        Parameters(params): Parameters<BdbGroupIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(uid = params.uid, "Tool called: enterprise_bdb_group_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_bdb_group(params.uid as u64).await
    }

    // =========================================================================
    // Enterprise Tools - OCSP Operations
    // =========================================================================

    #[tool(description = "Get OCSP (Online Certificate Status Protocol) configuration")]
    async fn enterprise_ocsp_config_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ocsp_config_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_ocsp_config().await
    }

    #[tool(description = "Get OCSP status showing certificate validation state")]
    async fn enterprise_ocsp_status_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ocsp_status_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_ocsp_status().await
    }

    #[tool(description = "Test OCSP connectivity and certificate validation")]
    async fn enterprise_ocsp_test(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ocsp_test");
        let tools = self.get_enterprise_tools().await?;
        tools.test_ocsp().await
    }

    // =========================================================================
    // Enterprise Tools - DNS Suffix Operations
    // =========================================================================

    #[tool(description = "List all DNS suffixes in the Redis Enterprise cluster")]
    async fn enterprise_suffixes_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_suffixes_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_suffixes().await
    }

    #[tool(description = "Get detailed information about a specific DNS suffix")]
    async fn enterprise_suffix_get(
        &self,
        Parameters(params): Parameters<SuffixNameParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_suffix_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_suffix(&params.name).await
    }

    #[tool(description = "Get cluster-level DNS suffixes")]
    async fn enterprise_cluster_suffixes_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_suffixes_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_suffixes().await
    }

    #[tool(description = "Delete a DNS suffix")]
    async fn enterprise_suffix_delete(
        &self,
        Parameters(params): Parameters<SuffixNameParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_suffix_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_suffix(&params.name).await
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
