//! Redis module management
//!
//! ## Overview
//! - List available modules
//! - Query module versions
//! - Configure module settings

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub uid: String,
    pub module_name: Option<String>,
    pub version: Option<u32>,
    pub semantic_version: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub command_line_args: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub min_redis_version: Option<String>,
    pub compatible_redis_version: Option<String>,
    pub display_name: Option<String>,
    pub is_bundled: Option<bool>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Module handler for managing Redis modules
pub struct ModuleHandler {
    client: RestClient,
}

/// Alias for backwards compatibility and intuitive plural naming
pub type ModulesHandler = ModuleHandler;

impl ModuleHandler {
    pub fn new(client: RestClient) -> Self {
        ModuleHandler { client }
    }

    /// List all modules
    pub async fn list(&self) -> Result<Vec<Module>> {
        self.client.get("/v1/modules").await
    }

    /// Get specific module
    pub async fn get(&self, uid: &str) -> Result<Module> {
        self.client.get(&format!("/v1/modules/{}", uid)).await
    }

    /// Upload new module (tries v2 first, falls back to v1)
    pub async fn upload(&self, module_data: Vec<u8>, file_name: &str) -> Result<Value> {
        // Try v2 first (returns action_uid for async tracking)
        match self
            .client
            .post_multipart("/v2/modules", module_data.clone(), "module", file_name)
            .await
        {
            Ok(response) => Ok(response),
            Err(crate::error::RestError::NotFound) => {
                // v2 endpoint doesn't exist, try v1
                self.client
                    .post_multipart("/v1/modules", module_data, "module", file_name)
                    .await
            }
            Err(e) => Err(e),
        }
    }

    /// Delete module
    pub async fn delete(&self, uid: &str) -> Result<()> {
        self.client.delete(&format!("/v1/modules/{}", uid)).await
    }

    /// Update module configuration
    pub async fn update(&self, uid: &str, updates: Value) -> Result<Module> {
        self.client
            .put(&format!("/v1/modules/{}", uid), &updates)
            .await
    }

    /// Configure modules for a specific database - POST /v1/modules/config/bdb/{uid}
    pub async fn config_bdb(&self, bdb_uid: u32, config: Value) -> Result<Module> {
        self.client
            .post(&format!("/v1/modules/config/bdb/{}", bdb_uid), &config)
            .await
    }
}
