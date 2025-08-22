//! Statistics endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, StatsHandler};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth};
use serde_json::json;

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

#[tokio::test]
async fn test_stats_cluster() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "cpu_usage": 25.5,
            "memory_usage": 75.2,
            "network_in": 1024000,
            "network_out": 2048000,
            "total_req": 150000
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let result = handler.cluster(None).await;

    assert!(result.is_ok());
    let _stats = result.unwrap();
    // StatsResponse contains structured data
}

#[tokio::test]
async fn test_stats_nodes() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/nodes/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "node_uid": 1,
                "cpu_usage": 20.0,
                "memory_usage": 60.0
            },
            {
                "node_uid": 2,
                "cpu_usage": 30.0,
                "memory_usage": 80.0
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

    let handler = StatsHandler::new(client);
    let result = handler.nodes(None).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert!(stats.is_array());
}

#[tokio::test]
async fn test_stats_databases() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/bdbs/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "bdb_uid": 1,
                "used_memory": 1048576,
                "total_req": 5000,
                "ops_per_sec": 100.5
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

    let handler = StatsHandler::new(client);
    let result = handler.databases(None).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert!(stats.is_array());
}