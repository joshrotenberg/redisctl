use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

#[derive(Debug, Clone, Subcommand)]
pub enum CmSettingsCommands {
    /// Get all cluster manager settings
    Get {
        /// Get specific setting by path using JMESPath
        #[arg(long)]
        setting: Option<String>,
    },

    /// Update cluster manager settings
    Set {
        /// Settings data (JSON file or inline, use @filename or - for stdin)
        #[arg(short, long)]
        data: String,

        /// Force update without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Update a specific setting
    #[command(name = "set-value")]
    SetValue {
        /// Setting name/path
        name: String,

        /// New value for the setting
        #[arg(long)]
        value: String,

        /// Force update without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Reset settings to defaults
    Reset {
        /// Force reset without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Export settings to file
    Export {
        /// Output file path (use - for stdout)
        #[arg(short, long, default_value = "-")]
        output: String,
    },

    /// Import settings from file
    Import {
        /// Input file path (use @filename or - for stdin)
        #[arg(short, long)]
        file: String,

        /// Force import without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Validate settings without applying
    Validate {
        /// Settings file to validate (use @filename or - for stdin)
        #[arg(short, long)]
        file: String,
    },

    /// List all setting categories
    #[command(name = "list-categories")]
    ListCategories,

    /// Get settings by category
    #[command(name = "get-category")]
    GetCategory {
        /// Category name
        category: String,
    },
}

impl CmSettingsCommands {
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
            CmSettingsCommands::Get { setting } => {
                let response: serde_json::Value = client
                    .get("/v1/cm_settings")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(s) = setting {
                    // Use the setting parameter as a JMESPath query
                    super::utils::apply_jmespath(&response, s)?
                } else if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            CmSettingsCommands::Set { data, force } => {
                if !force && !super::utils::confirm_action("Update cluster manager settings?")? {
                    return Ok(());
                }

                let json_data = super::utils::read_json_data(data)?;

                let response: serde_json::Value =
                    client
                        .put("/v1/cm_settings", &json_data)
                        .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Cluster manager settings updated successfully");

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            CmSettingsCommands::SetValue { name, value, force } => {
                if !force && !super::utils::confirm_action(&format!("Update setting '{}'?", name))?
                {
                    return Ok(());
                }

                // Get current settings
                let mut settings: serde_json::Value = client
                    .get("/v1/cm_settings")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Parse value as JSON if possible, otherwise as string
                let parsed_value: serde_json::Value =
                    serde_json::from_str(value).unwrap_or_else(|_| serde_json::json!(value));

                // Update the specific setting
                if name.contains('.') {
                    // Handle nested settings
                    let parts: Vec<&str> = name.split('.').collect();
                    let mut current = &mut settings;

                    for (i, part) in parts.iter().enumerate() {
                        if i == parts.len() - 1 {
                            current[part] = parsed_value.clone();
                        } else {
                            current = &mut current[part];
                        }
                    }
                } else {
                    settings[name] = parsed_value;
                }

                // Update settings
                let response: serde_json::Value = client
                    .put("/v1/cm_settings", &settings)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Setting '{}' updated to: {}", name, value);

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            CmSettingsCommands::Reset { force } => {
                if !force
                    && !super::utils::confirm_action(
                        "Reset all cluster manager settings to defaults?",
                    )?
                {
                    return Ok(());
                }

                // Reset by sending empty object
                let response: serde_json::Value = client
                    .put("/v1/cm_settings", &serde_json::json!({}))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Cluster manager settings reset to defaults");

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            CmSettingsCommands::Export { output } => {
                let settings: serde_json::Value = client
                    .get("/v1/cm_settings")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                if output == "-" {
                    // Output to stdout
                    super::utils::print_formatted_output(settings, output_format)?;
                } else {
                    // Write to file
                    let json_str = serde_json::to_string_pretty(&settings)
                        .context("Failed to serialize settings")?;
                    std::fs::write(output, json_str).context("Failed to write settings to file")?;
                    println!("Settings exported to: {}", output);
                }
            }

            CmSettingsCommands::Import { file, force } => {
                if !force
                    && !super::utils::confirm_action("Import cluster manager settings from file?")?
                {
                    return Ok(());
                }

                let json_data = super::utils::read_json_data(file)?;

                let response: serde_json::Value = client
                    .put("/v1/cm_settings", &json_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Settings imported successfully");

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            CmSettingsCommands::Validate { file } => {
                let json_data = super::utils::read_json_data(file)?;

                // Try to validate by doing a dry-run (if supported)
                // For now, just validate JSON structure
                if json_data.is_object() {
                    println!("Settings file is valid JSON");

                    // Check for known required fields if any
                    let obj = json_data.as_object().unwrap();

                    // List known categories/fields for informational purposes
                    println!("\nFound settings categories:");
                    for key in obj.keys() {
                        println!("  - {}", key);
                    }
                } else {
                    return Err(
                        anyhow::anyhow!("Invalid settings format: expected JSON object").into(),
                    );
                }
            }

            CmSettingsCommands::ListCategories => {
                let settings: serde_json::Value = client
                    .get("/v1/cm_settings")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Extract top-level keys as categories
                let categories = if let Some(obj) = settings.as_object() {
                    let cats: Vec<String> = obj.keys().cloned().collect();
                    serde_json::json!(cats)
                } else {
                    serde_json::json!([])
                };

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&categories, q)?
                } else {
                    categories
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            CmSettingsCommands::GetCategory { category } => {
                let settings: serde_json::Value = client
                    .get("/v1/cm_settings")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Extract specific category
                let category_data = &settings[category];

                if category_data.is_null() {
                    return Err(anyhow::anyhow!("Category '{}' not found", category).into());
                }

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(category_data, q)?
                } else {
                    category_data.clone()
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub async fn handle_cm_settings_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cm_settings_cmd: CmSettingsCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    cm_settings_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}
