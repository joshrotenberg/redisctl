//! Migrations endpoint tests for Redis Enterprise

use redis_enterprise::{
    CreateMigrationRequest, EnterpriseClient, MigrationEndpoint, MigrationsHandler,
};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
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

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_migration() -> serde_json::Value {
    json!({
        "migration_id": "migration-123",
        "source": {
            "endpoint_type": "external",
            "host": "source.redis.com",
            "port": 6379,
            "password": "source_password",
            "ssl": false
        },
        "target": {
            "endpoint_type": "bdb",
            "bdb_uid": 1
        },
        "status": "running",
        "progress": 45.5,
        "start_time": "2023-01-01T12:00:00Z"
    })
}

fn test_migration_completed() -> serde_json::Value {
    json!({
        "migration_id": "migration-456",
        "source": {
            "endpoint_type": "external",
            "host": "source.redis.com",
            "port": 6379,
            "password": "source_password",
            "ssl": true
        },
        "target": {
            "endpoint_type": "bdb",
            "bdb_uid": 2
        },
        "status": "completed",
        "progress": 100.0,
        "start_time": "2023-01-01T10:00:00Z",
        "end_time": "2023-01-01T11:30:00Z"
    })
}

fn test_migration_failed() -> serde_json::Value {
    json!({
        "migration_id": "migration-789",
        "source": {
            "endpoint_type": "bdb",
            "bdb_uid": 3
        },
        "target": {
            "endpoint_type": "external",
            "host": "target.redis.com",
            "port": 6380,
            "password": "target_password",
            "ssl": false
        },
        "status": "failed",
        "progress": 25.0,
        "start_time": "2023-01-01T14:00:00Z",
        "end_time": "2023-01-01T14:15:00Z",
        "error": "Connection timeout to target host"
    })
}

fn test_create_migration_request() -> CreateMigrationRequest {
    CreateMigrationRequest {
        source: MigrationEndpoint {
            endpoint_type: "external".to_string(),
            host: Some("source.redis.com".to_string()),
            port: Some(6379),
            bdb_uid: None,
            password: Some("source_password".to_string()),
            ssl: Some(false),
        },
        target: MigrationEndpoint {
            endpoint_type: "bdb".to_string(),
            host: None,
            port: None,
            bdb_uid: Some(1),
            password: None,
            ssl: None,
        },
        migration_type: Some("online".to_string()),
        key_pattern: Some("user:*".to_string()),
        flush_target: Some(false),
    }
}

fn test_bdb_to_bdb_migration_request() -> CreateMigrationRequest {
    CreateMigrationRequest {
        source: MigrationEndpoint {
            endpoint_type: "bdb".to_string(),
            host: None,
            port: None,
            bdb_uid: Some(1),
            password: None,
            ssl: None,
        },
        target: MigrationEndpoint {
            endpoint_type: "bdb".to_string(),
            host: None,
            port: None,
            bdb_uid: Some(2),
            password: None,
            ssl: None,
        },
        migration_type: Some("offline".to_string()),
        key_pattern: None,
        flush_target: Some(true),
    }
}

#[tokio::test]
async fn test_migrations_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/migrations"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_migration(),
            test_migration_completed(),
            test_migration_failed()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let migrations = result.unwrap();
    assert_eq!(migrations.len(), 3);
    assert_eq!(migrations[0].migration_id, "migration-123");
    assert_eq!(migrations[0].status, "running");
    assert_eq!(migrations[1].migration_id, "migration-456");
    assert_eq!(migrations[1].status, "completed");
    assert_eq!(migrations[2].migration_id, "migration-789");
    assert_eq!(migrations[2].status, "failed");
}

#[tokio::test]
async fn test_migrations_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/migrations"))
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

    let handler = MigrationsHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let migrations = result.unwrap();
    assert_eq!(migrations.len(), 0);
}

#[tokio::test]
async fn test_migrations_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/migrations/migration-123"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_migration()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.get("migration-123").await;

    assert!(result.is_ok());
    let migration = result.unwrap();
    assert_eq!(migration.migration_id, "migration-123");
    assert_eq!(migration.status, "running");
    assert_eq!(migration.progress, Some(45.5));
    assert_eq!(migration.source.endpoint_type, "external");
    assert_eq!(migration.target.endpoint_type, "bdb");
}

