//! JSON Schema endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, JsonSchemaHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_database_schema() -> serde_json::Value {
    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "uid": {
                "type": "integer",
                "description": "Database unique identifier"
            },
            "name": {
                "type": "string",
                "description": "Database name"
            },
            "memory_size": {
                "type": "integer",
                "description": "Memory size in bytes"
            },
            "port": {
                "type": "integer",
                "minimum": 1,
                "maximum": 65535
            },
            "replication": {
                "type": "boolean",
                "description": "Enable replication"
            }
        },
        "required": ["name", "memory_size"]
    })
}

fn test_cluster_schema() -> serde_json::Value {
    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "Cluster name"
            },
            "nodes": {
                "type": "array",
                "items": {
                    "type": "object"
                }
            },
            "license": {
                "type": "object"
            }
        },
        "required": ["name"]
    })
}

fn test_user_schema() -> serde_json::Value {
    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "uid": {
                "type": "integer"
            },
            "username": {
                "type": "string",
                "minLength": 3,
                "maxLength": 32
            },
            "email": {
                "type": "string",
                "format": "email"
            },
            "role": {
                "type": "string",
                "enum": ["admin", "db_viewer", "db_member", "cluster_viewer", "cluster_member"]
            }
        },
        "required": ["username", "role"]
    })
}

#[tokio::test]
async fn test_jsonschema_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            "bdb",
            "cluster",
            "node",
            "user",
            "crdb",
            "module",
            "redis_acl",
            "role"
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let schemas = result.unwrap();
    assert_eq!(schemas.len(), 8);
    assert!(schemas.contains(&"bdb".to_string()));
    assert!(schemas.contains(&"cluster".to_string()));
    assert!(schemas.contains(&"user".to_string()));
}

#[tokio::test]
async fn test_jsonschema_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let schemas = result.unwrap();
    assert_eq!(schemas.len(), 0);
}

#[tokio::test]
async fn test_jsonschema_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/bdb"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_database_schema()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.get("bdb").await;

    assert!(result.is_ok());
    let schema = result.unwrap();
    assert_eq!(schema["$schema"], "http://json-schema.org/draft-07/schema#");
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"]["name"].is_object());
    assert!(schema["required"].is_array());
}

#[tokio::test]
async fn test_jsonschema_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Schema not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_jsonschema_database_schema() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/bdb"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_database_schema()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.database_schema().await;

    assert!(result.is_ok());
    let schema = result.unwrap();
    assert!(schema["properties"]["uid"].is_object());
    assert!(schema["properties"]["memory_size"].is_object());
    assert_eq!(schema["properties"]["port"]["minimum"], 1);
    assert_eq!(schema["properties"]["port"]["maximum"], 65535);
}

#[tokio::test]
async fn test_jsonschema_cluster_schema() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/cluster"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_cluster_schema()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.cluster_schema().await;

    assert!(result.is_ok());
    let schema = result.unwrap();
    assert!(schema["properties"]["name"].is_object());
    assert!(schema["properties"]["nodes"].is_object());
    assert_eq!(schema["properties"]["nodes"]["type"], "array");
}

#[tokio::test]
async fn test_jsonschema_node_schema() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/node"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "uid": {"type": "integer"},
                "addr": {"type": "string", "format": "ipv4"},
                "port": {"type": "integer"},
                "status": {"type": "string", "enum": ["active", "inactive", "maintenance"]}
            }
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.node_schema().await;

    assert!(result.is_ok());
    let schema = result.unwrap();
    assert!(schema["properties"]["addr"].is_object());
    assert_eq!(schema["properties"]["addr"]["format"], "ipv4");
}

#[tokio::test]
async fn test_jsonschema_user_schema() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/user"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_user_schema()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.user_schema().await;

    assert!(result.is_ok());
    let schema = result.unwrap();
    assert!(schema["properties"]["username"].is_object());
    assert_eq!(schema["properties"]["username"]["minLength"], 3);
    assert_eq!(schema["properties"]["username"]["maxLength"], 32);
    assert_eq!(schema["properties"]["email"]["format"], "email");
}

#[tokio::test]
async fn test_jsonschema_crdb_schema() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/jsonschema/crdb"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "guid": {"type": "string"},
                "name": {"type": "string"},
                "instances": {
                    "type": "array",
                    "items": {"type": "object"}
                },
                "encryption": {"type": "boolean"},
                "replication": {"type": "boolean"}
            }
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.crdb_schema().await;

    assert!(result.is_ok());
    let schema = result.unwrap();
    assert!(schema["properties"]["guid"].is_object());
    assert!(schema["properties"]["instances"].is_object());
    assert_eq!(schema["properties"]["instances"]["type"], "array");
}

#[tokio::test]
async fn test_jsonschema_validate_valid() {
    let mock_server = MockServer::start().await;

    let valid_database = json!({
        "name": "my-database",
        "memory_size": 1073741824,
        "port": 6379,
        "replication": true
    });

    Mock::given(method("POST"))
        .and(path("/v1/jsonschema/bdb/validate"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&valid_database))
        .respond_with(success_response(json!({
            "valid": true,
            "errors": []
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.validate("bdb", &valid_database).await;

    assert!(result.is_ok());
    let validation = result.unwrap();
    assert!(validation["valid"].as_bool().unwrap());
    assert_eq!(validation["errors"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_jsonschema_validate_invalid() {
    let mock_server = MockServer::start().await;

    let invalid_database = json!({
        "name": "my-database",
        // Missing required field: memory_size
        "port": 999999, // Invalid port number
        "replication": "yes" // Invalid type, should be boolean
    });

    Mock::given(method("POST"))
        .and(path("/v1/jsonschema/bdb/validate"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&invalid_database))
        .respond_with(success_response(json!({
            "valid": false,
            "errors": [
                {
                    "field": "memory_size",
                    "error": "required field missing"
                },
                {
                    "field": "port",
                    "error": "value 999999 exceeds maximum of 65535"
                },
                {
                    "field": "replication",
                    "error": "expected boolean, got string"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.validate("bdb", &invalid_database).await;

    assert!(result.is_ok());
    let validation = result.unwrap();
    assert!(!validation["valid"].as_bool().unwrap());
    assert_eq!(validation["errors"].as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn test_jsonschema_validate_nonexistent_schema() {
    let mock_server = MockServer::start().await;

    let object = json!({"test": "data"});

    Mock::given(method("POST"))
        .and(path("/v1/jsonschema/nonexistent/validate"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&object))
        .respond_with(error_response(404, "Schema not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.validate("nonexistent", &object).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_jsonschema_validate_empty_object() {
    let mock_server = MockServer::start().await;

    let empty_object = json!({});

    Mock::given(method("POST"))
        .and(path("/v1/jsonschema/user/validate"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&empty_object))
        .respond_with(success_response(json!({
            "valid": false,
            "errors": [
                {
                    "field": "username",
                    "error": "required field missing"
                },
                {
                    "field": "role",
                    "error": "required field missing"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = JsonSchemaHandler::new(client);
    let result = handler.validate("user", &empty_object).await;

    assert!(result.is_ok());
    let validation = result.unwrap();
    assert!(!validation["valid"].as_bool().unwrap());
    assert_eq!(validation["errors"].as_array().unwrap().len(), 2);
}
