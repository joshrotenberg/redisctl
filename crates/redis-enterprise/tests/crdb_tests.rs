//! Active-Active (CRDB) endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, CrdbHandler, CreateCrdbRequest};
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

#[tokio::test]
async fn test_crdb_list() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/crdbs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "guid": "12345-abcdef",
                "name": "active-active-db",
                "status": "active",
                "memory_size": 1073741824,
                "instances": [
                    {
                        "id": 1,
                        "cluster": "cluster1.example.com",
                        "status": "active"
                    }
                ]
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

    let handler = CrdbHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let crdbs = result.unwrap();
    assert_eq!(crdbs.len(), 1);
}

#[tokio::test]
async fn test_crdb_get() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/v1/crdbs/12345-abcdef"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "guid": "12345-abcdef",
            "name": "active-active-db",
            "status": "active",
            "memory_size": 1073741824,
            "instances": []
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.get("12345-abcdef").await;

    assert!(result.is_ok());
    let crdb = result.unwrap();
    assert_eq!(crdb.guid, "12345-abcdef");
    assert_eq!(crdb.name, "active-active-db");
}

#[tokio::test]
async fn test_crdb_create() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v1/crdbs"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(json!({
            "guid": "new-crdb-guid",
            "name": "new-active-active",
            "status": "active",
            "memory_size": 2147483648u64,
            "instances": []
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let request = CreateCrdbRequest {
        name: "new-active-active".to_string(),
        memory_size: 2147483648,
        instances: vec![],
        encryption: None,
        data_persistence: None,
        eviction_policy: None,
    };
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let crdb = result.unwrap();
    assert_eq!(crdb.name, "new-active-active");
}

#[tokio::test]
async fn test_crdb_delete() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("DELETE"))
        .and(path("/v1/crdbs/12345-abcdef"))
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

    let handler = CrdbHandler::new(client);
    let result = handler.delete("12345-abcdef").await;

    assert!(result.is_ok());
}