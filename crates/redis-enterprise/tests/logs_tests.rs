//! Logs endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, LogsHandler, LogsQuery};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path, query_param};
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

fn test_log_entry() -> serde_json::Value {
    json!({
        "time": "2023-01-01T12:00:00Z",
        "type": "database_backup_completed",
        "message": "Database backup completed successfully",
        "node_uid": 1,
        "bdb_uid": 1,
        "user": "admin"
    })
}

fn test_warning_log() -> serde_json::Value {
    json!({
        "time": "2023-01-01T12:01:00Z",
        "type": "high_memory_usage",
        "message": "High memory usage detected on node 2",
        "node_uid": 2,
        "user": "system"
    })
}

fn test_error_log() -> serde_json::Value {
    json!({
        "time": "2023-01-01T12:02:00Z",
        "type": "network_error",
        "message": "Connection timeout to node 3",
        "node_uid": 3,
        "user": "monitor"
    })
}

// Tests
#[tokio::test]
async fn test_list_logs() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    let response_body = json!([test_log_entry(), test_warning_log(), test_error_log()]);

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(response_body))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].time, "2023-01-01T12:00:00Z");
    assert_eq!(logs[0].event_type, "database_backup_completed");
    assert_eq!(logs[1].event_type, "high_memory_usage");
    assert_eq!(logs[2].event_type, "network_error");
}

#[tokio::test]
async fn test_list_logs_empty_response() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 0);
}

#[tokio::test]
async fn test_list_logs_with_limit() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    let response_body = json!([test_log_entry()]);

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("limit", "10"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(response_body))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        stime: None,
        etime: None,
        order: None,
        limit: Some(10),
        offset: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok(), "Failed to list logs: {:?}", result.err());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
}

#[tokio::test]
async fn test_list_logs_with_offset() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    let response_body = json!([test_warning_log()]);

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("offset", "20"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(response_body))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        stime: None,
        etime: None,
        order: None,
        limit: None,
        offset: Some(20),
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
}

#[tokio::test]
async fn test_list_logs_with_time_filter() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    let response_body = json!([test_error_log()]);

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("stime", "2023-01-01T00:00:00Z"))
        .and(query_param("etime", "2023-01-02T00:00:00Z"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(response_body))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        stime: Some("2023-01-01T00:00:00Z".to_string()),
        etime: Some("2023-01-02T00:00:00Z".to_string()),
        order: None,
        limit: None,
        offset: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].event_type, "network_error");
}

#[tokio::test]
async fn test_list_logs_with_order() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    let response_body = json!([test_error_log(), test_warning_log(), test_log_entry()]);

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("order", "desc"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(response_body))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        stime: None,
        etime: None,
        order: Some("desc".to_string()),
        limit: None,
        offset: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 3);
    // In descending order
    assert_eq!(logs[0].event_type, "network_error");
    assert_eq!(logs[1].event_type, "high_memory_usage");
    assert_eq!(logs[2].event_type, "database_backup_completed");
}

#[tokio::test]
async fn test_list_logs_with_combined_filters() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    let response_body = json!([test_warning_log()]);

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("limit", "50"))
        .and(query_param("offset", "10"))
        .and(query_param("order", "asc"))
        .and(basic_auth("admin", "password123"))
        .respond_with(success_response(response_body))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        stime: None,
        etime: None,
        order: Some("asc".to_string()),
        limit: Some(50),
        offset: Some(10),
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs[0].event_type, "high_memory_usage");
}

#[tokio::test]
async fn test_list_logs_error_response() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("password123")
        .build()
        .expect("Failed to create test client");

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(basic_auth("admin", "password123"))
        .respond_with(error_response(403, "Insufficient permissions"))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("403"));
}

#[tokio::test]
async fn test_list_logs_authentication_error() {
    let mock_server = MockServer::start().await;
    let mock_url = mock_server.uri();

    let client = EnterpriseClient::builder()
        .base_url(&mock_url)
        .username("admin")
        .password("wrong_password")
        .build()
        .expect("Failed to create test client");

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .respond_with(error_response(401, "Authentication failed"))
        .mount(&mock_server)
        .await;

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_err());
}
