use anyhow::Result;
use redis_cloud::{CloudClient, CloudConfig};
use redis_common::{print_output, OutputFormat, Profile, ProfileCredentials};

use crate::cli::{
    AccountCommands, AclCommands, BackupCommands, CloudCommands, CrdbCommands, DatabaseCommands,
    PeeringCommands, RegionCommands, SubscriptionCommands, TaskCommands, TransitGatewayCommands,
    UserCommands,
};
use crate::commands::api::handle_cloud_api;

#[allow(dead_code)]
pub async fn handle_cloud_command(
    command: CloudCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    match command {
        CloudCommands::Api { command } => {
            let client = create_cloud_client(profile)?;
            handle_cloud_api(&client, command, output_format, query).await
        }
        CloudCommands::Database { command } => {
            handle_database_command(command, profile, output_format, query).await
        }
        CloudCommands::Subscription { command } => {
            handle_subscription_command(command, profile, output_format, query).await
        }
        CloudCommands::Account { command } => {
            handle_account_command(command, profile, output_format, query).await
        }
        CloudCommands::User { command } => {
            handle_user_command(command, profile, output_format, query).await
        }
        CloudCommands::Region { command } => {
            handle_region_command(command, profile, output_format, query).await
        }
        CloudCommands::Task { command } => {
            handle_task_command(command, profile, output_format, query).await
        }
        CloudCommands::Acl { command } => {
            handle_acl_command(command, profile, output_format, query).await
        }
        CloudCommands::Peering { command } => {
            handle_peering_command(command, profile, output_format, query).await
        }
        CloudCommands::TransitGateway { command } => {
            handle_transit_gateway_command(command, profile, output_format, query).await
        }
        CloudCommands::Backup { command } => {
            handle_backup_command(command, profile, output_format, query).await
        }
        CloudCommands::Crdb { command } => {
            handle_crdb_command(command, profile, output_format, query).await
        }
    }
}

pub async fn handle_database_command(
    command: DatabaseCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        DatabaseCommands::List => {
            // For Cloud, we need to list databases across all subscriptions
            let subscriptions = client.get_raw("/subscriptions").await?;
            let mut all_databases = Vec::new();

            if let Some(subs) = subscriptions.as_array() {
                for subscription in subs {
                    if let Some(subscription_id) = subscription.get("id").and_then(|id| id.as_u64())
                    {
                        let databases = client
                            .get_raw(&format!("/subscriptions/{}/databases", subscription_id))
                            .await?;
                        if let Some(dbs) = databases.as_array() {
                            all_databases.extend(dbs.iter().cloned());
                        }
                    }
                }
            }

            print_output(all_databases, output_format, query)?;
        }
        DatabaseCommands::Show { id } => {
            // Parse subscription_id:database_id format or just database_id
            let (subscription_id, database_id) = parse_database_id(&id)?;
            let database = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ))
                .await?;
            print_output(database, output_format, query)?;
        }
        DatabaseCommands::Create {
            name: _,
            memory_limit: _,
            modules: _,
        } => {
            anyhow::bail!("Database creation requires subscription context. Use 'redisctl cloud subscription create-database' instead.");
        }
        DatabaseCommands::Update {
            id,
            name,
            memory_limit,
        } => {
            let (subscription_id, database_id) = parse_database_id(&id)?;
            let mut update_data = serde_json::Map::new();

            if let Some(name) = name {
                update_data.insert("name".to_string(), serde_json::Value::String(name));
            }
            if let Some(memory_limit) = memory_limit {
                update_data.insert(
                    "memoryLimitInGb".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(memory_limit / 1024)),
                );
            }

            let database = client
                .put_raw(
                    &format!(
                        "/subscriptions/{}/databases/{}",
                        subscription_id, database_id
                    ),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(database, output_format, query)?;
        }
        DatabaseCommands::Delete { id, force: _ } => {
            let (subscription_id, database_id) = parse_database_id(&id)?;
            client
                .delete_raw(&format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ))
                .await?;
            println!("Database {} deleted successfully", id);
        }
        DatabaseCommands::Backup { id: _ } => {
            anyhow::bail!("Backup operations not yet implemented for Cloud databases");
        }
        DatabaseCommands::Import { id: _, url: _ } => {
            anyhow::bail!("Import operations not yet implemented for Cloud databases");
        }
    }

    Ok(())
}

