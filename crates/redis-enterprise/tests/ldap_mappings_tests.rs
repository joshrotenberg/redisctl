//! LDAP mappings endpoint tests for Redis Enterprise

use redis_enterprise::{
    CreateLdapMappingRequest, EnterpriseClient, LdapConfig, LdapMappingHandler, LdapServer,
};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
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

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_ldap_mapping() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "admin_mapping",
        "dn": "cn=admin,ou=users,dc=example,dc=com",
        "role": "DB Admin",
        "email": "admin@example.com",
        "role_uids": [1, 2]
    })
}

fn test_ldap_mapping_minimal() -> serde_json::Value {
    json!({
        "uid": 2,
        "name": "user_mapping",
        "dn": "cn=user,ou=users,dc=example,dc=com",
        "role": "DB Viewer"
    })
}

fn test_ldap_config() -> serde_json::Value {
    json!({
        "enabled": true,
        "servers": [
            {
                "host": "ldap.example.com",
                "port": 389,
                "use_tls": false,
                "starttls": true
            }
        ],
        "cache_refresh_interval": 3600,
        "authentication_query_suffix": "ou=users,dc=example,dc=com",
        "authorization_query_suffix": "ou=groups,dc=example,dc=com",
        "bind_dn": "cn=service,dc=example,dc=com",
        "bind_pass": "secret"
    })
}

fn test_create_ldap_mapping_request() -> CreateLdapMappingRequest {
    CreateLdapMappingRequest {
        name: "test_mapping".to_string(),
        dn: "cn=test,ou=users,dc=example,dc=com".to_string(),
        role: "DB Admin".to_string(),
        email: Some("test@example.com".to_string()),
        role_uids: Some(vec![1, 2]),
    }
}

fn test_ldap_config_obj() -> LdapConfig {
    LdapConfig {
        enabled: true,
        servers: Some(vec![LdapServer {
            host: "ldap.example.com".to_string(),
            port: 389,
            use_tls: Some(false),
            starttls: Some(true),
        }]),
        cache_refresh_interval: Some(3600),
        authentication_query_suffix: Some("ou=users,dc=example,dc=com".to_string()),
        authorization_query_suffix: Some("ou=groups,dc=example,dc=com".to_string()),
        bind_dn: Some("cn=service,dc=example,dc=com".to_string()),
        bind_pass: Some("secret".to_string()),
        extra: json!({}),
    }
}

#[tokio::test]
async fn test_ldap_mappings_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ldap_mappings"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_ldap_mapping(),
            test_ldap_mapping_minimal()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let mappings = result.unwrap();
    assert_eq!(mappings.len(), 2);
    assert_eq!(mappings[0].uid, 1);
    assert_eq!(mappings[0].name, "admin_mapping");
    assert_eq!(mappings[1].uid, 2);
    assert_eq!(mappings[1].name, "user_mapping");
}

#[tokio::test]
async fn test_ldap_mappings_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ldap_mappings"))
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

    let handler = LdapMappingHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let mappings = result.unwrap();
    assert_eq!(mappings.len(), 0);
}

#[tokio::test]
async fn test_ldap_mappings_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ldap_mappings/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ldap_mapping()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_ok());
    let mapping = result.unwrap();
    assert_eq!(mapping.uid, 1);
    assert_eq!(mapping.name, "admin_mapping");
    assert_eq!(mapping.dn, "cn=admin,ou=users,dc=example,dc=com");
    assert_eq!(mapping.role, "DB Admin");
    assert_eq!(mapping.email, Some("admin@example.com".to_string()));
    assert_eq!(mapping.role_uids, Some(vec![1, 2]));
}

#[tokio::test]
async fn test_ldap_mappings_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/ldap_mappings/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "LDAP mapping not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ldap_mappings_create() {
    let mock_server = MockServer::start().await;
    let request = test_create_ldap_mapping_request();

    Mock::given(method("POST"))
        .and(path("/v1/ldap_mappings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "uid": 3,
            "name": "test_mapping",
            "dn": "cn=test,ou=users,dc=example,dc=com",
            "role": "DB Admin",
            "email": "test@example.com",
            "role_uids": [1, 2]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let mapping = result.unwrap();
    assert_eq!(mapping.uid, 3);
    assert_eq!(mapping.name, "test_mapping");
    assert_eq!(mapping.dn, "cn=test,ou=users,dc=example,dc=com");
    assert_eq!(mapping.role, "DB Admin");
}

#[tokio::test]
async fn test_ldap_mappings_create_invalid() {
    let mock_server = MockServer::start().await;
    let request = CreateLdapMappingRequest {
        name: "".to_string(),
        dn: "invalid-dn".to_string(),
        role: "".to_string(),
        email: None,
        role_uids: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/ldap_mappings"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(400, "Invalid LDAP mapping data"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ldap_mappings_update() {
    let mock_server = MockServer::start().await;
    let request = test_create_ldap_mapping_request();

    Mock::given(method("PUT"))
        .and(path("/v1/ldap_mappings/1"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(success_response(json!({
            "uid": 1,
            "name": "test_mapping",
            "dn": "cn=test,ou=users,dc=example,dc=com",
            "role": "DB Admin",
            "email": "test@example.com",
            "role_uids": [1, 2]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.update(1, request).await;

    assert!(result.is_ok());
    let mapping = result.unwrap();
    assert_eq!(mapping.uid, 1);
    assert_eq!(mapping.name, "test_mapping");
}

#[tokio::test]
async fn test_ldap_mappings_update_nonexistent() {
    let mock_server = MockServer::start().await;
    let request = test_create_ldap_mapping_request();

    Mock::given(method("PUT"))
        .and(path("/v1/ldap_mappings/999"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(404, "LDAP mapping not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.update(999, request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ldap_mappings_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/ldap_mappings/1"))
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

    let handler = LdapMappingHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ldap_mappings_delete_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/ldap_mappings/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "LDAP mapping not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.delete(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_ldap_get_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/ldap"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_ldap_config()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.get_config().await;

    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.enabled);
    assert!(config.servers.is_some());
    assert_eq!(config.cache_refresh_interval, Some(3600));
}

#[tokio::test]
async fn test_ldap_update_config() {
    let mock_server = MockServer::start().await;
    let config = test_ldap_config_obj();

    Mock::given(method("PUT"))
        .and(path("/v1/cluster/ldap"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(success_response(test_ldap_config()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_ok());
    let updated_config = result.unwrap();
    assert!(updated_config.enabled);
    assert!(updated_config.servers.is_some());
}

#[tokio::test]
async fn test_ldap_update_config_invalid() {
    let mock_server = MockServer::start().await;
    let config = LdapConfig {
        enabled: true,
        servers: None,
        cache_refresh_interval: None,
        authentication_query_suffix: None,
        authorization_query_suffix: None,
        bind_dn: None,
        bind_pass: None,
        extra: json!({}),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/cluster/ldap"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&config))
        .respond_with(error_response(400, "Invalid LDAP configuration"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = LdapMappingHandler::new(client);
    let result = handler.update_config(config).await;

    assert!(result.is_err());
}
