//! Integration tests for Redis Enterprise user commands
//!
//! This module demonstrates user management in Redis Enterprise clusters.
//! Users are the accounts that can authenticate and access the cluster
//! with specific role-based permissions and database access controls.
//!
//! # Supported Operations
//!
//! - **List users** - Get all users with their roles and permissions
//! - **Get user info** - Retrieve detailed user account information
//! - **Create user** - Set up new user accounts with roles and access
//!
//! # User Management
//!
//! ## User Roles
//! - **Admin** - Full cluster administration privileges
//! - **DB Viewer** - Read-only access to database information
//! - **DB Member** - Read/write access to specific databases
//! - **Cluster Viewer** - Read-only access to cluster information
//! - **Cluster Member** - Limited cluster management access
//!
//! ## Access Control
//! - **Email-based authentication** - Users identified by email address
//! - **Password authentication** - Secure password-based login
//! - **Role-based permissions** - Fine-grained access control
//! - **Database-specific access** - Per-database permission assignment
//!
//! # Usage Examples
//!
//! ## List all users
//! ```bash
//! redis-enterprise user list
//! redis-enterprise user list --query '[].{email:email,role:role}'
//! ```
//!
//! ## Get user information
//! ```bash
//! redis-enterprise user get 1
//! redis-enterprise user get 5 --output json
//! ```
//!
//! ## Create new users
//! ```bash
//! # Create admin user
//! redis-enterprise user create --email admin@company.com --password secret123 --role admin
//! 
//! # Create database viewer (default role)
//! redis-enterprise user create --email viewer@company.com --password view123
//! 
//! # Create database member with specific role
//! redis-enterprise user create --email dev@company.com --password dev123 --role db_member
//! ```

use redis_enterprise_cli::handlers::handle_user_command;
use redis_enterprise_cli::commands::UserCommands;
use serde_json::json;
use wiremock::{
    matchers::{method, path, body_json},
    Mock, ResponseTemplate,
};

mod common;
use common::setup_mock_server;

