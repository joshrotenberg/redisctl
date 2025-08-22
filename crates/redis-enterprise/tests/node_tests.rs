//! Node endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, NodeHandler};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth};
use serde_json::json;

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn test_node() -> serde_json::Value {
    json!({
        "uid": 1,
        "addr": "10.0.0.1",
        "status": "active",
        "role": "master",
        "memory_total": 8589934592u64,
        "memory_available": 4294967296u64
    })
}

#[tokio::test]
async fn test_node_list() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_node(),
            {
                "uid": 2,
                "addr": "10.0.0.2",
                "status": "active",
                "role": "slave",
                "memory_total": 8589934592u64,
                "memory_available": 4294967296u64
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

    let handler = NodeHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let nodes = result.unwrap();
    assert_eq!(nodes.len(), 2);
}

#[tokio::test]
async fn test_node_get() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/nodes/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_node()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.uid, 1);
    assert_eq!(node.address, "10.0.0.1");
}

#[tokio::test]
async fn test_node_stats() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "cpu_usage": 25.5,
            "memory_usage": 50.0,
            "network_in": 1024,
            "network_out": 2048
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.stats(1).await;

    assert!(result.is_ok());
    let _stats = result.unwrap();
    // NodeStats struct would have these fields available
}