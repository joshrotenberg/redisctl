use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::{RedisCtlError, Result as CliResult};
use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum LocalCommands {
    /// Get local node master healthcheck
    #[command(name = "healthcheck")]
    MasterHealthcheck,

    /// List local services
    Services,

    /// Update local services configuration
    #[command(name = "services-update")]
    ServicesUpdate {
        /// Service configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },
}

impl LocalCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        conn_mgr: &ConnectionManager,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> CliResult<()> {
        let client = conn_mgr.create_enterprise_client(profile_name).await?;

        match self {
            LocalCommands::MasterHealthcheck => {
                let response: serde_json::Value = client
                    .get("/v1/local/node/master_healthcheck")
                    .await
                    .map_err(RedisCtlError::from)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            LocalCommands::Services => {
                let response: serde_json::Value = client
                    .get("/v1/local/services")
                    .await
                    .map_err(RedisCtlError::from)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            LocalCommands::ServicesUpdate { data } => {
                let json_data = super::utils::read_json_data(data)?;

                let response: serde_json::Value = client
                    .post("/v1/local/services", &json_data)
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
}

#[allow(dead_code)]
pub async fn handle_local_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    local_cmd: LocalCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    local_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}
