//! Redis ACL endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, RedisAclHandler, CreateRedisAclRequest};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth, body_json};
use serde_json::json;

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

fn test_redis_acl() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "read_only_acl",
        "acl": "+@read -@write",
        "description": "Read-only access to Redis commands"
    })
}

fn admin_redis_acl() -> serde_json::Value {
    json!({
        "uid": 2,
        "name": "admin_acl",
        "acl": "+@all",
        "description": "Full administrative access"
    })
}

fn minimal_redis_acl() -> serde_json::Value {
    json!({
        "uid": 3,
        "name": "minimal_acl",
        "acl": "+ping +info"
    })
}

fn custom_redis_acl() -> serde_json::Value {
    json!({
        "uid": 4,
        "name": "custom_access",
        "acl": "+@read +@write -flushdb -flushall -config",
        "description": "Custom read/write access with restrictions"
    })
}

#[tokio::test]
async fn test_redis_acl_list() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/redis_acls"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_redis_acl(),
            admin_redis_acl(),
            minimal_redis_acl(),
            custom_redis_acl()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let acls = result.unwrap();
    assert_eq!(acls.len(), 4);
    
    // Verify first ACL details
    let read_only_acl = &acls[0];
    assert_eq!(read_only_acl.uid, 1);
    assert_eq!(read_only_acl.name, "read_only_acl");
    assert_eq!(read_only_acl.acl, "+@read -@write");
    assert_eq!(read_only_acl.description, Some("Read-only access to Redis commands".to_string()));
    
    // Verify admin ACL
    let admin_acl = &acls[1];
    assert_eq!(admin_acl.uid, 2);
    assert_eq!(admin_acl.name, "admin_acl");
    assert_eq!(admin_acl.acl, "+@all");
    assert_eq!(admin_acl.description, Some("Full administrative access".to_string()));
    
    // Verify minimal ACL (no description)
    let minimal_acl = &acls[2];
    assert_eq!(minimal_acl.uid, 3);
    assert_eq!(minimal_acl.name, "minimal_acl");
    assert_eq!(minimal_acl.acl, "+ping +info");
    assert!(minimal_acl.description.is_none());
}

#[tokio::test]
async fn test_redis_acl_list_empty() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/redis_acls"))
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

    let handler = RedisAclHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let acls = result.unwrap();
    assert_eq!(acls.len(), 0);
}

#[tokio::test]
async fn test_redis_acl_get() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/redis_acls/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_redis_acl()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl.uid, 1);
    assert_eq!(acl.name, "read_only_acl");
    assert_eq!(acl.acl, "+@read -@write");
    assert_eq!(acl.description, Some("Read-only access to Redis commands".to_string()));
}

#[tokio::test]
async fn test_redis_acl_get_minimal() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/redis_acls/3"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(minimal_redis_acl()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.get(3).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl.uid, 3);
    assert_eq!(acl.name, "minimal_acl");
    assert_eq!(acl.acl, "+ping +info");
    assert!(acl.description.is_none());
}

#[tokio::test]
async fn test_redis_acl_create() {
    let mock_server = MockServer::start().await;
    
    let create_request = CreateRedisAclRequest {
        name: "new_acl".to_string(),
        acl: "+@read +@string -del".to_string(),
        description: Some("Custom read access with string operations".to_string()),
    };

    Mock::given(method("POST"))
        .and(path("/v1/redis_acls"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&create_request))
        .respond_with(created_response(json!({
            "uid": 5,
            "name": "new_acl",
            "acl": "+@read +@string -del",
            "description": "Custom read access with string operations"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.create(create_request).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl.uid, 5);
    assert_eq!(acl.name, "new_acl");
    assert_eq!(acl.acl, "+@read +@string -del");
    assert_eq!(acl.description, Some("Custom read access with string operations".to_string()));
}

#[tokio::test]
async fn test_redis_acl_create_minimal() {
    let mock_server = MockServer::start().await;
    
    let create_request = CreateRedisAclRequest {
        name: "simple_acl".to_string(),
        acl: "+ping".to_string(),
        description: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/redis_acls"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&create_request))
        .respond_with(created_response(json!({
            "uid": 6,
            "name": "simple_acl",
            "acl": "+ping"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.create(create_request).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl.uid, 6);
    assert_eq!(acl.name, "simple_acl");
    assert_eq!(acl.acl, "+ping");
    assert!(acl.description.is_none());
}

#[tokio::test]
async fn test_redis_acl_update() {
    let mock_server = MockServer::start().await;
    
    let update_request = CreateRedisAclRequest {
        name: "updated_acl".to_string(),
        acl: "+@all -flushall -config".to_string(),
        description: Some("Updated ACL with restrictions".to_string()),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/redis_acls/1"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&update_request))
        .respond_with(success_response(json!({
            "uid": 1,
            "name": "updated_acl",
            "acl": "+@all -flushall -config",
            "description": "Updated ACL with restrictions"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.update(1, update_request).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl.uid, 1);
    assert_eq!(acl.name, "updated_acl");
    assert_eq!(acl.acl, "+@all -flushall -config");
    assert_eq!(acl.description, Some("Updated ACL with restrictions".to_string()));
}

#[tokio::test]
async fn test_redis_acl_update_remove_description() {
    let mock_server = MockServer::start().await;
    
    let update_request = CreateRedisAclRequest {
        name: "no_desc_acl".to_string(),
        acl: "+@read".to_string(),
        description: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/redis_acls/2"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&update_request))
        .respond_with(success_response(json!({
            "uid": 2,
            "name": "no_desc_acl",
            "acl": "+@read"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.update(2, update_request).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl.uid, 2);
    assert_eq!(acl.name, "no_desc_acl");
    assert_eq!(acl.acl, "+@read");
    assert!(acl.description.is_none());
}

#[tokio::test]
async fn test_redis_acl_delete() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("DELETE"))
        .and(path("/v1/redis_acls/1"))
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

    let handler = RedisAclHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_redis_acl_get_nonexistent() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/redis_acls/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Redis ACL not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_redis_acl_create_invalid_name() {
    let mock_server = MockServer::start().await;
    
    let invalid_request = CreateRedisAclRequest {
        name: "".to_string(), // Invalid empty name
        acl: "+ping".to_string(),
        description: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/redis_acls"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&invalid_request))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid ACL name",
            "code": "INVALID_NAME"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.create(invalid_request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_redis_acl_create_invalid_acl_syntax() {
    let mock_server = MockServer::start().await;
    
    let invalid_request = CreateRedisAclRequest {
        name: "test_acl".to_string(),
        acl: "invalid-acl-syntax".to_string(), // Invalid ACL syntax
        description: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/redis_acls"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&invalid_request))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid ACL syntax",
            "code": "INVALID_ACL_SYNTAX"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.create(invalid_request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_redis_acl_update_nonexistent() {
    let mock_server = MockServer::start().await;
    
    let update_request = CreateRedisAclRequest {
        name: "updated_acl".to_string(),
        acl: "+@read".to_string(),
        description: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/redis_acls/999"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&update_request))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Redis ACL not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.update(999, update_request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_redis_acl_delete_nonexistent() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("DELETE"))
        .and(path("/v1/redis_acls/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Redis ACL not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RedisAclHandler::new(client);
    let result = handler.delete(999).await;

    assert!(result.is_err());
}