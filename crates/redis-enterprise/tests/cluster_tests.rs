//! Cluster endpoint tests for Redis Enterprise

use redis_enterprise::{ClusterHandler, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn test_cluster() -> serde_json::Value {
    json!({
        "name": "test-cluster",
        "nodes_count": 3,
        "version": "6.4.2-30",
        "license": "valid"
    })
}

#[tokio::test]
async fn test_cluster_actions_and_auditing() {
    let mock_server = MockServer::start().await;

    // List actions
    Mock::given(method("GET"))
        .and(path("/v1/cluster/actions"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!(["reset", "recover"])))
        .mount(&mock_server)
        .await;

    // Update auditing db conns
    Mock::given(method("PUT"))
        .and(path("/v1/cluster/auditing/db_conns"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"enabled": true})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let acts = handler.actions().await.unwrap();
    assert!(acts.is_array());

    let updated = handler
        .auditing_db_conns_update(json!({"enabled": true}))
        .await
        .unwrap();
    assert_eq!(updated["enabled"], true);
}

#[tokio::test]
async fn test_cluster_certs_policy_and_witness() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/cluster/certificates/rotate"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"rotated": true})))
        .mount(&mock_server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/v1/cluster/policy/restore_default"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"restored": true})))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/witness_disk"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"ok": true})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let r = handler.certificates_rotate().await.unwrap();
    assert_eq!(r["rotated"], true);

    let p = handler.policy_restore_default().await.unwrap();
    assert_eq!(p["restored"], true);

    let w = handler.witness_disk().await.unwrap();
    assert_eq!(w["ok"], true);
}

#[tokio::test]
async fn test_cluster_alert_detail_and_ldap_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/alerts/high_cpu"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"severity": "critical"})))
        .mount(&mock_server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/v1/cluster/ldap"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let detail = handler.alert_detail("high_cpu").await.unwrap();
    assert_eq!(detail["severity"], "critical");

    handler.ldap_delete().await.unwrap();
}

#[tokio::test]
async fn test_cluster_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cluster()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.info().await;

    assert!(result.is_ok());
    let _cluster = result.unwrap();
    // ClusterInfo struct would have these fields available
}

#[tokio::test]
async fn test_cluster_join_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap/join"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"status": "node_joined"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.join_node("10.0.0.2", "admin", "password").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "node_joined");
}

#[tokio::test]
async fn test_cluster_remove_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/nodes/2"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.remove_node(2).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()["message"], "Node 2 removed");
}

#[tokio::test]
async fn test_cluster_reset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/cluster/actions/reset"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"status": "cluster_reset"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.reset_raw().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "cluster_reset");
}

#[tokio::test]
async fn test_cluster_recover() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/cluster/actions/recover"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({"status": "cluster_recovered"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.recover_raw().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "cluster_recovered");
}
