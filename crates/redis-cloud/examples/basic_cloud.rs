//! Basic example of using the Redis Cloud API client
//!
//! This example shows how to:
//! - Connect to the Redis Cloud API
//! - Get account information
//! - List subscriptions
//!
//! Run with: cargo run --example basic_cloud

use redis_cloud::CloudClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API credentials from environment variables
    let api_key =
        env::var("REDIS_CLOUD_API_KEY").expect("REDIS_CLOUD_API_KEY environment variable not set");
    let api_secret = env::var("REDIS_CLOUD_API_SECRET")
        .expect("REDIS_CLOUD_API_SECRET environment variable not set");

    // Create the client using the builder pattern
    let client = CloudClient::builder()
        .api_key(&api_key)
        .api_secret(&api_secret)
        .build()?;

    // Get account information using raw API
    println!("Fetching account information...");
    let account = client.get_raw("/account").await?;
    println!("Account ID: {}", account["account"]["id"]);
    println!("Account Name: {}", account["account"]["name"]);
    println!();

    // List all subscriptions using raw API
    println!("Fetching subscriptions...");
    let subscriptions = client.get_raw("/subscriptions").await?;

    if let Some(subs) = subscriptions.as_array() {
        println!("Found {} subscription(s):", subs.len());
        for sub in subs {
            println!(
                "  - ID: {}, Name: {}, Status: {}",
                sub["id"], sub["name"], sub["status"]
            );
        }
    } else {
        println!("No subscriptions found");
    }

    Ok(())
}
