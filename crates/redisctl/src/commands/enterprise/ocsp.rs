use crate::cli::OutputFormat;
use crate::commands::enterprise::utils;
use crate::connection::ConnectionManager;
use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;
use serde_json::Value;

#[derive(Debug, Clone, Subcommand)]
pub enum OcspCommands {
    /// Get OCSP configuration
    Get,

    /// Update OCSP configuration
    Update {
        /// JSON data for OCSP configuration
        #[arg(long, required = true)]
        data: String,
    },

    /// Get OCSP status
    Status,

    /// Test OCSP validation
    Test {
        /// Optional test configuration JSON
        #[arg(long)]
        data: Option<String>,
    },

    /// Enable OCSP validation
    Enable,

    /// Disable OCSP validation
    Disable,
}

pub async fn handle_ocsp_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: OcspCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    match cmd {
        OcspCommands::Get => handle_ocsp_get(conn_mgr, profile_name, output_format, query).await,
        OcspCommands::Update { data } => {
            handle_ocsp_update(conn_mgr, profile_name, &data, output_format, query).await
        }
        OcspCommands::Status => {
            handle_ocsp_status(conn_mgr, profile_name, output_format, query).await
        }
        OcspCommands::Test { data } => {
            handle_ocsp_test(
                conn_mgr,
                profile_name,
                data.as_deref(),
                output_format,
                query,
            )
            .await
        }
        OcspCommands::Enable => {
            handle_ocsp_enable(conn_mgr, profile_name, output_format, query).await
        }
        OcspCommands::Disable => {
            handle_ocsp_disable(conn_mgr, profile_name, output_format, query).await
        }
    }
}

async fn handle_ocsp_get(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/ocsp")
        .await
        .context("Failed to get OCSP configuration")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ocsp_update(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value =
        serde_json::from_str(data).context("Invalid JSON data for OCSP configuration")?;

    let response = client
        .put_raw("/v1/ocsp", payload)
        .await
        .context("Failed to update OCSP configuration")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ocsp_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/ocsp/status")
        .await
        .context("Failed to get OCSP status")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ocsp_test(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload = if let Some(d) = data {
        serde_json::from_str(d).context("Invalid JSON data for OCSP test")?
    } else {
        serde_json::json!({})
    };

    let response = client
        .post_raw("/v1/ocsp/test", payload)
        .await
        .context("Failed to test OCSP validation")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ocsp_enable(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload = serde_json::json!({
        "enabled": true
    });

    let response = client
        .put_raw("/v1/ocsp", payload)
        .await
        .context("Failed to enable OCSP")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ocsp_disable(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload = serde_json::json!({
        "enabled": false
    });

    let response = client
        .put_raw("/v1/ocsp", payload)
        .await
        .context("Failed to disable OCSP")?;

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
    fn test_ocsp_commands() {
        use clap::CommandFactory;

        #[derive(clap::Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: OcspCommands,
        }

        TestCli::command().debug_assert();
    }
}
