//! Tests for common types

use redis_cloud::types::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[test]
fn test_task_state_update_serialization() {
    let task = TaskStateUpdate {
        task_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        command_type: Some("createDatabase".to_string()),
        status: Some(TaskStatus::ProcessingInProgress),
        description: Some("Creating database".to_string()),
        timestamp: Some("2023-12-01T10:00:00Z".to_string()),
        response: None,
        links: None,
    };

    let json_str = serde_json::to_string(&task).unwrap();
    let parsed: TaskStateUpdate = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.task_id, task.task_id);
    assert_eq!(parsed.command_type, task.command_type);
    assert!(matches!(
        parsed.status,
        Some(TaskStatus::ProcessingInProgress)
    ));
}

#[test]
fn test_task_status_enum_serialization() {
    // Test kebab-case serialization
    let status = TaskStatus::ProcessingInProgress;
    let json_str = serde_json::to_string(&status).unwrap();
    assert_eq!(json_str, "\"processing-in-progress\"");

    // Test deserialization
    let parsed: TaskStatus = serde_json::from_str("\"processing-completed\"").unwrap();
    assert!(matches!(parsed, TaskStatus::ProcessingCompleted));
}

#[test]
fn test_processor_response_with_error() {
    let response = ProcessorResponse {
        resource_id: None,
        additional_resource_id: None,
        resource: None,
        error: Some(ProcessorError::Unauthorized),
    };

    let json_str = serde_json::to_string(&response).unwrap();
    assert!(json_str.contains("\"UNAUTHORIZED\""));

    let parsed: ProcessorResponse = serde_json::from_str(&json_str).unwrap();
    assert!(matches!(parsed.error, Some(ProcessorError::Unauthorized)));
}

#[test]
fn test_processor_response_with_resource() {
    let resource_data = json!({
        "databaseId": 12345,
        "name": "my-database",
        "status": "active"
    });

    let response = ProcessorResponse {
        resource_id: Some(12345),
        additional_resource_id: None,
        resource: Some(resource_data.clone()),
        error: None,
    };

    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: ProcessorResponse = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.resource_id, Some(12345));
    assert_eq!(parsed.resource, Some(resource_data));
}

#[test]
fn test_cloud_tags() {
    let tags = CloudTags {
        tags: vec![
            CloudTag {
                key: "environment".to_string(),
                value: "production".to_string(),
            },
            CloudTag {
                key: "team".to_string(),
                value: "platform".to_string(),
            },
        ],
        extra: json!({}),
    };

    let json_str = serde_json::to_string(&tags).unwrap();
    let parsed: CloudTags = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.tags.len(), 2);
    assert_eq!(parsed.tags[0].key, "environment");
    assert_eq!(parsed.tags[0].value, "production");
}

#[test]
fn test_paginated_response() {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestData {
        items: Vec<String>,
    }

    let response = PaginatedResponse {
        data: TestData {
            items: vec!["item1".to_string(), "item2".to_string()],
        },
        offset: Some(0),
        limit: Some(10),
        total: Some(2),
    };

    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: PaginatedResponse<TestData> = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.offset, Some(0));
    assert_eq!(parsed.limit, Some(10));
    assert_eq!(parsed.total, Some(2));
    assert_eq!(parsed.data.items.len(), 2);
}

#[test]
fn test_link_with_optional_fields() {
    let link = Link {
        rel: "self".to_string(),
        href: "/subscriptions/123".to_string(),
        method: Some("GET".to_string()),
        r#type: Some("application/json".to_string()),
    };

    let json_str = serde_json::to_string(&link).unwrap();
    assert!(json_str.contains("\"rel\":\"self\""));
    assert!(json_str.contains("\"href\":\"/subscriptions/123\""));
    assert!(json_str.contains("\"method\":\"GET\""));
    assert!(json_str.contains("\"type\":\"application/json\""));
}

#[test]
fn test_cloud_provider_enum() {
    // Test uppercase serialization
    let provider = CloudProvider::Aws;
    let json_str = serde_json::to_string(&provider).unwrap();
    assert_eq!(json_str, "\"AWS\"");

    // Test deserialization
    let parsed: CloudProvider = serde_json::from_str("\"GCP\"").unwrap();
    assert!(matches!(parsed, CloudProvider::Gcp));
}

#[test]
fn test_data_persistence_enum() {
    // Test kebab-case serialization
    let persistence = DataPersistence::AofEvery1Sec;
    let json_str = serde_json::to_string(&persistence).unwrap();
    assert_eq!(json_str, "\"aof-every-1-sec\"");

    // Test deserialization
    let parsed: DataPersistence = serde_json::from_str("\"snapshot-every-6-hours\"").unwrap();
    assert!(matches!(parsed, DataPersistence::SnapshotEvery6Hours));
}

#[test]
fn test_database_status_enum() {
    let status = DatabaseStatus::ActiveChangePending;
    let json_str = serde_json::to_string(&status).unwrap();
    // Should be lowercase
    assert_eq!(json_str, "\"activechangepending\"");
}

#[test]
fn test_error_response() {
    let error = ErrorResponse {
        error: Some("VALIDATION_ERROR".to_string()),
        message: Some("Invalid request parameters".to_string()),
        description: Some("The 'name' field is required".to_string()),
        status_code: Some(400),
    };

    let json_str = serde_json::to_string(&error).unwrap();
    let parsed: ErrorResponse = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.error, Some("VALIDATION_ERROR".to_string()));
    assert_eq!(
        parsed.message,
        Some("Invalid request parameters".to_string())
    );
    assert_eq!(parsed.status_code, Some(400));
}

#[test]
fn test_empty_response() {
    let empty = EmptyResponse {};
    let json_str = serde_json::to_string(&empty).unwrap();
    assert_eq!(json_str, "{}");

    let parsed: EmptyResponse = serde_json::from_str("{}").unwrap();
    assert!(matches!(parsed, EmptyResponse {}));
}

// Test real-world JSON examples from the API
#[test]
fn test_deserialize_real_task_response() {
    let json = json!({
        "taskId": "ce6e0b70-7d7c-4c99-918f-3a0e1e8e7814",
        "commandType": "subscriptionCreateRequest",
        "status": "processing-completed",
        "description": "Creating subscription",
        "timestamp": "2023-12-01T10:30:45Z",
        "response": {
            "resourceId": 51234,
            "resource": {
                "subscriptionId": 51234,
                "name": "my-subscription",
                "status": "active"
            }
        },
        "links": []
    });

    let task: TaskStateUpdate = serde_json::from_value(json).unwrap();
    assert_eq!(
        task.task_id.unwrap(),
        "ce6e0b70-7d7c-4c99-918f-3a0e1e8e7814"
    );
    assert!(matches!(task.status, Some(TaskStatus::ProcessingCompleted)));
    assert!(task.response.is_some());

    let response = task.response.unwrap();
    assert_eq!(response.resource_id, Some(51234));
    assert!(response.resource.is_some());
}

// Test that unknown fields are handled gracefully
#[test]
fn test_extra_fields_preserved() {
    let json = json!({
        "tags": [
            {"key": "env", "value": "prod"}
        ],
        "unknownField": "someValue",
        "anotherField": 123
    });

    let tags: CloudTags = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(tags.tags.len(), 1);

    // Extra fields should be preserved in the `extra` field
    let extra = tags.extra;
    assert_eq!(extra["unknownField"], "someValue");
    assert_eq!(extra["anotherField"], 123);
}
