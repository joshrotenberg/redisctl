//! High-level workflow implementations

mod cluster;
mod database;

use anyhow::Result;
use redis_enterprise::EnterpriseClient;
use serde_json::Value;
use crate::commands::WorkflowCommands;

pub async fn handle_workflow_command(client: &EnterpriseClient, command: WorkflowCommands) -> Result<Value> {
    match command {
        WorkflowCommands::InitCluster { 
            name, 
            username, 
            password, 
            accept_eula, 
            license, 
            with_database 
        } => {
            cluster::init_cluster(
                client,
                name,
                username,
                password,
                accept_eula,
                license,
                with_database
            ).await
        }
        WorkflowCommands::SetupHa { replicas } => {
            cluster::setup_ha(client, replicas).await
        }
        WorkflowCommands::CreateDatabase { name, db_type } => {
            database::create_database(client, name, db_type).await
        }
    }
}