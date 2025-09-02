//! Log models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Log entries response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsResponse {
    pub logs: Vec<LogEntry>,
    pub total: Option<u32>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Individual log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
    pub database_id: Option<u32>,
    pub subscription_id: Option<u32>,
    pub user_id: Option<u32>,
    pub request_id: Option<String>,
    pub metadata: Option<Value>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// System log entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLogsResponse {
    pub logs: Vec<SystemLogEntry>,
    pub total: Option<u32>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// System log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub component: Option<String>,
    pub action: Option<String>,
    pub user: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: Option<Value>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Session log entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLogsResponse {
    pub logs: Vec<SessionLogEntry>,
    pub total: Option<u32>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Session log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLogEntry {
    pub timestamp: String,
    pub session_id: String,
    pub user_id: Option<u32>,
    pub action: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: Option<bool>,
    pub error_message: Option<String>,
    pub metadata: Option<Value>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Log level filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}