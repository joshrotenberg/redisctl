use clap::{ArgGroup, Subcommand};

#[derive(Debug, Subcommand)]
pub enum ModuleCommands {
    /// List all available modules
    #[command(visible_alias = "ls")]
    List,

    /// Get module details by UID or name
    #[command(group(ArgGroup::new("identifier").required(true).args(["uid", "name"])))]
    Get {
        /// Module UID
        #[arg(conflicts_with = "name")]
        uid: Option<String>,

        /// Module name (e.g., "ReJSON", "search")
        #[arg(long, conflicts_with = "uid")]
        name: Option<String>,
    },

    /// Upload new module
    Upload {
        /// Module file path (e.g., @module.zip)
        #[arg(long)]
        file: String,
    },

    /// Delete module
    #[command(visible_alias = "rm")]
    Delete {
        /// Module UID
        uid: String,

        /// Force deletion without confirmation
        #[arg(long, short)]
        force: bool,
    },

    /// Configure module for database
    #[command(name = "config-bdb")]
    ConfigBdb {
        /// Database UID
        bdb_uid: u32,

        /// Configuration data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },
}
