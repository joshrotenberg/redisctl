//! Cluster command implementations for Redis Enterprise

#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::RedisCtlError;
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

#[allow(clippy::too_many_arguments)]
pub async fn update_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    name: Option<&str>,
    email_alerts: Option<bool>,
    rack_aware: Option<bool>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    use crate::error::RedisCtlError;

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ClusterHandler::new(client);

    // Start with JSON data if provided, otherwise empty object
    let mut request_obj: serde_json::Map<String, serde_json::Value> = if let Some(json_data) = data
    {
        let parsed = read_json_data(json_data).context("Failed to parse JSON data")?;
        parsed
            .as_object()
            .cloned()
            .unwrap_or_else(serde_json::Map::new)
    } else {
        serde_json::Map::new()
    };

    // Override with first-class parameters if provided
    if let Some(n) = name {
        request_obj.insert("name".to_string(), serde_json::json!(n));
    }
    if let Some(alerts) = email_alerts {
        request_obj.insert("email_alerts".to_string(), serde_json::json!(alerts));
    }
    if let Some(rack) = rack_aware {
        request_obj.insert("rack_aware".to_string(), serde_json::json!(rack));
    }

    // Validate at least one update field is provided
    if request_obj.is_empty() {
        return Err(RedisCtlError::InvalidInput {
            message: "At least one update field is required (--name, --email-alerts, --rack-aware, or --data)".to_string(),
        });
    }

    let update_data = serde_json::Value::Object(request_obj);
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

