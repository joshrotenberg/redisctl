#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::commands::enterprise::utils;
use crate::connection::ConnectionManager;
use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;
use serde_json::Value;

#[derive(Debug, Clone, Subcommand)]
pub enum ServicesCommands {
    /// List all services
    List,

    /// Get service configuration
    Get {
        /// Service name
        service: String,
    },

    /// Update service configuration
    #[command(after_help = "EXAMPLES:
    # Enable a service
    redisctl enterprise services update cm_server --enabled true

    # Update service with timeout
    redisctl enterprise services update cm_server --timeout 30

    # Using JSON for full configuration
    redisctl enterprise services update cm_server --data @config.json")]
    Update {
        /// Service name
        service: String,
        /// Enable/disable the service
        #[arg(long)]
        enabled: Option<bool>,
        /// Service timeout in seconds
        #[arg(long)]
        timeout: Option<u32>,
        /// JSON data for service configuration (optional)
        #[arg(long, value_name = "FILE|JSON")]
        data: Option<String>,
    },

    /// Restart service
    Restart {
        /// Service name
        service: String,
    },

    /// Get service status
    Status {
        /// Service name
        service: String,
    },

    /// Enable service
    Enable {
        /// Service name
        service: String,
    },

    /// Disable service
    Disable {
        /// Service name
        service: String,
    },
}

pub async fn handle_services_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: ServicesCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    match cmd {
        ServicesCommands::List => {
            handle_services_list(conn_mgr, profile_name, output_format, query).await
        }
        ServicesCommands::Get { service } => {
            handle_services_get(conn_mgr, profile_name, &service, output_format, query).await
        }
        ServicesCommands::Update {
            service,
            enabled,
            timeout,
            data,
        } => {
            handle_services_update(
                conn_mgr,
                profile_name,
                &service,
                enabled,
                timeout,
                data.as_deref(),
                output_format,
                query,
            )
            .await
        }
        ServicesCommands::Restart { service } => {
            handle_services_restart(conn_mgr, profile_name, &service, output_format, query).await
        }
        ServicesCommands::Status { service } => {
            handle_services_status(conn_mgr, profile_name, &service, output_format, query).await
        }
        ServicesCommands::Enable { service } => {
            handle_services_enable(conn_mgr, profile_name, &service, output_format, query).await
        }
        ServicesCommands::Disable { service } => {
            handle_services_disable(conn_mgr, profile_name, &service, output_format, query).await
        }
    }
}

async fn handle_services_list(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Use /v1/local/services endpoint - /v1/services doesn't exist for GET
    let response = client
        .get::<Value>("/v1/local/services")
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_services_get(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    service: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = format!("/v1/services/{}", service);
    let response = client
        .get::<Value>(&endpoint)
        .await
        .context(format!("Failed to get service {}", service))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[allow(clippy::too_many_arguments)]
async fn handle_services_update(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    service: &str,
    enabled: Option<bool>,
    timeout: Option<u32>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Start with JSON from --data if provided, otherwise empty object
    let mut payload = if let Some(data_str) = data {
        utils::read_json_data(data_str)?
    } else {
        serde_json::json!({})
    };

    let payload_obj = payload.as_object_mut().unwrap();

    // CLI parameters override JSON values
    if let Some(e) = enabled {
        payload_obj.insert("enabled".to_string(), serde_json::json!(e));
    }
    if let Some(t) = timeout {
        payload_obj.insert("timeout".to_string(), serde_json::json!(t));
    }

    let endpoint = format!("/v1/services/{}", service);
    let response = client
        .put_raw(&endpoint, payload)
        .await
        .context(format!("Failed to update service {}", service))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_services_restart(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    service: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = format!("/v1/services/{}/restart", service);
    let response = client
        .post_raw(&endpoint, serde_json::json!({}))
        .await
        .context(format!("Failed to restart service {}", service))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_services_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    service: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = format!("/v1/services/{}/status", service);
    let response = client
        .get::<Value>(&endpoint)
        .await
        .context(format!("Failed to get status for service {}", service))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_services_enable(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    service: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload = serde_json::json!({
        "enabled": true
    });

    let endpoint = format!("/v1/services/{}", service);
    let response = client
        .put_raw(&endpoint, payload)
        .await
        .context(format!("Failed to enable service {}", service))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_services_disable(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    service: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload = serde_json::json!({
        "enabled": false
    });

    let endpoint = format!("/v1/services/{}", service);
    let response = client
        .put_raw(&endpoint, payload)
        .await
        .context(format!("Failed to disable service {}", service))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_services_commands() {
        use clap::CommandFactory;

        #[derive(clap::Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: ServicesCommands,
        }

        TestCli::command().debug_assert();
    }
}
