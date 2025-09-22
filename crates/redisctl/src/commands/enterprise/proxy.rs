use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;

use crate::{cli::OutputFormat, connection::ConnectionManager, error::Result as CliResult};

#[allow(dead_code)]
pub async fn handle_proxy_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    proxy_cmd: ProxyCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    proxy_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}

#[derive(Debug, Clone, Subcommand)]
pub enum ProxyCommands {
    /// List all proxies
    List,

    /// Get proxy details
    Get {
        /// Proxy UID
        uid: u64,
    },

    /// Update proxy configuration
    Update {
        /// Proxy UID
        uid: u64,
        /// JSON data for update (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },

    /// Update all proxies configuration
    #[command(name = "update-all")]
    UpdateAll {
        /// JSON data for update (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },
}

impl ProxyCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        conn_mgr: &ConnectionManager,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> CliResult<()> {
        handle_proxy_command_impl(conn_mgr, profile_name, self, output_format, query).await
    }
}

#[allow(dead_code)]
async fn handle_proxy_command_impl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &ProxyCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    match command {
        ProxyCommands::List => {
            let response: serde_json::Value = client
                .get("/v1/proxies")
                .await
                .map_err(RedisCtlError::from)?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        ProxyCommands::Get { uid } => {
            let response: serde_json::Value = client
                .get(&format!("/v1/proxies/{}", uid))
                .await
                .context(format!("Failed to get proxy {}", uid))?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        ProxyCommands::Update { uid, data } => {
            let payload = super::utils::read_json_data(data)?;

            let response: serde_json::Value = client
                .put(&format!("/v1/proxies/{}", uid), &payload)
                .await
                .context(format!("Failed to update proxy {}", uid))?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        ProxyCommands::UpdateAll { data } => {
            let payload = super::utils::read_json_data(data)?;

            let response: serde_json::Value = client
                .put("/v1/proxies", &payload)
                .await
                .map_err(RedisCtlError::from)?;

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
    fn test_proxy_command_parsing() {
        use clap::Parser;

        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: ProxyCommands,
        }

        // Test list command
        let cli = TestCli::parse_from(["test", "list"]);
        assert!(matches!(cli.cmd, ProxyCommands::List));

        // Test get command
        let cli = TestCli::parse_from(["test", "get", "1"]);
        if let ProxyCommands::Get { uid } = cli.cmd {
            assert_eq!(uid, 1);
        } else {
            panic!("Expected Get command");
        }

        // Test update command
        let cli = TestCli::parse_from(["test", "update", "1", "--data", "@proxy.json"]);
        if let ProxyCommands::Update { uid, data } = cli.cmd {
            assert_eq!(uid, 1);
            assert_eq!(data, "@proxy.json");
        } else {
            panic!("Expected Update command");
        }

        // Test update-all command
        let cli = TestCli::parse_from(["test", "update-all", "--data", "-"]);
        if let ProxyCommands::UpdateAll { data } = cli.cmd {
            assert_eq!(data, "-");
        } else {
            panic!("Expected UpdateAll command");
        }
    }
}
