//! Backup operations handler

use crate::{Result, client::CloudClient};
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

/// Handler for Cloud backup operations
pub struct CloudBackupHandler {
    client: CloudClient,
}

impl CloudBackupHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudBackupHandler { client }
    }

    /// List all backups for a database
    pub async fn list(&self, subscription_id: u32, database_id: u32) -> Result<Vec<CloudBackup>> {
        let response: Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/databases/{}/backups",
                subscription_id, database_id
            ))
            .await?;

        if let Some(backups) = response.get("backups") {
            serde_json::from_value(backups.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Create a backup
    pub async fn create(
        &self,
        subscription_id: u32,
        database_id: u32,
        description: Option<String>,
    ) -> Result<CloudBackup> {
        let request = CreateBackupRequest {
            database_id,
            description,
        };
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/backup",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    /// Get backup details
    pub async fn get(
        &self,
        subscription_id: u32,
        database_id: u32,
        backup_id: &str,
    ) -> Result<CloudBackup> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/backups/{}",
                subscription_id, database_id, backup_id
            ))
            .await
    }

    /// Restore from backup
    pub async fn restore(
        &self,
        subscription_id: u32,
        database_id: u32,
        backup_id: &str,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/backups/{}/restore",
                    subscription_id, database_id, backup_id
                ),
                &serde_json::json!({}),
            )
            .await
    }

    /// Delete backup
    pub async fn delete(
        &self,
        subscription_id: u32,
        database_id: u32,
        backup_id: &str,
    ) -> Result<()> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/databases/{}/backups/{}",
                subscription_id, database_id, backup_id
            ))
            .await
    }
}
