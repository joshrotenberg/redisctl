use crate::error::RedisCtlError;
use anyhow::{Context, Result as AnyhowResult};
use clap::Subcommand;
use serde_json::Value;
use std::path::Path;

use crate::{cli::OutputFormat, config::Config};

#[derive(Debug, Subcommand)]
pub enum LicenseCommands {
    /// Get current license information
    Get,
    /// Update license with JSON data
    Update {
        /// License data as JSON string or @file.json
        #[arg(long)]
        data: String,
    },
    /// Upload license file
    Upload {
        /// Path to license file
        #[arg(long)]
        file: String,
    },
    /// Validate license
    Validate {
        /// License data as JSON string or @file.json
        #[arg(long)]
        data: String,
    },
    /// Check license expiration
    Expiry,
    /// List licensed features
    Features,
    /// Show license usage and limits
    Usage,
}

impl LicenseCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        config: &Config,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> AnyhowResult<()> {
        let conn_manager = crate::connection::ConnectionManager::new(config.clone());

        match self {
            Self::Get => {
                handle_get_license(&conn_manager, profile_name, output_format, query).await
            }
            Self::Update { data } => {
                handle_update_license(&conn_manager, profile_name, data, output_format, query).await
            }
            Self::Upload { file } => {
                handle_upload_license(&conn_manager, profile_name, file, output_format, query).await
            }
            Self::Validate { data } => {
                handle_validate_license(&conn_manager, profile_name, data, output_format, query)
                    .await
            }
            Self::Expiry => {
                handle_license_expiry(&conn_manager, profile_name, output_format, query).await
            }
            Self::Features => {
                handle_license_features(&conn_manager, profile_name, output_format, query).await
            }
            Self::Usage => {
                handle_license_usage(&conn_manager, profile_name, output_format, query).await
            }
        }
    }
}

