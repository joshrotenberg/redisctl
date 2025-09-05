use redis_cloud::{CloudClient, FixedSubscriptionsHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_all_fixed_subscriptions_plans() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "plans": [
                {
                    "id": "plan-1",
                    "name": "Cache 250MB",
                    "size": 250,
                    "sizeMeasurementUnit": "MB",
                    "price": 10,
                    "region": "us-east-1"
                },
                {
                    "id": "plan-2",
                    "name": "Cache 1GB",
                    "size": 1,
                    "sizeMeasurementUnit": "GB",
                    "price": 25,
                    "region": "us-west-2"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.get_all_fixed_subscriptions_plans().await.unwrap();

    // Check that the extra field contains the expected plans
    assert!(result.extra.get("plans").is_some());
    let plans = result.extra.get("plans").unwrap().as_array().unwrap();
    assert_eq!(plans.len(), 2);
}

#[tokio::test]
async fn test_get_fixed_subscriptions_plans_by_subscription_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscription": {
                "subscriptionId": 123,
                "planId": "plan-1"
            },
            "plans": [
                {
                    "id": "plan-1",
                    "name": "Current Plan",
                    "size": 500,
                    "price": 15
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler
        .get_fixed_subscriptions_plans_by_subscription_id(123)
        .await
        .unwrap();

    // Check that the extra field contains the expected data
    assert!(result.extra.get("subscription").is_some());
    assert!(result.extra.get("plans").is_some());
}

#[tokio::test]
async fn test_get_fixed_subscriptions_plan_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Cache 2GB",
            "size": 2,
            "sizeMeasurementUnit": "GB",
            "price": 50,
            "region": "eu-west-1"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler
        .get_fixed_subscriptions_plan_by_id(123)
        .await
        .unwrap();

    assert_eq!(result.id, Some(123));
    assert_eq!(result.name, Some("Cache 2GB".to_string()));
}

#[tokio::test]
async fn test_get_redis_versions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/redis-versions"))
        .and(query_param("subscriptionId", "123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "redisVersions": [
                {
                    "version": "7.2",
                    "isDefault": true
                },
                {
                    "version": "7.0",
                    "isDefault": false
                },
                {
                    "version": "6.2",
                    "isDefault": false
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.get_fixed_redis_versions(123).await.unwrap();

    assert!(result.redis_versions.is_some());
    let versions = result.redis_versions.unwrap();
    assert_eq!(versions.len(), 3);
}

#[tokio::test]
async fn test_get_all_subscriptions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 456,
            "subscriptions": [
                {
                    "id": 123,
                    "name": "Production Fixed",
                    "status": "active",
                    "paymentMethod": "credit-card"
                },
                {
                    "id": 124,
                    "name": "Staging Fixed",
                    "status": "active",
                    "paymentMethod": "marketplace"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.get_all_fixed_subscriptions().await.unwrap();

    assert_eq!(result.account_id, Some(456));
    assert!(result.extra.get("subscriptions").is_some());
}

#[tokio::test]
async fn test_create_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-fixed-sub",
            "commandType": "CREATE_FIXED_SUBSCRIPTION",
            "status": "processing",
            "description": "Creating fixed subscription",
            "timestamp": "2024-01-01T00:00:00Z",
            "response": {
                "resourceId": 125
            }
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let request = redis_cloud::fixed_subscriptions::FixedSubscriptionCreateRequest {
        name: "New Fixed Subscription".to_string(),
        plan_id: 123,
        payment_method: Some("credit-card".to_string()),
        payment_method_id: Some(1001),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_fixed_subscription(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-fixed-sub".to_string()));
}

#[tokio::test]
async fn test_delete_subscription_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/fixed/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-fixed-sub",
            "commandType": "DELETE_FIXED_SUBSCRIPTION",
            "status": "processing",
            "description": "Deleting fixed subscription"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.delete_fixed_subscription_by_id(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-delete-fixed-sub".to_string()));
}

#[tokio::test]
async fn test_get_subscription_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Production Fixed",
            "status": "active",
            "paymentMethod": "credit-card",
            "numberOfDatabases": 5,
            "planId": 1,
            "createdDate": "2024-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.get_fixed_subscription_by_id(123).await.unwrap();

    assert_eq!(result.id, Some(123));
    assert_eq!(result.name, Some("Production Fixed".to_string()));
}

#[tokio::test]
async fn test_update_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/fixed/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-fixed-sub",
            "commandType": "UPDATE_FIXED_SUBSCRIPTION",
            "status": "processing",
            "description": "Updating fixed subscription"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let request = redis_cloud::fixed_subscriptions::FixedSubscriptionUpdateRequest {
        name: Some("Updated Fixed Subscription".to_string()),
        plan_id: Some(124),
        payment_method: Some("credit-card".to_string()),
        payment_method_id: Some(1002),
        subscription_id: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .update_fixed_subscription(123, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-fixed-sub".to_string()));
}

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": "Invalid API credentials"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("wrong-key".to_string())
        .api_secret("wrong-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.get_all_fixed_subscriptions().await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::AuthenticationFailed { .. }) => {}
        _ => panic!("Expected AuthenticationFailed error"),
    }
}

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/999"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Subscription not found"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let result = handler.get_fixed_subscription_by_id(999).await;

    assert!(result.is_err());
    if let Err(redis_cloud::CloudError::NotFound { message }) = result {
        assert!(message.contains("not found") || message.contains("404"));
    } else {
        panic!("Expected NotFound error");
    }
}

#[tokio::test]
async fn test_error_handling_500() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "error": "Internal server error"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedSubscriptionsHandler::new(client);
    let request = redis_cloud::fixed_subscriptions::FixedSubscriptionCreateRequest {
        name: "Test Subscription".to_string(),
        plan_id: 100,
        payment_method: Some("credit-card".to_string()),
        payment_method_id: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_fixed_subscription(&request).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}
