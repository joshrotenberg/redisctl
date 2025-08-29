//! Simple cluster setup example
//!
//! This example demonstrates the basic workflow for setting up a Redis Enterprise cluster.
//!
//! Run with:
//! ```bash
//! cargo run --example cluster_setup_simple
//! ```

use redis_enterprise::{EnterpriseClient, bdb::DatabaseHandler};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Configuration from environment or defaults
    let base_url =
        env::var("REDIS_ENTERPRISE_URL").unwrap_or_else(|_| "https://localhost:9443".to_string());
    let username =
        env::var("REDIS_ENTERPRISE_USER").unwrap_or_else(|_| "admin@redis.local".to_string());
    let password =
        env::var("REDIS_ENTERPRISE_PASSWORD").unwrap_or_else(|_| "Redis123!".to_string());
    let insecure = env::var("REDIS_ENTERPRISE_INSECURE")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    println!("Redis Enterprise Cluster Setup");
    println!("==============================");
    println!("URL: {}", base_url);
    println!("Username: {}", username);
    println!();

    // Create client
    let client = EnterpriseClient::builder()
        .base_url(&base_url)
        .username(&username)
        .password(&password)
        .insecure(insecure)
        .build()?;

    // Step 1: Check cluster status using raw API
    println!("Step 1: Checking cluster status...");
    match client.get::<serde_json::Value>("/v1/cluster").await {
        Ok(cluster) => {
            if let Some(name) = cluster.get("name") {
                println!("✓ Cluster is initialized: {}", name);
            }
        }
        Err(e) => {
            println!("⚠ Cluster check failed: {}", e);
            println!("→ You may need to bootstrap the cluster first");
        }
    }

    // Step 2: Create a database using raw API
    println!("\nStep 2: Creating database...");

    let db_request = json!({
        "name": "test-database",
        "memory_size": 104857600, // 100 MB
        "port": 12000,
        "replication": false,
        "persistence": "aof",
        "eviction_policy": "volatile-lru",
        "authentication_redis_pass": "testpass123"
    });

    match client
        .post::<serde_json::Value, serde_json::Value>("/v1/bdbs", &db_request)
        .await
    {
        Ok(db) => {
            if let Some(uid) = db.get("uid") {
                println!("✓ Database created with ID: {}", uid);

                // Wait for database to be active
                println!("→ Waiting for database to become active...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                // Get database info
                if let Ok(db_info) = client
                    .get::<serde_json::Value>(&format!("/v1/bdbs/{}", uid))
                    .await
                    && let Some(status) = db_info.get("status")
                {
                    println!("✓ Database status: {}", status);
                }
            }
        }
        Err(e) => {
            println!("⚠ Database creation failed: {}", e);
        }
    }

    // Step 3: List databases using typed handler
    println!("\nStep 3: Listing databases...");
    let db_handler = DatabaseHandler::new(client.clone());

    match db_handler.list().await {
        Ok(databases) => {
            println!("✓ Found {} database(s):", databases.len());
            for db in databases {
                println!("  - {} (ID: {}, Port: {:?})", db.name, db.uid, db.port);
            }
        }
        Err(e) => {
            println!("⚠ Failed to list databases: {}", e);
        }
    }

    // Step 4: Get cluster stats
    println!("\nStep 4: Cluster statistics...");
    match client
        .get::<serde_json::Value>("/v1/cluster/stats/last")
        .await
    {
        Ok(stats) => {
            println!("✓ Cluster stats retrieved");
            if let Some(conns) = stats.get("conns") {
                println!("  Connections: {}", conns);
            }
        }
        Err(e) => {
            println!("⚠ Could not get stats: {}", e);
        }
    }

    println!("\n✓ Setup example complete!");
    println!("\nNext steps:");
    println!("1. Connect to your database: redis-cli -p 12000 -a testpass123");
    println!("2. Explore the API using the client.get() and client.post() methods");
    println!("3. Use typed handlers for common operations");

    Ok(())
}
