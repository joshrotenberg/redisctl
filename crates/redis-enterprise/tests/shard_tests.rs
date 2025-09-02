//! Shard endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, ShardHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn master_shard() -> serde_json::Value {
    json!({
        "uid": "shard:1:1",
        "bdb_uid": 1,
        "node_uid": 1,
        "role": "master",
        "status": "active",
        "slots": "0-8191",
        "used_memory": 1048576,
        "backup_progress": 100.0,
        "import_progress": null
    })
}

fn replica_shard() -> serde_json::Value {
    json!({
        "uid": "shard:1:2",
        "bdb_uid": 1,
        "node_uid": 2,
        "role": "slave",
        "status": "active",
        "slots": "0-8191",
        "used_memory": 1048576
    })
}

fn backup_shard() -> serde_json::Value {
    json!({
        "uid": "shard:2:1",
        "bdb_uid": 2,
        "node_uid": 1,
        "role": "master",
        "status": "backing-up",
        "slots": "8192-16383",
        "used_memory": 2097152,
        "backup_progress": 45.5
    })
}

fn importing_shard() -> serde_json::Value {
    json!({
        "uid": "shard:3:1",
        "bdb_uid": 3,
        "node_uid": 3,
        "role": "master",
        "status": "importing",
        "slots": "0-16383",
        "used_memory": 512000,
        "import_progress": 78.2
    })
}

fn minimal_shard() -> serde_json::Value {
    json!({
        "uid": "shard:4:1",
        "bdb_uid": 4,
        "node_uid": 1,
        "role": "master",
        "status": "active"
    })
}

fn shard_stats() -> serde_json::Value {
    json!({
        "uid": "shard:1:1",
        "intervals": [
            {
                "interval": "1sec",
                "timestamps": [1640995200, 1640995260, 1640995320],
                "values": [
                    {"ops_per_sec": 1250.5, "memory_usage": 1048576},
                    {"ops_per_sec": 1180.2, "memory_usage": 1052000},
                    {"ops_per_sec": 1320.8, "memory_usage": 1045000}
                ]
            },
            {
                "interval": "1min",
                "timestamps": [1640995200, 1640995260],
                "values": [
                    {"ops_per_sec": 1200.0, "memory_usage": 1048576},
                    {"ops_per_sec": 1250.0, "memory_usage": 1050000}
                ]
            }
        ]
    })
}

#[tokio::test]
async fn test_shard_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            master_shard(),
            replica_shard(),
            backup_shard(),
            importing_shard(),
            minimal_shard()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert_eq!(shards.len(), 5);

    // Verify master shard details
    let master = &shards[0];
    assert_eq!(master.uid, "shard:1:1");
    assert_eq!(master.bdb_uid, 1);
    assert_eq!(master.node_uid, 1);
    assert_eq!(master.role, "master");
    assert_eq!(master.status, "active");
    assert_eq!(master.slots, Some("0-8191".to_string()));
    assert_eq!(master.used_memory, Some(1048576));
    assert_eq!(master.backup_progress, Some(100.0));
    assert!(master.import_progress.is_none());

    // Verify replica shard
    let replica = &shards[1];
    assert_eq!(replica.uid, "shard:1:2");
    assert_eq!(replica.role, "slave");
    assert_eq!(replica.status, "active");

    // Verify backup shard
    let backup = &shards[2];
    assert_eq!(backup.uid, "shard:2:1");
    assert_eq!(backup.status, "backing-up");
    assert_eq!(backup.backup_progress, Some(45.5));

    // Verify importing shard
    let importing = &shards[3];
    assert_eq!(importing.uid, "shard:3:1");
    assert_eq!(importing.status, "importing");
    assert_eq!(importing.import_progress, Some(78.2));

    // Verify minimal shard (no optional fields)
    let minimal = &shards[4];
    assert_eq!(minimal.uid, "shard:4:1");
    assert_eq!(minimal.role, "master");
    assert!(minimal.slots.is_none());
    assert!(minimal.used_memory.is_none());
    assert!(minimal.backup_progress.is_none());
    assert!(minimal.import_progress.is_none());
}

#[tokio::test]
async fn test_shard_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards"))
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

    let handler = ShardHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert_eq!(shards.len(), 0);
}