#[allow(clippy::too_many_arguments)]
pub async fn update_cluster_policy(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    default_shards_placement: Option<&str>,
    rack_aware: Option<bool>,
    default_redis_version: Option<&str>,
    persistent_node_removal: Option<bool>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Start with JSON from --data if provided, otherwise empty object
    let mut policy_data = if let Some(data_str) = data {
        read_json_data(data_str).context("Failed to parse policy data")?
    } else {
        serde_json::json!({})
    };

    let policy_obj = policy_data.as_object_mut().unwrap();

    // CLI parameters override JSON values
    if let Some(placement) = default_shards_placement {
        policy_obj.insert(
            "default_shards_placement".to_string(),
            serde_json::json!(placement),
        );
    }
    if let Some(rack) = rack_aware {
        policy_obj.insert("rack_aware".to_string(), serde_json::json!(rack));
    }
    if let Some(version) = default_redis_version {
        policy_obj.insert(
            "default_provisioned_redis_version".to_string(),
            serde_json::json!(version),
        );
    }
    if let Some(persistent) = persistent_node_removal {
        policy_obj.insert(
            "persistent_node_removal".to_string(),
            serde_json::json!(persistent),
        );
    }

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

#[allow(clippy::too_many_arguments)]
pub async fn bootstrap_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cluster_name: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let _handler = BootstrapHandler::new(client.clone());

    // Start with JSON from --data if provided, otherwise empty object
    let mut bootstrap_data = if let Some(data_str) = data {
        read_json_data(data_str).context("Failed to parse bootstrap data")?
    } else {
        serde_json::json!({})
    };

    // Build nested structure for bootstrap request
    // Structure: { "action": "create_cluster", "cluster": { "name": "..." }, "credentials": { "username": "...", "password": "..." } }
    let bootstrap_obj = bootstrap_data.as_object_mut().unwrap();

    // Set action if not already set
    if !bootstrap_obj.contains_key("action") {
        bootstrap_obj.insert("action".to_string(), serde_json::json!("create_cluster"));
    }

    // CLI parameters override JSON values
    if let Some(name) = cluster_name {
        let cluster = bootstrap_obj
            .entry("cluster")
            .or_insert(serde_json::json!({}));
        if let Some(cluster_obj) = cluster.as_object_mut() {
            cluster_obj.insert("name".to_string(), serde_json::json!(name));
        }
    }

    if username.is_some() || password.is_some() {
        let credentials = bootstrap_obj
            .entry("credentials")
            .or_insert(serde_json::json!({}));
        if let Some(creds_obj) = credentials.as_object_mut() {
            if let Some(user) = username {
                creds_obj.insert("username".to_string(), serde_json::json!(user));
            }
            if let Some(pass) = password {
                creds_obj.insert("password".to_string(), serde_json::json!(pass));
            }
        }
    }

    // Validate required fields
    let has_cluster_name = bootstrap_obj
        .get("cluster")
        .and_then(|c| c.get("name"))
        .is_some();
    let has_username = bootstrap_obj
        .get("credentials")
        .and_then(|c| c.get("username"))
        .is_some();
    let has_password = bootstrap_obj
        .get("credentials")
        .and_then(|c| c.get("password"))
        .is_some();

    if !has_cluster_name || !has_username || !has_password {
        return Err(RedisCtlError::InvalidInput {
            message: "Bootstrap requires --cluster-name, --username, and --password (or equivalent in --data)".to_string()
        });
    }

    // Use raw API since BootstrapRequest doesn't have Deserialize trait
    let result = client
        .post_raw("/v1/bootstrap", bootstrap_data)
        .await
        .map_err(RedisCtlError::from)?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn join_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    nodes_arg: &[String],
    username_arg: Option<&str>,
    password_arg: Option<&str>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Start with JSON from --data if provided, otherwise empty object
    let mut join_data = if let Some(data_str) = data {
        read_json_data(data_str).context("Failed to parse join data")?
    } else {
        serde_json::json!({})
    };

    let join_obj = join_data.as_object_mut().unwrap();

    // CLI parameters override JSON values
    if !nodes_arg.is_empty() {
        join_obj.insert("nodes".to_string(), serde_json::json!(nodes_arg));
    }
    if let Some(user) = username_arg {
        join_obj.insert("username".to_string(), serde_json::json!(user));
    }
    if let Some(pass) = password_arg {
        join_obj.insert("password".to_string(), serde_json::json!(pass));
    }

    // Extract required fields for join operation
    let nodes = join_obj
        .get("nodes")
        .and_then(|n| n.as_array())
        .and_then(|arr| arr.first())
        .and_then(|n| n.as_str())
        .ok_or_else(|| RedisCtlError::InvalidInput {
            message: "Join requires --nodes (or nodes in --data)".to_string(),
        })?
        .to_string();

    let username = join_obj
        .get("username")
        .and_then(|u| u.as_str())
        .ok_or_else(|| RedisCtlError::InvalidInput {
            message: "Join requires --username (or username in --data)".to_string(),
        })?
        .to_string();

    let password = join_obj
        .get("password")
        .and_then(|p| p.as_str())
        .ok_or_else(|| RedisCtlError::InvalidInput {
            message: "Join requires --password (or password in --data)".to_string(),
        })?
        .to_string();

    // Use ClusterHandler for join operation
    let cluster_handler = ClusterHandler::new(client);
    let result = cluster_handler
        .join_node(&nodes, &username, &password)
        .await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn recover_cluster(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Start with JSON from --data if provided, otherwise empty object
    let recovery_data = if let Some(data_str) = data {
        read_json_data(data_str).context("Failed to parse recovery data")?
    } else {
        serde_json::json!({})
    };

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

#[allow(clippy::too_many_arguments)]
pub async fn update_cluster_certificates(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    name: Option<&str>,
    certificate: Option<&str>,
    key: Option<&str>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Start with JSON from --data if provided, otherwise empty object
    let mut cert_data = if let Some(data_str) = data {
        read_json_data(data_str).context("Failed to parse certificate data")?
    } else {
        serde_json::json!({})
    };

    let cert_obj = cert_data.as_object_mut().unwrap();

    // CLI parameters override JSON values
    if let Some(cert_name) = name {
        cert_obj.insert("name".to_string(), serde_json::json!(cert_name));
    }
    if let Some(cert) = certificate {
        // Read certificate content - it could be a file reference
        let cert_content = read_json_data(cert).unwrap_or_else(|_| serde_json::json!(cert));
        let cert_str = cert_content.as_str().unwrap_or(cert);
        cert_obj.insert("certificate".to_string(), serde_json::json!(cert_str));
    }
    if let Some(k) = key {
        // Read key content - it could be a file reference
        let key_content = read_json_data(k).unwrap_or_else(|_| serde_json::json!(k));
        let key_str = key_content.as_str().unwrap_or(k);
        cert_obj.insert("key".to_string(), serde_json::json!(key_str));
    }

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

#[allow(clippy::too_many_arguments)]
pub async fn update_ocsp_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    enabled: Option<bool>,
    responder_url: Option<&str>,
    response_timeout: Option<u32>,
    query_frequency: Option<u32>,
    recovery_frequency: Option<u32>,
    recovery_max_tries: Option<u32>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let _handler = OcspHandler::new(client.clone());

    // Start with JSON from --data if provided, otherwise empty object
    let mut ocsp_data = if let Some(data_str) = data {
        read_json_data(data_str).context("Failed to parse OCSP data")?
    } else {
        serde_json::json!({})
    };

    let ocsp_obj = ocsp_data.as_object_mut().unwrap();

    // CLI parameters override JSON values
    if let Some(en) = enabled {
        ocsp_obj.insert("enabled".to_string(), serde_json::json!(en));
    }
    if let Some(url) = responder_url {
        ocsp_obj.insert("responder_url".to_string(), serde_json::json!(url));
    }
    if let Some(timeout) = response_timeout {
        ocsp_obj.insert("response_timeout".to_string(), serde_json::json!(timeout));
    }
    if let Some(freq) = query_frequency {
        ocsp_obj.insert("query_frequency".to_string(), serde_json::json!(freq));
    }
    if let Some(rec_freq) = recovery_frequency {
        ocsp_obj.insert(
            "recovery_frequency".to_string(),
            serde_json::json!(rec_freq),
        );
    }
    if let Some(max_tries) = recovery_max_tries {
        ocsp_obj.insert(
            "recovery_max_tries".to_string(),
            serde_json::json!(max_tries),
        );
    }

    // Use raw API since handler.update_config expects OcspConfig, not Value
    let result = client.put_raw("/v1/ocsp", ocsp_data).await?;
    let result_json = serde_json::to_value(result).context("Failed to serialize result")?;
    let data = handle_output(result_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
