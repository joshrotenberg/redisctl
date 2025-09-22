use crate::error::RedisCtlError;
use clap::Subcommand;

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

#[derive(Debug, Clone, Subcommand)]
pub enum BdbGroupCommands {
    /// List all database groups
    List,

    /// Get database group details
    Get {
        /// Database group UID
        uid: u32,
    },

    /// Create a new database group
    Create {
        /// JSON data for database group creation (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },

    /// Update a database group
    Update {
        /// Database group UID
        uid: u32,
        /// JSON data for update (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,
    },

    /// Delete a database group
    Delete {
        /// Database group UID
        uid: u32,
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Add database to group
    AddDatabase {
        /// Database group UID
        group_uid: u32,
        /// Database UID to add
        #[arg(long)]
        database: u32,
    },

    /// Remove database from group
    RemoveDatabase {
        /// Database group UID
        group_uid: u32,
        /// Database UID to remove
        #[arg(long)]
        database: u32,
    },

    /// List databases in group
    ListDatabases {
        /// Database group UID
        group_uid: u32,
    },
}

impl BdbGroupCommands {
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
            BdbGroupCommands::List => {
                let response: serde_json::Value = client
                    .get("/v1/bdb_groups")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            BdbGroupCommands::Get { uid } => {
                let response: serde_json::Value = client
                    .get(&format!("/v1/bdb_groups/{}", uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            BdbGroupCommands::Create { data } => {
                let json_data = super::utils::read_json_data(data)?;

                let response: serde_json::Value = client
                    .post("/v1/bdb_groups", &json_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            BdbGroupCommands::Update { uid, data } => {
                let json_data = super::utils::read_json_data(data)?;

                let response: serde_json::Value = client
                    .put(&format!("/v1/bdb_groups/{}", uid), &json_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            BdbGroupCommands::Delete { uid, force } => {
                if !force
                    && !super::utils::confirm_action(&format!("Delete database group {}?", uid))?
                {
                    return Ok(());
                }

                client
                    .delete(&format!("/v1/bdb_groups/{}", uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Database group {} deleted successfully", uid);
            }

            BdbGroupCommands::AddDatabase {
                group_uid,
                database,
            } => {
                // Get current group data
                let mut group_data: serde_json::Value = client
                    .get(&format!("/v1/bdb_groups/{}", group_uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Add database to the group
                if let Some(bdbs) = group_data["bdbs"].as_array_mut() {
                    if !bdbs.contains(&serde_json::json!(database)) {
                        bdbs.push(serde_json::json!(database));
                    } else {
                        println!("Database {} is already in group {}", database, group_uid);
                        return Ok(());
                    }
                } else {
                    group_data["bdbs"] = serde_json::json!([database]);
                }

                // Update the group
                let response: serde_json::Value = client
                    .put(&format!("/v1/bdb_groups/{}", group_uid), &group_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Database {} added to group {}", database, group_uid);

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            BdbGroupCommands::RemoveDatabase {
                group_uid,
                database,
            } => {
                // Get current group data
                let mut group_data: serde_json::Value = client
                    .get(&format!("/v1/bdb_groups/{}", group_uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Remove database from the group
                if let Some(bdbs) = group_data["bdbs"].as_array_mut() {
                    if let Some(pos) = bdbs.iter().position(|x| x == &serde_json::json!(database)) {
                        bdbs.remove(pos);
                    } else {
                        println!("Database {} is not in group {}", database, group_uid);
                        return Ok(());
                    }
                } else {
                    println!("Group {} has no databases", group_uid);
                    return Ok(());
                }

                // Update the group
                let response: serde_json::Value = client
                    .put(&format!("/v1/bdb_groups/{}", group_uid), &group_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Database {} removed from group {}", database, group_uid);

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            BdbGroupCommands::ListDatabases { group_uid } => {
                let response: serde_json::Value = client
                    .get(&format!("/v1/bdb_groups/{}", group_uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Extract just the databases list
                let databases = &response["bdbs"];

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(databases, q)?
                } else {
                    databases.clone()
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub async fn handle_bdb_group_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    bdb_group_cmd: BdbGroupCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    bdb_group_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}
