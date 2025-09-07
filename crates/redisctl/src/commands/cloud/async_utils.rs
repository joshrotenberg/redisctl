//! Shared utilities for handling asynchronous Cloud operations with --wait flag support

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::{RedisCtlError, Result as CliResult};
use crate::output::print_output;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Helper to print non-table output
fn print_json_or_yaml(data: Value, output_format: OutputFormat) -> CliResult<()> {
    match output_format {
        OutputFormat::Json => print_output(data, crate::output::OutputFormat::Json, None)?,
        OutputFormat::Yaml => print_output(data, crate::output::OutputFormat::Yaml, None)?,
        OutputFormat::Auto | OutputFormat::Table => {
            print_output(data, crate::output::OutputFormat::Json, None)?
        }
    }
    Ok(())
}

/// Common CLI arguments for async operations
#[derive(Args, Debug, Clone)]
pub struct AsyncOperationArgs {
    /// Wait for operation to complete
    #[arg(long)]
    pub wait: bool,

    /// Maximum time to wait in seconds
    #[arg(long, default_value = "300", requires = "wait")]
    pub wait_timeout: u64,

    /// Polling interval in seconds
    #[arg(long, default_value = "5", requires = "wait")]
    pub wait_interval: u64,
}

/// Handle an async operation response, optionally waiting for completion
pub async fn handle_async_response(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    response: Value,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
    success_message: &str,
) -> CliResult<()> {
    // Extract task ID from various possible locations
    let task_id = response
        .get("taskId")
        .or_else(|| response.get("task_id"))
        .or_else(|| response.get("response").and_then(|r| r.get("id")))
        .and_then(|v| v.as_str());

    // Apply JMESPath query if provided
    let result = if let Some(q) = query {
        crate::commands::cloud::utils::apply_jmespath(&response, q)?
    } else {
        response.clone()
    };

    // If we have a task ID and should wait
    if let Some(task_id) = task_id {
        if async_ops.wait {
            // Wait for the task to complete
            wait_for_task(
                conn_mgr,
                profile_name,
                task_id,
                async_ops.wait_timeout,
                async_ops.wait_interval,
                output_format,
            )
            .await?;

            // Print success message for table format
            if matches!(output_format, OutputFormat::Table) {
                println!("{}", success_message);
            }
            return Ok(());
        }
    }

    // Normal output without waiting
    match output_format {
        OutputFormat::Auto | OutputFormat::Table => {
            println!("{}", success_message);
            if let Some(task_id) = task_id {
                println!("Task ID: {}", task_id);
                println!(
                    "To wait for completion, run: redisctl cloud task wait {}",
                    task_id
                );
            }
        }
        OutputFormat::Json | OutputFormat::Yaml => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Wait for a task to complete
pub async fn wait_for_task(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    task_id: &str,
    timeout_secs: u64,
    interval_secs: u64,
    output_format: OutputFormat,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    let interval = Duration::from_secs(interval_secs);

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg} [{elapsed_precise}]")
            .unwrap(),
    );
    pb.set_message(format!("Waiting for task {}", task_id));

    loop {
        let task = fetch_task(&client, task_id).await?;
        let state = get_task_state(&task);

        pb.set_message(format!("Task {}: {}", task_id, format_task_state(&state)));

        if is_terminal_state(&state) {
            pb.finish_with_message(format!("Task {}: {}", task_id, format_task_state(&state)));

            match output_format {
                OutputFormat::Auto | OutputFormat::Table => {
                    print_task_details(&task)?;
                }
                OutputFormat::Json => {
                    print_output(task, crate::output::OutputFormat::Json, None)?;
                }
                OutputFormat::Yaml => {
                    print_output(task, crate::output::OutputFormat::Yaml, None)?;
                }
            }

            // Check if task failed
            if state == "failed" || state == "error" {
                return Err(RedisCtlError::InvalidInput {
                    message: format!("Task {} failed", task_id),
                });
            }

            return Ok(());
        }

        // Check timeout
        if start.elapsed() > timeout {
            pb.finish_with_message(format!("Task {} timed out", task_id));
            return Err(RedisCtlError::Timeout {
                message: format!(
                    "Task {} did not complete within {} seconds",
                    task_id, timeout_secs
                ),
            });
        }

        // Wait before next poll
        sleep(interval).await;
    }
}

/// Fetch task details from the API
async fn fetch_task(client: &redis_cloud::CloudClient, task_id: &str) -> CliResult<Value> {
    client
        .get_raw(&format!("/tasks/{}", task_id))
        .await
        .map_err(|e| RedisCtlError::ApiError {
            message: format!("Failed to fetch task {}: {}", task_id, e),
        })
}

/// Get task state from task response
fn get_task_state(task: &Value) -> String {
    task.get("status")
        .or_else(|| task.get("state"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Check if task is in a terminal state
fn is_terminal_state(state: &str) -> bool {
    matches!(
        state.to_lowercase().as_str(),
        "completed" | "complete" | "succeeded" | "success" | "failed" | "error" | "cancelled"
    )
}

/// Format task state for display
fn format_task_state(state: &str) -> String {
    match state.to_lowercase().as_str() {
        "completed" | "complete" | "succeeded" | "success" => format!("✓ {}", state),
        "failed" | "error" => format!("✗ {}", state),
        "cancelled" => format!("⊘ {}", state),
        "processing" | "running" | "in_progress" => format!("⟳ {}", state),
        _ => state.to_string(),
    }
}

/// Print detailed task information
fn print_task_details(task: &Value) -> CliResult<()> {
    println!("\nTask Details:");
    println!("-------------");

    if let Some(id) = task.get("taskId").or_else(|| task.get("id")) {
        println!("ID: {}", id);
    }

    if let Some(status) = task.get("status").or_else(|| task.get("state")) {
        println!("Status: {}", status);
    }

    if let Some(description) = task.get("description") {
        println!("Description: {}", description);
    }

    if let Some(progress) = task.get("progress") {
        println!("Progress: {}", progress);
    }

    if let Some(created) = task.get("createdAt").or_else(|| task.get("created_at")) {
        println!("Created: {}", created);
    }

    if let Some(updated) = task.get("updatedAt").or_else(|| task.get("updated_at")) {
        println!("Updated: {}", updated);
    }

    if let Some(error) = task.get("error").or_else(|| task.get("errorMessage")) {
        println!("Error: {}", error);
    }

    Ok(())
}
