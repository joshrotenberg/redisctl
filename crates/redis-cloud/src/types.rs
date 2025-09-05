//! Common types for Redis Cloud API
//!
//! This module contains shared types used across multiple endpoints.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Task Types (Most common - appears in 37 endpoints)
// ============================================================================

/// TaskStateUpdate - Line 9725 in OpenAPI spec
/// Represents the state of an asynchronous task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStateUpdate {
    /// UUID of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    /// Type of command being executed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Current status of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,

    /// Description of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Timestamp of the task update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// Response from the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ProcessorResponse>,

    /// HATEOAS links for related resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,
}

/// TaskStatus enum - Part of TaskStateUpdate schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Initialized,
    Received,
    ProcessingInProgress,
    ProcessingCompleted,
    ProcessingError,
}

/// ProcessorResponse - Line 14268 in OpenAPI spec
/// Contains the result or error from task processing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorResponse {
    /// ID of the primary resource created/modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i32>,

    /// ID of an additional resource if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_resource_id: Option<i32>,

    /// The resource object itself
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Value>,

    /// Error code if the operation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProcessorError>,
}

/// ProcessorError - Subset of massive error enum for common errors
/// Full enum has 600+ values, we'll add as needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessorError {
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "BAD_REQUEST")]
    BadRequest,
    #[serde(rename = "GENERAL_ERROR")]
    GeneralError,
    #[serde(other)]
    Other,
}

/// TasksStateUpdate - Collection of task updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksStateUpdate {
    pub tasks: Vec<TaskStateUpdate>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Tag Types (Used in database and subscription endpoints)
// ============================================================================

/// CloudTag - Individual tag with key-value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudTag {
    pub key: String,
    pub value: String,
}

/// CloudTags - Collection of tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudTags {
    pub tags: Vec<CloudTag>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Common Response Types
// ============================================================================

/// Generic paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The actual data items
    #[serde(flatten)]
    pub data: T,

    /// Pagination metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
}

/// HATEOAS Link object for API navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    pub href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Links collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    pub links: Vec<Link>,
}

// ============================================================================
// Common Enums used across multiple endpoints
// ============================================================================

/// Cloud provider enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
}

/// Database protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Redis,
    Memcached,
    Stack,
}

/// Data persistence options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPersistence {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "aof-every-1-sec")]
    AofEvery1Sec,
    #[serde(rename = "aof-every-write")]
    AofEveryWrite,
    #[serde(rename = "snapshot-every-1-hour")]
    SnapshotEvery1Hour,
    #[serde(rename = "snapshot-every-6-hours")]
    SnapshotEvery6Hours,
    #[serde(rename = "snapshot-every-12-hours")]
    SnapshotEvery12Hours,
}

/// Subscription status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Pending,
    Active,
    Deleting,
    Error,
}

/// Database status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseStatus {
    Pending,
    Active,
    ActiveChangePending,
    ImportPending,
    DeletePending,
    Recovery,
    Error,
}

// ============================================================================
// Utility Types
// ============================================================================

/// Empty response for successful operations with no body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}

/// Generic error response structure
/// Note: The actual API may return different error formats,
/// this is a common structure we'll adapt as needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,
}
