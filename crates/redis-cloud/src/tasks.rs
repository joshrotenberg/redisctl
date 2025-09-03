//! Asynchronous task tracking and management
//!
//! This module provides functionality for tracking long-running operations in
//! Redis Cloud. Many API operations are asynchronous and return a task ID that
//! can be used to monitor progress and completion status.
//!
//! # Overview
//!
//! Redis Cloud uses tasks for operations that may take time to complete, such as:
//! - Creating or deleting subscriptions
//! - Database creation, updates, and deletion
//! - Backup and restore operations
//! - Import/export operations
//! - Network configuration changes
//!
//! # Task Lifecycle
//!
//! 1. **Initiated**: Task is created and queued
//! 2. **Processing**: Task is being executed
//! 3. **Completed**: Task finished successfully
//! 4. **Failed**: Task encountered an error
//!
//! # Key Features
//!
//! - **Task Status**: Check current status of any task
//! - **Progress Tracking**: Monitor completion percentage for long operations
//! - **Result Retrieval**: Get operation results once completed
//! - **Error Information**: Access detailed error messages for failed tasks
//! - **Task History**: Query historical task information
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, TaskHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = TaskHandler::new(client);
//!
//! // Get task status
//! let task = handler.get_task_by_id("task-123".to_string()).await?;
//!
//! // Check if task is complete
//! if task.status == Some("completed".to_string()) {
//!     println!("Task completed successfully");
//!     if let Some(response) = task.response {
//!         println!("Result: {:?}", response);
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Models
// ============================================================================

/// ProcessorResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_resource_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// TaskStateUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ProcessorResponse>,

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

/// Handler for asynchronous task operations
///
/// Tracks and manages long-running operations, providing status updates,
/// progress monitoring, and result retrieval for asynchronous API calls.
pub struct TasksHandler {
    client: CloudClient,
}

impl TasksHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get tasks
    /// Gets a list of all currently running tasks for this account.
    ///
    /// GET /tasks
    pub async fn get_all_tasks(&self) -> Result<()> {
        self.client.get("/tasks").await
    }

    /// Get a single task
    /// Gets details and status of a single task by the Task ID.
    ///
    /// GET /tasks/{taskId}
    pub async fn get_task_by_id(&self, task_id: String) -> Result<TaskStateUpdate> {
        self.client.get(&format!("/tasks/{}", task_id)).await
    }
}
