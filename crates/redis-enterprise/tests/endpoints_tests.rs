//! Endpoints tests for Redis Enterprise

use redis_enterprise::{EndpointsHandler, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
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

fn test_endpoint() -> serde_json::Value {
    json!({
        "uid": "endpoint-1",
        "bdb_uid": 1,
        "node_uid": 1,
        "addr": "192.168.1.10",
        "port": 12000,
        "dns_name": "db1.cluster.local",
        "role": "master",
        "ssl": true,
        "status": "active"
    })
}

fn test_endpoint_minimal() -> serde_json::Value {
    json!({
        "uid": "endpoint-2",
        "bdb_uid": 2,
        "node_uid": 2,
        "addr": "192.168.1.11",
        "port": 12001
    })
}

fn test_endpoint_replica() -> serde_json::Value {
    json!({
        "uid": "endpoint-3",
        "bdb_uid": 1,
        "node_uid": 2,
        "addr": "192.168.1.11",
        "port": 12002,
        "dns_name": "db1-replica.cluster.local",
        "role": "replica",
        "ssl": false,
        "status": "active"
    })
}

fn test_endpoint_stats_data() -> serde_json::Value {
    json!({
        "uid": "endpoint-1",
        "intervals": [
            {
                "interval": "1sec",
                "timestamps": [1640995200, 1640995260, 1640995320],
                "values": [
                    {"ops_per_sec": 1000, "hits_per_sec": 800},
                    {"ops_per_sec": 1100, "hits_per_sec": 850},
                    {"ops_per_sec": 1050, "hits_per_sec": 820}
                ]
            },
            {
                "interval": "1hour",
                "timestamps": [1640991600, 1640995200],
                "values": [
                    {"ops_per_sec": 950, "hits_per_sec": 750},
                    {"ops_per_sec": 1050, "hits_per_sec": 820}
                ]
            }
        ]
    })
}

fn test_endpoint_stats_minimal_data() -> serde_json::Value {
    json!({
        "uid": "endpoint-2",
        "intervals": []
    })
}

#[tokio::test]
async fn test_endpoints_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_endpoint(),
            test_endpoint_minimal(),
            test_endpoint_replica()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 3);

    // Check first endpoint (full)
    assert_eq!(endpoints[0].uid, "endpoint-1");
    assert_eq!(endpoints[0].bdb_uid, 1);
    assert_eq!(endpoints[0].node_uid, 1);
    assert_eq!(endpoints[0].addr, "192.168.1.10");
    assert_eq!(endpoints[0].port, 12000);
    assert_eq!(endpoints[0].dns_name, Some("db1.cluster.local".to_string()));
    assert_eq!(endpoints[0].role, Some("master".to_string()));
    assert_eq!(endpoints[0].ssl, Some(true));
    assert_eq!(endpoints[0].status, Some("active".to_string()));

    // Check second endpoint (minimal)
    assert_eq!(endpoints[1].uid, "endpoint-2");
    assert_eq!(endpoints[1].bdb_uid, 2);
    assert_eq!(endpoints[1].node_uid, 2);
    assert_eq!(endpoints[1].addr, "192.168.1.11");
    assert_eq!(endpoints[1].port, 12001);
    assert!(endpoints[1].dns_name.is_none());
    assert!(endpoints[1].role.is_none());
    assert!(endpoints[1].ssl.is_none());
    assert!(endpoints[1].status.is_none());

    // Check third endpoint (replica)
    assert_eq!(endpoints[2].uid, "endpoint-3");
    assert_eq!(endpoints[2].role, Some("replica".to_string()));
    assert_eq!(endpoints[2].ssl, Some(false));
}

#[tokio::test]
async fn test_endpoints_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints"))
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

    let handler = EndpointsHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 0);
}

#[tokio::test]
async fn test_endpoint_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/endpoint-1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_endpoint()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.get("endpoint-1").await;

    assert!(result.is_ok());
    let endpoint = result.unwrap();
    assert_eq!(endpoint.uid, "endpoint-1");
    assert_eq!(endpoint.bdb_uid, 1);
    assert_eq!(endpoint.node_uid, 1);
    assert_eq!(endpoint.addr, "192.168.1.10");
    assert_eq!(endpoint.port, 12000);
    assert_eq!(endpoint.dns_name, Some("db1.cluster.local".to_string()));
    assert_eq!(endpoint.role, Some("master".to_string()));
    assert_eq!(endpoint.ssl, Some(true));
    assert_eq!(endpoint.status, Some("active".to_string()));
}

