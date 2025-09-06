//! Enterprise CRDB (Active-Active) command implementations

#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use serde_json::Value;

use super::utils::*;

/// List all CRDBs
pub async fn list_crdbs(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw("/v1/crdbs")
        .await
        .context("Failed to list CRDBs")?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get CRDB details
pub async fn get_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}", id))
        .await
        .context(format!("Failed to get CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Create a new CRDB
pub async fn create_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .post_raw("/v1/crdbs", json_data)
        .await
        .context("Failed to create CRDB")?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Update CRDB configuration
pub async fn update_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .put_raw(&format!("/v1/crdbs/{}", id), json_data)
        .await
        .context(format!("Failed to update CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Delete a CRDB
pub async fn delete_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Delete CRDB {}?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .delete_raw(&format!("/v1/crdbs/{}", id))
        .await
        .context(format!("Failed to delete CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get participating clusters
pub async fn get_participating_clusters(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/participating_clusters", id))
        .await
        .context(format!(
            "Failed to get participating clusters for CRDB {}",
            id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Add cluster to CRDB
pub async fn add_cluster_to_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .post_raw(
            &format!("/v1/crdbs/{}/participating_clusters", id),
            json_data,
        )
        .await
        .context(format!("Failed to add cluster to CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Remove cluster from CRDB
pub async fn remove_cluster_from_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    cluster_id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Remove cluster {} from CRDB {}?", cluster_id, id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .delete_raw(&format!(
            "/v1/crdbs/{}/participating_clusters/{}",
            id, cluster_id
        ))
        .await
        .context(format!(
            "Failed to remove cluster {} from CRDB {}",
            cluster_id, id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get CRDB instances
pub async fn get_crdb_instances(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/instances", id))
        .await
        .context(format!("Failed to get instances for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get specific CRDB instance
pub async fn get_crdb_instance(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    crdb_id: u32,
    instance_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/instances/{}", crdb_id, instance_id))
        .await
        .context(format!(
            "Failed to get instance {} for CRDB {}",
            instance_id, crdb_id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Update CRDB instance
pub async fn update_crdb_instance(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    crdb_id: u32,
    instance_id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .put_raw(
            &format!("/v1/crdbs/{}/instances/{}", crdb_id, instance_id),
            json_data,
        )
        .await
        .context(format!(
            "Failed to update instance {} for CRDB {}",
            instance_id, crdb_id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Flush CRDB instance data
pub async fn flush_crdb_instance(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    crdb_id: u32,
    instance_id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force
        && !confirm_action(&format!(
            "Flush data for instance {} in CRDB {}? This will delete all data!",
            instance_id, crdb_id
        ))?
    {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .put_raw(
            &format!("/v1/crdbs/{}/instances/{}/flush", crdb_id, instance_id),
            Value::Null,
        )
        .await
        .context(format!(
            "Failed to flush instance {} for CRDB {}",
            instance_id, crdb_id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get replication status
pub async fn get_replication_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/replication_status", id))
        .await
        .context(format!("Failed to get replication status for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get replication lag
pub async fn get_replication_lag(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/lag", id))
        .await
        .context(format!("Failed to get replication lag for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Force sync CRDB
pub async fn force_sync_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    source_cluster: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = serde_json::json!({
        "source_cluster": source_cluster
    });

    let response = client
        .post_raw(&format!("/v1/crdbs/{}/sync", id), json_data)
        .await
        .context(format!("Failed to force sync CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Pause replication
pub async fn pause_replication(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .put_raw(&format!("/v1/crdbs/{}/replication/pause", id), Value::Null)
        .await
        .context(format!("Failed to pause replication for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Resume replication
pub async fn resume_replication(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .put_raw(&format!("/v1/crdbs/{}/replication/resume", id), Value::Null)
        .await
        .context(format!("Failed to resume replication for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get CRDB tasks
pub async fn get_crdb_tasks(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/tasks", id))
        .await
        .context(format!("Failed to get tasks for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get specific CRDB task
pub async fn get_crdb_task(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    crdb_id: u32,
    task_id: String,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/tasks/{}", crdb_id, task_id))
        .await
        .context(format!(
            "Failed to get task {} for CRDB {}",
            task_id, crdb_id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Retry failed CRDB task
pub async fn retry_crdb_task(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    crdb_id: u32,
    task_id: String,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .post_raw(
            &format!("/v1/crdbs/{}/tasks/{}/retry", crdb_id, task_id),
            Value::Null,
        )
        .await
        .context(format!(
            "Failed to retry task {} for CRDB {}",
            task_id, crdb_id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Cancel running CRDB task
pub async fn cancel_crdb_task(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    crdb_id: u32,
    task_id: String,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .post_raw(
            &format!("/v1/crdbs/{}/tasks/{}/cancel", crdb_id, task_id),
            Value::Null,
        )
        .await
        .context(format!(
            "Failed to cancel task {} for CRDB {}",
            task_id, crdb_id
        ))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get CRDB statistics
pub async fn get_crdb_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/stats", id))
        .await
        .context(format!("Failed to get statistics for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get CRDB metrics
pub async fn get_crdb_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    interval: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let mut path = format!("/v1/crdbs/{}/metrics", id);
    if let Some(interval) = interval {
        path.push_str(&format!("?interval={}", interval));
    }

    let response = client
        .get_raw(&path)
        .await
        .context(format!("Failed to get metrics for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Run health check on CRDB
pub async fn health_check_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/crdbs/{}/health", id))
        .await
        .context(format!("Failed to run health check for CRDB {}", id))?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
