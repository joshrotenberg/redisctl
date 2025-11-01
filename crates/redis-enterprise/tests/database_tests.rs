//! Database (BDB) endpoint tests for Redis Enterprise

use redis_enterprise::bdb::CreateDatabaseRequest;
use redis_enterprise::{BdbHandler, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn test_database() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "test-db",
        "type": "redis",
        "memory_size": 1073741824,
        "port": 12000,
        "status": "active",
        "master_persistence": false,
        "data_persistence": "disabled",
        "max_aof_file_size": 322122547200u64,
        "recovery_wait_time": -1,
        "skip_import_analyze": "disabled",
        "sync_dedicated_threads": 5
    })
}

#[tokio::test]
async fn test_database_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_database(),
            {
                "uid": 2,
                "name": "test-db-2",
                "type": "redis",
                "memory_size": 2147483648u64,
                "port": 12001,
                "status": "active"
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

    let handler = BdbHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let databases = result.unwrap();
    assert_eq!(databases.len(), 2);
}

#[tokio::test]
async fn test_database_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_database()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.info(1).await;

    assert!(result.is_ok());
    let db = result.unwrap();
    assert_eq!(db.uid, 1);
    assert_eq!(db.name, "test-db");
}

#[tokio::test]
async fn test_database_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(test_database()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let request_data = CreateDatabaseRequest::builder()
        .name("test-db")
        .memory_size(1073741824)
        .port(12000)
        .build();
    let request = handler.create(request_data).await;

    assert!(request.is_ok());
    let db = request.unwrap();
    assert_eq!(db.uid, 1);
    assert_eq!(db.name, "test-db");
}

#[tokio::test]
async fn test_database_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/bdbs/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_ok());
}

// Database Action Tests

#[tokio::test]
async fn test_database_export() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/actions/export"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"task_id": "export-123"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.export(1, "ftp://backup/db1.rdb").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_import() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/actions/import"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"task_id": "import-456"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.import(1, "ftp://backup/db1.rdb", true).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_backup() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/actions/backup"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"backup_id": "backup-789"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.backup(1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_restore() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/actions/restore"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(
            json!({"action_uid": "act-restore-1", "status": "restored"}),
        ))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.restore(1, Some("backup-789")).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_get_shards() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/shards"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {"shard_id": 1, "role": "master"},
            {"shard_id": 2, "role": "slave"}
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.shards(1).await;

    assert!(result.is_ok());
    let shards = result.unwrap();
    assert!(shards.is_array());
}

#[tokio::test]
async fn test_database_upgrade() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/actions/upgrade"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(
            json!({"action_uid": "act-up-1", "status": "upgraded"}),
        ))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.upgrade(1, "search", "2.0").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_optimize_shards_placement_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/actions/optimize_shards_placement"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"status": "ok"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.optimize_shards_placement(1).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "ok");
}

#[tokio::test]
async fn test_database_recover_post() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/actions/recover"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"action_uid": "act-1"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let result = handler.recover(1).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().action_uid, "act-1");
}

#[tokio::test]
async fn test_database_peer_and_sync_stats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/peer_stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"peers": []})))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/1/syncer_state"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"state": "ok"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let peers = handler.peer_stats(1).await.unwrap();
    assert!(peers["peers"].is_array());

    let state = handler.syncer_state(1).await.unwrap();
    assert_eq!(state["state"], "ok");
}

#[tokio::test]
async fn test_bdbs_alerts_and_crdt_detail() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/alerts"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/crdt_sources/alerts/1/2/high_cpu"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"severity": "critical"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    let all = handler.alerts_all().await.unwrap();
    assert!(all.is_array());

    let detail = handler
        .crdt_source_alert_detail(1, 2, "high_cpu")
        .await
        .unwrap();
    assert_eq!(detail["severity"], "critical");
}

#[tokio::test]
async fn test_passwords_delete_and_reset_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/bdbs/1/passwords"))
        .and(basic_auth("admin", "password"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/v1/bdbs/1/actions/backup_reset_status"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"status": "reset"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);
    handler.passwords_delete(1).await.unwrap();
    let reset = handler.backup_reset_status(1).await.unwrap();
    assert_eq!(reset["status"], "reset");
}

#[tokio::test]
async fn test_database_upgrade_redis_version() {
    let mock_server = MockServer::start().await;

    // Mock data captured from: curl -k -u "admin@redis.local:Redis123!" -X POST \
    //   -H "Content-Type: application/json" -d '{"force_restart": true}' \
    //   https://localhost:9443/v1/bdbs/1/upgrade
    // Real API returns full BDB object with action_uid embedded
    Mock::given(method("POST"))
        .and(path("/v1/bdbs/1/upgrade"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "action_uid": "591d9dcb-ddd7-48a9-a04d-bd5d4d6834d0",
            "uid": 1,
            "name": "test-db",
            "status": "active",
            "redis_version": "7.4",
            "version": "7.4.2",
            "memory_size": 1073741824,
            "type": "redis",
            "replication": false,
            "persistence": "disabled",
            "port": 18367,
            "shards_count": 1,
            "oss_cluster": false
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BdbHandler::new(client);

    let request = redis_enterprise::bdb::DatabaseUpgradeRequest {
        redis_version: Some("7.4.2".to_string()),
        preserve_roles: Some(true),
        force_restart: Some(false),
        may_discard_data: Some(false),
        force_discard: Some(false),
        keep_crdt_protocol_version: Some(false),
        parallel_shards_upgrade: None,
        modules: None,
    };

    let result = handler.upgrade_redis_version(1, request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.action_uid, "591d9dcb-ddd7-48a9-a04d-bd5d4d6834d0");
    // Verify the flattened extra fields are captured
    assert_eq!(response.extra["uid"], 1);
    assert_eq!(response.extra["name"], "test-db");
    assert_eq!(response.extra["redis_version"], "7.4");
}