pub async fn handle_subscription_command(
    command: SubscriptionCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        SubscriptionCommands::List => {
            let subscriptions = client.get_raw("/subscriptions").await?;
            print_output(subscriptions, output_format, query)?;
        }
        SubscriptionCommands::Show { id } => {
            let subscription = client.get_raw(&format!("/subscriptions/{}", id)).await?;
            print_output(subscription, output_format, query)?;
        }
        SubscriptionCommands::Create {
            name: _,
            provider: _,
            region: _,
        } => {
            anyhow::bail!("Subscription creation not yet implemented");
        }
        SubscriptionCommands::Update { id: _, name: _ } => {
            anyhow::bail!("Subscription update not yet implemented");
        }
        SubscriptionCommands::Delete { id: _, force: _ } => {
            anyhow::bail!("Subscription deletion not yet implemented");
        }
    }

    Ok(())
}

pub async fn handle_account_command(
    command: AccountCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        AccountCommands::List => {
            let accounts = client.get_raw("/accounts").await?;
            print_output(accounts, output_format, query)?;
        }
        AccountCommands::Show { id } => {
            let account = client.get_raw(&format!("/accounts/{}", id)).await?;
            print_output(account, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_user_command(
    command: UserCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        UserCommands::List => {
            let users = client.get_raw("/users").await?;
            print_output(users, output_format, query)?;
        }
        UserCommands::Show { id } => {
            let user = client.get_raw(&format!("/users/{}", id)).await?;
            print_output(user, output_format, query)?;
        }
        UserCommands::Create {
            name: _,
            email: _,
            password: _,
            roles: _,
        } => {
            anyhow::bail!("User creation not yet implemented");
        }
        UserCommands::Update {
            id: _,
            email: _,
            password: _,
        } => {
            anyhow::bail!("User update not yet implemented");
        }
        UserCommands::Delete { id: _, force: _ } => {
            anyhow::bail!("User deletion not yet implemented");
        }
    }

    Ok(())
}

pub async fn handle_region_command(
    command: RegionCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        RegionCommands::List => {
            let regions = client.get_raw("/regions").await?;
            print_output(regions, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_task_command(
    command: TaskCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        TaskCommands::List => {
            let tasks = client.get_raw("/tasks").await?;
            print_output(tasks, output_format, query)?;
        }
        TaskCommands::Show { id } => {
            let task = client.get_raw(&format!("/tasks/{}", id)).await?;
            print_output(task, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_acl_command(
    command: AclCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        AclCommands::List {
            subscription_id,
            database_id,
        } => {
            let acls = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}/acls",
                    subscription_id, database_id
                ))
                .await?;
            print_output(acls, output_format, query)?;
        }
        AclCommands::Show {
            subscription_id,
            database_id,
            acl_id,
        } => {
            let acl = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}/acls/{}",
                    subscription_id, database_id, acl_id
                ))
                .await?;
            print_output(acl, output_format, query)?;
        }
        AclCommands::Create {
            subscription_id,
            database_id,
            name,
            rule,
        } => {
            let create_data = serde_json::json!({
                "name": name,
                "aclRule": rule
            });
            let acl = client
                .post_raw(
                    &format!(
                        "/subscriptions/{}/databases/{}/acls",
                        subscription_id, database_id
                    ),
                    create_data,
                )
                .await?;
            print_output(acl, output_format, query)?;
        }
        AclCommands::Update {
            subscription_id,
            database_id,
            acl_id,
            rule,
        } => {
            let update_data = serde_json::json!({
                "aclRule": rule
            });
            let acl = client
                .put_raw(
                    &format!(
                        "/subscriptions/{}/databases/{}/acls/{}",
                        subscription_id, database_id, acl_id
                    ),
                    update_data,
                )
                .await?;
            print_output(acl, output_format, query)?;
        }
        AclCommands::Delete {
            subscription_id,
            database_id,
            acl_id,
            force,
        } => {
            if !force {
                println!(
                    "Are you sure you want to delete ACL {}? Use --force to skip confirmation.",
                    acl_id
                );
                return Ok(());
            }
            client
                .delete_raw(&format!(
                    "/subscriptions/{}/databases/{}/acls/{}",
                    subscription_id, database_id, acl_id
                ))
                .await?;
            println!("ACL {} deleted successfully", acl_id);
        }
    }

    Ok(())
}

pub async fn handle_peering_command(
    command: PeeringCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        PeeringCommands::List { subscription_id } => {
            let peerings = client
                .get_raw(&format!("/subscriptions/{}/peerings", subscription_id))
                .await?;
            print_output(peerings, output_format, query)?;
        }
        PeeringCommands::Show {
            subscription_id,
            peering_id,
        } => {
            let peering = client
                .get_raw(&format!(
                    "/subscriptions/{}/peerings/{}",
                    subscription_id, peering_id
                ))
                .await?;
            print_output(peering, output_format, query)?;
        }
        PeeringCommands::Create {
            subscription_id,
            provider_account_id,
            vpc_id,
            vpc_cidr,
            region,
        } => {
            let create_data = serde_json::json!({
                "providerAccountId": provider_account_id,
                "vpcId": vpc_id,
                "vpcCidr": vpc_cidr,
                "region": region
            });
            let peering = client
                .post_raw(
                    &format!("/subscriptions/{}/peerings", subscription_id),
                    create_data,
                )
                .await?;
            print_output(peering, output_format, query)?;
        }
        PeeringCommands::Delete {
            subscription_id,
            peering_id,
            force,
        } => {
            if !force {
                println!(
                    "Are you sure you want to delete peering {}? Use --force to skip confirmation.",
                    peering_id
                );
                return Ok(());
            }
            client
                .delete_raw(&format!(
                    "/subscriptions/{}/peerings/{}",
                    subscription_id, peering_id
                ))
                .await?;
            println!("Peering {} deleted successfully", peering_id);
        }
    }

    Ok(())
}

