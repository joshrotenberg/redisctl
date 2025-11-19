//! Test for tower-resilience retry integration
//!
//! This test verifies that CloudClient works with tower-resilience retry middleware
//! now that CloudError implements Clone.

#![cfg(feature = "tower-integration")]

use redis_cloud::CloudClient;
use redis_cloud::tower_support::ApiRequest;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tower::{ServiceBuilder, ServiceExt};
use tower_resilience::retry::RetryLayer;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_tower_resilience_retry_with_cloneable_error() {
    let mock_server = MockServer::start().await;
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // Mock endpoint that fails twice then succeeds
    Mock::given(method("GET"))
        .and(path("/subscriptions"))
        .respond_with(move |_req: &wiremock::Request| {
            let count = counter_clone.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                // First two requests fail with 503
                ResponseTemplate::new(503).set_body_json(json!({
                    "error": "Service temporarily unavailable"
                }))
            } else {
                // Third request succeeds
                ResponseTemplate::new(200).set_body_json(json!({"subscriptions": []}))
            }
        })
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .expect("Failed to create client");

    // Build retry layer with tower-resilience - now works because CloudError is Clone!
    let retry_layer = RetryLayer::<redis_cloud::CloudError>::builder()
        .name("cloud-api-retry")
        .max_attempts(3)
        .exponential_backoff(Duration::from_millis(10))
        .build();

    let service = ServiceBuilder::new()
        .layer(retry_layer)
        .service(client.into_service());

    let response = service
        .oneshot(ApiRequest::get("/subscriptions"))
        .await
        .expect("Should succeed after retries");

    assert_eq!(response.status, 200);
    // Should have made 3 attempts (2 failures + 1 success)
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}
