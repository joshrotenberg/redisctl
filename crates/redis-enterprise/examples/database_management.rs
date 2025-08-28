//! Example of database management operations in Redis Enterprise
//!
//! This example shows how to:
//! - Create a new database
//! - Configure database settings
//! - Get database statistics
//!
//! Run with: cargo run --example database_management

use redis_enterprise::{BdbHandler, EnterpriseClient};
use std::env;

// Uncomment when using the database creation example
// use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get cluster credentials from environment variables
    let url =
        env::var("REDIS_ENTERPRISE_URL").unwrap_or_else(|_| "https://localhost:9443".to_string());
    let username =
        env::var("REDIS_ENTERPRISE_USER").unwrap_or_else(|_| "admin@redis.local".to_string());
    let password = env::var("REDIS_ENTERPRISE_PASSWORD")?;
    let insecure = env::var("REDIS_ENTERPRISE_INSECURE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    // Create the client
    let client = EnterpriseClient::builder()
        .base_url(&url)
        .username(&username)
        .password(&password)
        .insecure(insecure)
        .build()?;

    // List existing databases using handler
    println!("Listing existing databases...");
    let db_handler = BdbHandler::new(client.clone());
    let databases = db_handler.list().await?;

    for db in &databases {
        println!("Database {}: {}", db.uid, db.name);

        // Get statistics for this database using raw API
        println!("  Getting statistics...");
        let stats: serde_json::Value = client.get(&format!("/v1/bdbs/{}/stats", db.uid)).await?;

        if let Some(intervals) = stats["intervals"].as_array()
            && let Some(latest) = intervals.last()
        {
            println!("  Latest stats:");
            println!(
                "    Used memory: {} MB",
                latest["used_memory"].as_f64().unwrap_or(0.0) / 1024.0 / 1024.0
            );
            println!(
                "    Total requests: {}",
                latest["total_req"].as_u64().unwrap_or(0)
            );
            println!("    Connections: {}", latest["conns"].as_u64().unwrap_or(0));
        }
    }
    println!();

    // Example: Create a new database (commented out to prevent accidental creation)
    // Uncomment and modify as needed
    /*
    println!("Creating a new database...");
    let new_database = json!({
        "name": "example-db",
        "memory_size": 104857600, // 100 MB in bytes
        "type": "redis",
        "port": 12000,
        "replication": false,
        "persistence": "disabled",
        "eviction_policy": "volatile-lru",
        "sharding": false,
        "shard_count": 1,
        "module_list": [],
        "authentication_redis_pass": "SecurePassword123!"
    });

    let created_db = client
        .database()
        .create(new_database)
        .await?;

    println!("Created database: ID={}, Name={}, Port={}",
        created_db["uid"],
        created_db["name"],
        created_db["port"]
    );

    // Wait for database to become active
    let db_id = created_db["uid"].as_u64().unwrap();
    loop {
        let status = client.database().get(db_id).await?;
        if status["status"].as_str() == Some("active") {
            println!("Database is now active!");
            break;
        }
        println!("Waiting for database to become active...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    */

    Ok(())
}
