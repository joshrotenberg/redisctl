//! License endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, LicenseHandler, LicenseUpdateRequest};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn valid_license() -> serde_json::Value {
    json!({
        "license_key": "lic-123-456-789",
        "type_": "production",
        "expired": false,
        "expiration_date": "2025-12-31T23:59:59Z",
        "shards_limit": 100,
        "node_limit": 10,
        "features": ["clustering", "modules", "flash"],
        "owner": "test-company"
    })
}

fn expired_license() -> serde_json::Value {
    json!({
        "license_key": "lic-expired-123",
        "type_": "trial",
        "expired": true,
        "expiration_date": "2023-01-01T00:00:00Z",
        "shards_limit": 10,
        "node_limit": 3,
        "features": ["clustering"],
        "owner": "test-trial"
    })
}

fn license_usage() -> serde_json::Value {
    json!({
        "shards_used": 25,
        "shards_limit": 100,
        "nodes_used": 3,
        "nodes_limit": 10,
        "ram_used": 8589934592u64,
        "ram_limit": 34359738368u64
    })
}

fn minimal_license() -> serde_json::Value {
    json!({
        "license_key": "lic-minimal-789",
        "type_": "dev",
        "expired": false
    })
}

#[tokio::test]
async fn test_license_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/license"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(valid_license()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let license = result.unwrap();
    assert_eq!(license.license_key, "lic-123-456-789");
    assert_eq!(license.type_, "production");
    assert!(!license.expired);
    assert_eq!(
        license.expiration_date,
        Some("2025-12-31T23:59:59Z".to_string())
    );
    assert_eq!(license.shards_limit, Some(100));
    assert_eq!(license.node_limit, Some(10));
    assert_eq!(
        license.features,
        Some(vec![
            "clustering".to_string(),
            "modules".to_string(),
            "flash".to_string()
        ])
    );
    assert_eq!(license.owner, Some("test-company".to_string()));
}

#[tokio::test]
async fn test_license_get_expired() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/license"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(expired_license()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let license = result.unwrap();
    assert_eq!(license.license_key, "lic-expired-123");
    assert_eq!(license.type_, "trial");
    assert!(license.expired);
    assert_eq!(
        license.expiration_date,
        Some("2023-01-01T00:00:00Z".to_string())
    );
    assert_eq!(license.shards_limit, Some(10));
    assert_eq!(license.node_limit, Some(3));
}

#[tokio::test]
async fn test_license_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/license"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(minimal_license()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let license = result.unwrap();
    assert_eq!(license.license_key, "lic-minimal-789");
    assert_eq!(license.type_, "dev");
    assert!(!license.expired);
    assert!(license.expiration_date.is_none());
    assert!(license.shards_limit.is_none());
    assert!(license.node_limit.is_none());
    assert!(license.features.is_none());
    assert!(license.owner.is_none());
}

