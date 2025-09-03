use redis_cloud::{CloudClient, subscriptions::SubscriptionsHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_all_subscriptions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 456,
            "subscriptions": [
                {
                    "id": 123,
                    "name": "Production",
                    "status": "active",
                    "paymentMethodType": "credit-card"
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_all_subscriptions().await.unwrap();

    assert_eq!(result.account_id, Some(456));
    // The subscriptions are in the extra field
    assert!(result.extra.get("subscriptions").is_some());
}

#[tokio::test]
async fn test_create_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-sub",
            "commandType": "CREATE_SUBSCRIPTION",
            "status": "processing",
            "description": "Creating subscription",
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

    let handler = SubscriptionsHandler::new(client);
    // Using the SubscriptionCreateRequest with required fields
    let request = redis_cloud::subscriptions::SubscriptionCreateRequest {
        name: Some("New Subscription".to_string()),
        payment_method_id: Some(1001),
        payment_method: None,
        memory_storage: Some("ram".to_string()),
        persistent_storage_encryption_type: None,
        deployment_type: Some("single-region".to_string()),
        dry_run: None,
        cloud_providers: vec![redis_cloud::subscriptions::SubscriptionSpec {
            provider: Some("AWS".to_string()),
            cloud_account_id: Some(1001),
            regions: vec![redis_cloud::subscriptions::SubscriptionRegionSpec {
                region: "us-west-2".to_string(),
                multiple_availability_zones: None,
                preferred_availability_zones: None,
                networking: Some(
                    redis_cloud::subscriptions::SubscriptionRegionNetworkingSpec {
                        deployment_cidr: Some("10.0.0.0/20".to_string()),
                        vpc_id: None,
                        subnet_ids: None,
                        security_group_id: None,
                        extra: serde_json::Value::Null,
                    },
                ),
                extra: serde_json::Value::Null,
            }],
            extra: serde_json::Value::Null,
        }],
        databases: vec![],
        redis_version: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_subscription(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-sub".to_string()));
}

#[tokio::test]
async fn test_get_redis_versions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/redis-versions"))
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_redis_versions(Some(123)).await.unwrap();

    assert!(result.redis_versions.is_some());
}

#[tokio::test]
async fn test_delete_subscription_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-sub",
            "commandType": "DELETE_SUBSCRIPTION",
            "status": "processing"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SubscriptionsHandler::new(client);
    let result = handler.delete_subscription_by_id(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-delete-sub".to_string()));
}

#[tokio::test]
async fn test_get_subscription_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "name": "Production",
            "status": "active",
            "paymentMethodType": "credit-card"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_subscription_by_id(123).await.unwrap();

    assert_eq!(result.id, Some(123));
    assert_eq!(result.name, Some("Production".to_string()));
}

#[tokio::test]
async fn test_update_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-sub",
            "commandType": "UPDATE_SUBSCRIPTION",
            "status": "processing"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SubscriptionsHandler::new(client);
    let request = redis_cloud::subscriptions::BaseSubscriptionUpdateRequest {
        subscription_id: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_subscription(123, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-update-sub".to_string()));
}

#[tokio::test]
async fn test_get_cidr_white_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/cidr"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "response": {
                "security": {
                    "cidrIps": ["192.168.1.0/24", "10.0.0.0/8"],
                    "defaultForNewDatabases": true
                }
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_cidr_white_list(123).await.unwrap();

    assert!(result.response.is_some());
}

#[tokio::test]
async fn test_update_subscription_cidr_white_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/123/cidr"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-cidr",
            "commandType": "UPDATE_CIDR_WHITELIST",
            "status": "processing"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SubscriptionsHandler::new(client);
    let request = redis_cloud::subscriptions::CidrWhiteListUpdateRequest {
        subscription_id: None,
        cidr_ips: Some(vec!["192.168.0.0/16".to_string()]),
        security_group_ids: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .update_subscription_cidr_white_list(123, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-cidr".to_string()));
}

#[tokio::test]
async fn test_get_subscription_maintenance_windows() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/maintenance-windows"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "mode": "manual",
            "windows": [
                {
                    "startHour": 2,
                    "durationInHours": 4,
                    "days": ["Sunday", "Wednesday"]
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler
        .get_subscription_maintenance_windows(123)
        .await
        .unwrap();

    assert_eq!(result.mode, Some("manual".to_string()));
    assert!(result.windows.is_some());
}

#[tokio::test]
async fn test_update_subscription_maintenance_windows() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/123/maintenance-windows"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-maintenance",
            "commandType": "UPDATE_MAINTENANCE_WINDOWS",
            "status": "processing"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SubscriptionsHandler::new(client);
    let request = redis_cloud::subscriptions::SubscriptionMaintenanceWindowsSpec {
        mode: "automatic".to_string(),
        windows: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .update_subscription_maintenance_windows(123, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-maintenance".to_string()));
}

#[tokio::test]
async fn test_get_subscription_pricing() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/pricing"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscription": {
                "id": 123,
                "currentCost": 150.50,
                "estimatedCost": 175.00
            },
            "shardHourlyPrice": {
                "standard": 0.124,
                "multiAZ": 0.248
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_subscription_pricing(123).await.unwrap();

    // Check the extra field for subscription and pricing data
    assert!(result.pricing.is_some() || result.extra.get("subscription").is_some());
}

// Skipping test_delete_regions_from_active_active_subscription
// as the client doesn't yet support DELETE with body

#[tokio::test]
async fn test_get_regions_from_active_active_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/regions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "regions": [
                {
                    "region": "us-east-1",
                    "provider": "AWS",
                    "status": "active"
                },
                {
                    "region": "eu-west-1",
                    "provider": "AWS",
                    "status": "active"
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler
        .get_regions_from_active_active_subscription(123)
        .await
        .unwrap();

    // The regions are in the extra field
    assert!(result.extra.get("regions").is_some());
}

#[tokio::test]
async fn test_add_new_region_to_active_active_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/regions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-add-region",
            "commandType": "ADD_REGION",
            "status": "processing"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SubscriptionsHandler::new(client);
    let request = redis_cloud::subscriptions::ActiveActiveRegionCreateRequest {
        subscription_id: None,
        region: Some("ap-southeast-1".to_string()),
        vpc_id: None,
        deployment_cidr: "10.1.0.0/20".to_string(),
        dry_run: None,
        databases: None,
        resp_version: None,
        customer_managed_key_resource_name: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .add_new_region_to_active_active_subscription(123, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-add-region".to_string()));
}

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions"))
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_all_subscriptions().await;

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
        .and(path("/subscriptions/999"))
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

    let handler = SubscriptionsHandler::new(client);
    let result = handler.get_subscription_by_id(999).await;

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
        .and(path("/subscriptions"))
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

    let handler = SubscriptionsHandler::new(client);
    let request = redis_cloud::subscriptions::SubscriptionCreateRequest {
        name: Some("Test Subscription".to_string()),
        payment_method_id: None,
        payment_method: None,
        memory_storage: None,
        persistent_storage_encryption_type: None,
        deployment_type: Some("single-region".to_string()),
        dry_run: None,
        cloud_providers: vec![],
        databases: vec![],
        redis_version: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_subscription(&request).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}
