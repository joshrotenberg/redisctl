//! Task models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Background task/job in Redis Cloud
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub resource_type: Option<String>,
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

