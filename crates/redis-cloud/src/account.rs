//! Account operations and models

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Models
// ============================================================================

/// ModulesData
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulesData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<Module>>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RootAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootAccount {
    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Account system log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSystemLogEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<Region>>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs Account payment methods
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs database module information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// AccountSystemLogEntries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSystemLogEntries {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AccountSystemLogEntry>>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// SearchScalingFactorsData
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchScalingFactorsData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_performance_factors: Option<Vec<String>>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Account session log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSessionLogEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs data persistence information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPersistenceEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// DataPersistenceOptions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPersistenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<Vec<DataPersistenceEntry>>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// AccountSessionLogEntries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSessionLogEntries {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AccountSessionLogEntry>>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Account operations handler
pub struct AccountHandler {
    client: CloudClient,
}

impl AccountHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get current account
    /// Gets information on this account.
    ///
    /// GET /
    pub async fn get_current_account(&self) -> Result<RootAccount> {
        self.client.get("/").await
    }

    /// Get data persistence options
    /// Gets a list of all [data persistence](https://redis.io/docs/latest/operate/rc/databases/configuration/data-persistence/) options for this account.
    ///
    /// GET /data-persistence
    pub async fn get_data_persistence_options(&self) -> Result<DataPersistenceOptions> {
        self.client.get("/data-persistence").await
    }

    /// Get advanced capabilities
    /// Gets a list of Redis [advanced capabilities](https://redis.io/docs/latest/operate/rc/databases/configuration/advanced-capabilities/) (also known as modules) available for this account. Advanced capability support may differ based on subscription and database settings.
    ///
    /// GET /database-modules
    pub async fn get_supported_database_modules(&self) -> Result<ModulesData> {
        self.client.get("/database-modules").await
    }

    /// Get system logs
    /// Gets [system logs](https://redis.io/docs/latest/operate/rc/api/examples/audit-system-logs/) for this account.
    ///
    /// GET /logs
    pub async fn get_account_system_logs(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AccountSystemLogEntries> {
        let mut query = Vec::new();
        if let Some(v) = offset {
            query.push(format!("offset={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client.get(&format!("/logs{}", query_string)).await
    }

    /// Get payment methods
    /// Gets a list of all payment methods for this account.
    ///
    /// GET /payment-methods
    pub async fn get_account_payment_methods(&self) -> Result<PaymentMethods> {
        self.client.get("/payment-methods").await
    }

    /// Get query performance factors
    /// Gets a list of available [query performance factors](https://redis.io/docs/latest/operate/rc/databases/configuration/advanced-capabilities/#query-performance-factor).
    ///
    /// GET /query-performance-factors
    pub async fn get_supported_search_scaling_factors(&self) -> Result<SearchScalingFactorsData> {
        self.client.get("/query-performance-factors").await
    }

    /// Get available Pro plan regions
    /// Gets a list of available regions for Pro subscriptions. For Essentials subscriptions, use 'GET /fixed/plans'.
    ///
    /// GET /regions
    pub async fn get_supported_regions(&self, provider: Option<String>) -> Result<Regions> {
        let mut query = Vec::new();
        if let Some(v) = provider {
            query.push(format!("provider={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client.get(&format!("/regions{}", query_string)).await
    }

    /// Get session logs
    /// Gets session logs for this account.
    ///
    /// GET /session-logs
    pub async fn get_account_session_logs(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AccountSessionLogEntries> {
        let mut query = Vec::new();
        if let Some(v) = offset {
            query.push(format!("offset={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        let query_string = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        self.client
            .get(&format!("/session-logs{}", query_string))
            .await
    }
}
