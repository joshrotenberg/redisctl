//! Backup-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Backup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudBackup {
    pub backup_id: String,
    pub database_id: u32,
    pub status: String,
    pub created_at: String,
    pub size_bytes: Option<u64>,
    pub download_url: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Create backup request
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateBackupRequest {
    pub database_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
}
