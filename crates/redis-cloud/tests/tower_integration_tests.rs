//! Tests for Tower service integration
//!
//! These tests verify that CloudClient works correctly as a Tower service,
//! including middleware composition and service traits.

#![cfg(feature = "tower-integration")]

use redis_cloud::CloudClient;
use redis_cloud::tower_support::{ApiRequest, Method};
use serde_json::json;
use tower::{Service, ServiceExt};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_tower_service_get_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscriptions": []
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::get("/subscriptions");
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert!(response.body["subscriptions"].is_array());
}

#[tokio::test]
async fn test_tower_service_post_request() {
    let mock_server = MockServer::start().await;

    let request_body = json!({
        "name": "test-subscription",
        "cloudProviderId": 1
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "12345",
            "commandType": "subscriptionCreateRequest",
            "status": "processing-in-progress"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::post("/subscriptions", request_body);
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["taskId"], "12345");
    assert_eq!(response.body["status"], "processing-in-progress");
}

#[tokio::test]
async fn test_tower_service_put_request() {
    let mock_server = MockServer::start().await;

    let request_body = json!({
        "name": "updated-name"
    });

    Mock::given(method("PUT"))
        .and(path("/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "67890",
            "status": "processing-in-progress"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::put("/subscriptions/123", request_body);
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["taskId"], "67890");
}

#[tokio::test]
async fn test_tower_service_patch_request() {
    let mock_server = MockServer::start().await;

    let request_body = json!({
        "memory_limit_in_gb": 10.0
    });

    Mock::given(method("PATCH"))
        .and(path("/subscriptions/123/databases/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "patch-123",
            "status": "processing-in-progress"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::patch("/subscriptions/123/databases/456", request_body);
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["taskId"], "patch-123");
}

#[tokio::test]
async fn test_tower_service_delete_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "delete-123",
            "status": "processing-in-progress"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::delete("/subscriptions/123");
    let response = service
        .ready()
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["taskId"], "delete-123");
}

#[tokio::test]
async fn test_tower_service_oneshot() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/account"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 12345,
            "name": "Test Account"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let service = client.into_service();

    // Use oneshot for single request
    let request = ApiRequest::get("/account");
    let response = service
        .oneshot(request)
        .await
        .expect("Oneshot request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["id"], 12345);
    assert_eq!(response.body["name"], "Test Account");
}

#[tokio::test]
async fn test_tower_service_multiple_requests() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscriptions": []
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    // First request
    let response1 = service
        .ready()
        .await
        .expect("Service not ready")
        .call(ApiRequest::get("/subscriptions"))
        .await
        .expect("First request failed");

    assert_eq!(response1.status, 200);

    // Second request
    let response2 = service
        .ready()
        .await
        .expect("Service not ready")
        .call(ApiRequest::get("/account"))
        .await
        .expect("Second request failed");

    assert_eq!(response2.status, 200);
}

#[tokio::test]
async fn test_tower_service_error_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": {
                "type": "SUBSCRIPTION_NOT_FOUND",
                "status": "404 NOT_FOUND",
                "description": "Subscription not found"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    let request = ApiRequest::get("/subscriptions/999");
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
    let get_req = ApiRequest::get("/test");
    assert_eq!(get_req.method, Method::Get);
    assert_eq!(get_req.path, "/test");
    assert!(get_req.body.is_none());

    let post_req = ApiRequest::post("/test", json!({"key": "value"}));
    assert_eq!(post_req.method, Method::Post);
    assert_eq!(post_req.path, "/test");
    assert!(post_req.body.is_some());

    let put_req = ApiRequest::put("/test", json!({"key": "value"}));
    assert_eq!(put_req.method, Method::Put);
    assert!(put_req.body.is_some());

    let patch_req = ApiRequest::patch("/test", json!({"key": "value"}));
    assert_eq!(patch_req.method, Method::Patch);
    assert!(patch_req.body.is_some());

    let delete_req = ApiRequest::delete("/test");
    assert_eq!(delete_req.method, Method::Delete);
    assert!(delete_req.body.is_none());
}

#[tokio::test]
async fn test_tower_service_post_without_body_fails() {
    let mock_server = MockServer::start().await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    let mut service = client.into_service();

    // Manually construct a POST request without a body
    let request = ApiRequest {
        method: Method::Post,
        path: "/subscriptions".to_string(),
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
