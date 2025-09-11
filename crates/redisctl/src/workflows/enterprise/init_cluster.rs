//! Initialize Redis Enterprise cluster workflow
//!
//! This workflow automates the process of setting up a new Redis Enterprise cluster,
//! including bootstrap, waiting for initialization, creating admin user, and
//! optionally creating a default database.

use crate::workflows::{Workflow, WorkflowArgs, WorkflowContext, WorkflowResult};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use redis_enterprise::EnterpriseClient;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

pub struct InitClusterWorkflow;

impl InitClusterWorkflow {
    pub fn new() -> Self {
        Self
    }
}

impl Workflow for InitClusterWorkflow {
    fn name(&self) -> &str {
        "init-cluster"
    }

    fn description(&self) -> &str {
        "Initialize a Redis Enterprise cluster with bootstrap and optional database creation"
    }

    fn execute(
        &self,
        context: WorkflowContext,
        args: WorkflowArgs,
    ) -> Pin<Box<dyn Future<Output = Result<WorkflowResult>> + Send>> {
        Box::pin(async move {
            println!("🚀 Starting Redis Enterprise Cluster Initialization Workflow");
            println!();

            // Get parameters
            let cluster_name = args
                .get_string("name")
                .unwrap_or_else(|| "redis-cluster".to_string());
            let username = args
                .get_string("username")
                .unwrap_or_else(|| "admin@redis.local".to_string());
            let password = args
                .get_string("password")
                .context("Password is required for cluster initialization")?;
            let create_db = args.get_bool("create_database").unwrap_or(true);
            let db_name = args
                .get_string("database_name")
                .unwrap_or_else(|| "default-db".to_string());
            let db_memory_gb = args.get_i64("database_memory_gb").unwrap_or(1);

            // Create client
            let client = context
                .conn_mgr
                .create_enterprise_client(context.profile_name.as_deref())
                .await
                .context("Failed to create Enterprise client")?;

            // Step 1: Check if cluster is already initialized
            println!("📊 Step 1: Checking cluster status...");
            let needs_bootstrap = check_if_needs_bootstrap(&client).await?;

            if !needs_bootstrap {
                println!("✅ Cluster is already initialized!");
                return Ok(WorkflowResult::success("Cluster already initialized")
                    .with_output("cluster_name", &cluster_name)
                    .with_output("already_initialized", true));
            }

            // Step 2: Bootstrap the cluster
            println!("🔧 Step 2: Bootstrapping cluster '{}'...", cluster_name);
            let bootstrap_data = json!({
                "name": cluster_name,
                "username": username,
                "password": password,
                "action": "create_cluster"
            });

            let bootstrap_result = client
                .post_bootstrap("/v1/bootstrap", &bootstrap_data)
                .await
                .context("Failed to bootstrap cluster")?;

            // Check if bootstrap returned an action ID (async operation)
            if let Some(action_id) = bootstrap_result.get("action_uid").and_then(|v| v.as_str()) {
                // Wait for bootstrap to complete
                wait_for_action(&client, action_id, "Cluster bootstrap").await?;
            } else {
                // Bootstrap was synchronous, just wait a bit for cluster to stabilize
                println!("   Waiting for cluster to stabilize...");
                sleep(Duration::from_secs(5)).await;
            }

            println!("✅ Cluster bootstrapped successfully!");

            // Step 3: Verify cluster is ready
            println!("🔍 Step 3: Verifying cluster is ready...");
            wait_for_cluster_ready(&client).await?;
            println!("✅ Cluster is ready!");

            // Step 4: Optionally create a default database
            if create_db {
                println!("💾 Step 4: Creating default database '{}'...", db_name);

                let db_data = json!({
                    "name": db_name,
                    "memory_size": db_memory_gb * 1024 * 1024 * 1024,  // Convert GB to bytes
                    "type": "redis",
                    "replication": false,
                    "persistence": "disabled"
                });

                match client.post_raw("/v1/bdbs", db_data).await {
                    Ok(db_result) => {
                        // Check for async operation
                        if let Some(action_id) =
                            db_result.get("action_uid").and_then(|v| v.as_str())
                        {
                            wait_for_action(&client, action_id, "Database creation").await?;
                        }

                        let db_uid = db_result
                            .get("uid")
                            .or_else(|| db_result.get("resource_id"))
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0);

                        println!("✅ Database created successfully (ID: {})", db_uid);
                    }
                    Err(e) => {
                        // Database creation failed, but cluster is initialized
                        eprintln!("⚠️  Warning: Failed to create database: {}", e);
                        eprintln!("   Cluster is initialized but database creation failed.");
                        eprintln!("   You can create a database manually later.");
                    }
                }
            } else {
                println!("⏭️  Step 4: Skipping database creation (--no-database flag set)");
            }

            // Final summary
            println!();
            println!("🎉 Cluster Initialization Complete!");
            println!();
            println!("📋 Summary:");
            println!("   • Cluster Name: {}", cluster_name);
            println!("   • Admin User: {}", username);
            if create_db {
                println!("   • Database: {} ({}GB)", db_name, db_memory_gb);
            }
            println!();
            println!("🔗 Access the cluster:");
            println!("   • Web UI: https://localhost:8443");
            println!("   • API: https://localhost:9443");
            println!();
            println!("Next steps:");
            println!("   • redisctl enterprise database list");
            println!("   • redisctl enterprise node list");
            println!("   • redisctl enterprise cluster info");

            Ok(WorkflowResult::success("Cluster initialized successfully")
                .with_output("cluster_name", &cluster_name)
                .with_output("username", &username)
                .with_output("database_created", create_db)
                .with_output("database_name", &db_name))
        })
    }
}

