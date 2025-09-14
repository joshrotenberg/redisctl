use anyhow::Context;
use clap::Subcommand;

use crate::{cli::OutputFormat, connection::ConnectionManager, error::Result as CliResult};

#[allow(dead_code)]
pub async fn handle_migration_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    migration_cmd: MigrationCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    migration_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}

#[derive(Debug, Clone, Subcommand)]
pub enum MigrationCommands {
    /// Get migration status
    Get {
        /// Migration UID
        uid: u64,
    },

    /// Export database data
    Export {
        /// Database UID
        bdb_uid: u64,
    },

    /// Import database data
    Import {
        /// Database UID
        bdb_uid: u64,
        /// Import data (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },
}

impl MigrationCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        conn_mgr: &ConnectionManager,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> CliResult<()> {
        handle_migration_command_impl(conn_mgr, profile_name, self, output_format, query).await
    }
}

#[allow(dead_code)]
async fn handle_migration_command_impl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &MigrationCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    match command {
        MigrationCommands::Get { uid } => {
            let response: serde_json::Value = client
                .get(&format!("/v1/migrations/{}", uid))
                .await
                .context(format!("Failed to get migration {}", uid))?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        MigrationCommands::Export { bdb_uid } => {
            let response: serde_json::Value = client
                .post(
                    &format!("/v1/bdbs/{}/actions/export", bdb_uid),
                    &serde_json::json!({}),
                )
                .await
                .context(format!("Failed to export database {}", bdb_uid))?;

            let output_data = if let Some(q) = query {
                super::utils::apply_jmespath(&response, q)?
            } else {
                response
            };

            super::utils::print_formatted_output(output_data, output_format)?;
        }
        MigrationCommands::Import { bdb_uid, data } => {
            let payload = super::utils::read_json_data(data)?;

            let response: serde_json::Value = client
                .post(&format!("/v1/bdbs/{}/actions/import", bdb_uid), &payload)
                .await
                .context(format!("Failed to import data to database {}", bdb_uid))?;

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
    fn test_migration_command_parsing() {
        use clap::Parser;

        #[derive(Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: MigrationCommands,
        }

        // Test get command
        let cli = TestCli::parse_from(["test", "get", "1"]);
        if let MigrationCommands::Get { uid } = cli.cmd {
            assert_eq!(uid, 1);
        } else {
            panic!("Expected Get command");
        }

        // Test export command
        let cli = TestCli::parse_from(["test", "export", "2"]);
        if let MigrationCommands::Export { bdb_uid } = cli.cmd {
            assert_eq!(bdb_uid, 2);
        } else {
            panic!("Expected Export command");
        }

        // Test import command
        let cli = TestCli::parse_from(["test", "import", "3", "--data", "@import.json"]);
        if let MigrationCommands::Import { bdb_uid, data } = cli.cmd {
            assert_eq!(bdb_uid, 3);
            assert_eq!(data, "@import.json");
        } else {
            panic!("Expected Import command");
        }
    }
}
