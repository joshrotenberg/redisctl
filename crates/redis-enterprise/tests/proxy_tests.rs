//! Proxy endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, ProxyHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn active_proxy() -> serde_json::Value {
    json!({
        "uid": 1,
        "bdb_uid": 1,
        "node_uid": 1,
        "status": "active",
        "addr": "10.0.0.1",
        "port": 12000,
        "max_connections": 1000,
        "threads": 4
    })
}

fn standby_proxy() -> serde_json::Value {
    json!({
        "uid": 2,
        "bdb_uid": 1,
        "node_uid": 2,
        "status": "standby",
        "addr": "10.0.0.2",
        "port": 12001,
        "max_connections": 1000,
        "threads": 4
    })
}

fn minimal_proxy() -> serde_json::Value {
    json!({
        "uid": 3,
        "bdb_uid": 2,
        "node_uid": 1,
        "status": "active"
    })
}

fn proxy_stats() -> serde_json::Value {
    json!({
        "uid": 1,
        "intervals": [
            {
                "interval": "1sec",
                "timestamps": [1640995200, 1640995260, 1640995320],
                "values": [
                    {"connections": 25, "ops_per_sec": 150.5},
                    {"connections": 30, "ops_per_sec": 175.2},
                    {"connections": 28, "ops_per_sec": 160.8}
                ]
            },
            {
                "interval": "1min",
                "timestamps": [1640995200, 1640995260],
                "values": [
                    {"connections": 27, "ops_per_sec": 162.0},
                    {"connections": 29, "ops_per_sec": 168.0}
                ]
            }
        ]
    })
}

#[tokio::test]
async fn test_proxy_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            active_proxy(),
            standby_proxy(),
            minimal_proxy()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let proxies = result.unwrap();
    assert_eq!(proxies.len(), 3);

    // Verify active proxy details
    let active = &proxies[0];
    assert_eq!(active.uid, 1);
    assert_eq!(active.bdb_uid, 1);
    assert_eq!(active.node_uid, 1);
    assert_eq!(active.status, "active");
    assert_eq!(active.addr, Some("10.0.0.1".to_string()));
    assert_eq!(active.port, Some(12000));
    assert_eq!(active.max_connections, Some(1000));
    assert_eq!(active.threads, Some(4));

    // Verify standby proxy
    let standby = &proxies[1];
    assert_eq!(standby.uid, 2);
    assert_eq!(standby.status, "standby");
    assert_eq!(standby.addr, Some("10.0.0.2".to_string()));
    assert_eq!(standby.port, Some(12001));

    // Verify minimal proxy (no optional fields)
    let minimal = &proxies[2];
    assert_eq!(minimal.uid, 3);
    assert_eq!(minimal.bdb_uid, 2);
    assert_eq!(minimal.status, "active");
    assert!(minimal.addr.is_none());
    assert!(minimal.port.is_none());
    assert!(minimal.max_connections.is_none());
    assert!(minimal.threads.is_none());
}

#[tokio::test]
async fn test_proxy_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies"))
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

    let handler = ProxyHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let proxies = result.unwrap();
    assert_eq!(proxies.len(), 0);
}

#[tokio::test]
async fn test_proxy_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(active_proxy()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_ok());
    let proxy = result.unwrap();
    assert_eq!(proxy.uid, 1);
    assert_eq!(proxy.bdb_uid, 1);
    assert_eq!(proxy.node_uid, 1);
    assert_eq!(proxy.status, "active");
    assert_eq!(proxy.addr, Some("10.0.0.1".to_string()));
    assert_eq!(proxy.port, Some(12000));
    assert_eq!(proxy.max_connections, Some(1000));
    assert_eq!(proxy.threads, Some(4));
}

#[tokio::test]
async fn test_proxy_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/3"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(minimal_proxy()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.get(3).await;

    assert!(result.is_ok());
    let proxy = result.unwrap();
    assert_eq!(proxy.uid, 3);
    assert_eq!(proxy.bdb_uid, 2);
    assert_eq!(proxy.node_uid, 1);
    assert_eq!(proxy.status, "active");
    assert!(proxy.addr.is_none());
    assert!(proxy.port.is_none());
}

