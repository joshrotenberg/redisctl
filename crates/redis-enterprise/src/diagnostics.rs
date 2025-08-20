//! Diagnostic operations for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Diagnostic check request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_uids: Option<Vec<u32>>,
}

/// Diagnostic result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResult {
    pub check_name: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Diagnostic report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub report_id: String,
    pub timestamp: String,
    pub results: Vec<DiagnosticResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<DiagnosticSummary>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Diagnostic summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSummary {
    pub total_checks: u32,
    pub passed: u32,
    pub warnings: u32,
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
}
