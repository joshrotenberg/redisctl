//! Tests for Tower middleware composition with EnterpriseClient
//!
//! These tests demonstrate how EnterpriseClient integrates with Tower middleware
//! ecosystem for timeouts, rate limiting, buffering, and other patterns.

#![cfg(feature = "tower-integration")]

use redis_enterprise::EnterpriseClient;
use redis_enterprise::tower_support::ApiRequest;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::timeout::TimeoutLayer;
use tower::{Service, ServiceBuilder, ServiceExt};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_tower_with_timeout_middleware() {
    let mock_server = MockServer::start().await;

    // Mock a slow response
    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"name": "test-cluster"}))
                .set_delay(Duration::from_millis(100)),
        )
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    // Wrap with timeout middleware - should succeed (100ms delay < 500ms timeout)
    let service = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_millis(500)))
        .service(client.into_service());

    let response = service
        .oneshot(ApiRequest::get("/v1/cluster"))
        .await
        .expect("Request should succeed with sufficient timeout");

    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn test_tower_timeout_expires() {
    let mock_server = MockServer::start().await;

    // Mock a very slow response
    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!([]))
                .set_delay(Duration::from_millis(500)),
        )
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    // Wrap with short timeout - should timeout (500ms delay > 100ms timeout)
    let service = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_millis(100)))
        .service(client.into_service());

    let result = service.oneshot(ApiRequest::get("/v1/bdbs")).await;

    assert!(result.is_err(), "Request should timeout");
}

#[tokio::test]
async fn test_tower_with_rate_limiting() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name": "cluster1"})))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    // Rate limit: 2 requests per 500ms
    let mut service = ServiceBuilder::new()
        .layer(RateLimitLayer::new(2, Duration::from_millis(500)))
        .service(client.into_service());

    // First two requests should succeed immediately
    let start = std::time::Instant::now();

    service
        .ready()
        .await
        .expect("Service should be ready")
        .call(ApiRequest::get("/v1/cluster"))
        .await
        .expect("First request should succeed");

    service
        .ready()
        .await
        .expect("Service should be ready")
        .call(ApiRequest::get("/v1/cluster"))
        .await
        .expect("Second request should succeed");

    let elapsed = start.elapsed();

    // Both requests should complete quickly (under 100ms)
    assert!(
        elapsed < Duration::from_millis(100),
        "First two requests should not be rate limited"
    );

    // Third request should be delayed by rate limiter
    let start = std::time::Instant::now();

    service
        .ready()
        .await
        .expect("Service should be ready")
        .call(ApiRequest::get("/v1/cluster"))
        .await
        .expect("Third request should succeed after delay");

    let elapsed = start.elapsed();

    // Third request should be delayed (at least 300ms remaining from 500ms window)
    assert!(
        elapsed >= Duration::from_millis(300),
        "Third request should be rate limited"
    );
}

#[tokio::test]
async fn test_tower_with_buffer_layer() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    // Buffer layer with capacity of 10 requests
    let service = ServiceBuilder::new()
        .layer(BufferLayer::new(10))
        .service(client.into_service());

    // Make multiple concurrent requests
    let mut handles = vec![];
    for _ in 0..5 {
        let mut svc = service.clone();
        let handle = tokio::spawn(async move {
            svc.ready()
                .await
                .expect("Service should be ready")
                .call(ApiRequest::get("/v1/nodes"))
                .await
        });
        handles.push(handle);
    }

    // All requests should succeed
    for handle in handles {
        let result = handle.await.expect("Task should not panic");
        assert!(result.is_ok(), "Buffered request should succeed");
    }
}

#[tokio::test]
async fn test_tower_middleware_composition() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"name": "prod-cluster"}))
                .set_delay(Duration::from_millis(50)),
        )
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    // Compose multiple middleware layers
    let service = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        .layer(RateLimitLayer::new(10, Duration::from_secs(1)))
        .layer(BufferLayer::new(100))
        .service(client.into_service());

    let response = service
        .oneshot(ApiRequest::get("/v1/cluster"))
        .await
        .expect("Request with composed middleware should succeed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["name"], "prod-cluster");
}

#[tokio::test]
async fn test_tower_custom_middleware_request_counting() {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tower::Service;

    // Custom middleware that counts requests
    #[derive(Clone)]
    struct RequestCounter<S> {
        inner: S,
        counter: Arc<AtomicU32>,
    }

    impl<S, Request> Service<Request> for RequestCounter<S>
    where
        S: Service<Request>,
        S::Future: Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, request: Request) -> Self::Future {
            // Increment counter
            self.counter.fetch_add(1, Ordering::SeqCst);

            let fut = self.inner.call(request);
            Box::pin(fut)
        }
    }

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let counter = Arc::new(AtomicU32::new(0));
    let mut service = RequestCounter {
        inner: client.into_service(),
        counter: counter.clone(),
    };

    // Make 3 requests
    service
        .ready()
        .await
        .expect("Service ready")
        .call(ApiRequest::get("/v1/bdbs"))
        .await
        .expect("Request 1 failed");

    service
        .ready()
        .await
        .expect("Service ready")
        .call(ApiRequest::get("/v1/nodes"))
        .await
        .expect("Request 2 failed");

    service
        .ready()
        .await
        .expect("Service ready")
        .call(ApiRequest::get("/v1/bdbs"))
        .await
        .expect("Request 3 failed");

    // Counter should show 3 requests
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_tower_error_handling_through_middleware() {
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

    // Even with middleware, errors should propagate correctly
    let service = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        .layer(BufferLayer::new(10))
        .service(client.into_service());

    let result = service.oneshot(ApiRequest::get("/v1/bdbs/999")).await;

    assert!(
        result.is_err(),
        "404 error should propagate through middleware"
    );
}

#[tokio::test]
async fn test_tower_with_concurrent_requests_through_buffer() {
    let mock_server = MockServer::start().await;

    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(move |_req: &wiremock::Request| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(200).set_body_json(json!({"name": "cluster"}))
        })
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test-user")
        .password("test-pass")
        .build()
        .expect("Failed to create client");

    let service = ServiceBuilder::new()
        .layer(BufferLayer::new(50))
        .service(client.into_service());

    // Spawn 10 concurrent requests
    let mut handles = vec![];
    for _ in 0..10 {
        let mut svc = service.clone();
        let handle = tokio::spawn(async move {
            svc.ready()
                .await
                .expect("Service ready")
                .call(ApiRequest::get("/v1/cluster"))
                .await
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        handle.await.expect("Task panic").expect("Request failed");
    }

    // Verify all 10 requests were made
    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn test_tower_database_operations_with_middleware() {
    let mock_server = MockServer::start().await;

    // Mock database creation
    Mock::given(method("POST"))
        .and(path("/v1/bdbs"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({
                    "uid": 1,
                    "name": "test-db",
                    "memory_size": 1073741824
                }))
                .set_delay(Duration::from_millis(50)),
        )
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .expect("Failed to create client");

    // Compose timeout and rate limiting
    let service = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
        .service(client.into_service());

    let db_config = json!({
        "name": "test-db",
        "memory_size": 1073741824,
        "replication": true
    });

    let response = service
        .oneshot(ApiRequest::post("/v1/bdbs", db_config))
        .await
        .expect("Database creation should succeed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["uid"], 1);
    assert_eq!(response.body["name"], "test-db");
}
