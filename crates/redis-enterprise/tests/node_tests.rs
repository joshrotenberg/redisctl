//! Node endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, NodeHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
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

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn test_node() -> serde_json::Value {
    json!({
        "uid": 1,
        "addr": "10.0.0.1",
        "status": "active",
        "shard_list": [1, 2, 3],
        "total_memory": 8589934592u64,
        "cores": 8,
        "os_version": "Ubuntu 20.04",
        "ephemeral_storage_size": 107374182400.0,
        "persistent_storage_size": 214748364800.0,
        "rack_id": "rack-1",
        "accept_servers": true,
        "architecture": "x86_64",
        "shard_count": 3
    })
}

fn test_slave_node() -> serde_json::Value {
    json!({
        "uid": 2,
        "addr": "10.0.0.2",
        "status": "active",
        "shard_list": [4, 5],
        "total_memory": 8589934592u64,
        "cores": 4,
        "os_version": "Ubuntu 20.04",
        "rack_id": "rack-2",
        "accept_servers": true,
        "shard_count": 2
    })
}

fn test_node_stats_data() -> serde_json::Value {
    json!({
        "uid": 1,
        "cpu_user": 25.5,
        "cpu_system": 10.2,
        "cpu_idle": 64.3,
        "free_memory": 4294967296u64,
        "network_bytes_in": 1024000,
        "network_bytes_out": 2048000,
        "persistent_storage_free": 107374182400u64,
        "ephemeral_storage_free": 53687091200u64
    })
}

fn test_node_actions_data() -> serde_json::Value {
    json!([
        {
            "action": "maintenance_on",
            "description": "Put node into maintenance mode"
        },
        {
            "action": "maintenance_off",
            "description": "Take node out of maintenance mode"
        },
        {
            "action": "restart",
            "description": "Restart the node"
        }
    ])
}

#[tokio::test]
async fn test_nodes_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_node(), test_slave_node()])))
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
    assert_eq!(nodes[0].uid, 1);
    assert_eq!(nodes[0].addr.as_ref().unwrap(), "10.0.0.1");
    assert_eq!(nodes[1].uid, 2);
}

#[tokio::test]
async fn test_nodes_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
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

    let handler = NodeHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let nodes = result.unwrap();
    assert_eq!(nodes.len(), 0);
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
    assert_eq!(node.addr.as_ref().unwrap(), "10.0.0.1");
    assert_eq!(node.status, "active");
    assert_eq!(node.shard_list.as_ref().unwrap(), &vec![1, 2, 3]);
    assert_eq!(node.total_memory.unwrap(), 8589934592u64);
    assert_eq!(node.cores.unwrap(), 8);
    assert_eq!(node.rack_id.unwrap(), "rack-1");
}

#[tokio::test]
async fn test_node_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999"))
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

    let handler = NodeHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_node_update() {
    let mock_server = MockServer::start().await;

    let updates = json!({
        "rack_id": "rack-3"
    });

    let updated_node = json!({
        "uid": 1,
        "addr": "10.0.0.1",
        "status": "active",
        "rack_id": "rack-3",
        "total_memory": 8589934592u64
    });

    Mock::given(method("PUT"))
        .and(path("/v1/nodes/1"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&updates))
        .respond_with(success_response(updated_node))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.update(1, updates).await;

    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.uid, 1);
    assert_eq!(node.rack_id.unwrap(), "rack-3");
}

#[tokio::test]
async fn test_node_update_nonexistent() {
    let mock_server = MockServer::start().await;

    let updates = json!({
        "rack_id": "rack-4"
    });

    Mock::given(method("PUT"))
        .and(path("/v1/nodes/999"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&updates))
        .respond_with(error_response(404, "Node not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.update(999, updates).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_node_remove() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/nodes/2"))
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

    let handler = NodeHandler::new(client);
    let result = handler.remove(2).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_node_remove_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/nodes/999"))
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

    let handler = NodeHandler::new(client);
    let result = handler.remove(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_node_stats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/stats"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_node_stats_data()))
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
    let stats = result.unwrap();
    assert_eq!(stats.uid, 1);
    assert_eq!(stats.cpu_user.unwrap(), 25.5);
    assert_eq!(stats.cpu_system.unwrap(), 10.2);
    assert_eq!(stats.cpu_idle.unwrap(), 64.3);
    assert_eq!(stats.free_memory.unwrap(), 4294967296u64);
    assert_eq!(stats.network_bytes_in.unwrap(), 1024000);
    assert_eq!(stats.network_bytes_out.unwrap(), 2048000);
}

#[tokio::test]
async fn test_node_stats_nonexistent() {
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

    let handler = NodeHandler::new(client);
    let result = handler.stats(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_node_actions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/1/actions"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_node_actions_data()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.actions(1).await;

    assert!(result.is_ok());
    let actions = result.unwrap();
    assert!(actions.is_array());
    let actions_array = actions.as_array().unwrap();
    assert_eq!(actions_array.len(), 3);
}

#[tokio::test]
async fn test_node_actions_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes/999/actions"))
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

    let handler = NodeHandler::new(client);
    let result = handler.actions(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_node_execute_action_maintenance_on() {
    let mock_server = MockServer::start().await;

    let expected_request = json!({
        "action": "maintenance_on",
        "node_uid": 1
    });

    Mock::given(method("POST"))
        .and(path("/v1/nodes/1/actions"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&expected_request))
        .respond_with(success_response(json!({
            "action_uid": "action-123-abc",
            "status": "pending",
            "message": "Maintenance mode enabled for node 1"
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
    let result = handler.execute_action(1, "maintenance_on").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["action_uid"], "action-123-abc");
    assert_eq!(response["status"], "pending");
}

#[tokio::test]
async fn test_node_execute_action_maintenance_off() {
    let mock_server = MockServer::start().await;

    let expected_request = json!({
        "action": "maintenance_off",
        "node_uid": 1
    });

    Mock::given(method("POST"))
        .and(path("/v1/nodes/1/actions"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&expected_request))
        .respond_with(success_response(json!({
            "action_uid": "action-456-def",
            "status": "completed",
            "message": "Maintenance mode disabled for node 1"
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
    let result = handler.execute_action(1, "maintenance_off").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["action_uid"], "action-456-def");
    assert_eq!(response["status"], "completed");
}

#[tokio::test]
async fn test_node_execute_action_invalid() {
    let mock_server = MockServer::start().await;

    let expected_request = json!({
        "action": "invalid_action",
        "node_uid": 1
    });

    Mock::given(method("POST"))
        .and(path("/v1/nodes/1/actions"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&expected_request))
        .respond_with(error_response(400, "Invalid action"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.execute_action(1, "invalid_action").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_node_execute_action_nonexistent_node() {
    let mock_server = MockServer::start().await;

    let expected_request = json!({
        "action": "restart",
        "node_uid": 999
    });

    Mock::given(method("POST"))
        .and(path("/v1/nodes/999/actions"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&expected_request))
        .respond_with(error_response(404, "Node not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = NodeHandler::new(client);
    let result = handler.execute_action(999, "restart").await;

    assert!(result.is_err());
}
