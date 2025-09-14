use anyhow::Context;
use clap::Subcommand;

use crate::{cli::OutputFormat, connection::ConnectionManager, error::Result as CliResult};

#[allow(dead_code)]
pub async fn handle_endpoint_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    endpoint_cmd: EndpointCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    endpoint_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}

#[derive(Debug, Clone, Subcommand)]
pub enum EndpointCommands {
    /// Get endpoint statistics
    Stats,

    /// Check endpoint availability for a database
    Availability {
        /// Database UID
        bdb_uid: u64,
    },
}

impl EndpointCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        conn_mgr: &ConnectionManager,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> CliResult<()> {
        handle_endpoint_command_impl(conn_mgr, profile_name, self, output_format, query).await
    }
}

#[allow(dead_code)]
async fn handle_endpoint_command_impl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EndpointCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    match command {
        EndpointCommands::Stats => {
            let response: serde_json::Value = client
                .get("/v1/endpoints/stats")
                .await
                .context("Failed to get endpoint statistics")?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        EndpointCommands::Availability { bdb_uid } => {
            let response: serde_json::Value = client
                .get(&format!("/v1/local/bdbs/{}/endpoint/availability", bdb_uid))
                .await
                .context(format!(
                    "Failed to check endpoint availability for database {}",
                    bdb_uid
                ))?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_command_parsing() {
        use clap::Parser;

        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: EndpointCommands,
        }

        // Test stats command
        let cli = TestCli::parse_from(["test", "stats"]);
        assert!(matches!(cli.cmd, EndpointCommands::Stats));

        // Test availability command
        let cli = TestCli::parse_from(["test", "availability", "1"]);
        if let EndpointCommands::Availability { bdb_uid } = cli.cmd {
            assert_eq!(bdb_uid, 1);
        } else {
            panic!("Expected Availability command");
        }
    }
}
