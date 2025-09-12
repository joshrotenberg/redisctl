//! Diagnostics management for Redis Enterprise
//!
//! ## Overview
//! - List and query resources
//! - Create and update configurations
//! - Monitor status and metrics

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Diagnostic check request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DiagnosticRequest {
    /// Specific diagnostic checks to run (if not specified, runs all checks)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub checks: Option<Vec<String>>,
    /// Node UIDs to run diagnostics on (if not specified, runs on all nodes)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub node_uids: Option<Vec<u32>>,
    /// Database UIDs to run diagnostics on (if not specified, runs on all databases)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bdb_uids: Option<Vec<u32>>,
}

/// Diagnostic result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResult {
    /// Name of the diagnostic check performed
    pub check_name: String,
    /// Status of the check ('pass', 'warning', 'fail')
    pub status: String,
    /// Human-readable message describing the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Additional details about the check result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
    /// Recommended actions to resolve any issues found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Diagnostic report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    /// Unique identifier for this diagnostic report
    pub report_id: String,
    /// Timestamp when the report was generated
    pub timestamp: String,
    /// List of individual diagnostic check results
    pub results: Vec<DiagnosticResult>,
    /// Summary statistics of the diagnostic run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<DiagnosticSummary>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Diagnostic summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSummary {
    /// Total number of diagnostic checks performed
    pub total_checks: u32,
    /// Number of checks that passed
    pub passed: u32,
    /// Number of checks with warnings
    pub warnings: u32,
    /// Number of checks that failed
    pub failures: u32,
}

/// Diagnostics handler
pub struct DiagnosticsHandler {
    client: RestClient,
}

impl DiagnosticsHandler {
    pub fn new(client: RestClient) -> Self {
        DiagnosticsHandler { client }
    }

    /// Run diagnostic checks
    pub async fn run(&self, request: DiagnosticRequest) -> Result<DiagnosticReport> {
        self.client.post("/v1/diagnostics", &request).await
    }

    /// Get available diagnostic checks
    pub async fn list_checks(&self) -> Result<Vec<String>> {
        self.client.get("/v1/diagnostics/checks").await
    }

    /// Get last diagnostic report
    pub async fn get_last_report(&self) -> Result<DiagnosticReport> {
        self.client.get("/v1/diagnostics/last").await
    }

    /// Get specific diagnostic report
    pub async fn get_report(&self, report_id: &str) -> Result<DiagnosticReport> {
        self.client
            .get(&format!("/v1/diagnostics/reports/{}", report_id))
            .await
    }

    /// List all diagnostic reports
    pub async fn list_reports(&self) -> Result<Vec<DiagnosticReport>> {
        self.client.get("/v1/diagnostics/reports").await
    }

    /// Get diagnostics configuration/state - GET /v1/diagnostics
    pub async fn get_config(&self) -> Result<Value> {
        self.client.get("/v1/diagnostics").await
    }

    /// Update diagnostics configuration/state - PUT /v1/diagnostics
    pub async fn update_config(&self, body: Value) -> Result<Value> {
        self.client.put("/v1/diagnostics", &body).await
    }
}