/// Test listing all cluster users
/// 
/// This test demonstrates:
/// - Retrieving complete user account inventory
/// - Understanding user roles and permissions
/// - User access control overview
#[tokio::test]
async fn test_user_list() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!([
        {
            "uid": 1,
            "username": "admin@company.com",
            "email": "admin@company.com",
            "role": "admin",
            "created_time": "2024-08-01T10:00:00Z",
            "last_login": "2024-08-20T08:30:00Z",
            "status": "active",
            "permissions": {
                "cluster_admin": true,
                "database_access": "all",
                "user_management": true
            }
        },
        {
            "uid": 2, 
            "username": "viewer@company.com",
            "email": "viewer@company.com",
            "role": "db_viewer",
            "created_time": "2024-08-05T14:20:00Z",
            "last_login": "2024-08-20T09:15:00Z",
            "status": "active",
            "permissions": {
                "cluster_admin": false,
                "database_access": "read_only",
                "user_management": false
            }
        },
        {
            "uid": 3,
            "username": "dev@company.com",
            "email": "dev@company.com", 
            "role": "db_member",
            "created_time": "2024-08-10T16:45:00Z",
            "last_login": "2024-08-19T17:20:00Z",
            "status": "active",
            "permissions": {
                "cluster_admin": false,
                "database_access": "read_write",
                "user_management": false
            }
        },
        {
            "uid": 4,
            "username": "inactive@company.com",
            "email": "inactive@company.com",
            "role": "db_viewer", 
            "created_time": "2024-07-15T11:30:00Z",
            "last_login": "2024-08-01T12:00:00Z",
            "status": "inactive",
            "permissions": {
                "cluster_admin": false,
                "database_access": "none",
                "user_management": false
            }
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = UserCommands::List;
    let result = handle_user_command(&client, command).await.unwrap();

    let users = result.as_array().unwrap();
    assert_eq!(users.len(), 4);
    
    // Verify admin user
    assert_eq!(users[0]["uid"], 1);
    assert_eq!(users[0]["email"], "admin@company.com");
    assert_eq!(users[0]["role"], "admin");
    assert_eq!(users[0]["status"], "active");
    assert_eq!(users[0]["permissions"]["cluster_admin"], true);
    assert_eq!(users[0]["permissions"]["database_access"], "all");
    
    // Verify viewer user
    assert_eq!(users[1]["uid"], 2);
    assert_eq!(users[1]["email"], "viewer@company.com");
    assert_eq!(users[1]["role"], "db_viewer");
    assert_eq!(users[1]["permissions"]["cluster_admin"], false);
    assert_eq!(users[1]["permissions"]["database_access"], "read_only");
    
    // Verify member user
    assert_eq!(users[2]["uid"], 3);
    assert_eq!(users[2]["role"], "db_member");
    assert_eq!(users[2]["permissions"]["database_access"], "read_write");
    
    // Verify inactive user
    assert_eq!(users[3]["uid"], 4);
    assert_eq!(users[3]["status"], "inactive");
    assert_eq!(users[3]["permissions"]["database_access"], "none");
}

/// Test getting specific user information
/// 
/// This test demonstrates:
/// - Retrieving detailed user account information
/// - Understanding user permissions and access levels
/// - User activity and authentication history
#[tokio::test]
async fn test_user_get() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": 1,
        "username": "admin@company.com",
        "email": "admin@company.com",
        "role": "admin",
        "created_time": "2024-08-01T10:00:00Z",
        "last_login": "2024-08-20T08:30:00Z",
        "status": "active",
        "permissions": {
            "cluster_admin": true,
            "database_access": "all",
            "user_management": true,
            "module_management": true,
            "license_management": true
        },
        "authentication": {
            "method": "password",
            "two_factor_enabled": false,
            "failed_login_attempts": 0,
            "last_password_change": "2024-08-15T14:30:00Z"
        },
        "access_history": [
            {
                "timestamp": "2024-08-20T08:30:00Z",
                "action": "login",
                "ip_address": "192.168.1.100",
                "user_agent": "Redis Enterprise CLI 1.0"
            },
            {
                "timestamp": "2024-08-20T08:25:00Z",
                "action": "database_create",
                "target": "production-cache",
                "ip_address": "192.168.1.100"
            }
        ],
        "database_access": [
            {
                "database_id": 1,
                "database_name": "production-db",
                "access_level": "admin"
            },
            {
                "database_id": 2,
                "database_name": "staging-db", 
                "access_level": "admin"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v1/users/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Get {
        user: "1".to_string(),
    };
    let result = handle_user_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], 1);
    assert_eq!(result["email"], "admin@company.com");
    assert_eq!(result["role"], "admin");
    assert_eq!(result["status"], "active");
    
    // Verify permissions
    assert_eq!(result["permissions"]["cluster_admin"], true);
    assert_eq!(result["permissions"]["database_access"], "all");
    assert_eq!(result["permissions"]["user_management"], true);
    assert_eq!(result["permissions"]["module_management"], true);
    
    // Verify authentication settings
    assert_eq!(result["authentication"]["method"], "password");
    assert_eq!(result["authentication"]["two_factor_enabled"], false);
    assert_eq!(result["authentication"]["failed_login_attempts"], 0);
    
    // Verify access history
    let access_history = result["access_history"].as_array().unwrap();
    assert_eq!(access_history.len(), 2);
    assert_eq!(access_history[0]["action"], "login");
    assert_eq!(access_history[1]["action"], "database_create");
    
    // Verify database access
    let database_access = result["database_access"].as_array().unwrap();
    assert_eq!(database_access.len(), 2);
    assert_eq!(database_access[0]["database_name"], "production-db");
    assert_eq!(database_access[0]["access_level"], "admin");
}

/// Test creating user with admin role
/// 
/// This test demonstrates:
/// - Creating admin users with full privileges
/// - User account setup with comprehensive permissions
/// - Administrative user creation workflow
#[tokio::test]
async fn test_user_create_admin() {
    let (mock_server, client) = setup_mock_server().await;

    let expected_request = json!({
        "email": "newadmin@company.com",
        "password": "secure_admin_password123",
        "role": "admin"
    });

    let mock_response = json!({
        "uid": 5,
        "username": "newadmin@company.com",
        "email": "newadmin@company.com",
        "role": "admin",
        "created_time": "2024-08-20T12:00:00Z",
        "status": "active",
        "permissions": {
            "cluster_admin": true,
            "database_access": "all",
            "user_management": true,
            "module_management": true,
            "license_management": true
        }
    });

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Create {
        email: "newadmin@company.com".to_string(),
        password: "secure_admin_password123".to_string(),
        role: Some("admin".to_string()),
    };
    let result = handle_user_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], 5);
    assert_eq!(result["email"], "newadmin@company.com");
    assert_eq!(result["role"], "admin");
    assert_eq!(result["status"], "active");
    assert_eq!(result["permissions"]["cluster_admin"], true);
    assert_eq!(result["permissions"]["user_management"], true);
}

/// Test creating user with default role (db_viewer)
/// 
/// This test demonstrates:
/// - Creating users with default viewer permissions
/// - Basic user account setup workflow
/// - Default role assignment behavior
#[tokio::test]
async fn test_user_create_default_role() {
    let (mock_server, client) = setup_mock_server().await;

    let expected_request = json!({
        "email": "newuser@company.com",
        "password": "user_password123",
        "role": "db_viewer"  // Default role when none specified
    });

    let mock_response = json!({
        "uid": 6,
        "username": "newuser@company.com",
        "email": "newuser@company.com",
        "role": "db_viewer",
        "created_time": "2024-08-20T12:15:00Z",
        "status": "active",
        "permissions": {
            "cluster_admin": false,
            "database_access": "read_only",
            "user_management": false
        }
    });

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Create {
        email: "newuser@company.com".to_string(),
        password: "user_password123".to_string(),
        role: None, // Should use default db_viewer role
    };
    let result = handle_user_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], 6);
    assert_eq!(result["email"], "newuser@company.com");
    assert_eq!(result["role"], "db_viewer");
    assert_eq!(result["status"], "active");
    assert_eq!(result["permissions"]["cluster_admin"], false);
    assert_eq!(result["permissions"]["database_access"], "read_only");
    assert_eq!(result["permissions"]["user_management"], false);
}