pub async fn handle_transit_gateway_command(
    command: TransitGatewayCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        TransitGatewayCommands::List { subscription_id } => {
            let tgws = client
                .get_raw(&format!("/subscriptions/{}/tgws", subscription_id))
                .await?;
            print_output(tgws, output_format, query)?;
        }
        TransitGatewayCommands::Show {
            subscription_id,
            tgw_id,
        } => {
            let tgw = client
                .get_raw(&format!(
                    "/subscriptions/{}/tgws/{}",
                    subscription_id, tgw_id
                ))
                .await?;
            print_output(tgw, output_format, query)?;
        }
        TransitGatewayCommands::Create {
            subscription_id,
            tgw_id,
            aws_account_id,
            cidrs,
        } => {
            let create_data = serde_json::json!({
                "tgwId": tgw_id,
                "awsAccountId": aws_account_id,
                "cidrs": cidrs
            });
            let tgw = client
                .post_raw(
                    &format!("/subscriptions/{}/tgws", subscription_id),
                    create_data,
                )
                .await?;
            print_output(tgw, output_format, query)?;
        }
        TransitGatewayCommands::Delete {
            subscription_id,
            tgw_id,
            force,
        } => {
            if !force {
                println!(
                    "Are you sure you want to delete Transit Gateway attachment {}? Use --force to skip confirmation.",
                    tgw_id
                );
                return Ok(());
            }
            client
                .delete_raw(&format!(
                    "/subscriptions/{}/tgws/{}",
                    subscription_id, tgw_id
                ))
                .await?;
            println!("Transit Gateway attachment {} deleted successfully", tgw_id);
        }
    }

    Ok(())
}

