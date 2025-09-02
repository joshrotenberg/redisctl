//! Example of database management operations
//!
//! This example shows how to:
//! - List databases in a subscription
//! - Get database details
//! - Create a new database
//!
//! Run with: cargo run --example database_management

use redis_cloud::{CloudClient, CloudDatabaseHandler};
use std::env;

// Uncomment when using the database creation example
// use redis_cloud::CreateDatabaseRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API credentials from environment variables
    let api_key = env::var("REDIS_CLOUD_API_KEY")?;
    let api_secret = env::var("REDIS_CLOUD_API_SECRET")?;

    // Optional: specify subscription ID via env var or use a default
    let subscription_id: u32 = env::var("REDIS_CLOUD_SUBSCRIPTION_ID")
        .unwrap_or_else(|_| "123456".to_string())
        .parse()?;

    // Create the client and database handler
    let client = CloudClient::builder()
        .api_key(&api_key)
        .api_secret(&api_secret)
        .build()?;

    let db_handler = CloudDatabaseHandler::new(client.clone());

    // List all databases in the subscription
    println!("Listing databases in subscription {}...", subscription_id);
    let databases = db_handler.list(subscription_id).await?;

    if let Some(dbs) = databases.as_array() {
        println!("Found {} database(s):", dbs.len());
        for db in dbs {
            println!(
                "  - ID: {}, Name: {}, Status: {}, Memory: {} MB",
                db["databaseId"],
                db["name"],
                db["status"],
                db["memoryLimitInGb"].as_f64().unwrap_or(0.0) * 1024.0
            );
        }

        // Get details of the first database
        if let Some(first_db) = dbs.first() {
            let db_id = first_db["databaseId"].as_u64().unwrap() as u32;
            println!("\nGetting details for database {}...", db_id);

            let db_details = db_handler.get(subscription_id, db_id).await?;

            println!("Database details:");
            println!("  Protocol: {}", db_details["protocol"]);
            println!("  Endpoint: {}", db_details["publicEndpoint"]);
            println!(
                "  Security: {}",
                db_details["security"]["sslClientAuthentication"]
            );
        }
    } else {
        println!("No databases found");
    }

    // Example: Create a new database (commented out to prevent accidental creation)
    // Uncomment and modify as needed
    /*
    println!("\nCreating a new database...");
    // Using the new builder pattern for cleaner API
    let new_database = CreateDatabaseRequest::builder()
        .name("example-db")
        .memory_limit_in_gb(0.1) // 100 MB
        .data_persistence("none")
        .replication(false)
        .data_eviction("volatile-lru")
        .support_oss_cluster_api(false)
        .build();

    let created_db = db_handler
        .create(subscription_id, new_database)
        .await?;

    println!("Created database: ID={}, Name={}",
        created_db["databaseId"],
        created_db["name"]
    );
    */

    Ok(())
}
