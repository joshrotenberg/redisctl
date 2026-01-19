//! MCP (Model Context Protocol) server for Redis Cloud and Enterprise
//!
//! This crate provides an MCP server that exposes Redis Cloud and Enterprise
//! management operations as tools for AI systems.
//!
//! # Example
//!
//! ```no_run
//! use redisctl_mcp::RedisCtlMcp;
//! use rmcp::{ServiceExt, transport::stdio};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // profile=None, read_only=true, database_url=None
//!     let server = RedisCtlMcp::new(None, true, None)?;
//!     let service = server.serve(stdio()).await?;
//!     service.waiting().await?;
//!     Ok(())
//! }
//! ```

pub mod cloud_tools;
pub mod database_tools;
pub mod enterprise_tools;
pub mod error;
pub mod server;

pub use error::McpError;
pub use server::RedisCtlMcp;

/// Start the MCP server with stdio transport
pub async fn serve_stdio(
    profile: Option<&str>,
    read_only: bool,
    database_url: Option<&str>,
) -> anyhow::Result<()> {
    use rmcp::{ServiceExt, transport::stdio};
    use tracing::info;

    info!(
        profile = profile,
        read_only = read_only,
        database_url = database_url.map(|_| "[redacted]"),
        "Starting MCP server"
    );

    let server = RedisCtlMcp::new(profile, read_only, database_url)?;
    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
