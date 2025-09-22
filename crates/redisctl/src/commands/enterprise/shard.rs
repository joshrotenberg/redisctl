use crate::error::RedisCtlError;
use clap::Subcommand;

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

#[derive(Debug, Clone, Subcommand)]
pub enum ShardCommands {
    /// List all shards in the cluster
    List {
        /// Filter by node ID
        #[arg(long)]
        node: Option<u32>,

        /// Filter by database ID
        #[arg(long)]
        database: Option<u32>,

        /// Filter by role (master/slave)
        #[arg(long)]
        role: Option<String>,
    },

    /// Get shard details
    Get {
        /// Shard UID
        uid: u32,
    },

    /// List shards for a specific database
    #[command(name = "list-by-database")]
    ListByDatabase {
        /// Database UID
        bdb_uid: u32,
    },

    /// Perform shard failover
    Failover {
        /// Shard UID to failover
        uid: u32,

        /// Force failover without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Migrate shard to another node
    Migrate {
        /// Shard UID to migrate
        uid: u32,

        /// Target node UID
        #[arg(long)]
        target_node: u32,

        /// Force migration without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Perform bulk failover operation
    #[command(name = "bulk-failover")]
    BulkFailover {
        /// JSON data specifying shards to failover (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,

        /// Force without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Perform bulk migration operation
    #[command(name = "bulk-migrate")]
    BulkMigrate {
        /// JSON data specifying shard migrations (use @filename or - for stdin)
        #[arg(short, long)]
        data: String,

        /// Force without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Get shard statistics
    Stats {
        /// Shard UID (omit for all shards)
        uid: Option<u32>,

        /// Time interval (e.g., 1hour, 1day)
        #[arg(long, default_value = "1hour")]
        interval: String,

        /// Start time (ISO 8601 format)
        #[arg(long)]
        stime: Option<String>,

        /// End time (ISO 8601 format)
        #[arg(long)]
        etime: Option<String>,
    },

    /// Get latest shard statistics
    #[command(name = "stats-last")]
    StatsLast {
        /// Shard UID (omit for all shards)
        uid: Option<u32>,

        /// Time interval (e.g., 1sec, 1min)
        #[arg(long, default_value = "1sec")]
        interval: String,
    },

    /// Check shard health
    Health {
        /// Shard UID
        uid: u32,
    },

    /// Get shard configuration
    Config {
        /// Shard UID
        uid: u32,
    },
}

impl ShardCommands {
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
            ShardCommands::List {
                node,
                database,
                role,
            } => {
                let mut response: serde_json::Value = client
                    .get("/v1/shards")
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Apply filters if provided
                if (node.is_some() || database.is_some() || role.is_some())
                    && let Some(shards) = response.as_array_mut()
                {
                    shards.retain(|shard| {
                        let mut keep = true;

                        if let Some(n) = node {
                            keep = keep && shard["node"].as_u64() == Some(*n as u64);
                        }

                        if let Some(d) = database {
                            keep = keep && shard["bdb_uid"].as_u64() == Some(*d as u64);
                        }

                        if let Some(r) = role {
                            keep = keep && shard["role"].as_str() == Some(r);
                        }

                        keep
                    });
                }

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            ShardCommands::Get { uid } => {
                let response: serde_json::Value = client
                    .get(&format!("/v1/shards/{}", uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            ShardCommands::ListByDatabase { bdb_uid } => {
                let response: serde_json::Value = client
                    .get(&format!("/v1/bdbs/{}/shards", bdb_uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            ShardCommands::Failover { uid, force } => {
                if !force && !super::utils::confirm_action(&format!("Failover shard {}?", uid))? {
                    return Ok(());
                }

                let _: serde_json::Value = client
                    .post(
                        &format!("/v1/shards/{}/actions/failover", uid),
                        &serde_json::json!({}),
                    )
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Shard {} failover initiated successfully", uid);
            }

            ShardCommands::Migrate {
                uid,
                target_node,
                force,
            } => {
                if !force
                    && !super::utils::confirm_action(&format!(
                        "Migrate shard {} to node {}?",
                        uid, target_node
                    ))?
                {
                    return Ok(());
                }

                let migrate_data = serde_json::json!({
                    "target_node": target_node
                });

                let _: serde_json::Value = client
                    .post(
                        &format!("/v1/shards/{}/actions/migrate", uid),
                        &migrate_data,
                    )
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Shard {} migration to node {} initiated", uid, target_node);
            }

            ShardCommands::BulkFailover { data, force } => {
                if !force && !super::utils::confirm_action("Perform bulk shard failover?")? {
                    return Ok(());
                }

                let json_data = super::utils::read_json_data(data)?;

                let _: serde_json::Value = client
                    .post("/v1/shards/actions/failover", &json_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Bulk shard failover initiated successfully");
            }

            ShardCommands::BulkMigrate { data, force } => {
                if !force && !super::utils::confirm_action("Perform bulk shard migration?")? {
                    return Ok(());
                }

                let json_data = super::utils::read_json_data(data)?;

                let _: serde_json::Value = client
                    .post("/v1/shards/actions/migrate", &json_data)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                println!("Bulk shard migration initiated successfully");
            }

            ShardCommands::Stats {
                uid,
                interval,
                stime,
                etime,
            } => {
                let mut url = if let Some(u) = uid {
                    format!("/v1/shards/stats/{}", u)
                } else {
                    "/v1/shards/stats".to_string()
                };

                // Add query parameters
                let mut params = vec![format!("interval={}", interval)];
                if let Some(s) = stime {
                    params.push(format!("stime={}", s));
                }
                if let Some(e) = etime {
                    params.push(format!("etime={}", e));
                }

                if !params.is_empty() {
                    url.push_str(&format!("?{}", params.join("&")));
                }

                let response: serde_json::Value = client
                    .get(&url)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            ShardCommands::StatsLast { uid, interval } => {
                let url = if let Some(u) = uid {
                    format!("/v1/shards/stats/last/{}?interval={}", u, interval)
                } else {
                    format!("/v1/shards/stats/last?interval={}", interval)
                };

                let response: serde_json::Value = client
                    .get(&url)
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&response, q)?
                } else {
                    response
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            ShardCommands::Health { uid } => {
                // Get shard details and extract health-related information
                let response: serde_json::Value = client
                    .get(&format!("/v1/shards/{}", uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Extract health-relevant fields
                let health = serde_json::json!({
                    "uid": response["uid"],
                    "status": response["status"],
                    "role": response["role"],
                    "loading": response["loading"],
                    "node": response["node"],
                    "memory_usage": response["memory_usage"],
                    "cpu_usage": response["cpu_usage"],
                    "connections": response["connections"],
                });

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&health, q)?
                } else {
                    health
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }

            ShardCommands::Config { uid } => {
                // Get shard configuration details
                let response: serde_json::Value = client
                    .get(&format!("/v1/shards/{}", uid))
                    .await
        .map_err(|e| RedisCtlError::from(e))?;

                // Extract configuration-relevant fields
                let config = serde_json::json!({
                    "uid": response["uid"],
                    "bdb_uid": response["bdb_uid"],
                    "node": response["node"],
                    "role": response["role"],
                    "shard_key_regex": response["shard_key_regex"],
                    "backup": response["backup"],
                    "replication": response["replication"],
                    "persistence": response["persistence"],
                });

                let output_data = if let Some(q) = query {
                    super::utils::apply_jmespath(&config, q)?
                } else {
                    config
                };
                super::utils::print_formatted_output(output_data, output_format)?;
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub async fn handle_shard_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    shard_cmd: ShardCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    shard_cmd
        .execute(conn_mgr, profile_name, output_format, query)
        .await
}