#[tokio::test]
async fn test_shard_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/shard:1:1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(master_shard()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.get("shard:1:1").await;

    assert!(result.is_ok());
    let shard = result.unwrap();
    assert_eq!(shard.uid, "shard:1:1");
    assert_eq!(shard.bdb_uid, 1);
    assert_eq!(shard.node_uid, 1);
    assert_eq!(shard.role, "master");
    assert_eq!(shard.status, "active");
    assert_eq!(shard.slots, Some("0-8191".to_string()));
    assert_eq!(shard.used_memory, Some(1048576));
    assert_eq!(shard.backup_progress, Some(100.0));
}

#[tokio::test]
async fn test_shard_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/shard:4:1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(minimal_shard()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.get("shard:4:1").await;

    assert!(result.is_ok());
    let shard = result.unwrap();
    assert_eq!(shard.uid, "shard:4:1");
    assert_eq!(shard.bdb_uid, 4);
    assert_eq!(shard.node_uid, 1);
    assert_eq!(shard.role, "master");
    assert_eq!(shard.status, "active");
    assert!(shard.slots.is_none());
    assert!(shard.used_memory.is_none());
}

#[tokio::test]
async fn test_shard_stats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/shard:1:1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(shard_stats()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.stats("shard:1:1").await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.uid, "shard:1:1");
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
async fn test_shard_stats_metric() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/shard:1:1/stats/ops_per_sec"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "interval": "1sec",
            "timestamps": [1640995200, 1640995260, 1640995320],
            "values": [1250.5, 1180.2, 1320.8]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.stats_metric_raw("shard:1:1", "ops_per_sec").await;

    assert!(result.is_ok());
    let metric_stats = result.unwrap();
    assert_eq!(metric_stats["interval"], "1sec");
    assert_eq!(metric_stats["timestamps"].as_array().unwrap().len(), 3);
    assert_eq!(metric_stats["values"].as_array().unwrap().len(), 3);
    assert_eq!(metric_stats["values"][0], 1250.5);
}

#[tokio::test]
async fn test_shard_stats_metric_memory() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/shard:2:1/stats/memory_usage"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "interval": "1min",
            "timestamps": [1640995200, 1640995260],
            "values": [1048576, 1052000]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.stats_metric_raw("shard:2:1", "memory_usage").await;

    assert!(result.is_ok());
    let metric_stats = result.unwrap();
    assert_eq!(metric_stats["interval"], "1min");
    assert_eq!(metric_stats["values"][0], 1048576);
    assert_eq!(metric_stats["values"][1], 1052000);
}

#[tokio::test]
async fn test_shard_list_by_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/shards"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([master_shard(), replica_shard()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.list_by_database(1).await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert_eq!(shards.len(), 2);

    // Both shards should belong to database 1
    assert_eq!(shards[0].bdb_uid, 1);
    assert_eq!(shards[1].bdb_uid, 1);

    // Verify master and replica roles
    assert_eq!(shards[0].role, "master");
    assert_eq!(shards[1].role, "slave");
}

#[tokio::test]
async fn test_shard_list_by_database_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999/shards"))
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

    let handler = ShardHandler::new(client);
    let result = handler.list_by_database(999).await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert_eq!(shards.len(), 0);
}

#[tokio::test]
async fn test_shard_list_by_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/shards"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            master_shard(),
            backup_shard(),
            minimal_shard()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.list_by_node(1).await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert_eq!(shards.len(), 3);

    // All shards should be on node 1
    assert_eq!(shards[0].node_uid, 1);
    assert_eq!(shards[1].node_uid, 1);
    assert_eq!(shards[2].node_uid, 1);

    // Verify different databases
    assert_eq!(shards[0].bdb_uid, 1);
    assert_eq!(shards[1].bdb_uid, 2);
    assert_eq!(shards[2].bdb_uid, 4);
}

#[tokio::test]
async fn test_shard_list_by_node_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/shards"))
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

    let handler = ShardHandler::new(client);
    let result = handler.list_by_node(999).await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert_eq!(shards.len(), 0);
}

#[tokio::test]
async fn test_shard_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/nonexistent:shard"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Shard not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.get("nonexistent:shard").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_shard_stats_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/nonexistent:shard/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Shard not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ShardHandler::new(client);
    let result = handler.stats("nonexistent:shard").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_shard_stats_metric_invalid() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/shards/shard:1:1/stats/invalid_metric"))
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

    let handler = ShardHandler::new(client);
    let result = handler
        .stats_metric_raw("shard:1:1", "invalid_metric")
        .await;

    assert!(result.is_err());
}
