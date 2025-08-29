//! Redis module management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub uid: String,
    pub name: String,
    pub version: String,
    pub semantic_version: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub command_line_args: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub min_redis_version: Option<String>,
    pub min_redis_pack_version: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Module upload request
#[derive(Debug, Serialize)]
pub struct UploadModuleRequest {
    pub module: Vec<u8>, // Binary module data
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

    /// Upload new module
    pub async fn upload(&self, module_data: Vec<u8>) -> Result<Module> {
        // Note: This endpoint typically requires multipart/form-data
        // The actual implementation would need to handle file upload
        let request = UploadModuleRequest {
            module: module_data,
        };
        self.client.post("/v1/modules", &request).await
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
}
