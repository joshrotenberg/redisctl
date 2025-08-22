//! Alerts endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, AlertHandler};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth};
use serde_json::json;

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}


#[tokio::test]
async fn test_alerts_list() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/alerts"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "uid": "1",
                "severity": "high",
                "state": "active",
                "name": "node_memory_high",
                "description": "Node memory usage is high"
            },
            {
                "uid": "2",
                "severity": "medium",
                "state": "resolved",
                "name": "database_latency",
                "description": "Database latency is elevated"
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

    let handler = AlertHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let alerts = result.unwrap();
    assert_eq!(alerts.len(), 2);
}

#[tokio::test]
async fn test_alerts_get() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/alerts/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "uid": "1",
            "severity": "high",
            "state": "active",
            "name": "node_memory_high",
            "description": "Node memory usage is high",
            "timestamp": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = AlertHandler::new(client);
    let result = handler.get("1").await;

    assert!(result.is_ok());
    let alert = result.unwrap();
    assert_eq!(alert.uid, "1");
    assert_eq!(alert.severity, "high");
}

#[tokio::test]
async fn test_alerts_get_settings() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/cluster/alert_settings/node_memory_high"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "enabled": true,
            "threshold": {"value": 80, "unit": "percent"}
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = AlertHandler::new(client);
    let result = handler.get_settings("node_memory_high").await;

    assert!(result.is_ok());
    let settings = result.unwrap();
    assert_eq!(settings.enabled, true);
}