/// Test creating user with db_member role
/// 
/// This test demonstrates:
/// - Creating users with database read/write access
/// - Database member role permissions
/// - User creation with specific role assignment
#[tokio::test]
async fn test_user_create_db_member() {
    let (mock_server, client) = setup_mock_server().await;

    let expected_request = json!({
        "email": "developer@company.com",
        "password": "dev_password123",
        "role": "db_member"
    });

    let mock_response = json!({
        "uid": 7,
        "username": "developer@company.com",
        "email": "developer@company.com",
        "role": "db_member",
        "created_time": "2024-08-20T12:30:00Z",
        "status": "active",
        "permissions": {
            "cluster_admin": false,
            "database_access": "read_write",
            "user_management": false
        },
        "database_access": [
            {
                "database_id": 1,
                "database_name": "development-db",
                "access_level": "read_write"
            },
            {
                "database_id": 3,
                "database_name": "testing-db",
                "access_level": "read_write"
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .and(body_json(&expected_request))
        .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Create {
        email: "developer@company.com".to_string(),
        password: "dev_password123".to_string(),
        role: Some("db_member".to_string()),
    };
    let result = handle_user_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], 7);
    assert_eq!(result["email"], "developer@company.com");
    assert_eq!(result["role"], "db_member");
    assert_eq!(result["permissions"]["database_access"], "read_write");
    
    // Verify specific database access
    let database_access = result["database_access"].as_array().unwrap();
    assert_eq!(database_access.len(), 2);
    assert_eq!(database_access[0]["access_level"], "read_write");
    assert_eq!(database_access[1]["access_level"], "read_write");
}

/// Test user not found error
/// 
/// This test demonstrates:
/// - Handling non-existent user references
/// - User lookup failure scenarios
/// - Error response processing
#[tokio::test]
async fn test_user_get_not_found() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/users/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "User not found",
            "details": "User with ID 999 does not exist"
        })))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Get {
        user: "999".to_string(),
    };
    let result = handle_user_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("404"));
}

/// Test user creation validation errors
/// 
/// This test demonstrates:
/// - Handling invalid user creation requests
/// - Email format and password validation
/// - User creation constraint errors
#[tokio::test]
async fn test_user_create_validation_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid user data",
            "details": "Email format is invalid or password does not meet requirements",
            "validation_errors": [
                {
                    "field": "email",
                    "message": "Must be a valid email address"
                },
                {
                    "field": "password", 
                    "message": "Password must be at least 8 characters with mixed case and numbers"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Create {
        email: "invalid-email".to_string(), // Invalid email format
        password: "weak".to_string(),        // Weak password
        role: Some("admin".to_string()),
    };
    let result = handle_user_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("400"));
}

/// Test user creation with duplicate email
/// 
/// This test demonstrates:
/// - Handling duplicate user creation attempts
/// - Email uniqueness constraint enforcement
/// - Conflict resolution scenarios
#[tokio::test]
async fn test_user_create_duplicate_email() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .respond_with(ResponseTemplate::new(409).set_body_json(json!({
            "error": "User already exists",
            "details": "A user with email 'existing@company.com' already exists",
            "existing_user_id": 2
        })))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Create {
        email: "existing@company.com".to_string(),
        password: "password123".to_string(),
        role: Some("db_viewer".to_string()),
    };
    let result = handle_user_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("409"));
}

/// Test user creation with invalid role
/// 
/// This test demonstrates:
/// - Handling invalid role assignments
/// - Role validation and constraint enforcement
/// - Error handling for unsupported roles
#[tokio::test]
async fn test_user_create_invalid_role() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "Invalid role",
            "details": "Role 'invalid_role' is not supported",
            "valid_roles": ["admin", "db_viewer", "db_member", "cluster_viewer", "cluster_member"]
        })))
        .mount(&mock_server)
        .await;

    let command = UserCommands::Create {
        email: "testuser@company.com".to_string(),
        password: "password123".to_string(),
        role: Some("invalid_role".to_string()),
    };
    let result = handle_user_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("400"));
}

/// Test getting user with non-numeric ID
/// 
/// This test demonstrates:
/// - Input validation for user ID parameters
/// - Error handling for non-numeric user references
/// - User lookup parameter constraints
#[tokio::test]
async fn test_user_get_non_numeric_id() {
    let (_, client) = setup_mock_server().await;

    let command = UserCommands::Get {
        user: "not-a-number".to_string(),
    };
    let result = handle_user_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Please provide user ID"));
}

/// Test user list server error
/// 
/// This test demonstrates:
/// - Handling server errors during user listing
/// - Understanding user service failures
/// - Error response processing
#[tokio::test]
async fn test_user_list_server_error() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/users"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "error": "Internal server error",
            "details": "User service temporarily unavailable"
        })))
        .mount(&mock_server)
        .await;

    let command = UserCommands::List;
    let result = handle_user_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("500"));
}