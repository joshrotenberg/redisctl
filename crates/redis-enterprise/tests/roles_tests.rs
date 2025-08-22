//! Roles endpoint tests for Redis Enterprise

use redis_enterprise::{BdbRole, CreateRoleRequest, EnterpriseClient, RolesHandler};
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

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn test_role() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "test-role",
        "management": "admin",
        "data_access": "full",
        "bdb_roles": [
            {
                "bdb_uid": 1,
                "role": "admin",
                "redis_acl_uid": 1
            }
        ],
        "cluster_roles": ["cluster_admin"]
    })
}

fn test_role_minimal() -> serde_json::Value {
    json!({
        "uid": 2,
        "name": "minimal-role"
    })
}

fn test_built_in_role() -> serde_json::Value {
    json!({
        "uid": 10,
        "name": "Admin",
        "management": "admin",
        "data_access": "full"
    })
}

#[tokio::test]
async fn test_roles_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_role(), test_role_minimal()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 2);
    assert_eq!(roles[0].uid, 1);
    assert_eq!(roles[0].name, "test-role");
    assert_eq!(roles[1].uid, 2);
    assert_eq!(roles[1].name, "minimal-role");
}

#[tokio::test]
async fn test_roles_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles"))
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

    let handler = RolesHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 0);
}

#[tokio::test]
async fn test_role_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_role()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.uid, 1);
    assert_eq!(role.name, "test-role");
    assert_eq!(role.management, Some("admin".to_string()));
    assert_eq!(role.data_access, Some("full".to_string()));
    assert!(role.bdb_roles.is_some());
    assert!(role.cluster_roles.is_some());
}

#[tokio::test]
async fn test_role_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/2"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_role_minimal()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.get(2).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.uid, 2);
    assert_eq!(role.name, "minimal-role");
    assert!(role.management.is_none());
    assert!(role.data_access.is_none());
    assert!(role.bdb_roles.is_none());
    assert!(role.cluster_roles.is_none());
}

#[tokio::test]
async fn test_role_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Role not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_role_create() {
    let mock_server = MockServer::start().await;

    let request = CreateRoleRequest {
        name: "new-role".to_string(),
        management: Some("admin".to_string()),
        data_access: Some("full".to_string()),
        bdb_roles: Some(vec![BdbRole {
            bdb_uid: 1,
            role: "admin".to_string(),
            redis_acl_uid: Some(1),
        }]),
        cluster_roles: Some(vec!["cluster_admin".to_string()]),
    };

    Mock::given(method("POST"))
        .and(path("/v1/roles"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "uid": 3,
            "name": "new-role",
            "management": "admin",
            "data_access": "full",
            "bdb_roles": [
                {
                    "bdb_uid": 1,
                    "role": "admin",
                    "redis_acl_uid": 1
                }
            ],
            "cluster_roles": ["cluster_admin"]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.uid, 3);
    assert_eq!(role.name, "new-role");
    assert_eq!(role.management, Some("admin".to_string()));
}

#[tokio::test]
async fn test_role_create_minimal() {
    let mock_server = MockServer::start().await;

    let request = CreateRoleRequest {
        name: "minimal-role".to_string(),
        management: None,
        data_access: None,
        bdb_roles: None,
        cluster_roles: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/roles"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "uid": 4,
            "name": "minimal-role"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.uid, 4);
    assert_eq!(role.name, "minimal-role");
}

#[tokio::test]
async fn test_role_update() {
    let mock_server = MockServer::start().await;

    let request = CreateRoleRequest {
        name: "updated-role".to_string(),
        management: Some("viewer".to_string()),
        data_access: Some("read".to_string()),
        bdb_roles: None,
        cluster_roles: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/roles/1"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(success_response(json!({
            "uid": 1,
            "name": "updated-role",
            "management": "viewer",
            "data_access": "read"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.update(1, request).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.uid, 1);
    assert_eq!(role.name, "updated-role");
    assert_eq!(role.management, Some("viewer".to_string()));
    assert_eq!(role.data_access, Some("read".to_string()));
}

#[tokio::test]
async fn test_role_update_nonexistent() {
    let mock_server = MockServer::start().await;

    let request = CreateRoleRequest {
        name: "nonexistent".to_string(),
        management: None,
        data_access: None,
        bdb_roles: None,
        cluster_roles: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/roles/999"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(404, "Role not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.update(999, request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_role_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/roles/1"))
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

    let handler = RolesHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_role_delete_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/roles/999"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Role not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.delete(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_roles_built_in() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/builtin"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([test_built_in_role()])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.built_in().await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0].uid, 10);
    assert_eq!(roles[0].name, "Admin");
}

#[tokio::test]
async fn test_roles_built_in_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/builtin"))
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

    let handler = RolesHandler::new(client);
    let result = handler.built_in().await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 0);
}

#[tokio::test]
async fn test_role_users() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/1/users"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([1, 2, 3])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.users(1).await;

    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 3);
    assert_eq!(users, vec![1, 2, 3]);
}

#[tokio::test]
async fn test_role_users_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/2/users"))
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

    let handler = RolesHandler::new(client);
    let result = handler.users(2).await;

    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 0);
}

#[tokio::test]
async fn test_role_users_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/roles/999/users"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Role not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = RolesHandler::new(client);
    let result = handler.users(999).await;

    assert!(result.is_err());
}
