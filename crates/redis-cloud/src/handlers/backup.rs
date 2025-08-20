//! Backup operations handler

use crate::{
    client::CloudClient,
    models::{CloudBackup, CreateBackupRequest},
    Result,
};
use serde_json::Value;

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
