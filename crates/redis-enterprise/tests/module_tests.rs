//! Module endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, ModuleHandler};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth};
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

fn test_module() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "RedisSearch",
        "version": "2.6.1",
        "status": "loaded",
        "capabilities": ["search", "index"]
    })
}

#[tokio::test]
async fn test_module_list() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/modules"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_module(),
            {
                "uid": 2,
                "name": "RedisJSON",
                "version": "2.4.0",
                "status": "loaded",
                "capabilities": ["json"]
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

    let handler = ModuleHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let modules = result.unwrap();
    assert_eq!(modules.len(), 2);
}

#[tokio::test]
async fn test_module_get() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/modules/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_module()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ModuleHandler::new(client);
    let result = handler.get("1").await;

    assert!(result.is_ok());
    let module = result.unwrap();
    assert_eq!(module.uid, "1");
    assert_eq!(module.name, "RedisSearch");
}

#[tokio::test]
async fn test_module_upload() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v1/modules"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(test_module()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ModuleHandler::new(client);
    let result = handler.upload(vec![1, 2, 3, 4]).await; // Mock binary data

    assert!(result.is_ok());
    let module = result.unwrap();
    assert_eq!(module.uid, "1");
    assert_eq!(module.name, "RedisSearch");
}

#[tokio::test]
async fn test_module_delete() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("DELETE"))
        .and(path("/v1/modules/1"))
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

    let handler = ModuleHandler::new(client);
    let result = handler.delete("1").await;

    assert!(result.is_ok());
}