//! Cluster endpoint tests for Redis Enterprise
#![recursion_limit = "256"]

mod common;

use redis_enterprise::{ClusterHandler, ClusterInfo, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
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
        .respond_with(success_response(common::fixtures::cluster_info_response()))
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
    let cluster = result.unwrap();
    // Verify some key fields that had type mismatches
    assert_eq!(cluster.name, "test-cluster.local");
    assert!(cluster.sentinel_cipher_suites.is_some());
}

#[tokio::test]
async fn test_cluster_info_deserialization() {
    // This test explicitly validates that ClusterInfo can deserialize actual API responses
    let cluster_json = common::fixtures::cluster_info_response();

    // This would panic if deserialization fails with type mismatches
    let cluster_info: ClusterInfo = serde_json::from_value(cluster_json.clone()).unwrap();

    // Verify fields that previously had type mismatches
    assert_eq!(cluster_info.name, "test-cluster.local");

    // sentinel_cipher_suites was Option<String> but should be Option<Vec<String>>
    assert!(cluster_info.sentinel_cipher_suites.is_some());
    if let Some(cipher_suites) = &cluster_info.sentinel_cipher_suites {
        assert_eq!(cipher_suites.len(), 0); // Empty array in fixture
    }

    // password_complexity was Option<Value> but should be Option<bool>
    assert_eq!(cluster_info.password_complexity, Some(false));

    // mtls_certificate_authentication was Option<String> but should be Option<bool>
    assert_eq!(cluster_info.mtls_certificate_authentication, Some(false));

    // upgrade_mode was Option<String> but should be Option<bool>
    assert_eq!(cluster_info.upgrade_mode, Some(false));
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
        .respond_with(success_response(
            json!({"action_uid": "act-reset-1", "status": "cluster_reset"}),
        ))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.reset().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cluster_recover() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/cluster/actions/recover"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(
            json!({"action_uid": "act-recover-1", "status": "cluster_recovered"}),
        ))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ClusterHandler::new(client);
    let result = handler.recover().await;
    assert!(result.is_ok());
}
