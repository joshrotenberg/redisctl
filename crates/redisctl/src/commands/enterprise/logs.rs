use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum LogsCommands {
    /// List cluster event logs
    #[command(visible_alias = "ls")]
    #[command(after_help = "EXAMPLES:
    # Get recent logs
    redisctl enterprise logs list

    # Get logs since a specific time
    redisctl enterprise logs list --since 2024-01-01T00:00:00Z

    # Stream logs in real-time (like tail -f)
    redisctl enterprise logs list --follow

    # Stream logs with custom poll interval
    redisctl enterprise logs list --follow --poll-interval 5

    # Limit number of logs per fetch
    redisctl enterprise logs list --limit 50
")]
    List {
        /// Start time (ISO 8601 format)
        #[arg(long)]
        since: Option<String>,

        /// End time (ISO 8601 format)
        #[arg(long)]
        until: Option<String>,

        /// Sort order (asc or desc)
        #[arg(long, value_parser = ["asc", "desc"])]
        order: Option<String>,

        /// Maximum number of events to return
        #[arg(long)]
        limit: Option<u32>,

        /// Number of events to skip (for pagination)
        #[arg(long)]
        offset: Option<u32>,

        /// Follow log output (stream in real-time like tail -f)
        #[arg(long, short = 'f')]
        follow: bool,

        /// Poll interval in seconds when following logs
        #[arg(long, default_value = "2", requires = "follow")]
        poll_interval: u64,
    },
}
