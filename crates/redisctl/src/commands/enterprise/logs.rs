use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum LogsCommands {
    /// List cluster event logs
    #[command(visible_alias = "ls")]
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
    },
}
