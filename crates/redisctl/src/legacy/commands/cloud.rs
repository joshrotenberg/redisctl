use crate::config::{Profile, ProfileCredentials};
use crate::output::{OutputFormat, print_output};
use anyhow::Result;
use redis_cloud::CloudClient;

use crate::cli::{
    AccountCommands, AclCommands, ApiKeyCommands, BackupCommands, CloudAccountCommands,
    CloudCommands, CrdbCommands, DatabaseCommands, FixedPlanCommands, FlexiblePlanCommands,
    LogsCommands, MetricsCommands, PeeringCommands, PrivateServiceConnectCommands, RegionCommands,
    SsoCommands, SubscriptionCommands, TaskCommands, TransitGatewayCommands, UserCommands,
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
        CloudCommands::ApiKey { command } => {
            handle_api_key_command(command, profile, output_format, query).await
        }
        CloudCommands::Metrics { command } => {
            handle_metrics_command(command, profile, output_format, query).await
        }
        CloudCommands::Logs { command } => {
            handle_logs_command(command, profile, output_format, query).await
        }
        CloudCommands::CloudAccount { command } => {
            handle_cloud_account_command(command, profile, output_format, query).await
        }
        CloudCommands::FixedPlan { command } => {
            handle_fixed_plan_command(command, profile, output_format, query).await
        }
        CloudCommands::FlexiblePlan { command } => {
            handle_flexible_plan_command(command, profile, output_format, query).await
        }
        CloudCommands::PrivateServiceConnect { command } => {
            handle_private_service_connect_command(command, profile, output_format, query).await
        }
        CloudCommands::Sso { command } => {
            handle_sso_command(command, profile, output_format, query).await
        }
        CloudCommands::Billing { command } => {
            let client = create_cloud_client(profile)?;
            crate::commands::cloud_billing::handle_billing_command(
                command,
                &client,
                output_format,
                query,
            )
            .await
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
            // Use typed API to list all databases
            let handler = redis_cloud::CloudDatabaseHandler::new(client.clone());
            let databases = handler.list_all().await?;
            let value = serde_json::to_value(databases)?;
            print_output(value, output_format, query)?;
        }
        DatabaseCommands::Show { id } => {
            // Parse subscription_id:database_id format or just database_id
            let (subscription_id, database_id) = parse_database_id(&id)?;
            let handler = redis_cloud::CloudDatabaseHandler::new(client.clone());
            let database = handler.get(subscription_id, database_id).await?;
            let value = serde_json::to_value(database)?;
            print_output(value, output_format, query)?;
        }
        DatabaseCommands::Create {
            name: _,
            memory_limit: _,
            modules: _,
        } => {
            anyhow::bail!(
                "Database creation requires subscription context. Use 'redisctl cloud subscription create-database' instead."
            );
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
            let handler = redis_cloud::CloudDatabaseHandler::new(client.clone());
            handler.delete(subscription_id, database_id).await?;
            println!("Database {} deleted successfully", id);
        }
        DatabaseCommands::Backup { id } => {
            let (subscription_id, database_id) = parse_database_id(&id)?;
            let handler = redis_cloud::CloudDatabaseHandler::new(client.clone());
            let task = handler.backup(subscription_id, database_id).await?;
            print_output(task, output_format, query)?;
        }
        DatabaseCommands::Import { id, url } => {
            let (subscription_id, database_id) = parse_database_id(&id)?;
            let handler = redis_cloud::CloudDatabaseHandler::new(client.clone());
            let import_data = serde_json::json!({
                "source_type": "ftp",
                "import_from_uri": [url]
            });
            let task = handler
                .import(subscription_id, database_id, import_data)
                .await?;
            print_output(task, output_format, query)?;
        }
        DatabaseCommands::Export { id, format } => {
            let (subscription_id, database_id) = parse_database_id(&id)?;
            let export_data = serde_json::json!({
                "format": format
            });
            let task = client
                .post_raw(
                    &format!(
                        "/subscriptions/{}/databases/{}/export",
                        subscription_id, database_id
                    ),
                    export_data,
                )
                .await?;
            print_output(task, output_format, query)?;
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
            let handler = redis_cloud::CloudSubscriptionHandler::new(client.clone());
            let subscriptions = handler.list().await?;
            let value = serde_json::to_value(subscriptions)?;
            print_output(value, output_format, query)?;
        }
        SubscriptionCommands::Show { id } => {
            let handler = redis_cloud::CloudSubscriptionHandler::new(client.clone());
            let subscription = handler.get(id.parse()?).await?;
            let value = serde_json::to_value(subscription)?;
            print_output(value, output_format, query)?;
        }
        SubscriptionCommands::Create {
            name,
            provider,
            region,
        } => {
            let create_data = serde_json::json!({
                "name": name,
                "cloudProvider": provider,
                "region": region
            });
            let subscription = client.post_raw("/subscriptions", create_data).await?;
            print_output(subscription, output_format, query)?;
        }
        SubscriptionCommands::Update { id, name } => {
            let mut update_data = serde_json::Map::new();
            if let Some(name) = name {
                update_data.insert("name".to_string(), serde_json::Value::String(name));
            }
            if update_data.is_empty() {
                anyhow::bail!("No update fields provided");
            }
            let subscription = client
                .put_raw(
                    &format!("/subscriptions/{}", id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(subscription, output_format, query)?;
        }
        SubscriptionCommands::Delete { id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete subscription {}? Use --force to skip confirmation.",
                    id
                );
                return Ok(());
            }
            let handler = redis_cloud::CloudSubscriptionHandler::new(client.clone());
            handler.delete(id.parse()?).await?;
            println!("Subscription {} deleted successfully", id);
        }
        SubscriptionCommands::Pricing { id } => {
            let handler = redis_cloud::CloudSubscriptionHandler::new(client.clone());
            let pricing = handler.pricing(id.parse()?).await?;
            print_output(pricing, output_format, query)?;
        }
        SubscriptionCommands::Databases { id } => {
            let handler = redis_cloud::CloudDatabaseHandler::new(client.clone());
            let databases = handler.list(id.parse()?).await?;
            print_output(databases, output_format, query)?;
        }
        SubscriptionCommands::CidrList { id } => {
            let handler = redis_cloud::CloudSubscriptionHandler::new(client.clone());
            let cidr = handler.get_cidr_whitelist(id.parse()?).await?;
            print_output(cidr, output_format, query)?;
        }
        SubscriptionCommands::CidrUpdate { id, cidrs } => {
            let handler = redis_cloud::CloudSubscriptionHandler::new(client.clone());
            let cidr_list: Vec<&str> = cidrs.split(',').map(|s| s.trim()).collect();
            let update_data = serde_json::json!({
                "cidr": cidr_list
            });
            let cidr = handler
                .update_cidr_whitelist(id.parse()?, update_data)
                .await?;
            print_output(cidr, output_format, query)?;
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
            // Note: /accounts endpoint doesn't exist in CloudAccountHandler
            let accounts = client.get_raw("/accounts").await?;
            print_output(accounts, output_format, query)?;
        }
        AccountCommands::Show { id } => {
            // Note: /accounts/{id} endpoint doesn't exist in CloudAccountHandler
            let account = client.get_raw(&format!("/accounts/{}", id)).await?;
            print_output(account, output_format, query)?;
        }
        AccountCommands::Info => {
            let handler = redis_cloud::CloudAccountHandler::new(client.clone());
            let account_info = handler.get().await?;
            let value = serde_json::to_value(account_info)?;
            print_output(value, output_format, query)?;
        }
        AccountCommands::Owner => {
            let handler = redis_cloud::CloudAccountHandler::new(client.clone());
            let owner = handler.owner().await?;
            print_output(owner, output_format, query)?;
        }
        AccountCommands::Users => {
            let handler = redis_cloud::CloudAccountHandler::new(client.clone());
            let users = handler.users().await?;
            print_output(users, output_format, query)?;
        }
        AccountCommands::PaymentMethods => {
            let handler = redis_cloud::CloudAccountHandler::new(client.clone());
            let payment_methods = handler.get_payment_methods().await?;
            print_output(payment_methods, output_format, query)?;
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
            let handler = redis_cloud::CloudUserHandler::new(client.clone());
            let users = handler.list().await?;
            let value = serde_json::to_value(users)?;
            print_output(value, output_format, query)?;
        }
        UserCommands::Show { id } => {
            let handler = redis_cloud::CloudUserHandler::new(client.clone());
            let user = handler.get(id.parse()?).await?;
            let value = serde_json::to_value(user)?;
            print_output(value, output_format, query)?;
        }
        UserCommands::Create {
            name,
            email,
            password,
            roles,
        } => {
            let mut create_data = serde_json::json!({
                "name": name
            });

            if let Some(email) = email {
                create_data["email"] = serde_json::Value::String(email);
            }
            if let Some(password) = password {
                create_data["password"] = serde_json::Value::String(password);
            }
            if !roles.is_empty() {
                create_data["roles"] = serde_json::Value::Array(
                    roles.into_iter().map(serde_json::Value::String).collect(),
                );
            }

            let user = client.post_raw("/users", create_data).await?;
            print_output(user, output_format, query)?;
        }
        UserCommands::Update {
            id,
            email,
            password,
        } => {
            let mut update_data = serde_json::Map::new();
            if let Some(email) = email {
                update_data.insert("email".to_string(), serde_json::Value::String(email));
            }
            if let Some(password) = password {
                update_data.insert("password".to_string(), serde_json::Value::String(password));
            }
            if update_data.is_empty() {
                anyhow::bail!("No update fields provided");
            }
            let user = client
                .put_raw(
                    &format!("/users/{}", id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(user, output_format, query)?;
        }
        UserCommands::Delete { id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete user {}? Use --force to skip confirmation.",
                    id
                );
                return Ok(());
            }
            let handler = redis_cloud::CloudUserHandler::new(client.clone());
            handler.delete(id.parse()?).await?;
            println!("User {} deleted successfully", id);
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
            // Note: CloudRegionHandler list() requires a provider parameter
            // Using raw API for now
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
            let handler = redis_cloud::CloudTaskHandler::new(client.clone());
            let tasks = handler.list().await?;
            let value = serde_json::to_value(tasks)?;
            print_output(value, output_format, query)?;
        }
        TaskCommands::Show { id } => {
            let handler = redis_cloud::CloudTaskHandler::new(client.clone());
            let task = handler.get(&id).await?;
            let value = serde_json::to_value(task)?;
            print_output(value, output_format, query)?;
        }
        TaskCommands::Wait { id, timeout } => {
            use std::time::{Duration, Instant};
            use tokio::time::sleep;

            let start = Instant::now();
            let timeout_duration = Duration::from_secs(timeout);

            loop {
                let handler = redis_cloud::CloudTaskHandler::new(client.clone());
                let task = handler.get(&id).await?;
                let task = serde_json::to_value(task)?;

                // Check if task has a status field and if it's completed
                if let Some(status) = task.get("status").and_then(|s| s.as_str()) {
                    match status {
                        "completed" => {
                            println!("Task {} completed successfully", id);
                            print_output(task, output_format, query)?;
                            break;
                        }
                        "failed" => {
                            println!("Task {} failed", id);
                            print_output(task, output_format, query)?;
                            anyhow::bail!("Task failed");
                        }
                        _ => {
                            // Task still running, check timeout
                            if start.elapsed() > timeout_duration {
                                println!(
                                    "Timeout waiting for task {} after {} seconds",
                                    id, timeout
                                );
                                print_output(task, output_format, query)?;
                                anyhow::bail!("Task wait timeout");
                            }
                            // Wait 5 seconds before checking again
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                } else {
                    // No status field, print task and break
                    print_output(task, output_format, query)?;
                    break;
                }
            }
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
            let handler = redis_cloud::CloudAclHandler::new(client.clone());
            let acls = handler.list(subscription_id, database_id).await?;
            print_output(acls, output_format, query)?;
        }
        AclCommands::Show {
            subscription_id,
            database_id,
            acl_id,
        } => {
            let handler = redis_cloud::CloudAclHandler::new(client.clone());
            let acl = handler.get(subscription_id, database_id, acl_id).await?;
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
                        "/subscriptions/{}/databases/{}/acl",
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
                        "/subscriptions/{}/databases/{}/acl/{}",
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
            let handler = redis_cloud::CloudAclHandler::new(client.clone());
            handler.delete(subscription_id, database_id, acl_id).await?;
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
            let handler = redis_cloud::CloudPeeringHandler::new(client.clone());
            let peerings = handler.list(subscription_id).await?;
            print_output(peerings, output_format, query)?;
        }
        PeeringCommands::Show {
            subscription_id,
            peering_id,
        } => {
            let handler = redis_cloud::CloudPeeringHandler::new(client.clone());
            let peering = handler.get(subscription_id, &peering_id).await?;
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
            let handler = redis_cloud::CloudPeeringHandler::new(client.clone());
            handler.delete(subscription_id, &peering_id).await?;
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
                        "/subscriptions/{}/databases/{}/backup",
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
            let restore_data = serde_json::json!({"backupId": backup_id});
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
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            let crdbs = handler.list().await?;
            print_output(crdbs, output_format, query)?;
        }
        CrdbCommands::Show { crdb_id } => {
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            let crdb = handler.get(crdb_id).await?;
            print_output(crdb, output_format, query)?;
        }
        CrdbCommands::Create {
            name,
            memory_limit,
            regions,
        } => {
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            let create_data = serde_json::json!({
                "name": name,
                "memoryLimitInGb": memory_limit as f64 / 1024.0,
                "regions": regions
            });
            let crdb = handler.create(create_data).await?;
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
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            let crdb = handler
                .update(crdb_id, serde_json::Value::Object(update_data))
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
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            handler.delete(crdb_id).await?;
            println!("CRDB {} deleted successfully", crdb_id);
        }
        CrdbCommands::AddRegion { crdb_id, region } => {
            let add_data = serde_json::json!({
                "region": region
            });
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            let result = handler.add_region(crdb_id, add_data).await?;
            print_output(result, output_format, query)?;
        }
        CrdbCommands::RemoveRegion { crdb_id, region_id } => {
            let handler = redis_cloud::CloudCrdbHandler::new(client.clone());
            handler.remove_region(crdb_id, region_id).await?;
            println!("Region {} removed from CRDB {}", region_id, crdb_id);
        }
    }

    Ok(())
}

pub async fn handle_api_key_command(
    command: ApiKeyCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        ApiKeyCommands::List => {
            let handler = redis_cloud::CloudApiKeyHandler::new(client.clone());
            let keys = handler.list().await?;
            print_output(keys, output_format, query)?;
        }
        ApiKeyCommands::Show { key_id } => {
            let handler = redis_cloud::CloudApiKeyHandler::new(client.clone());
            let key = handler.get(key_id).await?;
            print_output(key, output_format, query)?;
        }
        ApiKeyCommands::Create { name, role } => {
            let create_data = serde_json::json!({
                "name": name,
                "role": role
            });
            let key = client.post_raw("/api-keys", create_data).await?;
            print_output(key, output_format, query)?;
        }
        ApiKeyCommands::Update { key_id, name, role } => {
            let mut update_data = serde_json::Map::new();
            if let Some(name) = name {
                update_data.insert("name".to_string(), serde_json::Value::String(name));
            }
            if let Some(role) = role {
                update_data.insert("role".to_string(), serde_json::Value::String(role));
            }
            let key = client
                .put_raw(
                    &format!("/api-keys/{}", key_id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(key, output_format, query)?;
        }
        ApiKeyCommands::Delete { key_id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete API key {}? Use --force to skip confirmation.",
                    key_id
                );
                return Ok(());
            }
            let handler = redis_cloud::CloudApiKeyHandler::new(client.clone());
            handler.delete(key_id).await?;
            println!("API key {} deleted successfully", key_id);
        }
        ApiKeyCommands::Regenerate { key_id } => {
            let handler = redis_cloud::CloudApiKeyHandler::new(client.clone());
            let result = handler.regenerate(key_id).await?;
            print_output(result, output_format, query)?;
        }
        ApiKeyCommands::Enable { key_id } => {
            let handler = redis_cloud::CloudApiKeyHandler::new(client.clone());
            let result = handler.enable(key_id).await?;
            print_output(result, output_format, query)?;
        }
        ApiKeyCommands::Disable { key_id } => {
            let handler = redis_cloud::CloudApiKeyHandler::new(client.clone());
            let result = handler.disable(key_id).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_metrics_command(
    command: MetricsCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        MetricsCommands::Database {
            subscription_id,
            database_id,
            metric,
            period,
        } => {
            let metrics = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}/metrics?metric={}&period={}",
                    subscription_id, database_id, metric, period
                ))
                .await?;
            print_output(metrics, output_format, query)?;
        }
        MetricsCommands::Subscription {
            subscription_id,
            metric,
            period,
        } => {
            let metrics = client
                .get_raw(&format!(
                    "/subscriptions/{}/metrics?metric={}&period={}",
                    subscription_id, metric, period
                ))
                .await?;
            print_output(metrics, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_logs_command(
    command: LogsCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        LogsCommands::Database {
            subscription_id,
            database_id,
            log_type,
            limit,
            offset,
        } => {
            let logs = client
                .get_raw(&format!(
                    "/subscriptions/{}/databases/{}/logs?type={}&limit={}&offset={}",
                    subscription_id, database_id, log_type, limit, offset
                ))
                .await?;
            print_output(logs, output_format, query)?;
        }
        LogsCommands::System { limit, offset } => {
            let logs = client
                .get_raw(&format!("/logs/system?limit={}&offset={}", limit, offset))
                .await?;
            print_output(logs, output_format, query)?;
        }
        LogsCommands::Session { limit, offset } => {
            let logs = client
                .get_raw(&format!("/logs/session?limit={}&offset={}", limit, offset))
                .await?;
            print_output(logs, output_format, query)?;
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
        CloudClient::builder()
            .api_key(api_key.clone())
            .api_secret(api_secret.clone())
            .base_url(api_url.clone())
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(Into::into)
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

pub async fn handle_cloud_account_command(
    command: CloudAccountCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        CloudAccountCommands::List => {
            let handler = redis_cloud::CloudAccountsHandler::new(client.clone());
            let accounts = handler.list().await?;
            let value = serde_json::to_value(accounts)?;
            print_output(value, output_format, query)?;
        }
        CloudAccountCommands::Show { account_id } => {
            let account = client
                .get_raw(&format!("/cloud-accounts/{}", account_id))
                .await?;
            print_output(account, output_format, query)?;
        }
        CloudAccountCommands::Create {
            name,
            provider,
            access_key_id,
            secret_access_key,
        } => {
            let payload = serde_json::json!({
                "name": name,
                "provider": provider,
                "accessKeyId": access_key_id,
                "secretAccessKey": secret_access_key
            });
            let account = client.post_raw("/cloud-accounts", payload).await?;
            print_output(account, output_format, query)?;
        }
        CloudAccountCommands::Update {
            account_id,
            name,
            access_key_id,
            secret_access_key,
        } => {
            let mut payload = serde_json::Map::new();
            if let Some(name) = name {
                payload.insert("name".to_string(), serde_json::Value::String(name));
            }
            if let Some(access_key_id) = access_key_id {
                payload.insert(
                    "accessKeyId".to_string(),
                    serde_json::Value::String(access_key_id),
                );
            }
            if let Some(secret_access_key) = secret_access_key {
                payload.insert(
                    "secretAccessKey".to_string(),
                    serde_json::Value::String(secret_access_key),
                );
            }
            let account = client
                .put_raw(
                    &format!("/cloud-accounts/{}", account_id),
                    serde_json::Value::Object(payload),
                )
                .await?;
            print_output(account, output_format, query)?;
        }
        CloudAccountCommands::Delete { account_id, force } => {
            if !force {
                return Err(anyhow::anyhow!(
                    "This operation requires --force flag to confirm deletion"
                ));
            }
            client
                .delete_raw(&format!("/cloud-accounts/{}", account_id))
                .await?;
            println!("Cloud account {} deleted successfully", account_id);
        }
    }

    Ok(())
}

pub async fn handle_fixed_plan_command(
    command: FixedPlanCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        FixedPlanCommands::List => {
            let handler = redis_cloud::CloudFixedHandler::new(client.clone());
            let plans = handler.plans().await?;
            print_output(plans, output_format, query)?;
        }
        FixedPlanCommands::Show { plan_id } => {
            let handler = redis_cloud::CloudFixedHandler::new(client.clone());
            let plan = handler.plan(plan_id).await?;
            print_output(plan, output_format, query)?;
        }
        FixedPlanCommands::Plans { region } => {
            let plans = client
                .get_raw(&format!("/fixed-plans/regions/{}/plans", region))
                .await?;
            print_output(plans, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_flexible_plan_command(
    command: FlexiblePlanCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        FlexiblePlanCommands::List => {
            let plans = client.get_raw("/flexible-plans").await?;
            print_output(plans, output_format, query)?;
        }
        FlexiblePlanCommands::Show { plan_id } => {
            let plan = client
                .get_raw(&format!("/flexible-plans/{}", plan_id))
                .await?;
            print_output(plan, output_format, query)?;
        }
        FlexiblePlanCommands::Create {
            name,
            memory_limit_in_gb,
            maximum_databases,
        } => {
            let payload = serde_json::json!({
                "name": name,
                "memoryLimitInGb": memory_limit_in_gb,
                "maximumDatabases": maximum_databases
            });
            let plan = client.post_raw("/flexible-plans", payload).await?;
            print_output(plan, output_format, query)?;
        }
        FlexiblePlanCommands::Update {
            plan_id,
            name,
            memory_limit_in_gb,
            maximum_databases,
        } => {
            let mut payload = serde_json::Map::new();
            if let Some(name) = name {
                payload.insert("name".to_string(), serde_json::Value::String(name));
            }
            if let Some(memory_limit_in_gb) = memory_limit_in_gb {
                payload.insert(
                    "memoryLimitInGb".to_string(),
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(memory_limit_in_gb).unwrap(),
                    ),
                );
            }
            if let Some(maximum_databases) = maximum_databases {
                payload.insert(
                    "maximumDatabases".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(maximum_databases)),
                );
            }
            let plan = client
                .put_raw(
                    &format!("/flexible-plans/{}", plan_id),
                    serde_json::Value::Object(payload),
                )
                .await?;
            print_output(plan, output_format, query)?;
        }
        FlexiblePlanCommands::Delete { plan_id, force } => {
            if !force {
                return Err(anyhow::anyhow!(
                    "This operation requires --force flag to confirm deletion"
                ));
            }
            client
                .delete_raw(&format!("/flexible-plans/{}", plan_id))
                .await?;
            println!("Flexible plan {} deleted successfully", plan_id);
        }
    }

    Ok(())
}

pub async fn handle_private_service_connect_command(
    command: PrivateServiceConnectCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        PrivateServiceConnectCommands::List { subscription_id } => {
            let endpoints = client
                .get_raw(&format!(
                    "/subscriptions/{}/private-service-connect",
                    subscription_id
                ))
                .await?;
            print_output(endpoints, output_format, query)?;
        }
        PrivateServiceConnectCommands::Show {
            subscription_id,
            endpoint_id,
        } => {
            let endpoint = client
                .get_raw(&format!(
                    "/subscriptions/{}/private-service-connect/{}",
                    subscription_id, endpoint_id
                ))
                .await?;
            print_output(endpoint, output_format, query)?;
        }
        PrivateServiceConnectCommands::Create {
            subscription_id,
            service_name,
            allowed_principals,
        } => {
            let payload = serde_json::json!({
                "serviceName": service_name,
                "allowedPrincipals": allowed_principals.split(',').collect::<Vec<_>>()
            });
            let endpoint = client
                .post_raw(
                    &format!("/subscriptions/{}/private-service-connect", subscription_id),
                    payload,
                )
                .await?;
            print_output(endpoint, output_format, query)?;
        }
        PrivateServiceConnectCommands::Update {
            subscription_id,
            endpoint_id,
            service_name,
            allowed_principals,
        } => {
            let mut payload = serde_json::Map::new();
            if let Some(service_name) = service_name {
                payload.insert(
                    "serviceName".to_string(),
                    serde_json::Value::String(service_name),
                );
            }
            if let Some(allowed_principals) = allowed_principals {
                payload.insert(
                    "allowedPrincipals".to_string(),
                    serde_json::Value::Array(
                        allowed_principals
                            .split(',')
                            .map(|s| serde_json::Value::String(s.trim().to_string()))
                            .collect(),
                    ),
                );
            }
            let endpoint = client
                .put_raw(
                    &format!(
                        "/subscriptions/{}/private-service-connect/{}",
                        subscription_id, endpoint_id
                    ),
                    serde_json::Value::Object(payload),
                )
                .await?;
            print_output(endpoint, output_format, query)?;
        }
        PrivateServiceConnectCommands::Delete {
            subscription_id,
            endpoint_id,
            force,
        } => {
            if !force {
                return Err(anyhow::anyhow!(
                    "This operation requires --force flag to confirm deletion"
                ));
            }
            client
                .delete_raw(&format!(
                    "/subscriptions/{}/private-service-connect/{}",
                    subscription_id, endpoint_id
                ))
                .await?;
            println!(
                "Private Service Connect endpoint {} deleted successfully",
                endpoint_id
            );
        }
    }

    Ok(())
}

pub async fn handle_sso_command(
    command: SsoCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_cloud_client(profile)?;

    match command {
        SsoCommands::Show => {
            let sso_config = client.get_raw("/sso").await?;
            print_output(sso_config, output_format, query)?;
        }
        SsoCommands::Update {
            provider,
            login_url,
            logout_url,
            enabled,
        } => {
            let mut update_data = serde_json::json!({
                "provider": provider,
                "loginUrl": login_url
            });

            if let Some(logout_url) = logout_url {
                update_data["logoutUrl"] = serde_json::Value::String(logout_url);
            }
            if let Some(enabled) = enabled {
                update_data["enabled"] = serde_json::Value::Bool(enabled);
            }

            let sso_config = client.put_raw("/sso", update_data).await?;
            print_output(sso_config, output_format, query)?;
        }
        SsoCommands::Delete { force } => {
            if !force {
                println!(
                    "Are you sure you want to delete SSO configuration? Use --force to skip confirmation."
                );
                return Ok(());
            }
            client.delete_raw("/sso").await?;
            println!("SSO configuration deleted successfully");
        }
        SsoCommands::Test { email } => {
            let test_data = serde_json::json!({
                "email": email
            });
            let result = client.post_raw("/sso/test", test_data).await?;
            print_output(result, output_format, query)?;
        }

        // SAML specific commands
        SsoCommands::SamlShow => {
            let saml_config = client.get_raw("/sso/saml").await?;
            print_output(saml_config, output_format, query)?;
        }
        SsoCommands::SamlUpdate {
            issuer,
            sso_url,
            certificate,
        } => {
            let mut update_data = serde_json::json!({
                "issuer": issuer,
                "ssoUrl": sso_url
            });

            if let Some(certificate) = certificate {
                update_data["certificate"] = serde_json::Value::String(certificate);
            }

            let saml_config = client.put_raw("/sso/saml", update_data).await?;
            print_output(saml_config, output_format, query)?;
        }
        SsoCommands::SamlMetadata => {
            let metadata = client.get_raw("/sso/saml/metadata").await?;
            print_output(metadata, output_format, query)?;
        }
        SsoCommands::SamlUploadCert { certificate } => {
            let cert_data = serde_json::json!({
                "certificate": certificate
            });
            let result = client.post_raw("/sso/saml/certificate", cert_data).await?;
            print_output(result, output_format, query)?;
        }

        // User mapping commands
        SsoCommands::UserList => {
            let users = client.get_raw("/sso/users").await?;
            print_output(users, output_format, query)?;
        }
        SsoCommands::UserShow { id } => {
            let user = client.get_raw(&format!("/sso/users/{}", id)).await?;
            print_output(user, output_format, query)?;
        }
        SsoCommands::UserCreate {
            email,
            local_user_id,
            role,
        } => {
            let create_data = serde_json::json!({
                "email": email,
                "localUserId": local_user_id,
                "role": role
            });
            let user = client.post_raw("/sso/users", create_data).await?;
            print_output(user, output_format, query)?;
        }
        SsoCommands::UserUpdate {
            id,
            local_user_id,
            role,
        } => {
            let mut update_data = serde_json::Map::new();
            if let Some(local_user_id) = local_user_id {
                update_data.insert(
                    "localUserId".to_string(),
                    serde_json::Value::Number(local_user_id.into()),
                );
            }
            if let Some(role) = role {
                update_data.insert("role".to_string(), serde_json::Value::String(role));
            }
            if update_data.is_empty() {
                anyhow::bail!("No update fields provided");
            }

            let user = client
                .put_raw(
                    &format!("/sso/users/{}", id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(user, output_format, query)?;
        }
        SsoCommands::UserDelete { id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete SSO user mapping {}? Use --force to skip confirmation.",
                    id
                );
                return Ok(());
            }
            client.delete_raw(&format!("/sso/users/{}", id)).await?;
            println!("SSO user mapping {} deleted successfully", id);
        }

        // Group mapping commands
        SsoCommands::GroupList => {
            let groups = client.get_raw("/sso/groups").await?;
            print_output(groups, output_format, query)?;
        }
        SsoCommands::GroupCreate { name, role } => {
            let create_data = serde_json::json!({
                "name": name,
                "role": role
            });
            let group = client.post_raw("/sso/groups", create_data).await?;
            print_output(group, output_format, query)?;
        }
        SsoCommands::GroupUpdate { id, role } => {
            let update_data = serde_json::json!({
                "role": role
            });
            let group = client
                .put_raw(&format!("/sso/groups/{}", id), update_data)
                .await?;
            print_output(group, output_format, query)?;
        }
        SsoCommands::GroupDelete { id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete SSO group mapping {}? Use --force to skip confirmation.",
                    id
                );
                return Ok(());
            }
            client.delete_raw(&format!("/sso/groups/{}", id)).await?;
            println!("SSO group mapping {} deleted successfully", id);
        }
    }

    Ok(())
}
