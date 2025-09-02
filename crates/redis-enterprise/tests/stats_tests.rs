//! Statistics endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, StatsHandler, StatsQuery};
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

fn test_cluster_stats() -> serde_json::Value {
    json!({
        "intervals": [
            {
                "time": "2023-01-01T12:00:00Z",
                "metrics": {
                    "cpu_usage": 25.5,
                    "memory_usage": 75.2,
                    "network_in": 1024000,
                    "network_out": 2048000,
                    "total_req": 150000,
                    "active_databases": 5
                }
            },
            {
                "time": "2023-01-01T12:01:00Z",
                "metrics": {
                    "cpu_usage": 27.1,
                    "memory_usage": 76.8,
                    "network_in": 1100000,
                    "network_out": 2200000,
                    "total_req": 155000,
                    "active_databases": 5
                }
            }
        ]
    })
}

fn test_node_stats() -> serde_json::Value {
    json!({
        "intervals": [
            {
                "time": "2023-01-01T12:00:00Z",
                "metrics": {
                    "cpu_user": 15.5,
                    "cpu_system": 5.2,
                    "cpu_idle": 79.3,
                    "free_memory": 4294967296u64,
                    "network_bytes_in": 512000,
                    "network_bytes_out": 1024000
                }
            }
        ]
    })
}

fn test_database_stats() -> serde_json::Value {
    json!({
        "intervals": [
            {
                "time": "2023-01-01T12:00:00Z",
                "metrics": {
                    "used_memory": 1048576,
                    "total_req": 5000,
                    "ops_per_sec": 100.5,
                    "hits": 4500,
                    "misses": 500,
                    "evicted_objects": 0
                }
            }
        ]
    })
}

fn test_shard_stats() -> serde_json::Value {
    json!({
        "intervals": [
            {
                "time": "2023-01-01T12:00:00Z",
                "metrics": {
                    "used_memory": 524288,
                    "total_req": 2500,
                    "ops_per_sec": 50.0,
                    "keyspace_hits": 2250,
                    "keyspace_misses": 250
                }
            }
        ]
    })
}

fn test_cluster_last_stats() -> serde_json::Value {
    json!({
        "time": "2023-01-01T12:02:00Z",
        "metrics": {
            "cpu_usage": 28.3,
            "memory_usage": 77.5,
            "network_in": 1150000,
            "network_out": 2300000,
            "total_req": 158000
        }
    })
}

fn test_node_last_stats() -> serde_json::Value {
    json!({
        "time": "2023-01-01T12:02:00Z",
        "metrics": {
            "cpu_user": 16.2,
            "cpu_system": 5.8,
            "cpu_idle": 78.0,
            "free_memory": 4194304000u64,
            "network_bytes_in": 520000,
            "network_bytes_out": 1040000
        }
    })
}

fn test_database_last_stats() -> serde_json::Value {
    json!({
        "time": "2023-01-01T12:02:00Z",
        "metrics": {
            "used_memory": 1100000,
            "total_req": 5200,
            "ops_per_sec": 105.2,
            "hits": 4680,
            "misses": 520
        }
    })
}

#[tokio::test]
async fn test_stats_cluster() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cluster_stats()))
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
    let stats = result.unwrap();
    assert_eq!(stats.intervals.len(), 2);
    assert_eq!(stats.intervals[0].time, "2023-01-01T12:00:00Z");
    assert_eq!(stats.intervals[0].metrics["cpu_usage"], 25.5);
}

#[tokio::test]
async fn test_stats_cluster_with_query() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats"))
        .and(query_param("interval", "5min"))
        .and(query_param("metrics", "cpu_usage,memory_usage"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cluster_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let query = StatsQuery {
        interval: Some("5min".to_string()),
        stime: None,
        etime: None,
        metrics: Some("cpu_usage,memory_usage".to_string()),
    };
    let result = handler.cluster(Some(query)).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.intervals.len(), 2);
}

#[tokio::test]
async fn test_stats_cluster_last() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/stats/last"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cluster_last_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let stats = handler.cluster_last().await.unwrap();
    assert_eq!(stats.metrics["cpu_usage"], 28.3);
    assert_eq!(stats.metrics["memory_usage"], 77.5);
}

