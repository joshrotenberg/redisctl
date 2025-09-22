use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;
use redis_enterprise::DiagnosticsHandler;

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

#[derive(Debug, Clone, Subcommand)]
pub enum DiagnosticsCommands {
    /// Get diagnostics configuration
    Get,

    /// Update diagnostics configuration
    Update {
        /// JSON data for configuration update (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },

    /// Run diagnostic checks
    Run {
        /// Specific diagnostic checks to run (comma-separated)
        #[arg(long)]
        checks: Option<String>,

        /// Node UIDs to run diagnostics on (comma-separated)
        #[arg(long)]
        nodes: Option<String>,

        /// Database UIDs to run diagnostics on (comma-separated)
        #[arg(long)]
        databases: Option<String>,
    },

    /// List available diagnostic checks
    #[command(name = "list-checks")]
    ListChecks,

    /// Get the last diagnostic report
    #[command(name = "last-report")]
    LastReport,

    /// Get a specific diagnostic report by ID
    #[command(name = "get-report")]
    GetReport {
        /// Report ID
        report_id: String,
    },

    /// List all diagnostic reports
    #[command(name = "list-reports")]
    ListReports,
}

impl DiagnosticsCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        conn_mgr: &ConnectionManager,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> CliResult<()> {
        let client = conn_mgr.create_enterprise_client(profile_name).await?;
        let handler = DiagnosticsHandler::new(client.clone());

        match self {
            DiagnosticsCommands::Get => {
                let config = handler
                    .get_config()
                    .await
                    .map_err(RedisCtlError::from)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&config, q)?
                } else {
                    config
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            DiagnosticsCommands::Update { data } => {
                let json_data = super::utils::read_json_data(data)?;
                let result = handler
                    .update_config(json_data)
                    .await
                    .map_err(RedisCtlError::from)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&result, q)?
                } else {
                    result
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            DiagnosticsCommands::Run {
                checks,
                nodes,
                databases,
            } => {
                // Create the request directly as JSON
                let mut request = serde_json::json!({});

                if let Some(checks_list) = parse_comma_separated(checks) {
                    request["checks"] = serde_json::json!(checks_list);
                }

                if let Some(nodes_list) = parse_comma_separated_u32(nodes) {
                    request["node_uids"] = serde_json::json!(nodes_list);
                }

                if let Some(databases_list) = parse_comma_separated_u32(databases) {
                    request["bdb_uids"] = serde_json::json!(databases_list);
                }

                // Use the raw POST method
                let report: serde_json::Value = client
                    .post("/v1/diagnostics", &request)
                    .await
                    .map_err(RedisCtlError::from)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&report, q)?
                } else {
                    report
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            DiagnosticsCommands::ListChecks => {
                let checks = handler
                    .list_checks()
                    .await
                    .map_err(RedisCtlError::from)?;

                // Convert to JSON Value for output
                let response = serde_json::to_value(&checks)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            DiagnosticsCommands::LastReport => {
                let report = handler
                    .get_last_report()
                    .await
                    .map_err(RedisCtlError::from)?;

                // Convert to JSON Value for output
                let response = serde_json::to_value(&report)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            DiagnosticsCommands::GetReport { report_id } => {
                let report = handler
                    .get_report(report_id)
                    .await
                    .context(format!("Failed to get diagnostic report {}", report_id))?;

                // Convert to JSON Value for output
                let response = serde_json::to_value(&report)?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            DiagnosticsCommands::ListReports => {
                let reports = handler
                    .list_reports()
                    .await
                    .map_err(RedisCtlError::from)?;

                // Convert to JSON Value for output
                let response = serde_json::to_value(&reports)?;

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
pub async fn handle_diagnostics_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    diagnostics_cmd: DiagnosticsCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    diagnostics_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}

// Helper functions
#[allow(dead_code)]
fn parse_comma_separated(input: &Option<String>) -> Option<Vec<String>> {
    input.as_ref().map(|s| {
        s.split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect()
    })
}

#[allow(dead_code)]
fn parse_comma_separated_u32(input: &Option<String>) -> Option<Vec<u32>> {
    input.as_ref().and_then(|s| {
        let values: Result<Vec<u32>, _> = s
            .split(',')
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u32>())
            .collect();
        values.ok()
    })
}
