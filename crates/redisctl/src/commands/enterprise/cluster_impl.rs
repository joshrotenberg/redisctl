//! Cluster command implementations for Redis Enterprise

#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::alerts::AlertHandler;
use redis_enterprise::bootstrap::BootstrapHandler;
use redis_enterprise::cluster::ClusterHandler;
use redis_enterprise::debuginfo::DebugInfoHandler;
use redis_enterprise::license::LicenseHandler;
use redis_enterprise::ocsp::OcspHandler;

use super::utils::*;

// ============================================================================
// Cluster Configuration Commands
// ============================================================================

pub async fn get_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ClusterHandler::new(client);
    let info = handler.info().await?;
    let info_json = serde_json::to_value(info).context("Failed to serialize cluster info")?;
    let data = handle_output(info_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ClusterHandler::new(client);

    let update_data = read_json_data(data).context("Failed to parse cluster data")?;
    let result = handler.update(update_data).await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_cluster_policy(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Cluster policies are typically part of the cluster info or a separate endpoint
    let policy = match client.get_raw("/v1/cluster/policy").await {
        Ok(result) => result,
        Err(_) => match client.get_raw("/v1/cluster/policies").await {
            Ok(result) => result,
            Err(_) => serde_json::json!({
                "message": "Policy endpoint not available"
            }),
        },
    };

    let data = handle_output(policy, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_cluster_policy(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let policy_data = read_json_data(data).context("Failed to parse policy data")?;
    let result = match client
        .put_raw("/v1/cluster/policy", policy_data.clone())
        .await
    {
        Ok(result) => result,
        Err(_) => client.put_raw("/v1/cluster/policies", policy_data).await?,
    };

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_cluster_license(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = LicenseHandler::new(client.clone());
    let license = handler.get().await?;
    let license_json = serde_json::to_value(license).context("Failed to serialize license")?;
    let data = handle_output(license_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_cluster_license(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    license_file: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let _handler = LicenseHandler::new(client.clone());

    // Read license file content
    let license_content = if let Some(file_path) = license_file.strip_prefix('@') {
        std::fs::read_to_string(file_path)
            .context(format!("Failed to read license file: {}", file_path))?
    } else {
        license_file.to_string()
    };

    // LicenseHandler.update expects LicenseUpdateRequest, not &str
    // Use the raw API instead
    let result = client
        .put_raw(
            "/v1/license",
            serde_json::json!({"license": license_content}),
        )
        .await?;
    let result_json = result;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// Cluster Operations Commands
// ============================================================================

pub async fn bootstrap_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let _handler = BootstrapHandler::new(client.clone());

    let bootstrap_data = read_json_data(data).context("Failed to parse bootstrap data")?;
    // Use raw API since BootstrapRequest doesn't have Deserialize trait
    let result = client
        .post_raw("/v1/bootstrap", bootstrap_data)
        .await
        .context("Failed to bootstrap cluster")?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn join_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let join_data = read_json_data(data).context("Failed to parse join data")?;

    // Extract required fields for join operation
    let nodes = join_data
        .get("nodes")
        .and_then(|n| n.as_array())
        .and_then(|arr| arr.first())
        .and_then(|n| n.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing 'nodes' field in join data"))?;

    let username = join_data
        .get("username")
        .and_then(|u| u.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing 'username' field in join data"))?;

    let password = join_data
        .get("password")
        .and_then(|p| p.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing 'password' field in join data"))?;

    // Use ClusterHandler for join operation
    let cluster_handler = ClusterHandler::new(client);
    let result = cluster_handler.join_node(nodes, username, password).await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn recover_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let recovery_data = read_json_data(data).context("Failed to parse recovery data")?;
    let result = client
        .post_raw("/v1/cluster/recover", recovery_data)
        .await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn reset_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    force: bool,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    if !force {
        eprintln!("WARNING: This will completely reset the cluster!");
        eprintln!("All data, configurations, and databases will be lost.");
        if !confirm_action("Are you absolutely sure you want to reset the cluster?")? {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    client
        .post_raw("/v1/cluster/reset", serde_json::json!({}))
        .await?;
    println!("Cluster reset initiated");
    Ok(())
}

// ============================================================================
// Cluster Monitoring Commands
// ============================================================================

pub async fn get_cluster_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ClusterHandler::new(client);
    let stats = handler.stats().await?;
    let data = handle_output(stats, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_cluster_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    interval: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = if let Some(interval) = interval {
        format!("/v1/cluster/metrics?interval={}", interval)
    } else {
        "/v1/cluster/metrics".to_string()
    };

    let metrics = client.get_raw(&endpoint).await?;
    let data = handle_output(metrics, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_cluster_alerts(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = AlertHandler::new(client);
    let alerts = handler.list().await?;
    let alerts_json = serde_json::to_value(alerts).context("Failed to serialize alerts")?;
    let data = handle_output(alerts_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_cluster_events(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    limit: Option<u32>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = if let Some(limit) = limit {
        format!("/v1/cluster/events?limit={}", limit)
    } else {
        "/v1/cluster/events".to_string()
    };

    let events = client.get_raw(&endpoint).await.unwrap_or_else(|_| {
        serde_json::json!({
            "message": "Events endpoint not available"
        })
    });

    let data = handle_output(events, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_audit_log(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    from_date: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = if let Some(from) = from_date {
        format!("/v1/cluster/audit_log?from={}", from)
    } else {
        "/v1/cluster/audit_log".to_string()
    };

    let audit_log = client.get_raw(&endpoint).await.unwrap_or_else(|_| {
        serde_json::json!({
            "message": "Audit log endpoint not available"
        })
    });

    let data = handle_output(audit_log, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// Cluster Maintenance Commands
// ============================================================================

pub async fn enable_maintenance_mode(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let result = client
        .post_raw(
            "/v1/cluster/maintenance_mode",
            serde_json::json!({"enabled": true}),
        )
        .await?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn disable_maintenance_mode(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let result = client
        .post_raw(
            "/v1/cluster/maintenance_mode",
            serde_json::json!({"enabled": false}),
        )
        .await?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn collect_debug_info(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let _handler = DebugInfoHandler::new(client.clone());

    // Use raw API since handler.create expects CreateCrdbRequest
    let result = client
        .post_raw("/v1/debuginfo", serde_json::json!({}))
        .await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn check_cluster_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ClusterHandler::new(client);

    // Get cluster info and check status
    let info = handler.info().await?;
    let status = serde_json::json!({
        "name": info.name,
        "status": info.status,
        "license_expired": info.license_expired,
        "nodes_count": info.nodes.as_ref().map(|n| n.len()),
        "databases_count": info.databases.as_ref().map(|d| d.len()),
        "total_memory": info.total_memory,
        "used_memory": info.used_memory,
        "memory_usage_percent": if let (Some(total), Some(used)) = (info.total_memory, info.used_memory) {
            if total > 0 {
                Some((used as f64 / total as f64) * 100.0)
            } else {
                None
            }
        } else {
            None
        }
    });

    let data = handle_output(status, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// Certificates & Security Commands
// ============================================================================

pub async fn get_cluster_certificates(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let certs = client.get_raw("/v1/cluster/certificates").await?;
    let data = handle_output(certs, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_cluster_certificates(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let cert_data = read_json_data(data).context("Failed to parse certificate data")?;
    let result = client
        .put_raw("/v1/cluster/certificates", cert_data)
        .await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn rotate_certificates(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let result = client
        .post_raw("/v1/cluster/certificates/rotate", serde_json::json!({}))
        .await?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_ocsp_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = OcspHandler::new(client);

    let config = handler.get_config().await?;
    let config_json = serde_json::to_value(config).context("Failed to serialize OCSP config")?;
    let data = handle_output(config_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_ocsp_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let _handler = OcspHandler::new(client.clone());

    let ocsp_data = read_json_data(data).context("Failed to parse OCSP data")?;
    // Use raw API since handler.update_config expects OcspConfig, not Value
    let result = client.put_raw("/v1/ocsp", ocsp_data).await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
