//! Raw API access command handler

#![allow(dead_code)]

use anyhow::{Context, Result};
use clap::Subcommand;
use redis_cloud::CloudClient;
use redis_common::{print_output, OutputFormat};
use redis_enterprise::EnterpriseClient;
use serde_json::Value;
use std::fs;

#[derive(Subcommand)]
pub enum ApiCommands {
    /// Execute GET request
    #[command(name = "GET")]
    Get {
        /// API path (e.g., /v1/bdbs or /subscriptions)
        path: String,
        /// Query parameters
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
        /// Additional headers
        #[arg(long = "header", value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },
    /// Execute POST request
    #[command(name = "POST")]
    Post {
        /// API path
        path: String,
        /// Request body (JSON string or @filename)
        #[arg(long)]
        data: Option<String>,
        /// Query parameters
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
        /// Additional headers
        #[arg(long = "header", value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },
    /// Execute PUT request
    #[command(name = "PUT")]
    Put {
        /// API path
        path: String,
        /// Request body (JSON string or @filename)
        #[arg(long)]
        data: Option<String>,
        /// Query parameters
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
        /// Additional headers
        #[arg(long = "header", value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },
    /// Execute PATCH request
    #[command(name = "PATCH")]
    Patch {
        /// API path
        path: String,
        /// Request body (JSON string or @filename)
        #[arg(long)]
        data: Option<String>,
        /// Query parameters
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
        /// Additional headers
        #[arg(long = "header", value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },
    /// Execute DELETE request
    #[command(name = "DELETE")]
    Delete {
        /// API path
        path: String,
        /// Query parameters
        #[arg(long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
        /// Additional headers
        #[arg(long = "header", value_parser = parse_key_val)]
        headers: Vec<(String, String)>,
    },
}

/// Parse key-value pairs from command line
pub fn parse_key_val(s: &str) -> Result<(String, String)> {
    let pos = s
        .find('=')
        .ok_or_else(|| anyhow::anyhow!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

/// Load request body from string or file
pub fn load_body(data: &Option<String>) -> Result<Value> {
    match data {
        None => Ok(Value::Null),
        Some(d) if d.starts_with('@') => {
            let path = &d[1..];
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path))?;
            serde_json::from_str(&content)
                .with_context(|| format!("Invalid JSON in file: {}", path))
        }
        Some(d) => serde_json::from_str(d).with_context(|| "Invalid JSON in --data argument"),
    }
}

/// Build path with query parameters
pub fn build_path_with_params(path: &str, params: &[(String, String)]) -> String {
    if params.is_empty() {
        path.to_string()
    } else {
        let query_string: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();
        format!("{}?{}", path, query_string.join("&"))
    }
}

/// Execute API command for Enterprise
pub async fn handle_enterprise_api(
    client: &EnterpriseClient,
    command: ApiCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let result = match command {
        ApiCommands::Get {
            path,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            client.get_raw(&full_path).await?
        }
        ApiCommands::Post {
            path,
            data,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            let body = load_body(&data)?;
            client.post_raw(&full_path, body).await?
        }
        ApiCommands::Put {
            path,
            data,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            let body = load_body(&data)?;
            client.put_raw(&full_path, body).await?
        }
        ApiCommands::Patch {
            path,
            data,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            let body = load_body(&data)?;
            client.patch_raw(&full_path, body).await?
        }
        ApiCommands::Delete {
            path,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            client.delete_raw(&full_path).await?
        }
    };

    print_output(result, output_format, query)?;
    Ok(())
}

/// Execute API command for Cloud
pub async fn handle_cloud_api(
    client: &CloudClient,
    command: ApiCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let result = match command {
        ApiCommands::Get {
            path,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            client.get_raw(&full_path).await?
        }
        ApiCommands::Post {
            path,
            data,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            let body = load_body(&data)?;
            client.post_raw(&full_path, body).await?
        }
        ApiCommands::Put {
            path,
            data,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            let body = load_body(&data)?;
            client.put_raw(&full_path, body).await?
        }
        ApiCommands::Patch {
            path,
            data,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            let body = load_body(&data)?;
            client.patch_raw(&full_path, body).await?
        }
        ApiCommands::Delete {
            path,
            params,
            headers: _,
        } => {
            let full_path = build_path_with_params(&path, &params);
            client.delete_raw(&full_path).await?
        }
    };

    print_output(result, output_format, query)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_key_val() {
        let result = parse_key_val("key=value").unwrap();
        assert_eq!(result, ("key".to_string(), "value".to_string()));

        let result = parse_key_val("complex=value=with=equals").unwrap();
        assert_eq!(
            result,
            ("complex".to_string(), "value=with=equals".to_string())
        );

        assert!(parse_key_val("no_equals").is_err());
    }

    #[test]
    fn test_load_body_from_string() {
        let json_string = r#"{"key": "value"}"#.to_string();
        let result = load_body(&Some(json_string)).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_load_body_none() {
        let result = load_body(&None).unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn test_load_body_from_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.json");
        fs::write(&file_path, r#"{"name": "test", "value": 42}"#).unwrap();

        let file_arg = format!("@{}", file_path.display());
        let result = load_body(&Some(file_arg)).unwrap();
        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_build_path_with_params() {
        let params = vec![
            ("limit".to_string(), "10".to_string()),
            ("offset".to_string(), "20".to_string()),
        ];

        let result = build_path_with_params("/api/items", &params);
        assert!(result.contains("/api/items?"));
        assert!(result.contains("limit=10"));
        assert!(result.contains("offset=20"));
        assert!(result.contains("&"));
    }

    #[test]
    fn test_build_path_with_special_chars() {
        let params = vec![
            ("name".to_string(), "test database".to_string()),
            ("tag".to_string(), "prod/v1".to_string()),
        ];

        let result = build_path_with_params("/api/search", &params);
        assert!(result.contains("name=test%20database"));
        assert!(result.contains("tag=prod%2Fv1"));
    }

    #[test]
    fn test_build_path_no_params() {
        let params = vec![];
        let result = build_path_with_params("/api/items", &params);
        assert_eq!(result, "/api/items");
    }
}
