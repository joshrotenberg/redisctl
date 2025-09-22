#![allow(dead_code)]

use crate::error::RedisCtlError;

use anyhow::{Context, Result as AnyhowResult};
use chrono::Local;
use clap::Subcommand;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::OutputFormat;
use crate::commands::cloud::async_utils::AsyncOperationArgs;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

/// Support package generation commands for troubleshooting
#[derive(Subcommand, Debug, Clone)]
pub enum SupportPackageCommands {
    /// Generate full cluster support package
    Cluster {
        /// Output file path (defaults to ./support-package-cluster-{timestamp}.tar.gz)
        #[arg(long = "file", short = 'f')]
        file: Option<PathBuf>,

        /// Use new API endpoint (/v1/cluster/debuginfo) instead of deprecated one
        #[arg(long)]
        use_new_api: bool,

        /// Skip pre-flight checks
        #[arg(long)]
        skip_checks: bool,

        /// Async operation options
        #[command(flatten)]
        async_ops: AsyncOperationArgs,
    },

    /// Generate database-specific support package
    Database {
        /// Database UID
        uid: u32,

        /// Output file path (defaults to ./support-package-database-{uid}-{timestamp}.tar.gz)
        #[arg(long = "file", short = 'f')]
        file: Option<PathBuf>,

        /// Use new API endpoint (/v1/bdbs/{uid}/debuginfo) instead of deprecated one
        #[arg(long)]
        use_new_api: bool,

        /// Skip pre-flight checks
        #[arg(long)]
        skip_checks: bool,

        /// Async operation options
        #[command(flatten)]
        async_ops: AsyncOperationArgs,
    },

    /// Generate node-specific support package
    Node {
        /// Node UID (optional, all nodes if not specified)
        uid: Option<u32>,

        /// Output file path (defaults to ./support-package-node-{uid}-{timestamp}.tar.gz)
        #[arg(long = "file", short = 'f')]
        file: Option<PathBuf>,

        /// Use new API endpoint (/v1/nodes/{uid}/debuginfo) instead of deprecated one
        #[arg(long)]
        use_new_api: bool,

        /// Skip pre-flight checks
        #[arg(long)]
        skip_checks: bool,

        /// Async operation options
        #[command(flatten)]
        async_ops: AsyncOperationArgs,
    },

    /// List available support packages (if supported by API)
    List,

    /// Check status of support package generation
    Status {
        /// Task ID from async operation
        task_id: String,
    },
}

/// Result structure for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct SupportPackageResult {
    pub success: bool,
    pub package_type: String,
    pub file_path: String,
    pub file_size: usize,
    pub file_size_display: String,
    pub elapsed_seconds: u64,
    pub cluster_name: Option<String>,
    pub cluster_version: Option<String>,
    pub message: String,
    pub timestamp: String,
}

/// Handle the support-package command
pub async fn handle_support_package_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: SupportPackageCommands,
    output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    match cmd {
        SupportPackageCommands::Cluster {
            file,
            use_new_api,
            skip_checks,
            async_ops,
        } => {
            let output_path = file.unwrap_or_else(|| {
                let timestamp = Local::now().format("%Y%m%dT%H%M%S");
                PathBuf::from(format!("support-package-cluster-{}.tar.gz", timestamp))
            });

            if !skip_checks {
                perform_preflight_checks(&output_path)?;
            }

            generate_cluster_package(
                conn_mgr,
                profile_name,
                output_path,
                use_new_api,
                &async_ops,
                output_format,
            )
            .await
        }

        SupportPackageCommands::Database {
            uid,
            file,
            use_new_api,
            skip_checks,
            async_ops,
        } => {
            let output_path = file.unwrap_or_else(|| {
                let timestamp = Local::now().format("%Y%m%dT%H%M%S");
                PathBuf::from(format!(
                    "support-package-database-{}-{}.tar.gz",
                    uid, timestamp
                ))
            });

            if !skip_checks {
                perform_preflight_checks(&output_path)?;
            }

            generate_database_package(
                conn_mgr,
                profile_name,
                uid,
                output_path,
                use_new_api,
                &async_ops,
                output_format,
            )
            .await
        }

        SupportPackageCommands::Node {
            uid,
            file,
            use_new_api,
            skip_checks,
            async_ops,
        } => {
            let output_path = file.unwrap_or_else(|| {
                let timestamp = Local::now().format("%Y%m%dT%H%M%S");
                let prefix = if let Some(node_uid) = uid {
                    format!("support-package-node-{}", node_uid)
                } else {
                    "support-package-nodes".to_string()
                };
                PathBuf::from(format!("{}-{}.tar.gz", prefix, timestamp))
            });

            if !skip_checks {
                perform_preflight_checks(&output_path)?;
            }

            generate_node_package(
                conn_mgr,
                profile_name,
                uid,
                output_path,
                use_new_api,
                &async_ops,
                output_format,
            )
            .await
        }

        SupportPackageCommands::List => list_support_packages(conn_mgr, profile_name).await,

        SupportPackageCommands::Status { task_id } => {
            check_support_package_status(conn_mgr, profile_name, &task_id).await
        }
    }
}