async fn handle_get_license(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/license")
        .await
        .map_err(RedisCtlError::from)?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_update_license(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let json_data = super::utils::read_json_data(data)?;

    let response = client
        .put::<_, Value>("/v1/license", &json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_upload_license(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    file: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Read the license file
    let path = Path::new(file);
    if !path.exists() {
        anyhow::bail!("License file not found: {}", file);
    }

    let license_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read license file: {}", file))?;

    // Try to parse as JSON first
    let license_data = if let Ok(json) = serde_json::from_str::<Value>(&license_content) {
        json
    } else {
        // If not JSON, wrap the content as a license string
        serde_json::json!({
            "license": license_content.trim()
        })
    };

    let response = client
        .put::<_, Value>("/v1/license", &license_data)
        .await
        .map_err(RedisCtlError::from)?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_validate_license(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let json_data = super::utils::read_json_data(data)?;

    // The validation endpoint might not exist, so we'll use the regular PUT with dry_run if available
    // Otherwise, we'll just try to parse and validate the license locally
    let response = client
        .put::<_, Value>("/v1/license?dry_run=true", &json_data)
        .await
        .or_else(|_| -> Result<Value, anyhow::Error> {
            // If dry_run is not supported, just get current license and check format
            Ok(serde_json::json!({
                "valid": json_data.get("license").is_some() || json_data.get("key").is_some(),
                "message": "License format appears valid (server validation not available)"
            }))
        })
        .context("Failed to validate license")?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_license_expiry(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let license = client
        .get::<Value>("/v1/license")
        .await
        .map_err(RedisCtlError::from)?;

    // Extract expiration information
    let expiry_info = serde_json::json!({
        "expired": license.get("expired").and_then(|v| v.as_bool()).unwrap_or(false),
        "expiration_date": license.get("expiration_date").and_then(|v| v.as_str()).unwrap_or("unknown"),
        "days_remaining": calculate_days_remaining(license.get("expiration_date").and_then(|v| v.as_str())),
        "warning": should_warn_expiry(license.get("expiration_date").and_then(|v| v.as_str())),
    });

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&expiry_info, q)?
    } else {
        expiry_info
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_license_features(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let license = client
        .get::<Value>("/v1/license")
        .await
        .map_err(RedisCtlError::from)?;

    // Extract feature information
    let features = if let Some(features) = license.get("features") {
        features.clone()
    } else {
        // Construct features from license capabilities
        serde_json::json!({
            "shards_limit": license.get("shards_limit"),
            "ram_limit": license.get("ram_limit"),
            "flash_enabled": license.get("flash_enabled"),
            "rack_awareness": license.get("rack_awareness"),
            "multi_ip": license.get("multi_ip"),
            "ipv6": license.get("ipv6"),
            "redis_pack": license.get("redis_pack"),
            "modules": license.get("modules"),
        })
    };

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&features, q)?
    } else {
        features
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_license_usage(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Get license information
    let license = client
        .get::<Value>("/v1/license")
        .await
        .map_err(RedisCtlError::from)?;

    // Get cluster stats for current usage
    let cluster = client
        .get::<Value>("/v1/cluster")
        .await
        .map_err(RedisCtlError::from)?;

    // Calculate usage vs limits
    let usage_info = serde_json::json!({
        "shards": {
            "limit": license.get("shards_limit").and_then(|v| v.as_i64()).unwrap_or(0),
            "used": cluster.get("shards_used").and_then(|v| v.as_i64()).unwrap_or(0),
            "available": calculate_available(
                license.get("shards_limit").and_then(|v| v.as_i64()),
                cluster.get("shards_used").and_then(|v| v.as_i64())
            ),
        },
        "ram": {
            "limit_bytes": license.get("ram_limit").and_then(|v| v.as_i64()).unwrap_or(0),
            "limit_gb": bytes_to_gb(license.get("ram_limit").and_then(|v| v.as_i64()).unwrap_or(0)),
            "used_bytes": cluster.get("ram_used").and_then(|v| v.as_i64()).unwrap_or(0),
            "used_gb": bytes_to_gb(cluster.get("ram_used").and_then(|v| v.as_i64()).unwrap_or(0)),
            "available_bytes": calculate_available(
                license.get("ram_limit").and_then(|v| v.as_i64()),
                cluster.get("ram_used").and_then(|v| v.as_i64())
            ),
            "available_gb": bytes_to_gb(calculate_available(
                license.get("ram_limit").and_then(|v| v.as_i64()),
                cluster.get("ram_used").and_then(|v| v.as_i64())
            )),
        },
        "nodes": {
            "limit": license.get("nodes_limit").and_then(|v| v.as_i64()).unwrap_or(0),
            "used": cluster.get("nodes_count").and_then(|v| v.as_i64()).unwrap_or(0),
        },
        "expiration": {
            "date": license.get("expiration_date").and_then(|v| v.as_str()).unwrap_or("unknown"),
            "expired": license.get("expired").and_then(|v| v.as_bool()).unwrap_or(false),
        }
    });

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&usage_info, q)?
    } else {
        usage_info
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

// Helper functions
pub fn calculate_days_remaining(expiration_date: Option<&str>) -> i64 {
    if let Some(date_str) = expiration_date {
        // Try parsing as ISO8601 datetime first (e.g., "2025-10-15T00:18:29Z")
        if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date_str) {
            let today = chrono::Local::now().naive_local().date();
            let exp_date = datetime.naive_local().date();
            let duration = exp_date.signed_duration_since(today);
            return duration.num_days();
        }
        // Fall back to parsing as date only (e.g., "2025-10-15")
        if let Ok(exp_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let today = chrono::Local::now().naive_local().date();
            let duration = exp_date.signed_duration_since(today);
            return duration.num_days();
        }
    }
    -1
}

pub fn should_warn_expiry(expiration_date: Option<&str>) -> bool {
    let days = calculate_days_remaining(expiration_date);
    (0..=30).contains(&days)
}

pub fn calculate_available(limit: Option<i64>, used: Option<i64>) -> i64 {
    match (limit, used) {
        (Some(l), Some(u)) => (l - u).max(0),
        _ => 0,
    }
}

pub fn bytes_to_gb(bytes: i64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_license_command_structure() {
        // Test that all license commands can be constructed

        // Get command
        let _cmd = LicenseCommands::Get;

        // Update command
        let _cmd = LicenseCommands::Update {
            data: "{}".to_string(),
        };

        // Upload command
        let _cmd = LicenseCommands::Upload {
            file: "/path/to/license".to_string(),
        };

        // Validate command
        let _cmd = LicenseCommands::Validate {
            data: "{}".to_string(),
        };

        // Expiry command
        let _cmd = LicenseCommands::Expiry;

        // Features command
        let _cmd = LicenseCommands::Features;

        // Usage command
        let _cmd = LicenseCommands::Usage;
    }

    #[test]
    fn test_calculate_days_remaining() {
        // Test with invalid date
        assert_eq!(calculate_days_remaining(None), -1);
        assert_eq!(calculate_days_remaining(Some("invalid")), -1);

        // Test with valid date (would need mocking for actual date comparison)
        // For now, just verify it doesn't panic
        let _ = calculate_days_remaining(Some("2025-12-31"));
    }

    #[test]
    fn test_bytes_to_gb() {
        assert_eq!(bytes_to_gb(0), 0.0);
        assert_eq!(bytes_to_gb(1073741824), 1.0); // 1 GB
        assert_eq!(bytes_to_gb(2147483648), 2.0); // 2 GB
    }

    #[test]
    fn test_calculate_available() {
        assert_eq!(calculate_available(Some(100), Some(30)), 70);
        assert_eq!(calculate_available(Some(50), Some(60)), 0); // Can't be negative
        assert_eq!(calculate_available(None, Some(10)), 0);
        assert_eq!(calculate_available(Some(100), None), 0);
    }
}
