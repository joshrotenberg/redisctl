//! Integration tests for Redis Enterprise module commands
//!
//! This module demonstrates how to interact with Redis Enterprise modules through the CLI.
//! Redis modules extend Redis functionality with additional data structures and commands.
//!
//! # Supported Operations
//!
//! - **List modules** - Get all installed modules
//! - **Get module info** - Retrieve details about a specific module
//! - **Upload module** - Install a new Redis module from file
//! - **Delete module** - Remove an installed module
//! - **Update module** - Modify module configuration
//!
//! # Usage Examples
//!
//! ## List all modules
//! ```bash
//! redis-enterprise module list
//! redis-enterprise module list --output json
//! ```
//!
//! ## Get module information
//! ```bash
//! redis-enterprise module get module_uid
//! redis-enterprise module get module_uid --query 'name'
//! ```
//!
//! ## Upload a new module
//! ```bash
//! redis-enterprise module upload --file /path/to/module.so
//! redis-enterprise module upload --file module.so --name "CustomModule" --description "My custom module"
//! ```
//!
//! ## Delete a module
//! ```bash
//! redis-enterprise module delete module_uid --yes
//! ```
//!
//! ## Update module configuration
//! ```bash
//! redis-enterprise module update module_uid --description "Updated description"
//! redis-enterprise module update module_uid --from-json config.json
//! ```

use redis_enterprise_cli::handlers::handle_module_command;
use redis_enterprise_cli::commands::ModuleCommands;
use serde_json::json;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use std::io::Write;
use wiremock::{
    matchers::{method, path, body_json},
    Mock, ResponseTemplate,
};

mod common;
use common::{setup_mock_server, create_temp_json_file};

#[tokio::test]
async fn test_module_list() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!([
        {
            "uid": "module1",
            "name": "RedisTimeSeries",
            "version": "1.8.0",
            "semantic_version": "1.8.0",
            "author": "Redis Labs",
            "description": "Time-series data structure for Redis",
            "license": "Redis Source Available License"
        },
        {
            "uid": "module2", 
            "name": "RedisSearch",
            "version": "2.6.0",
            "semantic_version": "2.6.0",
            "author": "Redis Labs",
            "description": "Full-text search and secondary indexing for Redis",
            "license": "Redis Source Available License"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/modules"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::List;
    let result = handle_module_command(&client, command).await.unwrap();

    let modules = result.as_array().unwrap();
    assert_eq!(modules.len(), 2);
    assert_eq!(modules[0]["name"], "RedisTimeSeries");
    assert_eq!(modules[1]["name"], "RedisSearch");
}

#[tokio::test]
async fn test_module_get() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": "module1",
        "name": "RedisTimeSeries",
        "version": "1.8.0",
        "semantic_version": "1.8.0",
        "author": "Redis Labs",
        "description": "Time-series data structure for Redis",
        "homepage": "https://github.com/RedisTimeSeries/RedisTimeSeries",
        "license": "Redis Source Available License",
        "capabilities": ["TS.CREATE", "TS.ADD", "TS.GET"],
        "min_redis_version": "6.0.0"
    });

    Mock::given(method("GET"))
        .and(path("/v1/modules/module1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::Get {
        uid: "module1".to_string(),
    };
    let result = handle_module_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], "module1");
    assert_eq!(result["name"], "RedisTimeSeries");
    assert_eq!(result["version"], "1.8.0");
    assert_eq!(result["capabilities"].as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn test_module_get_not_found() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v1/modules/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Module not found"
        })))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::Get {
        uid: "nonexistent".to_string(),
    };
    let result = handle_module_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("404"));
}

