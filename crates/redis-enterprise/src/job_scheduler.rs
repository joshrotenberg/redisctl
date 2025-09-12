//! Scheduled job management
//!
//! ## Overview
//! - Configure scheduled jobs
//! - Query job history
//! - Manage job execution

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Scheduled job information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledJob {
    /// Unique identifier for the scheduled job
    pub job_id: String,
    /// Human-readable name for the job
    pub name: String,
    /// Type of job (backup, cleanup, rotation, etc.)
    pub job_type: String,
    /// Cron-style schedule expression for when the job runs
    pub schedule: String,
    /// Whether the scheduled job is currently enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Timestamp of the last job execution (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<String>,
    /// Timestamp of the next scheduled job execution (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run: Option<String>,
    /// Job-specific parameters and configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create scheduled job request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateScheduledJobRequest {
    /// Human-readable name for the new job
    #[builder(setter(into))]
    pub name: String,
    /// Type of job to create (backup, cleanup, rotation, etc.)
    #[builder(setter(into))]
    pub job_type: String,
    /// Cron-style schedule expression defining when the job should run
    #[builder(setter(into))]
    pub schedule: String,
    /// Whether the job should be enabled immediately upon creation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enabled: Option<bool>,
    /// Job-specific parameters and configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub params: Option<Value>,
}

/// Job execution history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecution {
    /// Unique identifier for this job execution instance
    pub execution_id: String,
    /// ID of the scheduled job that was executed
    pub job_id: String,
    /// Timestamp when the job execution started (ISO 8601 format)
    pub start_time: String,
    /// Timestamp when the job execution finished (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Execution status (running, completed, failed, cancelled)
    pub status: String,
    /// Error description if the job execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Job scheduler handler
pub struct JobSchedulerHandler {
    client: RestClient,
}

impl JobSchedulerHandler {
    pub fn new(client: RestClient) -> Self {
        JobSchedulerHandler { client }
    }

    /// List all scheduled jobs
    pub async fn list(&self) -> Result<Vec<ScheduledJob>> {
        self.client.get("/v1/job_scheduler").await
    }

    /// Get specific scheduled job
    pub async fn get(&self, job_id: &str) -> Result<ScheduledJob> {
        self.client
            .get(&format!("/v1/job_scheduler/{}", job_id))
            .await
    }

    /// Create a new scheduled job
    pub async fn create(&self, request: CreateScheduledJobRequest) -> Result<ScheduledJob> {
        self.client.post("/v1/job_scheduler", &request).await
    }

    /// Update a scheduled job
    pub async fn update(
        &self,
        job_id: &str,
        request: CreateScheduledJobRequest,
    ) -> Result<ScheduledJob> {
        self.client
            .put(&format!("/v1/job_scheduler/{}", job_id), &request)
            .await
    }

    /// Delete a scheduled job
    pub async fn delete(&self, job_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/job_scheduler/{}", job_id))
            .await
    }

    /// Trigger job execution
    pub async fn trigger(&self, job_id: &str) -> Result<JobExecution> {
        self.client
            .post(
                &format!("/v1/job_scheduler/{}/trigger", job_id),
                &Value::Null,
            )
            .await
    }

    /// Get job execution history
    pub async fn history(&self, job_id: &str) -> Result<Vec<JobExecution>> {
        self.client
            .get(&format!("/v1/job_scheduler/{}/history", job_id))
            .await
    }

    /// Update job scheduler globally - PUT /v1/job_scheduler
    pub async fn update_all(&self, body: Value) -> Result<Vec<ScheduledJob>> {
        self.client.put("/v1/job_scheduler", &body).await
    }
}
