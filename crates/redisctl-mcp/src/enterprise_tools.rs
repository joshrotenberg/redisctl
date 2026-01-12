//! Enterprise tools implementation
//!
//! Wraps Redis Enterprise API client operations for MCP tool invocation.

use redis_enterprise::{
    AlertHandler, BdbHandler, ClusterHandler, CreateDatabaseRequest, EnterpriseClient,
    LicenseHandler, LogsHandler, NodeHandler, ShardHandler, StatsHandler,
};
use redisctl_config::Config;
use rmcp::{ErrorData as RmcpError, model::*};
use tracing::debug;

/// Enterprise tools wrapper
#[derive(Clone)]
pub struct EnterpriseTools {
    client: EnterpriseClient,
}

impl EnterpriseTools {
    /// Create new Enterprise tools instance
    pub fn new(profile: Option<&str>) -> anyhow::Result<Self> {
        let config = Config::load()?;

        // Resolve profile name: explicit > default > error
        let profile_name = match profile {
            Some(name) => name.to_string(),
            None => config.resolve_enterprise_profile(None)?,
        };

        debug!(profile = %profile_name, "Loading Enterprise client from profile");

        let profile_config = config
            .profiles
            .get(&profile_name)
            .ok_or_else(|| anyhow::anyhow!("Enterprise profile '{}' not found", profile_name))?;

        let (url, username, password, insecure) =
            profile_config.enterprise_credentials().ok_or_else(|| {
                anyhow::anyhow!("Profile '{}' is not an Enterprise profile", profile_name)
            })?;

        let mut builder = EnterpriseClient::builder()
            .base_url(url)
            .username(username)
            .insecure(insecure);

        if let Some(pwd) = password {
            builder = builder.password(pwd);
        }

        let client = builder.build()?;

        Ok(Self { client })
    }

    fn to_result(&self, value: serde_json::Value) -> Result<CallToolResult, RmcpError> {
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
        )]))
    }

    fn to_error(&self, err: impl std::fmt::Display) -> RmcpError {
        RmcpError::internal_error(err.to_string(), None)
    }

    /// Get cluster information
    pub async fn get_cluster(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ClusterHandler::new(self.client.clone());
        let cluster = handler.info().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(cluster).map_err(|e| self.to_error(e))?)
    }

    /// List all nodes
    pub async fn list_nodes(&self) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        let nodes = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(nodes).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific node
    pub async fn get_node(&self, node_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        let node = handler
            .get(node_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(node).map_err(|e| self.to_error(e))?)
    }

    /// List all databases
    pub async fn list_databases(&self) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let dbs = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(dbs).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific database
    pub async fn get_database(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let db = handler
            .get(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(db).map_err(|e| self.to_error(e))?)
    }

    /// Get database statistics
    pub async fn get_database_stats(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = StatsHandler::new(self.client.clone());
        let stats = handler
            .database(database_id as u32, None)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(stats).map_err(|e| self.to_error(e))?)
    }

    /// List all shards
    pub async fn list_shards(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ShardHandler::new(self.client.clone());
        let shards = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(shards).map_err(|e| self.to_error(e))?)
    }

    /// List active alerts
    pub async fn list_alerts(&self) -> Result<CallToolResult, RmcpError> {
        let handler = AlertHandler::new(self.client.clone());
        let alerts = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(alerts).map_err(|e| self.to_error(e))?)
    }

    /// Get cluster logs
    pub async fn get_logs(&self) -> Result<CallToolResult, RmcpError> {
        let handler = LogsHandler::new(self.client.clone());
        let logs = handler.list(None).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(logs).map_err(|e| self.to_error(e))?)
    }

    /// Get license information
    pub async fn get_license(&self) -> Result<CallToolResult, RmcpError> {
        let handler = LicenseHandler::new(self.client.clone());
        let license = handler.get().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(license).map_err(|e| self.to_error(e))?)
    }

    /// Create a new database
    pub async fn create_database(
        &self,
        name: &str,
        memory_size_mb: Option<u64>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());

        let memory_size = memory_size_mb.unwrap_or(100) * 1024 * 1024; // Default 100MB

        let request = CreateDatabaseRequest::builder()
            .name(name)
            .memory_size(memory_size)
            .build();

        let db = handler
            .create(request)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(db).map_err(|e| self.to_error(e))?)
    }
}