#[tokio::test]
async fn test_proxy_stats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(proxy_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.stats(1).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.uid, 1);
    assert_eq!(stats.intervals.len(), 2);

    // Verify 1sec interval
    let sec_interval = &stats.intervals[0];
    assert_eq!(sec_interval.interval, "1sec");
    assert_eq!(sec_interval.timestamps.len(), 3);
    assert_eq!(sec_interval.values.len(), 3);
    assert_eq!(sec_interval.timestamps[0], 1640995200);

    // Verify 1min interval
    let min_interval = &stats.intervals[1];
    assert_eq!(min_interval.interval, "1min");
    assert_eq!(min_interval.timestamps.len(), 2);
    assert_eq!(min_interval.values.len(), 2);
}

#[tokio::test]
async fn test_proxy_stats_metric() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/1/stats/connections"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "interval": "1sec",
            "timestamps": [1640995200, 1640995260, 1640995320],
            "values": [25, 30, 28]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let metric_stats = handler.stats_metric(1, "connections").await.unwrap();
    assert_eq!(metric_stats.interval, "1sec");
    assert_eq!(metric_stats.timestamps.len(), 3);
    assert_eq!(metric_stats.values.len(), 3);
    assert_eq!(metric_stats.values[0], 25);
    assert_eq!(metric_stats.values[1], 30);
    assert_eq!(metric_stats.values[2], 28);
}

#[tokio::test]
async fn test_proxy_stats_metric_ops() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/2/stats/ops_per_sec"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "interval": "1min",
            "timestamps": [1640995200, 1640995260],
            "values": [162.0, 168.0]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let metric_stats = handler.stats_metric(2, "ops_per_sec").await.unwrap();
    assert_eq!(metric_stats.interval, "1min");
    assert_eq!(metric_stats.values[0], 162.0);
    assert_eq!(metric_stats.values[1], 168.0);
}

#[tokio::test]
async fn test_proxy_list_by_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/proxies"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([active_proxy(), standby_proxy()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.list_by_database(1).await;

    assert!(result.is_ok());
    let proxies = result.unwrap();
    assert_eq!(proxies.len(), 2);

    // Both proxies should belong to database 1
    assert_eq!(proxies[0].bdb_uid, 1);
    assert_eq!(proxies[1].bdb_uid, 1);

    // Verify active and standby status
    assert_eq!(proxies[0].status, "active");
    assert_eq!(proxies[1].status, "standby");
}

#[tokio::test]
async fn test_proxy_list_by_database_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999/proxies"))
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

    let handler = ProxyHandler::new(client);
    let result = handler.list_by_database(999).await;

    assert!(result.is_ok());
    let proxies = result.unwrap();
    assert_eq!(proxies.len(), 0);
}

#[tokio::test]
async fn test_proxy_list_by_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/proxies"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([active_proxy(), minimal_proxy()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.list_by_node(1).await;

    assert!(result.is_ok());
    let proxies = result.unwrap();
    assert_eq!(proxies.len(), 2);

    // Both proxies should be on node 1
    assert_eq!(proxies[0].node_uid, 1);
    assert_eq!(proxies[1].node_uid, 1);

    // Verify different databases
    assert_eq!(proxies[0].bdb_uid, 1);
    assert_eq!(proxies[1].bdb_uid, 2);
}

#[tokio::test]
async fn test_proxy_list_by_node_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/proxies"))
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

    let handler = ProxyHandler::new(client);
    let result = handler.list_by_node(999).await;

    assert!(result.is_ok());
    let proxies = result.unwrap();
    assert_eq!(proxies.len(), 0);
}

#[tokio::test]
async fn test_proxy_reload() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/proxies/1/actions/reload"))
        .and(basic_auth("admin", "password"))
        .and(wiremock::matchers::body_json(json!(null)))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.reload(1).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_proxy_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Proxy not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_proxy_stats_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/999/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Proxy not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.stats(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_proxy_stats_metric_invalid() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/proxies/1/stats/invalid_metric"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid metric name",
            "code": "INVALID_METRIC"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.stats_metric(1, "invalid_metric").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_proxy_reload_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/proxies/999/actions/reload"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Proxy not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ProxyHandler::new(client);
    let result = handler.reload(999).await;

    assert!(result.is_err());
}
