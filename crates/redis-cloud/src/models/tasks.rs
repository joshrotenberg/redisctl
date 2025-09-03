//! Task models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Background task/job in Redis Cloud
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "taskId", alias = "id")]
    pub task_id: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,

    #[serde(rename = "type")]
    pub task_type: Option<String>,
    /// Present on some task types
    #[serde(rename = "databaseId")]
    pub database_id: Option<u64>,
    /// Optional human-readable message/description
    pub message: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Task list response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,

    #[serde(flatten)]
    pub extra: Value,
}
