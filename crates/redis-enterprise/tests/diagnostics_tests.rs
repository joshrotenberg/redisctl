//! Diagnostics endpoint tests for Redis Enterprise

use redis_enterprise::{DiagnosticRequest, DiagnosticsHandler, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn test_diagnostic_result() -> serde_json::Value {
    json!({
        "check_name": "memory_usage",
        "status": "passed",
        "message": "Memory usage is within acceptable limits",
        "details": {
            "used_memory": "2GB",
            "total_memory": "8GB",
            "usage_percentage": 25
        },
        "recommendations": []
    })
}

fn test_diagnostic_result_warning() -> serde_json::Value {
    json!({
        "check_name": "disk_space",
        "status": "warning",
        "message": "Disk space is running low",
        "details": {
            "used_disk": "7GB",
            "total_disk": "10GB",
            "usage_percentage": 70
        },
        "recommendations": [
            "Consider adding more disk space",
            "Clean up old log files"
        ]
    })
}

fn test_diagnostic_result_failed() -> serde_json::Value {
    json!({
        "check_name": "connectivity",
        "status": "failed",
        "message": "Cannot connect to node",
        "details": {
            "node_id": 3,
            "error": "Connection timeout"
        },
        "recommendations": [
            "Check network connectivity",
            "Verify node status"
        ]
    })
}

fn test_diagnostic_report() -> serde_json::Value {
    json!({
        "report_id": "report-123-abc",
        "timestamp": "2023-01-01T12:00:00Z",
        "results": [
            test_diagnostic_result(),
            test_diagnostic_result_warning()
        ],
        "summary": {
            "total_checks": 2,
            "passed": 1,
            "warnings": 1,
            "failures": 0
        }
    })
}

fn test_diagnostic_report_with_failures() -> serde_json::Value {
    json!({
        "report_id": "report-456-def",
        "timestamp": "2023-01-01T13:00:00Z",
        "results": [
            test_diagnostic_result(),
            test_diagnostic_result_warning(),
            test_diagnostic_result_failed()
        ],
        "summary": {
            "total_checks": 3,
            "passed": 1,
            "warnings": 1,
            "failures": 1
        }
    })
}

fn test_minimal_diagnostic_report() -> serde_json::Value {
    json!({
        "report_id": "report-minimal",
        "timestamp": "2023-01-01T14:00:00Z",
        "results": []
    })
}

#[tokio::test]
async fn test_diagnostics_run() {
    let mock_server = MockServer::start().await;

    let request = DiagnosticRequest {
        checks: Some(vec!["memory_usage".to_string(), "disk_space".to_string()]),
        node_uids: Some(vec![1, 2]),
        bdb_uids: Some(vec![1]),
    };

    Mock::given(method("POST"))
        .and(path("/v1/diagnostics"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(test_diagnostic_report()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.run(request).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-123-abc");
    assert_eq!(report.results.len(), 2);
    assert!(report.summary.is_some());
    let summary = report.summary.unwrap();
    assert_eq!(summary.total_checks, 2);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.warnings, 1);
    assert_eq!(summary.failures, 0);
}

#[tokio::test]
async fn test_diagnostics_run_minimal() {
    let mock_server = MockServer::start().await;

    let request = DiagnosticRequest {
        checks: None,
        node_uids: None,
        bdb_uids: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/diagnostics"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(test_minimal_diagnostic_report()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.run(request).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-minimal");
    assert_eq!(report.results.len(), 0);
    assert!(report.summary.is_none());
}

#[tokio::test]
async fn test_diagnostics_run_with_failures() {
    let mock_server = MockServer::start().await;

    let request = DiagnosticRequest {
        checks: Some(vec![
            "memory_usage".to_string(),
            "disk_space".to_string(),
            "connectivity".to_string(),
        ]),
        node_uids: Some(vec![1, 2, 3]),
        bdb_uids: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/diagnostics"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(test_diagnostic_report_with_failures()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.run(request).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-456-def");
    assert_eq!(report.results.len(), 3);

    // Check specific results
    assert_eq!(report.results[0].check_name, "memory_usage");
    assert_eq!(report.results[0].status, "passed");
    assert_eq!(report.results[1].check_name, "disk_space");
    assert_eq!(report.results[1].status, "warning");
    assert_eq!(report.results[2].check_name, "connectivity");
    assert_eq!(report.results[2].status, "failed");

    let summary = report.summary.unwrap();
    assert_eq!(summary.total_checks, 3);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.warnings, 1);
    assert_eq!(summary.failures, 1);
}

#[tokio::test]
async fn test_diagnostics_list_checks() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/checks"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            "memory_usage",
            "disk_space",
            "connectivity",
            "cluster_health",
            "database_status",
            "replication_lag"
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.list_checks().await;

    assert!(result.is_ok());
    let checks = result.unwrap();
    assert_eq!(checks.len(), 6);
    assert!(checks.contains(&"memory_usage".to_string()));
    assert!(checks.contains(&"disk_space".to_string()));
    assert!(checks.contains(&"connectivity".to_string()));
    assert!(checks.contains(&"cluster_health".to_string()));
    assert!(checks.contains(&"database_status".to_string()));
    assert!(checks.contains(&"replication_lag".to_string()));
}

#[tokio::test]
async fn test_diagnostics_list_checks_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/checks"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.list_checks().await;

    assert!(result.is_ok());
    let checks = result.unwrap();
    assert_eq!(checks.len(), 0);
}

#[tokio::test]
async fn test_diagnostics_get_last_report() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/last"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_diagnostic_report()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.get_last_report().await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-123-abc");
    assert_eq!(report.results.len(), 2);
    assert!(report.summary.is_some());
}

#[tokio::test]
async fn test_diagnostics_get_last_report_none() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/last"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "No diagnostic reports found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.get_last_report().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_diagnostics_get_report() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/reports/report-123-abc"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_diagnostic_report()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.get_report("report-123-abc").await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-123-abc");
    assert_eq!(report.results.len(), 2);

    // Verify specific diagnostic results
    assert_eq!(report.results[0].check_name, "memory_usage");
    assert_eq!(report.results[0].status, "passed");
    assert!(report.results[0].message.is_some());
    assert!(report.results[0].details.is_some());

    assert_eq!(report.results[1].check_name, "disk_space");
    assert_eq!(report.results[1].status, "warning");
    assert!(report.results[1].recommendations.is_some());
}

#[tokio::test]
async fn test_diagnostics_get_report_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/reports/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Diagnostic report not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.get_report("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_diagnostics_list_reports() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/reports"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_diagnostic_report(),
            test_diagnostic_report_with_failures(),
            test_minimal_diagnostic_report()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.list_reports().await;

    assert!(result.is_ok());
    let reports = result.unwrap();
    assert_eq!(reports.len(), 3);

    // Check each report
    assert_eq!(reports[0].report_id, "report-123-abc");
    assert_eq!(reports[0].results.len(), 2);

    assert_eq!(reports[1].report_id, "report-456-def");
    assert_eq!(reports[1].results.len(), 3);

    assert_eq!(reports[2].report_id, "report-minimal");
    assert_eq!(reports[2].results.len(), 0);
}

#[tokio::test]
async fn test_diagnostics_list_reports_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/diagnostics/reports"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = DiagnosticsHandler::new(client);
    let result = handler.list_reports().await;

    assert!(result.is_ok());
    let reports = result.unwrap();
    assert_eq!(reports.len(), 0);
}
