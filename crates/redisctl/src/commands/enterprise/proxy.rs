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
    #[command(after_help = "EXAMPLES:
    # Update proxy threads
    redisctl enterprise proxy update 1 --threads 4

    # Update proxy max connections
    redisctl enterprise proxy update 1 --max-connections 1000

    # Using JSON for full configuration
    redisctl enterprise proxy update 1 --data @proxy.json")]
    Update {
        /// Proxy UID
        uid: u64,
        /// Number of proxy threads
        #[arg(long)]
        threads: Option<u32>,
        /// Maximum client connections
        #[arg(long)]
        max_connections: Option<u32>,
        /// Enable/disable the proxy
        #[arg(long)]
        enabled: Option<bool>,
        /// JSON data for update (optional, use @filename or - for stdin)
        #[arg(short, long, value_name = "FILE|JSON")]
        data: Option<String>,
    },

    /// Update all proxies configuration
    #[command(
        name = "update-all",
        after_help = "EXAMPLES:
    # Update all proxies threads
    redisctl enterprise proxy update-all --threads 4

    # Using JSON for full configuration
    redisctl enterprise proxy update-all --data @proxy.json"
    )]
    UpdateAll {
        /// Number of proxy threads
        #[arg(long)]
        threads: Option<u32>,
        /// Maximum client connections
        #[arg(long)]
        max_connections: Option<u32>,
        /// JSON data for update (optional, use @filename or - for stdin)
        #[arg(short, long, value_name = "FILE|JSON")]
        data: Option<String>,
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
        ProxyCommands::Update {
            uid,
            threads,
            max_connections,
            enabled,
            data,
        } => {
            let mut payload = if let Some(data_str) = data {
                super::utils::read_json_data(data_str)?
            } else {
                serde_json::json!({})
            };

            let payload_obj = payload.as_object_mut().unwrap();
            if let Some(t) = threads {
                payload_obj.insert("threads".to_string(), serde_json::json!(t));
            }
            if let Some(mc) = max_connections {
                payload_obj.insert("max_connections".to_string(), serde_json::json!(mc));
            }
            if let Some(e) = enabled {
                payload_obj.insert("enabled".to_string(), serde_json::json!(e));
            }

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
        ProxyCommands::UpdateAll {
            threads,
            max_connections,
            data,
        } => {
            let mut payload = if let Some(data_str) = data {
                super::utils::read_json_data(data_str)?
            } else {
                serde_json::json!({})
            };

            let payload_obj = payload.as_object_mut().unwrap();
            if let Some(t) = threads {
                payload_obj.insert("threads".to_string(), serde_json::json!(t));
            }
            if let Some(mc) = max_connections {
                payload_obj.insert("max_connections".to_string(), serde_json::json!(mc));
            }

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
        let cli = TestCli::parse_from(["test", "update", "1", "--threads", "4"]);
        if let ProxyCommands::Update { uid, threads, .. } = cli.cmd {
            assert_eq!(uid, 1);
            assert_eq!(threads, Some(4));
        } else {
            panic!("Expected Update command");
        }

        // Test update-all command
        let cli = TestCli::parse_from(["test", "update-all", "--threads", "4"]);
        if let ProxyCommands::UpdateAll { threads, .. } = cli.cmd {
            assert_eq!(threads, Some(4));
        } else {
            panic!("Expected UpdateAll command");
        }
    }
}