#[tokio::test]
async fn test_endpoint_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/endpoint-2"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_endpoint_minimal()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.get("endpoint-2").await;

    assert!(result.is_ok());
    let endpoint = result.unwrap();
    assert_eq!(endpoint.uid, "endpoint-2");
    assert_eq!(endpoint.bdb_uid, 2);
    assert_eq!(endpoint.node_uid, 2);
    assert_eq!(endpoint.addr, "192.168.1.11");
    assert_eq!(endpoint.port, 12001);
    assert!(endpoint.dns_name.is_none());
    assert!(endpoint.role.is_none());
    assert!(endpoint.ssl.is_none());
    assert!(endpoint.status.is_none());
}

#[tokio::test]
async fn test_endpoint_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Endpoint not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_endpoint_stats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/endpoint-1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_endpoint_stats_data()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.stats("endpoint-1").await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.uid, "endpoint-1");
    assert_eq!(stats.intervals.len(), 2);

    // Check first interval
    assert_eq!(stats.intervals[0].interval, "1sec");
    assert_eq!(stats.intervals[0].timestamps.len(), 3);
    assert_eq!(stats.intervals[0].values.len(), 3);

    // Check second interval
    assert_eq!(stats.intervals[1].interval, "1hour");
    assert_eq!(stats.intervals[1].timestamps.len(), 2);
    assert_eq!(stats.intervals[1].values.len(), 2);
}

#[tokio::test]
async fn test_endpoint_stats_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/endpoint-2/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_endpoint_stats_minimal_data()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.stats("endpoint-2").await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.uid, "endpoint-2");
    assert_eq!(stats.intervals.len(), 0);
}

#[tokio::test]
async fn test_endpoint_stats_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/nonexistent/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Endpoint not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.stats("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_endpoints_all_stats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_endpoint_stats_data(),
            test_endpoint_stats_minimal_data()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.all_stats().await;

    assert!(result.is_ok());
    let all_stats = result.unwrap();
    assert_eq!(all_stats.len(), 2);

    // Check first endpoint stats
    assert_eq!(all_stats[0].uid, "endpoint-1");
    assert_eq!(all_stats[0].intervals.len(), 2);

    // Check second endpoint stats
    assert_eq!(all_stats[1].uid, "endpoint-2");
    assert_eq!(all_stats[1].intervals.len(), 0);
}

#[tokio::test]
async fn test_endpoints_all_stats_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/endpoints/stats"))
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

    let handler = EndpointsHandler::new(client);
    let result = handler.all_stats().await;

    assert!(result.is_ok());
    let all_stats = result.unwrap();
    assert_eq!(all_stats.len(), 0);
}

#[tokio::test]
async fn test_endpoints_list_by_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/endpoints"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_endpoint(),
            test_endpoint_replica()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_database(1).await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 2);

    // Both endpoints should belong to database 1
    assert_eq!(endpoints[0].bdb_uid, 1);
    assert_eq!(endpoints[0].uid, "endpoint-1");
    assert_eq!(endpoints[0].role, Some("master".to_string()));

    assert_eq!(endpoints[1].bdb_uid, 1);
    assert_eq!(endpoints[1].uid, "endpoint-3");
    assert_eq!(endpoints[1].role, Some("replica".to_string()));
}

#[tokio::test]
async fn test_endpoints_list_by_database_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999/endpoints"))
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

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_database(999).await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 0);
}

#[tokio::test]
async fn test_endpoints_list_by_database_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999/endpoints"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Database not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_database(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_endpoints_list_by_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/endpoints"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_endpoint()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_node(1).await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 1);

    // Endpoint should belong to node 1
    assert_eq!(endpoints[0].node_uid, 1);
    assert_eq!(endpoints[0].uid, "endpoint-1");
    assert_eq!(endpoints[0].addr, "192.168.1.10");
}

#[tokio::test]
async fn test_endpoints_list_by_node_multiple() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/2/endpoints"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_endpoint_minimal(),
            test_endpoint_replica()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_node(2).await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 2);

    // Both endpoints should belong to node 2
    assert_eq!(endpoints[0].node_uid, 2);
    assert_eq!(endpoints[0].uid, "endpoint-2");

    assert_eq!(endpoints[1].node_uid, 2);
    assert_eq!(endpoints[1].uid, "endpoint-3");
}

#[tokio::test]
async fn test_endpoints_list_by_node_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/endpoints"))
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

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_node(999).await;

    assert!(result.is_ok());
    let endpoints = result.unwrap();
    assert_eq!(endpoints.len(), 0);
}

#[tokio::test]
async fn test_endpoints_list_by_node_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/endpoints"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Node not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = EndpointsHandler::new(client);
    let result = handler.list_by_node(999).await;

    assert!(result.is_err());
}
