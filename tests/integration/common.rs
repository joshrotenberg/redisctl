//! Common utilities for integration tests
//!
//! This module provides shared functionality for testing Redis Enterprise CLI commands
//! against mock servers. It demonstrates how to set up proper test environments and
//! can serve as examples for API usage patterns.

use redis_enterprise::EnterpriseClient;
use wiremock::MockServer;

/// Setup a mock server and client for testing
///
/// # Returns
/// A tuple containing (MockServer, EnterpriseClient) ready for testing
///
/// # Examples
/// ```
/// let (mock_server, client) = setup_mock_server().await;
/// 
/// // Configure mock responses
/// Mock::given(method("GET"))
///     .and(path("/v1/cluster"))
///     .respond_with(ResponseTemplate::new(200).set_body_json(&response))
///     .mount(&mock_server)
///     .await;
///
/// // Test your command
/// let result = handle_cluster_command(&client, command).await;
/// ```
pub async fn setup_mock_server() -> (MockServer, EnterpriseClient) {
    let mock_server = MockServer::start().await;
    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("test_user")
        .password("test_password")
        .insecure(true) // For testing with mock server
        .build()
        .expect("Failed to create test client");
    (mock_server, client)
}

/// Create a temporary JSON file with the given content
///
/// # Arguments
/// * `content` - JSON value to write to the file
///
/// # Returns
/// A NamedTempFile that will be cleaned up when dropped
///
/// # Examples
/// ```
/// let temp_file = create_temp_json_file(json!({
///     "name": "test",
///     "value": 123
/// }));
/// 
/// // Use temp_file.path() to get the file path
/// let command = SomeCommand {
///     from_json: Some(temp_file.path().to_path_buf()),
/// };
/// ```
pub fn create_temp_json_file(content: serde_json::Value) -> tempfile::NamedTempFile {
    use std::io::Write;
    
    let mut temp_file = tempfile::NamedTempFile::new()
        .expect("Failed to create temporary file");
    temp_file.write_all(content.to_string().as_bytes())
        .expect("Failed to write to temporary file");
    temp_file.flush()
        .expect("Failed to flush temporary file");
    temp_file
}

/// Standard test timeout for async operations
#[allow(dead_code)]
pub const TEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_mock_server() {
        let (mock_server, _client) = setup_mock_server().await;
        
        // Verify the client is configured correctly
        assert!(mock_server.uri().starts_with("http://"));
        
        // The client should be ready to use (this is a basic smoke test)
        assert!(!mock_server.uri().is_empty());
    }

    #[test]
    fn test_create_temp_json_file() {
        let content = serde_json::json!({
            "test": "value",
            "number": 42
        });
        
        let temp_file = create_temp_json_file(content.clone());
        
        // Verify the file exists and contains the correct content
        let file_content = std::fs::read_to_string(temp_file.path())
            .expect("Failed to read temp file");
        let parsed: serde_json::Value = serde_json::from_str(&file_content)
            .expect("Failed to parse JSON from temp file");
        
        assert_eq!(parsed, content);
    }
}