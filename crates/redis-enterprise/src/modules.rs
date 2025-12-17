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

    // Additional fields from API audit
    /// Whether the module supports BigStore (Auto Tiering) version 2
    pub bigstore_version_2_support: Option<bool>,

    /// Name of the capability this module provides
    pub capability_name: Option<String>,

    /// Redis command used to configure this module
    pub config_command: Option<String>,

    /// CRDB (Conflict-free Replicated Database) configuration
    /// The API returns an empty object {} for modules without CRDB support
    pub crdb: Option<Value>,

    /// Module dependencies
    /// The API returns an empty object {} for modules without dependencies
    pub dependencies: Option<Value>,

    /// Contact email address of the module author
    pub email: Option<String>,

    /// Minimum Redis Enterprise version required for this module
    pub min_redis_pack_version: Option<String>,

    /// List of platforms this module supports (e.g., 'linux-x64', 'linux-arm64')
    pub platforms: Option<Vec<String>>,

    /// SHA256 checksum of the module binary for verification
    pub sha256: Option<String>,

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
    ///
    /// Note: Some Redis Enterprise versions (particularly RE 8.x) do not support
    /// module upload via the REST API. In those cases, use the Admin UI or
    /// node-level CLI tools (rladmin) to upload modules.
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
                match self
                    .client
                    .post_multipart("/v1/modules", module_data, "module", file_name)
                    .await
                {
                    Ok(response) => Ok(response),
                    Err(crate::error::RestError::ApiError { code: 405, .. }) => {
                        Err(crate::error::RestError::ValidationError(
                            "Module upload via REST API is not supported in this Redis Enterprise version. \
                             Use the Admin UI or rladmin CLI to upload modules.".to_string()
                        ))
                    }
                    Err(e) => Err(e),
                }
            }
            Err(crate::error::RestError::ApiError { code: 405, .. }) => {
                Err(crate::error::RestError::ValidationError(
                    "Module upload via REST API is not supported in this Redis Enterprise version. \
                     Use the Admin UI or rladmin CLI to upload modules.".to_string()
                ))
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
