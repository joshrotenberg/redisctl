//! Usage report endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, UsageReportConfig, UsageReportHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_usage_report() -> serde_json::Value {
    json!({
        "report_id": "report-2023-01-01",
        "timestamp": "2023-01-01T12:00:00Z",
        "period_start": "2022-12-01T00:00:00Z",
        "period_end": "2022-12-31T23:59:59Z",
        "cluster_name": "production-cluster",
        "databases": [
            {
                "bdb_uid": 1,
                "name": "redis-db-1",
                "memory_used_avg": 1073741824,
                "memory_used_peak": 2147483648u64,
                "ops_per_sec_avg": 1500.5,
                "bandwidth_avg": 5368709120u64,
                "shard_count": 2
            },
            {
                "bdb_uid": 2,
                "name": "redis-db-2",
                "memory_used_avg": 536870912,
                "memory_used_peak": 1073741824,
                "ops_per_sec_avg": 750.25,
                "bandwidth_avg": 2684354560u64
            }
        ],
        "nodes": [
            {
                "node_uid": 1,
                "cpu_usage_avg": 45.5,
                "memory_usage_avg": 17179869184u64,
                "persistent_storage_usage": 107374182400u64,
                "ephemeral_storage_usage": 53687091200u64
            },
            {
                "node_uid": 2,
                "cpu_usage_avg": 38.2,
                "memory_usage_avg": 21474836480u64,
                "persistent_storage_usage": 85899345920u64,
                "ephemeral_storage_usage": 42949672960u64
            }
        ],
        "summary": {
            "total_memory_gb": 36.0,
            "total_ops": 2250,
            "total_bandwidth_gb": 7.5,
            "database_count": 2,
            "node_count": 2,
            "shard_count": 3
        }
    })
}

fn test_usage_report_minimal() -> serde_json::Value {
    json!({
        "report_id": "report-2023-01-02",
        "timestamp": "2023-01-02T12:00:00Z",
        "period_start": "2023-01-01T00:00:00Z",
        "period_end": "2023-01-01T23:59:59Z",
        "cluster_name": "test-cluster",
        "summary": {
            "total_memory_gb": 8.0,
            "total_ops": 500,
            "total_bandwidth_gb": 2.0,
            "database_count": 1,
            "node_count": 1,
            "shard_count": 1
        }
    })
}

fn test_usage_report_config() -> serde_json::Value {
    json!({
        "enabled": true,
        "email_recipients": ["admin@example.com", "ops@example.com"],
        "frequency": "monthly",
        "include_databases": true,
        "include_nodes": true
    })
}

fn test_usage_report_config_disabled() -> serde_json::Value {
    json!({
        "enabled": false
    })
}

fn test_usage_report_config_obj() -> UsageReportConfig {
    UsageReportConfig {
        enabled: true,
        email_recipients: Some(vec![
            "admin@example.com".to_string(),
            "ops@example.com".to_string(),
        ]),
        frequency: Some("monthly".to_string()),
        include_databases: Some(true),
        include_nodes: Some(true),
    }
}

fn test_csv_content() -> String {
    "report_id,timestamp,database_name,memory_used_avg,ops_per_sec_avg\nreport-2023-01-01,2023-01-01T12:00:00Z,redis-db-1,1073741824,1500.5\nreport-2023-01-01,2023-01-01T12:00:00Z,redis-db-2,536870912,750.25".to_string()
}

#[tokio::test]
async fn test_usage_report_latest() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/latest"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_usage_report()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.latest().await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-2023-01-01");
    assert_eq!(report.cluster_name, "production-cluster");
    assert!(report.databases.is_some());
    assert!(report.nodes.is_some());
    assert!(report.summary.is_some());

    let databases = report.databases.unwrap();
    assert_eq!(databases.len(), 2);
    assert_eq!(databases[0].bdb_uid, 1);
    assert_eq!(databases[0].name, "redis-db-1");
    assert_eq!(databases[0].shard_count, Some(2));

    let nodes = report.nodes.unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].node_uid, 1);
    assert_eq!(nodes[0].cpu_usage_avg, 45.5);

    let summary = report.summary.unwrap();
    assert_eq!(summary.total_memory_gb, 36.0);
    assert_eq!(summary.database_count, 2);
    assert_eq!(summary.node_count, 2);
}

