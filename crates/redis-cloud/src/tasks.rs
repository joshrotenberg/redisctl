//! Asynchronous task tracking for Redis Cloud
//!
//! ## Overview
//! - Track long-running operations
//! - Query task status and progress
//! - Retrieve task results

use crate::client::CloudClient;
use crate::error::Result;
use crate::types::TaskStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Task information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "taskId")]
    pub task_id: String,

    pub status: TaskStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "createdTimestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,

    #[serde(
        rename = "lastUpdatedTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated_timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,

    #[serde(rename = "commandType", skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<TaskResponse>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TaskLink>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Task response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TaskError>,
}

/// Task error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskError {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Task link for navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLink {
    pub href: String,
    pub rel: String,

    #[serde(rename = "type")]
    pub link_type: String,
}

/// Task handler
pub struct TaskHandler {
    client: CloudClient,
}

impl TaskHandler {
    pub fn new(client: CloudClient) -> Self {
        TaskHandler { client }
    }

    /// List all tasks
    pub async fn list(&self) -> Result<Vec<Task>> {
        self.client.get("/tasks").await
    }

    /// Get a specific task
    pub async fn get(&self, task_id: &str) -> Result<Task> {
        self.client.get(&format!("/tasks/{}", task_id)).await
    }

    /// Wait for task completion
    pub async fn wait(&self, task_id: &str, max_wait_seconds: Option<u32>) -> Result<Task> {
        let max_wait = max_wait_seconds.unwrap_or(300);
        let interval = 2u32;
        let mut elapsed = 0u32;

        loop {
            let task = self.get(task_id).await?;

            match task.status {
                TaskStatus::Completed | TaskStatus::Failed => return Ok(task),
                _ => {
                    if elapsed >= max_wait {
                        return Err(crate::error::CloudError::Other(format!(
                            "Task {} did not complete within {} seconds",
                            task_id, max_wait
                        )));
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(interval as u64)).await;
                    elapsed += interval;
                }
            }
        }
    }
}
