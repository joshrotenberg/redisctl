use anyhow::Result;
use redis_cloud::{CloudClient, CloudConfig};
use redis_common::{print_output, OutputFormat, Profile, ProfileCredentials};

use crate::cli::{
    AccountCommands, AclCommands, CloudCommands, DatabaseCommands, RegionCommands,
    SubscriptionCommands, TaskCommands, UserCommands,
};

pub async fn handle_cloud_command(
    command: CloudCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    match command {
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
    _command: AclCommands,
    _profile: &Profile,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> Result<()> {
    anyhow::bail!("ACL commands not yet implemented for Cloud");
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