#[tokio::test]
async fn test_migrations_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/migrations/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Migration not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_migrations_create() {
    let mock_server = MockServer::start().await;
    let request = test_create_migration_request();

    Mock::given(method("POST"))
        .and(path("/v1/migrations"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "migration_id": "migration-new",
            "source": {
                "endpoint_type": "external",
                "host": "source.redis.com",
                "port": 6379,
                "password": "source_password",
                "ssl": false
            },
            "target": {
                "endpoint_type": "bdb",
                "bdb_uid": 1
            },
            "status": "created",
            "progress": 0.0
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let migration = result.unwrap();
    assert_eq!(migration.migration_id, "migration-new");
    assert_eq!(migration.status, "created");
    assert_eq!(migration.progress, Some(0.0));
}

#[tokio::test]
async fn test_migrations_create_bdb_to_bdb() {
    let mock_server = MockServer::start().await;
    let request = test_bdb_to_bdb_migration_request();

    Mock::given(method("POST"))
        .and(path("/v1/migrations"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "migration_id": "migration-bdb-bdb",
            "source": {
                "endpoint_type": "bdb",
                "bdb_uid": 1
            },
            "target": {
                "endpoint_type": "bdb",
                "bdb_uid": 2
            },
            "status": "created",
            "progress": 0.0
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let migration = result.unwrap();
    assert_eq!(migration.migration_id, "migration-bdb-bdb");
    assert_eq!(migration.status, "created");
}

#[tokio::test]
async fn test_migrations_create_invalid() {
    let mock_server = MockServer::start().await;
    let request = CreateMigrationRequest {
        source: MigrationEndpoint {
            endpoint_type: "invalid".to_string(),
            host: None,
            port: None,
            bdb_uid: None,
            password: None,
            ssl: None,
        },
        target: MigrationEndpoint {
            endpoint_type: "invalid".to_string(),
            host: None,
            port: None,
            bdb_uid: None,
            password: None,
            ssl: None,
        },
        migration_type: None,
        key_pattern: None,
        flush_target: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/migrations"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(400, "Invalid migration configuration"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_migrations_start() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/migrations/migration-123/start"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "migration_id": "migration-123",
            "source": {
                "endpoint_type": "external",
                "host": "source.redis.com",
                "port": 6379
            },
            "target": {
                "endpoint_type": "bdb",
                "bdb_uid": 1
            },
            "status": "running",
            "progress": 0.0,
            "start_time": "2023-01-01T12:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.start("migration-123").await;

    assert!(result.is_ok());
    let migration = result.unwrap();
    assert_eq!(migration.migration_id, "migration-123");
    assert_eq!(migration.status, "running");
    assert!(migration.start_time.is_some());
}

#[tokio::test]
async fn test_migrations_start_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/migrations/nonexistent/start"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Migration not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.start("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_migrations_pause() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/migrations/migration-123/pause"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "migration_id": "migration-123",
            "source": {
                "endpoint_type": "external",
                "host": "source.redis.com",
                "port": 6379
            },
            "target": {
                "endpoint_type": "bdb",
                "bdb_uid": 1
            },
            "status": "paused",
            "progress": 25.0
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.pause("migration-123").await;

    assert!(result.is_ok());
    let migration = result.unwrap();
    assert_eq!(migration.migration_id, "migration-123");
    assert_eq!(migration.status, "paused");
}

#[tokio::test]
async fn test_migrations_resume() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/migrations/migration-123/resume"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!({
            "migration_id": "migration-123",
            "source": {
                "endpoint_type": "external",
                "host": "source.redis.com",
                "port": 6379
            },
            "target": {
                "endpoint_type": "bdb",
                "bdb_uid": 1
            },
            "status": "running",
            "progress": 25.0
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.resume("migration-123").await;

    assert!(result.is_ok());
    let migration = result.unwrap();
    assert_eq!(migration.migration_id, "migration-123");
    assert_eq!(migration.status, "running");
}

#[tokio::test]
async fn test_migrations_cancel() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/migrations/migration-123"))
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

    let handler = MigrationsHandler::new(client);
    let result = handler.cancel("migration-123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_migrations_cancel_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/migrations/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Migration not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.cancel("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_migrations_pause_already_paused() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/migrations/migration-123/pause"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(400, "Migration is already paused"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.pause("migration-123").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_migrations_resume_not_paused() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/migrations/migration-123/resume"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(400, "Migration is not paused"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = MigrationsHandler::new(client);
    let result = handler.resume("migration-123").await;

    assert!(result.is_err());
}
