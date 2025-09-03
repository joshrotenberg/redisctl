//! Job scheduler management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Scheduled job information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledJob {
    pub job_id: String,
    pub name: String,
    pub job_type: String,
    pub schedule: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create scheduled job request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateScheduledJobRequest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub job_type: String,
    #[builder(setter(into))]
    pub schedule: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub params: Option<Value>,
}

/// Job execution history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecution {
    pub execution_id: String,
    pub job_id: String,
    pub start_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    pub status: String,
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
