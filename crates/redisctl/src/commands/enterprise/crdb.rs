//! Enterprise CRDB command handler

use crate::cli::{EnterpriseCrdbCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

use super::crdb_impl;

/// Handle enterprise CRDB commands
pub async fn handle_crdb_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &EnterpriseCrdbCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        EnterpriseCrdbCommands::List => {
            crdb_impl::list_crdbs(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseCrdbCommands::Get { id } => {
            crdb_impl::get_crdb(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::Create { data } => {
            crdb_impl::create_crdb(conn_mgr, profile_name, data, output_format, query).await
        }
        EnterpriseCrdbCommands::Update { id, data } => {
            crdb_impl::update_crdb(conn_mgr, profile_name, *id, data, output_format, query).await
        }
        EnterpriseCrdbCommands::Delete { id, force } => {
            crdb_impl::delete_crdb(conn_mgr, profile_name, *id, *force, output_format, query).await
        }
        EnterpriseCrdbCommands::GetClusters { id } => {
            crdb_impl::get_participating_clusters(conn_mgr, profile_name, *id, output_format, query)
                .await
        }
        EnterpriseCrdbCommands::AddCluster { id, data } => {
            crdb_impl::add_cluster_to_crdb(conn_mgr, profile_name, *id, data, output_format, query)
                .await
        }
        EnterpriseCrdbCommands::RemoveCluster {
            id,
            cluster_id,
            force,
        } => {
            crdb_impl::remove_cluster_from_crdb(
                conn_mgr,
                profile_name,
                *id,
                *cluster_id,
                *force,
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::GetInstances { id } => {
            crdb_impl::get_crdb_instances(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::GetInstance {
            crdb_id,
            instance_id,
        } => {
            crdb_impl::get_crdb_instance(
                conn_mgr,
                profile_name,
                *crdb_id,
                *instance_id,
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::UpdateInstance {
            crdb_id,
            instance_id,
            data,
        } => {
            crdb_impl::update_crdb_instance(
                conn_mgr,
                profile_name,
                *crdb_id,
                *instance_id,
                data,
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::FlushInstance {
            crdb_id,
            instance_id,
            force,
        } => {
            crdb_impl::flush_crdb_instance(
                conn_mgr,
                profile_name,
                *crdb_id,
                *instance_id,
                *force,
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::ReplicationStatus { id } => {
            crdb_impl::get_replication_status(conn_mgr, profile_name, *id, output_format, query)
                .await
        }
        EnterpriseCrdbCommands::ReplicationLag { id } => {
            crdb_impl::get_replication_lag(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::ForceSync { id, source_cluster } => {
            crdb_impl::force_sync_crdb(
                conn_mgr,
                profile_name,
                *id,
                *source_cluster,
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::PauseReplication { id } => {
            crdb_impl::pause_replication(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::ResumeReplication { id } => {
            crdb_impl::resume_replication(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::GetTasks { id } => {
            crdb_impl::get_crdb_tasks(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::GetTask { crdb_id, task_id } => {
            crdb_impl::get_crdb_task(
                conn_mgr,
                profile_name,
                *crdb_id,
                task_id.clone(),
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::RetryTask { crdb_id, task_id } => {
            crdb_impl::retry_crdb_task(
                conn_mgr,
                profile_name,
                *crdb_id,
                task_id.clone(),
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::CancelTask { crdb_id, task_id } => {
            crdb_impl::cancel_crdb_task(
                conn_mgr,
                profile_name,
                *crdb_id,
                task_id.clone(),
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::Stats { id } => {
            crdb_impl::get_crdb_stats(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseCrdbCommands::Metrics { id, interval } => {
            crdb_impl::get_crdb_metrics(
                conn_mgr,
                profile_name,
                *id,
                interval.as_deref(),
                output_format,
                query,
            )
            .await
        }
        EnterpriseCrdbCommands::HealthCheck { id } => {
            crdb_impl::health_check_crdb(conn_mgr, profile_name, *id, output_format, query).await
        }
    }
}