#[tokio::test]
async fn test_usage_report_latest_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/latest"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "No usage reports found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.latest().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_usage_report_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_usage_report(),
            test_usage_report_minimal()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let reports = result.unwrap();
    assert_eq!(reports.len(), 2);
    assert_eq!(reports[0].report_id, "report-2023-01-01");
    assert_eq!(reports[0].cluster_name, "production-cluster");
    assert_eq!(reports[1].report_id, "report-2023-01-02");
    assert_eq!(reports[1].cluster_name, "test-cluster");
}

#[tokio::test]
async fn test_usage_report_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report"))
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

    let handler = UsageReportHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let reports = result.unwrap();
    assert_eq!(reports.len(), 0);
}

#[tokio::test]
async fn test_usage_report_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/report-2023-01-01"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_usage_report()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.get("report-2023-01-01").await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-2023-01-01");
    assert_eq!(report.cluster_name, "production-cluster");
    assert!(report.databases.is_some());
    assert!(report.nodes.is_some());
}

#[tokio::test]
async fn test_usage_report_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Usage report not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_usage_report_generate() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/usage_report/generate"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(json!({
            "report_id": "report-new",
            "timestamp": "2023-01-03T12:00:00Z",
            "period_start": "2023-01-01T00:00:00Z",
            "period_end": "2023-01-31T23:59:59Z",
            "cluster_name": "production-cluster",
            "summary": {
                "total_memory_gb": 48.0,
                "total_ops": 3000,
                "total_bandwidth_gb": 10.0,
                "database_count": 3,
                "node_count": 3,
                "shard_count": 5
            }
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.generate().await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.report_id, "report-new");
    assert_eq!(report.cluster_name, "production-cluster");
    assert!(report.summary.is_some());

    let summary = report.summary.unwrap();
    assert_eq!(summary.total_memory_gb, 48.0);
    assert_eq!(summary.database_count, 3);
}

#[tokio::test]
async fn test_usage_report_generate_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/usage_report/generate"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(500, "Failed to generate usage report"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.generate().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_usage_report_get_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/config"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_usage_report_config()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.get_config().await;

    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.enabled);
    assert_eq!(
        config.email_recipients,
        Some(vec![
            "admin@example.com".to_string(),
            "ops@example.com".to_string()
        ])
    );
    assert_eq!(config.frequency, Some("monthly".to_string()));
    assert_eq!(config.include_databases, Some(true));
    assert_eq!(config.include_nodes, Some(true));
}

#[tokio::test]
async fn test_usage_report_get_config_disabled() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/config"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_usage_report_config_disabled()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.get_config().await;

    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(!config.enabled);
    assert!(config.email_recipients.is_none());
}

#[tokio::test]
async fn test_usage_report_update_config() {
    let mock_server = MockServer::start().await;
    let config = test_usage_report_config_obj();

    Mock::given(method("PUT"))
        .and(path("/v1/usage_report/config"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(success_response(test_usage_report_config()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_ok());
    let updated_config = result.unwrap();
    assert!(updated_config.enabled);
    assert_eq!(updated_config.frequency, Some("monthly".to_string()));
}

#[tokio::test]
async fn test_usage_report_update_config_disable() {
    let mock_server = MockServer::start().await;
    let config = UsageReportConfig {
        enabled: false,
        email_recipients: None,
        frequency: None,
        include_databases: None,
        include_nodes: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/usage_report/config"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(success_response(test_usage_report_config_disabled()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_ok());
    let updated_config = result.unwrap();
    assert!(!updated_config.enabled);
}

#[tokio::test]
async fn test_usage_report_update_config_invalid() {
    let mock_server = MockServer::start().await;
    let config = UsageReportConfig {
        enabled: true,
        email_recipients: Some(vec!["invalid-email".to_string()]),
        frequency: Some("invalid".to_string()),
        include_databases: None,
        include_nodes: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/usage_report/config"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(error_response(400, "Invalid usage report configuration"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_usage_report_download_csv() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/report-2023-01-01/csv"))
        .and(basic_auth("admin", "password"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(test_csv_content())
                .append_header("content-type", "text/csv"),
        )
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.download_csv("report-2023-01-01").await;

    assert!(result.is_ok());
    let csv_content = result.unwrap();
    assert!(csv_content.contains("report_id,timestamp,database_name"));
    assert!(csv_content.contains("redis-db-1"));
    assert!(csv_content.contains("redis-db-2"));
}

#[tokio::test]
async fn test_usage_report_download_csv_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/nonexistent/csv"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Usage report not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.download_csv("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_usage_report_download_csv_generation_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/usage_report/report-2023-01-01/csv"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(500, "Failed to generate CSV"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UsageReportHandler::new(client);
    let result = handler.download_csv("report-2023-01-01").await;

    assert!(result.is_err());
}