/// Perform pre-flight checks before generating support package
fn perform_preflight_checks(output_path: &Path) -> AnyhowResult<()> {
    // Check if output file already exists
    if output_path.exists() {
        eprintln!("Warning: File {} already exists", output_path.display());
        eprint!("Overwrite? (y/N): ");
        let mut response = String::new();
        std::io::stdin().read_line(&mut response)?;
        if !response.trim().eq_ignore_ascii_case("y") {
            return Err(anyhow::anyhow!("Operation cancelled by user"));
        }
    }

    // Check write permissions to parent directory
    let parent_dir = output_path.parent().unwrap_or(Path::new("."));
    let parent_dir = if parent_dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent_dir
    };
    if !parent_dir.exists() {
        return Err(anyhow::anyhow!(
            "Output directory {} does not exist",
            parent_dir.display()
        ));
    }

    // Check available disk space (warn if less than 1GB)
    if let Ok(metadata) = fs::metadata(parent_dir) {
        // This is platform-specific and would need proper implementation
        // For now, just a placeholder check
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            // Basic check - would need proper disk space checking
            if metadata.blocks() < 1000000 {
                eprintln!("Warning: Low disk space detected");
            }
        }
    }

    Ok(())
}

/// Generate cluster support package
async fn generate_cluster_package(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_path: PathBuf,
    use_new_api: bool,
    _async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Try to get cluster info for display (non-blocking if it fails)
    let mut cluster_name = None;
    let mut cluster_version = None;

    if let Ok(cluster_info) = client.get::<serde_json::Value>("/v1/cluster").await {
        cluster_name = cluster_info
            .get("name")
            .and_then(|v| v.as_str())
            .map(String::from);
        cluster_version = cluster_info
            .get("software_version")
            .and_then(|v| v.as_str())
            .map(String::from);
    }

    // Only show interactive output if not in JSON mode
    let spinner = if matches!(output_format, OutputFormat::Json) {
        None
    } else {
        println!("Redis Enterprise Support Package");
        println!("================================");

        if let Some(ref name) = cluster_name {
            println!("Cluster: {}", name);
        }
        if let Some(ref version) = cluster_version {
            println!("Version: {}", version);
        }

        println!("\nOutput: {}", output_path.display());
        println!("\nGenerating support package...");

        // Show progress spinner
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        spinner.set_message("Collecting cluster data...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(spinner)
    };

    let start_time = std::time::Instant::now();

    // Use the appropriate endpoint based on flag
    let debuginfo_handler = redis_enterprise::debuginfo::DebugInfoHandler::new(client);
    let data = if use_new_api {
        debuginfo_handler
            .cluster_debuginfo_binary()
            .await
        .map_err(|e| RedisCtlError::from(e))?
    } else {
        debuginfo_handler
            .all_binary()
            .await
        .map_err(|e| RedisCtlError::from(e))?
    };

    if let Some(spinner) = spinner {
        spinner.finish_and_clear();
    }

    // Save to file
    fs::write(&output_path, &data).context(format!(
        "Failed to save support package to {:?}",
        output_path
    ))?;

    let elapsed = start_time.elapsed();
    let file_size = data.len();
    let size_display = format_file_size(file_size);

    // Output based on format
    match output_format {
        OutputFormat::Json => {
            let result = SupportPackageResult {
                success: true,
                package_type: "cluster".to_string(),
                file_path: output_path.display().to_string(),
                file_size,
                file_size_display: size_display,
                elapsed_seconds: elapsed.as_secs(),
                cluster_name,
                cluster_version,
                message: "Support package created successfully".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            // Display success message with helpful information
            println!("\n✓ Support package created successfully");
            println!("  File: {}", output_path.display());
            println!("  Size: {}", size_display);
            println!("  Time: {}s", elapsed.as_secs());

            println!("\nNext steps:");
            println!("1. Upload to Redis Support: https://support.redis.com/upload");
            println!("2. Reference your case number when uploading");
            println!("3. Delete local file after upload to free space");
        }
    }

    Ok(())
}

/// Generate database support package
async fn generate_database_package(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: u32,
    output_path: PathBuf,
    use_new_api: bool,
    _async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Try to get database info for display
    let mut database_name = None;
    if let Ok(db_info) = client
        .get::<serde_json::Value>(&format!("/v1/bdbs/{}", uid))
        .await
    {
        database_name = db_info
            .get("name")
            .and_then(|v| v.as_str())
            .map(String::from);
    }

    // Only show interactive output if not in JSON mode
    let spinner = if matches!(output_format, OutputFormat::Json) {
        None
    } else {
        println!("Redis Enterprise Support Package");
        println!("================================");
        println!("Database: {}", uid);

        if let Some(ref name) = database_name {
            println!("Name: {}", name);
        }

        println!("\nOutput: {}", output_path.display());
        println!("\nGenerating support package...");

        // Show progress spinner
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        spinner.set_message(format!("Collecting database {} data...", uid));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(spinner)
    };

    let start_time = std::time::Instant::now();

    // Use the appropriate endpoint based on flag
    let debuginfo_handler = redis_enterprise::debuginfo::DebugInfoHandler::new(client);
    let data = if use_new_api {
        debuginfo_handler
            .database_debuginfo_binary(uid)
            .await
            .context(format!("Failed to collect debug info for database {}", uid))?
    } else {
        debuginfo_handler
            .all_bdb_binary(uid)
            .await
            .context(format!("Failed to collect debug info for database {}", uid))?
    };

    if let Some(spinner) = spinner {
        spinner.finish_and_clear();
    }

    // Save to file
    fs::write(&output_path, &data).context(format!(
        "Failed to save support package to {:?}",
        output_path
    ))?;

    let elapsed = start_time.elapsed();
    let file_size = data.len();
    let size_display = format_file_size(file_size);

    // Output based on format
    match output_format {
        OutputFormat::Json => {
            let result = SupportPackageResult {
                success: true,
                package_type: format!("database-{}", uid),
                file_path: output_path.display().to_string(),
                file_size,
                file_size_display: size_display,
                elapsed_seconds: elapsed.as_secs(),
                cluster_name: Some(format!("Database {}", uid)), // Use DB info as context
                cluster_version: database_name, // Store DB name in cluster_version field
                message: "Database support package created successfully".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            println!("\n✓ Database support package created successfully");
            println!("  File: {}", output_path.display());
            println!("  Size: {}", size_display);
            println!("  Time: {}s", elapsed.as_secs());

            println!("\nNext steps:");
            println!("1. Upload to Redis Support: https://support.redis.com/upload");
            println!("2. Reference your case number when uploading");
            println!("3. Delete local file after upload to free space");
        }
    }

    Ok(())
}

/// Generate node support package
async fn generate_node_package(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: Option<u32>,
    output_path: PathBuf,
    use_new_api: bool,
    _async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Try to get node info for display
    let mut node_address = None;
    if let Some(node_uid) = uid
        && let Ok(node_info) = client
            .get::<serde_json::Value>(&format!("/v1/nodes/{}", node_uid))
            .await
    {
        node_address = node_info
            .get("addr")
            .and_then(|v| v.as_str())
            .map(String::from);
    }

    // Only show interactive output if not in JSON mode
    let spinner = if matches!(output_format, OutputFormat::Json) {
        None
    } else {
        println!("Redis Enterprise Support Package");
        println!("================================");

        if let Some(node_uid) = uid {
            println!("Node: {}", node_uid);
            if let Some(ref addr) = node_address {
                println!("Address: {}", addr);
            }
        } else {
            println!("Nodes: All");
        }

        println!("\nOutput: {}", output_path.display());
        println!("\nGenerating support package...");

        // Show progress spinner
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        let msg = if let Some(node_uid) = uid {
            format!("Collecting node {} data...", node_uid)
        } else {
            "Collecting all nodes data...".to_string()
        };
        spinner.set_message(msg);
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(spinner)
    };

    let start_time = std::time::Instant::now();

    // Use the appropriate endpoint based on flag
    let debuginfo_handler = redis_enterprise::debuginfo::DebugInfoHandler::new(client);
    let data = if let Some(node_uid) = uid {
        if use_new_api {
            debuginfo_handler
                .node_debuginfo_binary(node_uid)
                .await
                .context(format!(
                    "Failed to collect debug info for node {}",
                    node_uid
                ))?
        } else {
            // Old API doesn't support specific node ID, use node endpoint instead
            debuginfo_handler
                .node_binary()
                .await
        .map_err(|e| RedisCtlError::from(e))?
        }
    } else if use_new_api {
        debuginfo_handler
            .nodes_debuginfo_binary()
            .await
        .map_err(|e| RedisCtlError::from(e))?
    } else {
        debuginfo_handler
            .node_binary()
            .await
        .map_err(|e| RedisCtlError::from(e))?
    };

    if let Some(spinner) = spinner {
        spinner.finish_and_clear();
    }

    // Save to file
    fs::write(&output_path, &data).context(format!(
        "Failed to save support package to {:?}",
        output_path
    ))?;

    let elapsed = start_time.elapsed();
    let file_size = data.len();
    let size_display = format_file_size(file_size);

    // Output based on format
    match output_format {
        OutputFormat::Json => {
            let package_type = if let Some(node_uid) = uid {
                format!("node-{}", node_uid)
            } else {
                "nodes".to_string()
            };

            let result = SupportPackageResult {
                success: true,
                package_type,
                file_path: output_path.display().to_string(),
                file_size,
                file_size_display: size_display,
                elapsed_seconds: elapsed.as_secs(),
                cluster_name: uid.map(|id| format!("Node {}", id)),
                cluster_version: node_address, // Store node address in cluster_version field
                message: if uid.is_some() {
                    "Node support package created successfully".to_string()
                } else {
                    "Nodes support package created successfully".to_string()
                },
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            let package_type = if uid.is_some() { "Node" } else { "Nodes" };
            println!("\n✓ {} support package created successfully", package_type);
            println!("  File: {}", output_path.display());
            println!("  Size: {}", size_display);
            println!("  Time: {}s", elapsed.as_secs());

            println!("\nNext steps:");
            println!("1. Upload to Redis Support: https://support.redis.com/upload");
            println!("2. Reference your case number when uploading");
            println!("3. Delete local file after upload to free space");
        }
    }

    Ok(())
}

/// List available support packages (placeholder - API doesn't support this yet)
async fn list_support_packages(
    _conn_mgr: &ConnectionManager,
    _profile_name: Option<&str>,
) -> CliResult<()> {
    // The current API doesn't support listing support packages
    // This is a placeholder for future functionality
    eprintln!("Note: Listing support packages is not currently supported by the API");
    eprintln!("Support packages are generated on-demand and not stored on the server");
    Ok(())
}

/// Check status of support package generation
async fn check_support_package_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    task_id: &str,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Check if this is an async task status
    let debuginfo_handler = redis_enterprise::debuginfo::DebugInfoHandler::new(client);

    match debuginfo_handler.status(task_id).await {
        Ok(status) => {
            println!("Support Package Generation Status");
            println!("=================================");
            println!("Task ID: {}", status.task_id);
            println!("Status: {}", status.status);

            if let Some(progress) = status.progress {
                println!("Progress: {:.0}%", progress);
            }

            if let Some(error) = status.error {
                println!("Error: {}", error);
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to get status for task {}: {}", task_id, e);
            eprintln!("\nNote: Status checking is only available for async operations");
            Err(e.into())
        }
    }
}

/// Format file size for display
fn format_file_size(size: usize) -> String {
    if size > 1_000_000_000 {
        format!("{:.1} GB", size as f64 / 1_073_741_824.0)
    } else if size > 1_000_000 {
        format!("{:.1} MB", size as f64 / 1_048_576.0)
    } else if size > 1_000 {
        format!("{:.1} KB", size as f64 / 1_024.0)
    } else {
        format!("{} bytes", size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 bytes");
        assert_eq!(format_file_size(1_500), "1.5 KB");
        assert_eq!(format_file_size(1_500_000), "1.4 MB");
        assert_eq!(format_file_size(1_500_000_000), "1.4 GB");
    }
}
