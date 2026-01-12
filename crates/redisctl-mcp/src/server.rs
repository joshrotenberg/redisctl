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
        description = "Create a new Redis Enterprise database. Requires name, optionally memory_size_mb (default 100) and port."
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
                "Server is in read-only mode. Use --read-only=false to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_database(&params.name, params.memory_size_mb)
            .await
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
