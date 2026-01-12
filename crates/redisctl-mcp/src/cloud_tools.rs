//! Cloud tools implementation
//!
//! Wraps Redis Cloud API client operations for MCP tool invocation.

use redis_cloud::{AccountHandler, CloudClient, DatabaseHandler, SubscriptionHandler, TaskHandler};
use redisctl_config::Config;
use rmcp::{ErrorData as RmcpError, model::*};
use tracing::debug;

/// Cloud tools wrapper
#[derive(Clone)]
pub struct CloudTools {
    client: CloudClient,
}

impl CloudTools {
    /// Create new Cloud tools instance
    pub fn new(profile: Option<&str>) -> anyhow::Result<Self> {
        let config = Config::load()?;

        // Resolve profile name: explicit > default > error
        let profile_name = match profile {
            Some(name) => name.to_string(),
            None => config.resolve_cloud_profile(None)?,
        };

        debug!(profile = %profile_name, "Loading Cloud client from profile");

        let profile_config = config
            .profiles
            .get(&profile_name)
            .ok_or_else(|| anyhow::anyhow!("Cloud profile '{}' not found", profile_name))?;

        let (api_key, api_secret, api_url) = profile_config
            .cloud_credentials()
            .ok_or_else(|| anyhow::anyhow!("Profile '{}' is not a Cloud profile", profile_name))?;

        let client = CloudClient::builder()
            .api_key(api_key)
            .api_secret(api_secret)
            .base_url(api_url.to_string())
            .build()?;

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

    /// Get account information
    pub async fn get_account(&self) -> Result<CallToolResult, RmcpError> {
        let handler = AccountHandler::new(self.client.clone());
        let account = handler
            .get_current_account()
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(account).map_err(|e| self.to_error(e))?)
    }

    /// List all subscriptions
    pub async fn list_subscriptions(&self) -> Result<CallToolResult, RmcpError> {
        let handler = SubscriptionHandler::new(self.client.clone());
        let subs = handler
            .get_all_subscriptions()
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(subs).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific subscription
    pub async fn get_subscription(
        &self,
        subscription_id: i64,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = SubscriptionHandler::new(self.client.clone());
        let sub = handler
            .get_subscription_by_id(subscription_id as i32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(sub).map_err(|e| self.to_error(e))?)
    }

    /// List databases in a subscription
    pub async fn list_databases(&self, subscription_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = DatabaseHandler::new(self.client.clone());
        let dbs = handler
            .get_subscription_databases(subscription_id as i32, None, None)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(dbs).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific database
    pub async fn get_database(
        &self,
        subscription_id: i64,
        database_id: i64,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = DatabaseHandler::new(self.client.clone());
        let db = handler
            .get_subscription_database_by_id(subscription_id as i32, database_id as i32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(db).map_err(|e| self.to_error(e))?)
    }

    /// List tasks
    pub async fn list_tasks(&self) -> Result<CallToolResult, RmcpError> {
        let handler = TaskHandler::new(self.client.clone());
        let tasks = handler
            .get_all_tasks()
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(tasks).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific task
    pub async fn get_task(&self, task_id: &str) -> Result<CallToolResult, RmcpError> {
        let handler = TaskHandler::new(self.client.clone());
        let task = handler
            .get_task_by_id(task_id.to_string())
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(task).map_err(|e| self.to_error(e))?)
    }
}