pub async fn handle_backup_command(
    command: BackupCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        BackupCommands::List {
            subscription_id,
            database_id,
        } => {
            let backups = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}/backups",
                    subscription_id, database_id
                ))
                .await?;
            print_output(backups, output_format, query)?;
        }
        BackupCommands::Show {
            subscription_id,
            database_id,
            backup_id,
        } => {
            let backup = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}/backups/{}",
                    subscription_id, database_id, backup_id
                ))
                .await?;
            print_output(backup, output_format, query)?;
        }
        BackupCommands::Create {
            subscription_id,
            database_id,
        } => {
            let backup = client
                .post_raw(
                    &format!(
                        "/subscriptions/{}/databases/{}/backups",
                        subscription_id, database_id
                    ),
                    serde_json::json!({}),
                )
                .await?;
            print_output(backup, output_format, query)?;
        }
        BackupCommands::Restore {
            subscription_id,
            database_id,
            backup_id,
        } => {
            let restore_data = serde_json::json!({
                "backupId": backup_id
            });
            let result = client
                .post_raw(
                    &format!(
                        "/subscriptions/{}/databases/{}/restore",
                        subscription_id, database_id
                    ),
                    restore_data,
                )
                .await?;
            print_output(result, output_format, query)?;
        }
        BackupCommands::Delete {
            subscription_id,
            database_id,
            backup_id,
            force,
        } => {
            if !force {
                println!(
                    "Are you sure you want to delete backup {}? Use --force to skip confirmation.",
                    backup_id
                );
                return Ok(());
            }
            client
                .delete_raw(&format!(
                    "/subscriptions/{}/databases/{}/backups/{}",
                    subscription_id, database_id, backup_id
                ))
                .await?;
            println!("Backup {} deleted successfully", backup_id);
        }
    }

    Ok(())
}

pub async fn handle_crdb_command(
    command: CrdbCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        CrdbCommands::List => {
            let crdbs = client.get_raw("/crdbs").await?;
            print_output(crdbs, output_format, query)?;
        }
        CrdbCommands::Show { crdb_id } => {
            let crdb = client.get_raw(&format!("/crdbs/{}", crdb_id)).await?;
            print_output(crdb, output_format, query)?;
        }
        CrdbCommands::Create {
            name,
            memory_limit,
            regions,
        } => {
            let create_data = serde_json::json!({
                "name": name,
                "memoryLimitInGb": memory_limit as f64 / 1024.0,
                "regions": regions
            });
            let crdb = client.post_raw("/crdbs", create_data).await?;
            print_output(crdb, output_format, query)?;
        }
        CrdbCommands::Update {
            crdb_id,
            name,
            memory_limit,
        } => {
            let mut update_data = serde_json::Map::new();
            if let Some(name) = name {
                update_data.insert("name".to_string(), serde_json::Value::String(name));
            }
            if let Some(memory_limit) = memory_limit {
                update_data.insert(
                    "memoryLimitInGb".to_string(),
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(memory_limit as f64 / 1024.0).unwrap(),
                    ),
                );
            }
            let crdb = client
                .put_raw(
                    &format!("/crdbs/{}", crdb_id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(crdb, output_format, query)?;
        }
        CrdbCommands::Delete { crdb_id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete CRDB {}? Use --force to skip confirmation.",
                    crdb_id
                );
                return Ok(());
            }
            client.delete_raw(&format!("/crdbs/{}", crdb_id)).await?;
            println!("CRDB {} deleted successfully", crdb_id);
        }
        CrdbCommands::AddRegion { crdb_id, region } => {
            let add_data = serde_json::json!({
                "region": region
            });
            let result = client
                .post_raw(&format!("/crdbs/{}/regions", crdb_id), add_data)
                .await?;
            print_output(result, output_format, query)?;
        }
        CrdbCommands::RemoveRegion { crdb_id, region_id } => {
            client
                .delete_raw(&format!("/crdbs/{}/regions/{}", crdb_id, region_id))
                .await?;
            println!("Region {} removed from CRDB {}", region_id, crdb_id);
        }
    }

    Ok(())
}

pub fn create_cloud_client(profile: &Profile) -> Result<CloudClient> {
    if let ProfileCredentials::Cloud {
        api_key,
        api_secret,
        api_url,
    } = &profile.credentials
    {
        let config = CloudConfig {
            api_key: api_key.clone(),
            api_secret: api_secret.clone(),
            base_url: api_url.clone(),
            timeout: std::time::Duration::from_secs(30),
        };
        CloudClient::new(config).map_err(Into::into)
    } else {
        anyhow::bail!("Invalid profile type for Cloud commands")
    }
}

pub fn parse_database_id(id: &str) -> Result<(u32, u32)> {
    if let Some((sub_id, db_id)) = id.split_once(':') {
        Ok((sub_id.parse()?, db_id.parse()?))
    } else {
        anyhow::bail!(
            "Database ID must be in format 'subscription_id:database_id' for Cloud databases"
        )
    }
}
