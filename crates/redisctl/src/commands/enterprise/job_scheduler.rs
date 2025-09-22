use crate::error::RedisCtlError;
use clap::Subcommand;

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

#[derive(Debug, Clone, Subcommand)]
pub enum JobSchedulerCommands {
    /// Get job scheduler configuration/settings
    Get,

    /// Update job scheduler settings
    Update {
        /// JSON data for configuration update (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },
}

impl JobSchedulerCommands {
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
            JobSchedulerCommands::Get => {
                // Get job scheduler configuration/settings
                let response: serde_json::Value = client
                    .get("/v1/job_scheduler")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            JobSchedulerCommands::Update { data } => {
                let json_data = super::utils::read_json_data(data)?;

                let response: serde_json::Value = client
                    .put("/v1/job_scheduler", &json_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

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
pub async fn handle_job_scheduler_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    job_scheduler_cmd: JobSchedulerCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    job_scheduler_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}
