//! Cluster initialization workflow

use anyhow::Result;
use redis_enterprise::EnterpriseClient;
use serde_json::Value;
use std::path::PathBuf;
use tracing::{info, warn};

/// Initialize a new Redis Enterprise cluster
pub async fn init_cluster(
    client: &EnterpriseClient,
    name: String,
    username: String,
    password: String,
    accept_eula: bool,
    license: Option<PathBuf>,
    with_database: Option<String>,
) -> Result<Value> {
    if !accept_eula {
        anyhow::bail!("You must accept the EULA with --accept-eula to initialize the cluster");
    }
    
    info!("Initializing Redis Enterprise cluster '{}'", name);
    
    // Step 1: Bootstrap the cluster
    let license_content = if let Some(path) = license {
        Some(std::fs::read_to_string(path)?)
    } else {
        None
    };
    
    let bootstrap_request = serde_json::json!({
        "action": "create_cluster",
        "cluster": {
            "name": name
        },
        "node": {
            "paths": {
                "persistent_path": "/var/opt/redislabs/persist",
                "ephemeral_path": "/var/opt/redislabs/tmp"
            }
        },
        "credentials": {
            "username": username.clone(),
            "password": password.clone()
        },
        "license_file": license_content
    });
    
    info!("Bootstrapping cluster");
    client.post_bootstrap("/v1/bootstrap/create_cluster", &bootstrap_request).await?;
    
    // Step 2: Wait for cluster to be ready
    info!("Waiting for cluster to become active");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
    // Step 3: Create initial database if requested
    if let Some(db_name) = with_database {
        info!("Creating initial database '{}'", db_name);
        
        // Need to create a new authenticated client with credentials
        // For now we'll use the existing client's connection
        // In a real scenario, we'd need to track the base URL from the client
        
        warn!("Database creation requires authentication. Please create manually after bootstrap.");
    }
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("Cluster '{}' initialized successfully", name)
    }))
}

/// Setup high availability for the cluster
pub async fn setup_ha(_client: &EnterpriseClient, _replicas: u32) -> Result<Value> {
    anyhow::bail!("HA setup workflow not yet implemented")
}