use crate::cli::OutputFormat;
use crate::commands::enterprise::utils;
use crate::connection::ConnectionManager;
use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;
use serde_json::Value;

#[derive(Debug, Clone, Subcommand)]
pub enum BootstrapCommands {
    /// Get bootstrap status
    Status,

    /// Bootstrap new cluster
    #[command(name = "create-cluster")]
    CreateCluster {
        /// JSON data for cluster creation
        #[arg(long, required = true)]
        data: String,
    },

    /// Join existing cluster
    #[command(name = "join-cluster")]
    JoinCluster {
        /// JSON data for joining cluster
        #[arg(long, required = true)]
        data: String,
    },

    /// Validate bootstrap configuration
    Validate {
        /// Action to validate (create_cluster, join_cluster)
        action: String,

        /// JSON data to validate
        #[arg(long, required = true)]
        data: String,
    },
}

#[allow(dead_code)]
pub async fn handle_bootstrap_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: BootstrapCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    match cmd {
        BootstrapCommands::Status => {
            handle_bootstrap_status(conn_mgr, profile_name, output_format, query).await
        }
        BootstrapCommands::CreateCluster { data } => {
            handle_create_cluster(conn_mgr, profile_name, &data, output_format, query).await
        }
        BootstrapCommands::JoinCluster { data } => {
            handle_join_cluster(conn_mgr, profile_name, &data, output_format, query).await
        }
        BootstrapCommands::Validate { action, data } => {
            handle_validate_bootstrap(conn_mgr, profile_name, &action, &data, output_format, query)
                .await
        }
    }
}

#[allow(dead_code)]
async fn handle_bootstrap_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/bootstrap")
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[allow(dead_code)]
async fn handle_create_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value =
        serde_json::from_str(data).context("Invalid JSON data for cluster creation")?;

    let response = client
        .post_raw("/v1/bootstrap/create_cluster", payload)
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[allow(dead_code)]
async fn handle_join_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value =
        serde_json::from_str(data).context("Invalid JSON data for joining cluster")?;

    let response = client
        .post_raw("/v1/bootstrap/join_cluster", payload)
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[allow(dead_code)]
async fn handle_validate_bootstrap(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    action: &str,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value = serde_json::from_str(data).context("Invalid JSON data for validation")?;

    let endpoint = format!("/v1/bootstrap/validate/{}", action);
    let response = client
        .post_raw(&endpoint, payload)
        .await
        .context(format!("Failed to validate {} configuration", action))?;

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
    fn test_bootstrap_commands() {
        use clap::CommandFactory;

        #[derive(clap::Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: BootstrapCommands,
        }

        TestCli::command().debug_assert();
    }
}
