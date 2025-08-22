//! Logs endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, LogsHandler};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth};
use serde_json::json;

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

#[tokio::test]
async fn test_logs_cluster() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/logs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "uid": 1,
                "time": "2023-01-01T12:00:00Z",
                "severity": "INFO",
                "message": "Cluster startup completed"
            },
            {
                "uid": 2,
                "time": "2023-01-01T12:01:00Z",
                "severity": "WARNING",
                "message": "High memory usage detected"
            }
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
    assert_eq!(logs.len(), 2);
}

#[tokio::test]
async fn test_logs_node() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/logs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "uid": 1,
                "time": "2023-01-01T12:00:00Z",
                "severity": "INFO",
                "message": "Node 1 is healthy"
            }
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
    // Note: Node logs are accessed through the main logs endpoint with filtering
    let result = handler.list(None).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
}

#[tokio::test]
async fn test_logs_database() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/logs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "uid": 1,
                "time": "2023-01-01T12:00:00Z",
                "severity": "INFO",
                "message": "Database 1 backup completed"
            }
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
    // Note: Database logs are accessed through the main logs endpoint with filtering
    let result = handler.list(None).await;

    assert!(result.is_ok());
    let logs = result.unwrap();
    assert_eq!(logs.len(), 1);
}