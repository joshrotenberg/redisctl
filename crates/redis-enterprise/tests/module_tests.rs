//! Module endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, ModuleHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
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

fn test_module() -> serde_json::Value {
    json!({
        "uid": "1",
        "module_name": "RedisSearch",
        "version": 20601,
        "semantic_version": "2.6.1",
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
                "uid": "2",
                "module_name": "RedisJSON",
                "version": 20400,
                "semantic_version": "2.4.0",
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
    assert_eq!(module.module_name, Some("RedisSearch".to_string()));
}

#[tokio::test]
async fn test_module_upload() {
    let mock_server = MockServer::start().await;

    // Mock v2 endpoint as not found
    Mock::given(method("POST"))
        .and(path("/v2/modules"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    // Mock v1 endpoint as success
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
    let result = handler.upload(vec![1, 2, 3, 4], "test.zip").await; // Mock binary data

    assert!(result.is_ok());
    let response = result.unwrap();
    // Response is now a Value, not a Module
    assert_eq!(response["uid"], "1");
    assert_eq!(response["module_name"], "RedisSearch");
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