#[tokio::test]
async fn test_license_update() {
    let mock_server = MockServer::start().await;

    let update_request = LicenseUpdateRequest {
        license: "new-license-key-12345".to_string(),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/license"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&update_request))
        .respond_with(success_response(json!({
            "license_key": "new-license-key-12345",
            "type_": "production",
            "expired": false,
            "expiration_date": "2026-12-31T23:59:59Z",
            "shards_limit": 200,
            "node_limit": 20,
            "features": ["clustering", "modules", "flash", "search"],
            "owner": "updated-company"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.update(update_request).await;

    assert!(result.is_ok());
    let license = result.unwrap();
    assert_eq!(license.license_key, "new-license-key-12345");
    assert_eq!(license.type_, "production");
    assert!(!license.expired);
    assert_eq!(license.shards_limit, Some(200));
    assert_eq!(license.node_limit, Some(20));
    assert_eq!(
        license.features,
        Some(vec![
            "clustering".to_string(),
            "modules".to_string(),
            "flash".to_string(),
            "search".to_string()
        ])
    );
}

#[tokio::test]
async fn test_license_usage() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/license/usage"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(license_usage()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.usage().await;

    assert!(result.is_ok());
    let usage = result.unwrap();
    assert_eq!(usage.shards_used, 25);
    assert_eq!(usage.shards_limit, 100);
    assert_eq!(usage.nodes_used, 3);
    assert_eq!(usage.nodes_limit, 10);
    assert_eq!(usage.ram_used, Some(8589934592));
    assert_eq!(usage.ram_limit, Some(34359738368));
}

#[tokio::test]
async fn test_license_usage_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/license/usage"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "shards_used": 5,
            "shards_limit": 10,
            "nodes_used": 1,
            "nodes_limit": 3
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.usage().await;

    assert!(result.is_ok());
    let usage = result.unwrap();
    assert_eq!(usage.shards_used, 5);
    assert_eq!(usage.shards_limit, 10);
    assert_eq!(usage.nodes_used, 1);
    assert_eq!(usage.nodes_limit, 3);
    assert!(usage.ram_used.is_none());
    assert!(usage.ram_limit.is_none());
}

#[tokio::test]
async fn test_license_validate_valid() {
    let mock_server = MockServer::start().await;

    let validate_request = LicenseUpdateRequest {
        license: "valid-license-to-validate".to_string(),
    };

    Mock::given(method("POST"))
        .and(path("/v1/license/validate"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&validate_request))
        .respond_with(success_response(json!({
            "license_key": "valid-license-to-validate",
            "type_": "production",
            "expired": false,
            "expiration_date": "2025-06-30T23:59:59Z",
            "shards_limit": 50,
            "node_limit": 5,
            "features": ["clustering", "modules"],
            "owner": "validation-company"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.validate("valid-license-to-validate").await;

    assert!(result.is_ok());
    let license = result.unwrap();
    assert_eq!(license.license_key, "valid-license-to-validate");
    assert_eq!(license.type_, "production");
    assert!(!license.expired);
    assert_eq!(license.shards_limit, Some(50));
    assert_eq!(license.node_limit, Some(5));
}

#[tokio::test]
async fn test_license_validate_expired() {
    let mock_server = MockServer::start().await;

    let validate_request = LicenseUpdateRequest {
        license: "expired-license-key".to_string(),
    };

    Mock::given(method("POST"))
        .and(path("/v1/license/validate"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&validate_request))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "License expired",
            "code": "LICENSE_EXPIRED"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.validate("expired-license-key").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_license_cluster_license() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/license"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "license_key": "cluster-license-789",
            "type_": "enterprise",
            "expired": false,
            "expiration_date": "2024-12-31T23:59:59Z",
            "shards_limit": 1000,
            "node_limit": 100,
            "features": ["clustering", "modules", "flash", "search", "json"],
            "owner": "enterprise-customer"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.cluster_license().await;

    assert!(result.is_ok());
    let license = result.unwrap();
    assert_eq!(license.license_key, "cluster-license-789");
    assert_eq!(license.type_, "enterprise");
    assert!(!license.expired);
    assert_eq!(license.shards_limit, Some(1000));
    assert_eq!(license.node_limit, Some(100));
    assert_eq!(
        license.features,
        Some(vec![
            "clustering".to_string(),
            "modules".to_string(),
            "flash".to_string(),
            "search".to_string(),
            "json".to_string()
        ])
    );
    assert_eq!(license.owner, Some("enterprise-customer".to_string()));
}

#[tokio::test]
async fn test_license_get_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/license"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "License not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_license_update_invalid() {
    let mock_server = MockServer::start().await;

    let invalid_request = LicenseUpdateRequest {
        license: "invalid-license-format".to_string(),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/license"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&invalid_request))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid license format",
            "code": "INVALID_LICENSE"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LicenseHandler::new(client);
    let result = handler.update(invalid_request).await;

    assert!(result.is_err());
}