/// Test successful module upload with binary file
/// 
/// This test demonstrates:
/// - Creating a temporary module file
/// - Uploading the module with metadata
/// - Verifying the response contains expected module information
#[tokio::test]
async fn test_module_upload_success() {
    let (mock_server, client) = setup_mock_server().await;

    // Create a temporary module file with fake binary content
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"fake redis module binary content").unwrap();
    temp_file.flush().unwrap();

    let mock_response = json!({
        "uid": "new_module_123",
        "name": "TestModule",
        "version": "1.0.0",
        "semantic_version": "1.0.0",
        "author": "Test Author",
        "description": "Test module uploaded successfully",
        "license": "MIT",
        "capabilities": ["MODULE.CMD1", "MODULE.CMD2"]
    });

    Mock::given(method("POST"))
        .and(path("/v1/modules"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::Upload {
        file: temp_file.path().to_path_buf(),
        name: Some("TestModule".to_string()),
        description: Some("Test module".to_string()),
    };
    let result = handle_module_command(&client, command).await.unwrap();

    assert_eq!(result["uid"], "new_module_123");
    assert_eq!(result["name"], "TestModule");
    assert_eq!(result["version"], "1.0.0");
    assert_eq!(result["capabilities"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_module_upload_file_not_found() {
    let (_, client) = setup_mock_server().await;

    let command = ModuleCommands::Upload {
        file: PathBuf::from("/nonexistent/path/module.so"),
        name: Some("TestModule".to_string()),
        description: Some("Test module".to_string()),
    };
    let result = handle_module_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to read module file"));
}

#[tokio::test]
async fn test_module_delete_success() {
    let (mock_server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/modules/module1"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::Delete {
        uid: "module1".to_string(),
        yes: true, // Skip confirmation
    };
    let result = handle_module_command(&client, command).await.unwrap();

    assert_eq!(result["deleted"], true);
    assert_eq!(result["uid"], "module1");
}

#[tokio::test]
async fn test_module_update_with_description() {
    let (mock_server, client) = setup_mock_server().await;

    let mock_response = json!({
        "uid": "module1",
        "name": "RedisTimeSeries",
        "version": "1.8.0",
        "description": "Updated description"
    });

    Mock::given(method("PUT"))
        .and(path("/v1/modules/module1"))
        .and(body_json(json!({"description": "Updated description"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::Update {
        uid: "module1".to_string(),
        from_json: None,
        description: Some("Updated description".to_string()),
    };
    let result = handle_module_command(&client, command).await.unwrap();

    assert_eq!(result["description"], "Updated description");
}

/// Test module update using JSON configuration file
/// 
/// This test demonstrates:
/// - Using JSON files for complex module updates
/// - Updating multiple module fields at once
/// - Proper file handling with temporary files
#[tokio::test]
async fn test_module_update_from_json() {
    let (mock_server, client) = setup_mock_server().await;

    // Create update configuration as JSON
    let update_data = json!({
        "description": "Updated module description from JSON file",
        "homepage": "https://github.com/company/redis-module",
        "command_line_args": "--enable-feature-x --timeout 30"
    });
    
    let temp_file = create_temp_json_file(update_data.clone());

    let mock_response = json!({
        "uid": "module1",
        "name": "RedisTimeSeries",
        "version": "1.8.0",
        "description": "Updated module description from JSON file",
        "homepage": "https://github.com/company/redis-module",
        "command_line_args": "--enable-feature-x --timeout 30"
    });

    Mock::given(method("PUT"))
        .and(path("/v1/modules/module1"))
        .and(body_json(update_data))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let command = ModuleCommands::Update {
        uid: "module1".to_string(),
        from_json: Some(temp_file.path().to_path_buf()),
        description: None,
    };
    let result = handle_module_command(&client, command).await.unwrap();

    assert_eq!(result["description"], "Updated module description from JSON file");
    assert_eq!(result["homepage"], "https://github.com/company/redis-module");
    assert_eq!(result["command_line_args"], "--enable-feature-x --timeout 30");
}

#[tokio::test]
async fn test_module_update_no_changes() {
    let (_, client) = setup_mock_server().await;

    let command = ModuleCommands::Update {
        uid: "module1".to_string(),
        from_json: None,
        description: None,
    };
    let result = handle_module_command(&client, command).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No updates specified"));
}

#[tokio::test]
async fn test_module_update_invalid_json_file() {
    let (_, client) = setup_mock_server().await;

    let command = ModuleCommands::Update {
        uid: "module1".to_string(),
        from_json: Some(PathBuf::from("/nonexistent/file.json")),
        description: None,
    };
    let result = handle_module_command(&client, command).await;

    assert!(result.is_err());
}