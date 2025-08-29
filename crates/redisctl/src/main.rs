use crate::config::Config;
use anyhow::Result;
use clap::Parser;
use tracing::info;

mod cli;
mod commands;
mod config;
mod error;
mod output;
mod router;

use cli::Cli;
use router::route_command;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("Starting redisctl");

    // Load configuration
    let config = Config::load()?;

    // Route and execute command
    route_command(cli, &config).await
}
