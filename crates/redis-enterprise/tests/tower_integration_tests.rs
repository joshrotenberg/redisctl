//! Tests for Tower service integration
//!
//! These tests verify that EnterpriseClient works correctly as a Tower service,
//! including middleware composition and service traits.

#![cfg(feature = "tower-integration")]

use redis_enterprise::EnterpriseClient;
use redis_enterprise::tower_support::{ApiRequest, Method};
use serde_json::json;
use tower::{Service, ServiceExt};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_tower_service_get_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .and(header(
            "authorization",
            "Basic dGVzdC11c2VyOnRlc3QtcGFzcw==",
        )) // test-user:test-pass
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "test-cluster",
            "nodes_count": 3
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::get("/v1/cluster");
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["name"], "test-cluster");
    assert_eq!(response.body["nodes_count"], 3);
}

#[tokio::test]
async fn test_tower_service_post_request() {
    let mock_server = MockServer::start().await;

    let request_body = json!({
        "name": "test-database",
        "memory_size": 1073741824,
        "replication": true
    });

    Mock::given(method("POST"))
        .and(path("/v1/bdbs"))
        .and(header(
            "authorization",
            "Basic dGVzdC11c2VyOnRlc3QtcGFzcw==",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "uid": 1,
            "name": "test-database",
            "memory_size": 1073741824
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::post("/v1/bdbs", request_body);
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["uid"], 1);
    assert_eq!(response.body["name"], "test-database");
}

#[tokio::test]
async fn test_tower_service_put_request() {
    let mock_server = MockServer::start().await;

    let request_body = json!({
        "memory_size": 2147483648i64
    });

    Mock::given(method("PUT"))
        .and(path("/v1/bdbs/1"))
        .and(header(
            "authorization",
            "Basic dGVzdC11c2VyOnRlc3QtcGFzcw==",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "uid": 1,
            "memory_size": 2147483648i64
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::put("/v1/bdbs/1", request_body);
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["uid"], 1);
    assert_eq!(response.body["memory_size"], 2147483648i64);
}

#[tokio::test]
async fn test_tower_service_patch_request() {
    let mock_server = MockServer::start().await;

    let request_body = json!({
        "name": "updated-name"
    });

    Mock::given(method("PATCH"))
        .and(path("/v1/bdbs/1"))
        .and(header(
            "authorization",
            "Basic dGVzdC11c2VyOnRlc3QtcGFzcw==",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "uid": 1,
            "name": "updated-name"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::patch("/v1/bdbs/1", request_body);
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["name"], "updated-name");
}

#[tokio::test]
async fn test_tower_service_delete_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/bdbs/1"))
        .and(header(
            "authorization",
            "Basic dGVzdC11c2VyOnRlc3QtcGFzcw==",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::delete("/v1/bdbs/1");
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn test_tower_service_oneshot() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .and(header(
            "authorization",
            "Basic dGVzdC11c2VyOnRlc3QtcGFzcw==",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "uid": 1,
                "addr": "192.168.1.10",
                "status": "active"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let service = client.into_service();

    // Use oneshot for single request
    let request = ApiRequest::get("/v1/nodes");
    let response = service
        .oneshot(request)
        .await
        .expect("Oneshot request failed");

    assert_eq!(response.status, 200);
    assert!(response.body.is_array());
    assert_eq!(response.body[0]["uid"], 1);
}

#[tokio::test]
async fn test_tower_service_multiple_requests() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "cluster1"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    // First request
    let response1 = service
        .ready()
        .await
        .expect("Service not ready")
        .call(ApiRequest::get("/v1/cluster"))
        .await
        .expect("First request failed");

    assert_eq!(response1.status, 200);
    assert_eq!(response1.body["name"], "cluster1");

    // Second request
    let response2 = service
        .ready()
        .await
        .expect("Service not ready")
        .call(ApiRequest::get("/v1/nodes"))
        .await
        .expect("Second request failed");

    assert_eq!(response2.status, 200);
    assert!(response2.body.is_array());

    // Third request
    let response3 = service
        .ready()
        .await
        .expect("Service not ready")
        .call(ApiRequest::get("/v1/bdbs"))
        .await
        .expect("Third request failed");

    assert_eq!(response3.status, 200);
    assert!(response3.body.is_array());
}

#[tokio::test]
async fn test_tower_service_error_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error_code": "bdb_not_exist",
            "description": "Database does not exist"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::get("/v1/bdbs/999");
    let result = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_tower_service_authentication_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error_code": "unauthorized"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("wrong-user")
        .password("wrong-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::get("/v1/cluster");
    let result = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_api_request_method_constructors() {
    // Test all the convenience constructors
    let get_req = ApiRequest::get("/v1/cluster");
    assert_eq!(get_req.method, Method::Get);
    assert_eq!(get_req.path, "/v1/cluster");
    assert!(get_req.body.is_none());

    let post_req = ApiRequest::post("/v1/bdbs", json!({"name": "test"}));
    assert_eq!(post_req.method, Method::Post);
    assert_eq!(post_req.path, "/v1/bdbs");
    assert!(post_req.body.is_some());

    let put_req = ApiRequest::put("/v1/bdbs/1", json!({"name": "updated"}));
    assert_eq!(put_req.method, Method::Put);
    assert!(put_req.body.is_some());

    let patch_req = ApiRequest::patch("/v1/bdbs/1", json!({"name": "patched"}));
    assert_eq!(patch_req.method, Method::Patch);
    assert!(patch_req.body.is_some());

    let delete_req = ApiRequest::delete("/v1/bdbs/1");
    assert_eq!(delete_req.method, Method::Delete);
    assert!(delete_req.body.is_none());
}

#[tokio::test]
async fn test_tower_service_post_without_body_fails() {
    let mock_server = MockServer::start().await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    // Manually construct a POST request without a body
    let request = ApiRequest {
        method: Method::Post,
        path: "/v1/bdbs".to_string(),
        body: None,
    };

    let result = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("body"));
}

#[tokio::test]
async fn test_tower_service_put_without_body_fails() {
    let mock_server = MockServer::start().await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    // Manually construct a PUT request without a body
    let request = ApiRequest {
        method: Method::Put,
        path: "/v1/bdbs/1".to_string(),
        body: None,
    };

    let result = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("body"));
}

#[tokio::test]
async fn test_tower_service_patch_without_body_fails() {
    let mock_server = MockServer::start().await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    // Manually construct a PATCH request without a body
    let request = ApiRequest {
        method: Method::Patch,
        path: "/v1/bdbs/1".to_string(),
        body: None,
    };

    let result = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("body"));
}
