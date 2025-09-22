use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;

use crate::{cli::OutputFormat, connection::ConnectionManager, error::Result as CliResult};

#[allow(dead_code)]
pub async fn handle_suffix_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    suffix_cmd: SuffixCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    suffix_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}

#[derive(Debug, Clone, Subcommand)]
pub enum SuffixCommands {
    /// List all DNS suffixes
    List,

    /// Get a specific DNS suffix by name
    Get {
        /// DNS suffix name
        name: String,
    },
}

impl SuffixCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        conn_mgr: &ConnectionManager,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> CliResult<()> {
        handle_suffix_command_impl(conn_mgr, profile_name, self, output_format, query).await
    }
}

#[allow(dead_code)]
async fn handle_suffix_command_impl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &SuffixCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    match command {
        SuffixCommands::List => {
            let response: serde_json::Value = client
                .get("/v1/suffixes")
                .await
                .map_err(RedisCtlError::from)?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        SuffixCommands::Get { name } => {
            let response: serde_json::Value = client
                .get(&format!("/v1/suffix/{}", name))
                .await
                .context(format!("Failed to get DNS suffix '{}'", name))?;

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
    fn test_suffix_command_parsing() {
        use clap::Parser;

        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: SuffixCommands,
        }

        // Test list command
        let cli = TestCli::parse_from(["test", "list"]);
        assert!(matches!(cli.cmd, SuffixCommands::List));

        // Test get command
        let cli = TestCli::parse_from(["test", "get", "example.redis.local"]);
        if let SuffixCommands::Get { name } = cli.cmd {
            assert_eq!(name, "example.redis.local");
        } else {
            panic!("Expected Get command");
        }
    }
}