/// Check if the cluster needs bootstrap
async fn check_if_needs_bootstrap(client: &EnterpriseClient) -> Result<bool> {
    match client.get_raw("/v1/bootstrap").await {
        Ok(status) => {
            // Check if cluster is already bootstrapped
            if let Some(state) = status.get("state").and_then(|v| v.as_str()) {
                Ok(state == "unconfigured" || state == "new")
            } else {
                // If we can't determine state, assume it needs bootstrap
                Ok(true)
            }
        }
        Err(_) => {
            // If we can't get status, cluster might not be initialized
            Ok(true)
        }
    }
}

/// Wait for cluster to be ready
async fn wait_for_cluster_ready(client: &EnterpriseClient) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Waiting for cluster to be ready...");

    let max_attempts = 60; // 5 minutes with 5 second intervals
    for attempt in 1..=max_attempts {
        pb.set_message(format!(
            "Waiting for cluster to be ready... (attempt {}/{})",
            attempt, max_attempts
        ));

        match client.get_raw("/v1/cluster").await {
            Ok(cluster) => {
                if let Some(state) = cluster.get("state").and_then(|v| v.as_str()) {
                    if state == "active" || state == "ready" {
                        pb.finish_and_clear();
                        return Ok(());
                    }
                }
            }
            Err(_) => {
                // Cluster might still be initializing
            }
        }

        sleep(Duration::from_secs(5)).await;
    }

    pb.finish_and_clear();
    anyhow::bail!("Cluster did not become ready within 5 minutes")
}

/// Wait for an async action to complete
async fn wait_for_action(
    client: &EnterpriseClient,
    action_id: &str,
    operation_name: &str,
) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Waiting for {} to complete...", operation_name));

    let max_attempts = 120; // 10 minutes with 5 second intervals
    for attempt in 1..=max_attempts {
        pb.set_message(format!(
            "Waiting for {} to complete... (attempt {}/{})",
            operation_name, attempt, max_attempts
        ));

        match client.get_raw(&format!("/v1/actions/{}", action_id)).await {
            Ok(action) => {
                if let Some(status) = action.get("status").and_then(|v| v.as_str()) {
                    match status {
                        "completed" | "done" => {
                            pb.finish_and_clear();
                            return Ok(());
                        }
                        "failed" | "error" => {
                            pb.finish_and_clear();
                            let error_msg = action
                                .get("error")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error");
                            anyhow::bail!("{} failed: {}", operation_name, error_msg);
                        }
                        _ => {
                            // Still in progress
                        }
                    }
                }
            }
            Err(_) => {
                // Action might not be available yet
            }
        }

        sleep(Duration::from_secs(5)).await;
    }

    pb.finish_and_clear();
    anyhow::bail!("{} did not complete within 10 minutes", operation_name)
}
