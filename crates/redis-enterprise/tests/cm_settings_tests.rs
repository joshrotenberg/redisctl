//! Cluster Manager settings tests for Redis Enterprise

use redis_enterprise::{CmSettings, CmSettingsHandler, EnterpriseClient};
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

fn test_cm_settings_full() -> serde_json::Value {
    json!({
        "cm_port": 8080,
        "cm_session_timeout": 1800,
        "auto_recovery": true,
        "auto_failover": true,
        "slave_ha": true,
        "slave_ha_grace_period": 300,
        "max_simultaneous_backups": 3
    })
}

fn test_cm_settings_minimal() -> serde_json::Value {
    json!({
        "cm_port": 9443
    })
}

fn test_cm_settings_defaults() -> serde_json::Value {
    json!({
        "cm_port": 8443,
        "cm_session_timeout": 3600,
        "auto_recovery": false,
        "auto_failover": false,
        "slave_ha": false,
        "slave_ha_grace_period": 900,
        "max_simultaneous_backups": 1
    })
}

#[tokio::test]
async fn test_cm_settings_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cm_settings_full()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let settings = result.unwrap();
    assert_eq!(settings.cm_port, Some(8080));
    assert_eq!(settings.cm_session_timeout, Some(1800));
    assert_eq!(settings.auto_recovery, Some(true));
    assert_eq!(settings.auto_failover, Some(true));
    assert_eq!(settings.slave_ha, Some(true));
    assert_eq!(settings.slave_ha_grace_period, Some(300));
    assert_eq!(settings.max_simultaneous_backups, Some(3));
}

#[tokio::test]
async fn test_cm_settings_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cm_settings_minimal()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let settings = result.unwrap();
    assert_eq!(settings.cm_port, Some(9443));
    assert!(settings.cm_session_timeout.is_none());
    assert!(settings.auto_recovery.is_none());
    assert!(settings.auto_failover.is_none());
    assert!(settings.slave_ha.is_none());
    assert!(settings.slave_ha_grace_period.is_none());
    assert!(settings.max_simultaneous_backups.is_none());
}

#[tokio::test]
async fn test_cm_settings_get_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(500, "Internal server error"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_cm_settings_update_full() {
    let mock_server = MockServer::start().await;

    let settings = CmSettings {
        cm_port: Some(8080),
        cm_session_timeout: Some(2400),
        auto_recovery: Some(true),
        auto_failover: Some(true),
        slave_ha: Some(true),
        slave_ha_grace_period: Some(600),
        max_simultaneous_backups: Some(5),
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&settings))
        .respond_with(success_response(json!({
            "cm_port": 8080,
            "cm_session_timeout": 2400,
            "auto_recovery": true,
            "auto_failover": true,
            "slave_ha": true,
            "slave_ha_grace_period": 600,
            "max_simultaneous_backups": 5
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.update(settings).await;

    assert!(result.is_ok());
    let updated_settings = result.unwrap();
    assert_eq!(updated_settings.cm_port, Some(8080));
    assert_eq!(updated_settings.cm_session_timeout, Some(2400));
    assert_eq!(updated_settings.auto_recovery, Some(true));
    assert_eq!(updated_settings.auto_failover, Some(true));
    assert_eq!(updated_settings.slave_ha, Some(true));
    assert_eq!(updated_settings.slave_ha_grace_period, Some(600));
    assert_eq!(updated_settings.max_simultaneous_backups, Some(5));
}

#[tokio::test]
async fn test_cm_settings_update_partial() {
    let mock_server = MockServer::start().await;

    let settings = CmSettings {
        cm_port: Some(9090),
        cm_session_timeout: None,
        auto_recovery: Some(false),
        auto_failover: None,
        slave_ha: None,
        slave_ha_grace_period: None,
        max_simultaneous_backups: Some(2),
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&settings))
        .respond_with(success_response(json!({
            "cm_port": 9090,
            "auto_recovery": false,
            "max_simultaneous_backups": 2
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.update(settings).await;

    assert!(result.is_ok());
    let updated_settings = result.unwrap();
    assert_eq!(updated_settings.cm_port, Some(9090));
    assert!(updated_settings.cm_session_timeout.is_none());
    assert_eq!(updated_settings.auto_recovery, Some(false));
    assert!(updated_settings.auto_failover.is_none());
    assert!(updated_settings.slave_ha.is_none());
    assert!(updated_settings.slave_ha_grace_period.is_none());
    assert_eq!(updated_settings.max_simultaneous_backups, Some(2));
}

#[tokio::test]
async fn test_cm_settings_update_minimal() {
    let mock_server = MockServer::start().await;

    let settings = CmSettings {
        cm_port: None,
        cm_session_timeout: None,
        auto_recovery: None,
        auto_failover: None,
        slave_ha: None,
        slave_ha_grace_period: None,
        max_simultaneous_backups: None,
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&settings))
        .respond_with(success_response(json!({})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.update(settings).await;

    assert!(result.is_ok());
    let updated_settings = result.unwrap();
    assert!(updated_settings.cm_port.is_none());
    assert!(updated_settings.cm_session_timeout.is_none());
    assert!(updated_settings.auto_recovery.is_none());
    assert!(updated_settings.auto_failover.is_none());
    assert!(updated_settings.slave_ha.is_none());
    assert!(updated_settings.slave_ha_grace_period.is_none());
    assert!(updated_settings.max_simultaneous_backups.is_none());
}

#[tokio::test]
async fn test_cm_settings_update_invalid() {
    let mock_server = MockServer::start().await;

    let settings = CmSettings {
        cm_port: Some(65535),        // High port number
        cm_session_timeout: Some(0), // Invalid timeout
        auto_recovery: Some(true),
        auto_failover: Some(true),
        slave_ha: Some(true),
        slave_ha_grace_period: Some(0),    // Invalid grace period
        max_simultaneous_backups: Some(0), // Invalid backup count
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&settings))
        .respond_with(error_response(400, "Invalid settings values"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.update(settings).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_cm_settings_reset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/cm_settings"))
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

    let handler = CmSettingsHandler::new(client);
    let result = handler.reset().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cm_settings_reset_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(403, "Reset not allowed"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);
    let result = handler.reset().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_cm_settings_workflow() {
    let mock_server = MockServer::start().await;

    // First get current settings
    Mock::given(method("GET"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cm_settings_defaults()))
        .mount(&mock_server)
        .await;

    // Then update settings
    let new_settings = CmSettings {
        cm_port: Some(8080),
        cm_session_timeout: Some(1800),
        auto_recovery: Some(true),
        auto_failover: Some(true),
        slave_ha: Some(true),
        slave_ha_grace_period: Some(300),
        max_simultaneous_backups: Some(3),
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/cm_settings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&new_settings))
        .respond_with(success_response(test_cm_settings_full()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CmSettingsHandler::new(client);

    // Get current settings
    let current_result = handler.get().await;
    assert!(current_result.is_ok());
    let current_settings = current_result.unwrap();
    assert_eq!(current_settings.cm_port, Some(8443));
    assert_eq!(current_settings.auto_recovery, Some(false));

    // Update settings
    let update_result = handler.update(new_settings).await;
    assert!(update_result.is_ok());
    let updated_settings = update_result.unwrap();
    assert_eq!(updated_settings.cm_port, Some(8080));
    assert_eq!(updated_settings.auto_recovery, Some(true));
    assert_eq!(updated_settings.max_simultaneous_backups, Some(3));
}
