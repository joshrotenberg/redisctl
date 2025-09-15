//! Test for cluster info endpoint with realistic data

use redis_enterprise::{ClusterHandler, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

#[tokio::test]
async fn test_cluster_info() {
    let mock_server = MockServer::start().await;

    // Mock response focusing on fields that were causing deserialization issues
    let cluster_response = json!({
        "name": "cluster.local",
        "created_time": "2025-09-15T21:22:00Z",
        "cnm_http_port": 8080,
        "cnm_https_port": 9443,
        "email_alerts": false,
        "rack_aware": false,
        "bigstore_driver": "speedb",
        // Fields that had type mismatches
        "password_complexity": false,  // Was Option<Value>, now Option<bool>
        "upgrade_mode": false,  // Was Option<String>, now Option<bool>
        "mtls_certificate_authentication": false,  // Was Option<String>, now Option<bool>
        "sentinel_cipher_suites": [],  // Was Option<String>, now Option<Vec<String>>
        "sentinel_cipher_suites_tls_1_3": "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256",  // Was Option<Vec<Value>>, now Option<String>
        "data_cipher_suites_tls_1_3": []  // Array type
    });

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(cluster_response))
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
    let info = result.unwrap();

    // Verify key fields
    assert_eq!(info.name, "cluster.local");
    assert_eq!(info.cnm_http_port, Some(8080));
    assert_eq!(info.cnm_https_port, Some(9443));
    assert_eq!(info.email_alerts, Some(false));
    assert_eq!(info.rack_aware, Some(false));
    assert_eq!(info.bigstore_driver, Some("speedb".to_string()));

    // Verify fields that were causing deserialization issues
    assert_eq!(info.password_complexity, Some(false));
    assert_eq!(info.upgrade_mode, Some(false));
    assert_eq!(info.mtls_certificate_authentication, Some(false));

    // Verify array fields
    assert_eq!(info.sentinel_cipher_suites, Some(vec![]));
    assert_eq!(
        info.sentinel_cipher_suites_tls_1_3,
        Some(
            "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256"
                .to_string()
        )
    );
    assert_eq!(info.data_cipher_suites_tls_1_3, Some(vec![]));
}

#[tokio::test]
async fn test_cluster_info_minimal() {
    let mock_server = MockServer::start().await;

    // Test with minimal required fields
    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "name": "minimal-cluster"
        })))
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
    let info = result.unwrap();
    assert_eq!(info.name, "minimal-cluster");

    // All optional fields should be None
    assert!(info.cnm_http_port.is_none());
    assert!(info.password_complexity.is_none());
    assert!(info.sentinel_cipher_suites.is_none());
}