#[tokio::test]
async fn test_stats_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_node_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let result = handler.node(1, None).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.intervals.len(), 1);
    assert_eq!(stats.intervals[0].time, "2023-01-01T12:00:00Z");
    assert_eq!(stats.intervals[0].metrics["cpu_user"], 15.5);
}

#[tokio::test]
async fn test_stats_node_with_query() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/stats"))
        .and(query_param("interval", "1hour"))
        .and(query_param("stime", "2023-01-01T10:00:00Z"))
        .and(query_param("etime", "2023-01-01T14:00:00Z"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_node_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let query = StatsQuery {
        interval: Some("1hour".to_string()),
        stime: Some("2023-01-01T10:00:00Z".to_string()),
        etime: Some("2023-01-01T14:00:00Z".to_string()),
        metrics: None,
    };
    let result = handler.node(1, Some(query)).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.intervals.len(), 1);
}

#[tokio::test]
async fn test_stats_node_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/stats"))
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

    let handler = StatsHandler::new(client);
    let result = handler.node(999, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_stats_node_last() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/stats/last"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_node_last_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let stats = handler.node_last(1).await.unwrap();
    assert_eq!(stats.metrics["cpu_user"], 16.2);
    assert_eq!(stats.metrics["free_memory"], 4194304000u64);
}

#[tokio::test]
async fn test_stats_node_last_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/stats/last"))
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

    let handler = StatsHandler::new(client);
    let result = handler.node_last(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_stats_nodes() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "stats": [
                {"uid": 1, "intervals": []},
                {"uid": 2, "intervals": []}
            ]
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
    let stats = handler.nodes(None).await.unwrap();
    assert_eq!(stats.stats.len(), 2);
    assert_eq!(stats.stats[0].uid, 1);
    assert_eq!(stats.stats[1].uid, 2);
}

#[tokio::test]
async fn test_stats_nodes_with_query() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/stats"))
        .and(query_param("interval", "1min"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"stats": []})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let query = StatsQuery {
        interval: Some("1min".to_string()),
        stime: None,
        etime: None,
        metrics: None,
    };
    let stats = handler.nodes(Some(query)).await.unwrap();
    assert!(stats.stats.is_empty());
}

#[tokio::test]
async fn test_stats_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_database_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let result = handler.database(1, None).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.intervals.len(), 1);
    assert_eq!(stats.intervals[0].metrics["used_memory"], 1048576);
    assert_eq!(stats.intervals[0].metrics["ops_per_sec"], 100.5);
}

#[tokio::test]
async fn test_stats_database_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999/stats"))
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

    let handler = StatsHandler::new(client);
    let result = handler.database(999, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_stats_database_last() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/stats/last"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_database_last_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let stats = handler.database_last(1).await.unwrap();
    assert_eq!(stats.metrics["used_memory"], 1100000);
    assert_eq!(stats.metrics["ops_per_sec"], 105.2);
}

#[tokio::test]
async fn test_stats_databases() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "stats": [
                {"uid": 1, "intervals": []},
                {"uid": 2, "intervals": []}
            ]
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
    let stats = handler.databases(None).await.unwrap();
    assert_eq!(stats.stats.len(), 2);
    assert_eq!(stats.stats[0].uid, 1);
    assert_eq!(stats.stats[1].uid, 2);
}

#[tokio::test]
async fn test_stats_shard() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_shard_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let result = handler.shard(1, None).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.intervals.len(), 1);
    assert_eq!(stats.intervals[0].metrics["used_memory"], 524288);
    assert_eq!(stats.intervals[0].metrics["ops_per_sec"], 50.0);
}

#[tokio::test]
async fn test_stats_shard_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/999/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Shard not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = StatsHandler::new(client);
    let result = handler.shard(999, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_stats_shards() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "stats": [
                {"uid": 1, "intervals": []},
                {"uid": 2, "intervals": []}
            ]
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
    let result = handler.shards(None).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.stats.len(), 2);
    assert_eq!(stats.stats[0].uid, 1);
    assert_eq!(stats.stats[1].uid, 2);
}
