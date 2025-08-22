//! OCSP endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, OcspConfig, OcspHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_ocsp_config() -> serde_json::Value {
    json!({
        "enabled": true,
        "responder_url": "http://ocsp.example.com",
        "response_timeout": 30,
        "query_frequency": 3600,
        "recovery_frequency": 60,
        "recovery_max_tries": 5
    })
}

fn test_ocsp_config_disabled() -> serde_json::Value {
    json!({
        "enabled": false
    })
}

fn test_ocsp_status() -> serde_json::Value {
    json!({
        "status": "good",
        "last_update": "2023-01-01T12:00:00Z",
        "next_update": "2023-01-01T13:00:00Z",
        "certificate_status": "valid"
    })
}

fn test_ocsp_status_revoked() -> serde_json::Value {
    json!({
        "status": "revoked",
        "last_update": "2023-01-01T12:00:00Z",
        "next_update": "2023-01-01T13:00:00Z",
        "certificate_status": "revoked",
        "revocation_time": "2023-01-01T10:00:00Z",
        "revocation_reason": "keyCompromise"
    })
}

fn test_ocsp_test_result_success() -> serde_json::Value {
    json!({
        "success": true,
        "message": "OCSP responder is reachable",
        "response_time_ms": 150
    })
}

fn test_ocsp_test_result_failure() -> serde_json::Value {
    json!({
        "success": false,
        "message": "Connection timeout to OCSP responder",
        "response_time_ms": 5000
    })
}

fn test_ocsp_config_obj() -> OcspConfig {
    OcspConfig {
        enabled: true,
        responder_url: Some("http://ocsp.example.com".to_string()),
        response_timeout: Some(30),
        query_frequency: Some(3600),
        recovery_frequency: Some(60),
        recovery_max_tries: Some(5),
        extra: json!({}),
    }
}

fn test_ocsp_config_minimal() -> OcspConfig {
    OcspConfig {
        enabled: false,
        responder_url: None,
        response_timeout: None,
        query_frequency: None,
        recovery_frequency: None,
        recovery_max_tries: None,
        extra: json!({}),
    }
}

#[tokio::test]
async fn test_ocsp_get_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ocsp_config()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.get_config().await;

    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.enabled);
    assert_eq!(
        config.responder_url,
        Some("http://ocsp.example.com".to_string())
    );
    assert_eq!(config.response_timeout, Some(30));
    assert_eq!(config.query_frequency, Some(3600));
    assert_eq!(config.recovery_frequency, Some(60));
    assert_eq!(config.recovery_max_tries, Some(5));
}

#[tokio::test]
async fn test_ocsp_get_config_disabled() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ocsp_config_disabled()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.get_config().await;

    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(!config.enabled);
    assert!(config.responder_url.is_none());
}

#[tokio::test]
async fn test_ocsp_update_config() {
    let mock_server = MockServer::start().await;
    let config = test_ocsp_config_obj();

    Mock::given(method("PUT"))
        .and(path("/v1/ocsp"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(success_response(test_ocsp_config()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_ok());
    let updated_config = result.unwrap();
    assert!(updated_config.enabled);
    assert_eq!(
        updated_config.responder_url,
        Some("http://ocsp.example.com".to_string())
    );
}

#[tokio::test]
async fn test_ocsp_update_config_disable() {
    let mock_server = MockServer::start().await;
    let config = test_ocsp_config_minimal();

    Mock::given(method("PUT"))
        .and(path("/v1/ocsp"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(success_response(test_ocsp_config_disabled()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_ok());
    let updated_config = result.unwrap();
    assert!(!updated_config.enabled);
}

#[tokio::test]
async fn test_ocsp_update_config_invalid() {
    let mock_server = MockServer::start().await;
    let config = OcspConfig {
        enabled: true,
        responder_url: Some("invalid-url".to_string()),
        response_timeout: Some(0),
        query_frequency: None,
        recovery_frequency: None,
        recovery_max_tries: None,
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/ocsp"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(error_response(400, "Invalid OCSP configuration"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ocsp_get_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp/status"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ocsp_status()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.get_status().await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "good");
    assert_eq!(status.last_update, Some("2023-01-01T12:00:00Z".to_string()));
    assert_eq!(status.next_update, Some("2023-01-01T13:00:00Z".to_string()));
    assert_eq!(status.certificate_status, Some("valid".to_string()));
    assert!(status.revocation_time.is_none());
    assert!(status.revocation_reason.is_none());
}

#[tokio::test]
async fn test_ocsp_get_status_revoked() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp/status"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ocsp_status_revoked()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.get_status().await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "revoked");
    assert_eq!(status.certificate_status, Some("revoked".to_string()));
    assert_eq!(
        status.revocation_time,
        Some("2023-01-01T10:00:00Z".to_string())
    );
    assert_eq!(status.revocation_reason, Some("keyCompromise".to_string()));
}

#[tokio::test]
async fn test_ocsp_get_status_not_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp/status"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "OCSP not configured"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.get_status().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ocsp_test_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp/test"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ocsp_test_result_success()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.test().await;

    assert!(result.is_ok());
    let test_result = result.unwrap();
    assert!(test_result.success);
    assert_eq!(
        test_result.message,
        Some("OCSP responder is reachable".to_string())
    );
    assert_eq!(test_result.response_time_ms, Some(150));
}

#[tokio::test]
async fn test_ocsp_test_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp/test"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ocsp_test_result_failure()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.test().await;

    assert!(result.is_ok());
    let test_result = result.unwrap();
    assert!(!test_result.success);
    assert_eq!(
        test_result.message,
        Some("Connection timeout to OCSP responder".to_string())
    );
    assert_eq!(test_result.response_time_ms, Some(5000));
}

#[tokio::test]
async fn test_ocsp_test_not_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ocsp/test"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(400, "OCSP not enabled"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.test().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ocsp_query() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/ocsp/query"))
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

    let handler = OcspHandler::new(client);
    let result = handler.query().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ocsp_query_not_configured() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/ocsp/query"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(400, "OCSP not enabled"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.query().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ocsp_clear_cache() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/ocsp/cache"))
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

    let handler = OcspHandler::new(client);
    let result = handler.clear_cache().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ocsp_clear_cache_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/ocsp/cache"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(500, "Failed to clear OCSP cache"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = OcspHandler::new(client);
    let result = handler.clear_cache().await;

    assert!(result.is_err());
}
