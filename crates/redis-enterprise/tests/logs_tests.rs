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
        "id": 123456,
        "time": "2023-01-01T12:00:00Z",
        "level": "INFO",
        "component": "database",
        "message": "Database backup completed successfully",
        "node_uid": 1,
        "bdb_uid": 1,
        "user": "admin"
    })
}

fn test_warning_log() -> serde_json::Value {
    json!({
        "id": 123457,
        "time": "2023-01-01T12:01:00Z",
        "level": "WARNING",
        "component": "cluster",
        "message": "High memory usage detected on node 2",
        "node_uid": 2,
        "user": "system"
    })
}

fn test_error_log() -> serde_json::Value {
    json!({
        "id": 123458,
        "time": "2023-01-01T12:02:00Z",
        "level": "ERROR",
        "component": "network",
        "message": "Connection timeout to node 3",
        "node_uid": 3,
        "user": "monitor"
    })
}

#[tokio::test]
async fn test_logs_list_all() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_log_entry(),
            test_warning_log(),
            test_error_log()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].id, 123456);
    assert_eq!(logs[0].level, "INFO");
    assert_eq!(logs[1].level, "WARNING");
    assert_eq!(logs[2].level, "ERROR");
}

#[tokio::test]
async fn test_logs_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
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

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 0);
}

#[tokio::test]
async fn test_logs_list_with_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("limit", "10"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_log_entry()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: Some(10),
        offset: None,
        level: None,
        component: None,
        node_uid: None,
        bdb_uid: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
}

#[tokio::test]
async fn test_logs_list_with_offset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("offset", "20"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_warning_log()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: None,
        offset: Some(20),
        level: None,
        component: None,
        node_uid: None,
        bdb_uid: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
}

#[tokio::test]
async fn test_logs_list_filter_by_level() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("level", "ERROR"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_error_log()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: None,
        offset: None,
        level: Some("ERROR".to_string()),
        component: None,
        node_uid: None,
        bdb_uid: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].level, "ERROR");
}

#[tokio::test]
async fn test_logs_list_filter_by_component() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("component", "database"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_log_entry()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: None,
        offset: None,
        level: None,
        component: Some("database".to_string()),
        node_uid: None,
        bdb_uid: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].component.as_ref().unwrap(), "database");
}

#[tokio::test]
async fn test_logs_list_filter_by_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("node_uid", "1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_log_entry()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: None,
        offset: None,
        level: None,
        component: None,
        node_uid: Some(1),
        bdb_uid: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].node_uid.unwrap(), 1);
}

#[tokio::test]
async fn test_logs_list_filter_by_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("bdb_uid", "1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_log_entry()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: None,
        offset: None,
        level: None,
        component: None,
        node_uid: None,
        bdb_uid: Some(1),
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].bdb_uid.unwrap(), 1);
}

#[tokio::test]
async fn test_logs_list_complex_query() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(query_param("limit", "50"))
        .and(query_param("offset", "10"))
        .and(query_param("level", "WARNING"))
        .and(query_param("node_uid", "2"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_warning_log()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let query = LogsQuery {
        limit: Some(50),
        offset: Some(10),
        level: Some("WARNING".to_string()),
        component: None,
        node_uid: Some(2),
        bdb_uid: None,
    };
    let result = handler.list(Some(query)).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].level, "WARNING");
    assert_eq!(logs[0].node_uid.unwrap(), 2);
}

#[tokio::test]
async fn test_logs_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs/123456"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_log_entry()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let result = handler.get(123456).await;

    assert!(result.is_ok());
    let log = result.unwrap();
    assert_eq!(log.id, 123456);
    assert_eq!(log.level, "INFO");
    assert_eq!(log.component.unwrap(), "database");
    assert_eq!(log.message, "Database backup completed successfully");
    assert_eq!(log.node_uid.unwrap(), 1);
    assert_eq!(log.bdb_uid.unwrap(), 1);
    assert_eq!(log.user.unwrap(), "admin");
}

#[tokio::test]
async fn test_logs_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs/999999"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Log entry not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let result = handler.get(999999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_logs_list_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(500, "Internal server error"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LogsHandler::new(client);
    let result = handler.list(None).await;

    assert!(result.is_err());
}
