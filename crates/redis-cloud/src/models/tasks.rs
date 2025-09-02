//! Task models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Background task/job in Redis Cloud
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(alias = "taskId", alias = "id")]
    pub id: String,
    pub status: String,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,

    #[serde(alias = "type")]
    pub resource_type: Option<String>,
    #[serde(alias = "databaseId")]
    pub resource_id: Option<String>,
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
