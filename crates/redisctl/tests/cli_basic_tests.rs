use assert_cmd::Command;
use predicates::prelude::*;

/// Helper to create a test command
fn redisctl() -> Command {
    Command::cargo_bin("redisctl").unwrap()
}

#[test]
fn test_help_flag() {
    redisctl()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Redis management CLI"))
        .stdout(predicate::str::contains("EXAMPLES:"));
}

#[test]
fn test_help_short_flag() {
    redisctl()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version_flag() {
    redisctl()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("redisctl"))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_version_short_flag() {
    redisctl()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("redisctl"));
}

#[test]
fn test_no_args_shows_help() {
    redisctl()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_invalid_subcommand() {
    redisctl()
        .arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_profile_help() {
    redisctl()
        .arg("profile")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Profile management"));
}

#[test]
fn test_cloud_help() {
    redisctl()
        .arg("cloud")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cloud-specific"));
}

#[test]
fn test_enterprise_help() {
    redisctl()
        .arg("enterprise")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Enterprise-specific"));
}

#[test]
fn test_api_help() {
    redisctl()
        .arg("api")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Raw API access"));
}

#[test]
fn test_output_format_json() {
    // Test that -o json flag is accepted (doesn't test actual output)
    redisctl()
        .arg("profile")
        .arg("list")
        .arg("-o")
        .arg("json")
        .assert()
        .success();
}

#[test]
fn test_output_format_yaml() {
    redisctl()
        .arg("profile")
        .arg("list")
        .arg("-o")
        .arg("yaml")
        .assert()
        .success();
}

#[test]
fn test_output_format_table() {
    redisctl()
        .arg("profile")
        .arg("list")
        .arg("-o")
        .arg("table")
        .assert()
        .success();
}

#[test]
fn test_invalid_output_format() {
    redisctl()
        .arg("profile")
        .arg("list")
        .arg("-o")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn test_verbose_flag() {
    redisctl()
        .arg("-v")
        .arg("profile")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_multiple_verbose_flags() {
    redisctl()
        .arg("-vvv")
        .arg("profile")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_config_file_flag() {
    redisctl()
        .arg("--config-file")
        .arg("/tmp/test-config.toml")
        .arg("profile")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_profile_flag() {
    // Just test that the flag is accepted, actual profile doesn't need to exist for this test
    redisctl()
        .arg("--profile")
        .arg("nonexistent")
        .arg("profile")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_query_flag() {
    redisctl()
        .arg("profile")
        .arg("list")
        .arg("--query")
        .arg("profiles")
        .assert()
        .success();
}

#[test]
fn test_global_flags_before_subcommand() {
    redisctl()
        .arg("-v")
        .arg("-o")
        .arg("json")
        .arg("profile")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_profile_set_missing_required_args() {
    redisctl()
        .arg("profile")
        .arg("set")
        .arg("test-profile")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_profile_set_missing_deployment_type() {
    redisctl()
        .arg("profile")
        .arg("set")
        .arg("test-profile")
        .arg("--api-key")
        .arg("key")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--type"));
}

#[test]
fn test_profile_show_missing_name() {
    redisctl()
        .arg("profile")
        .arg("show")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_profile_remove_missing_name() {
    redisctl()
        .arg("profile")
        .arg("remove")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_payment_method_help() {
    redisctl()
        .arg("cloud")
        .arg("payment-method")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Payment method operations"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_payment_method_list_help() {
    redisctl()
        .arg("cloud")
        .arg("payment-method")
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List payment methods"));
}

// === SLOW-LOG COMMAND TESTS ===

#[test]
fn test_cloud_database_slow_log_help() {
    redisctl()
        .arg("cloud")
        .arg("database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Get slow query log"))
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("--offset"));
}

#[test]
fn test_cloud_database_slow_log_has_default_limit() {
    redisctl()
        .arg("cloud")
        .arg("database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("default: 100"));
}

#[test]
fn test_cloud_database_slow_log_has_default_offset() {
    redisctl()
        .arg("cloud")
        .arg("database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("default: 0"));
}

#[test]
fn test_cloud_fixed_database_slow_log_help() {
    redisctl()
        .arg("cloud")
        .arg("fixed-database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Get slow query log"))
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("--offset"));
}

#[test]
fn test_cloud_fixed_database_slow_log_has_defaults() {
    redisctl()
        .arg("cloud")
        .arg("fixed-database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("default: 100"))
        .stdout(predicate::str::contains("default: 0"));
}

#[test]
fn test_cloud_database_slow_log_offset_description() {
    // Both should use "Offset for pagination" consistently
    redisctl()
        .arg("cloud")
        .arg("database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Offset for pagination"));
}

#[test]
fn test_cloud_fixed_database_slow_log_offset_description() {
    // Both should use "Offset for pagination" consistently
    redisctl()
        .arg("cloud")
        .arg("fixed-database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Offset for pagination"));
}

#[test]
fn test_slow_log_descriptions_match() {
    // Ensure both commands have the same description
    let database_output = redisctl()
        .arg("cloud")
        .arg("database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let fixed_database_output = redisctl()
        .arg("cloud")
        .arg("fixed-database")
        .arg("slow-log")
        .arg("--help")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let database_desc = String::from_utf8_lossy(&database_output);
    let fixed_database_desc = String::from_utf8_lossy(&fixed_database_output);

    // Both should say "Get slow query log"
    assert!(database_desc.contains("Get slow query log"));
    assert!(fixed_database_desc.contains("Get slow query log"));
}

// === FILES-KEY COMMAND TESTS ===

#[test]
fn test_files_key_help() {
    redisctl()
        .arg("files-key")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Files.com API key management"))
        .stdout(predicate::str::contains("set"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn test_files_key_set_help() {
    redisctl()
        .arg("files-key")
        .arg("set")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Store Files.com API key"));
}

#[test]
fn test_files_key_get_help() {
    redisctl()
        .arg("files-key")
        .arg("get")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Get the currently configured Files.com API key",
        ));
}

#[test]
fn test_files_key_remove_help() {
    redisctl()
        .arg("files-key")
        .arg("remove")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Remove Files.com API key"));
}

// === API COMMAND ADDITIONAL TESTS ===

#[test]
fn test_api_help_shows_examples() {
    redisctl()
        .arg("api")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("EXAMPLES:"))
        .stdout(predicate::str::contains("api cloud get /subscriptions"))
        .stdout(predicate::str::contains("api enterprise get /v1/cluster"));
}

#[test]
fn test_completions_help() {
    redisctl()
        .arg("completions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate shell completions"))
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"));
}

// === CLOUD SUBCOMMAND HELP TESTS ===

#[test]
fn test_cloud_account_help() {
    redisctl()
        .arg("cloud")
        .arg("account")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Account operations"));
}

#[test]
fn test_cloud_account_get_help() {
    redisctl()
        .arg("cloud")
        .arg("account")
        .arg("get")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Get account information"));
}

#[test]
fn test_cloud_subscription_help() {
    redisctl()
        .arg("cloud")
        .arg("subscription")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Subscription operations"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn test_cloud_subscription_list_help() {
    redisctl()
        .arg("cloud")
        .arg("subscription")
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List all subscriptions"));
}

#[test]
fn test_cloud_database_help() {
    redisctl()
        .arg("cloud")
        .arg("database")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Database operations"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn test_cloud_database_list_help() {
    redisctl()
        .arg("cloud")
        .arg("database")
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List all databases"));
}

#[test]
fn test_cloud_user_help() {
    redisctl()
        .arg("cloud")
        .arg("user")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("User operations"));
}

#[test]
fn test_cloud_acl_help() {
    redisctl()
        .arg("cloud")
        .arg("acl")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("ACL"));
}

#[test]
fn test_cloud_task_help() {
    redisctl()
        .arg("cloud")
        .arg("task")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Task operations"));
}

#[test]
fn test_cloud_task_get_help() {
    redisctl()
        .arg("cloud")
        .arg("task")
        .arg("get")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Get task status"));
}

#[test]
fn test_cloud_connectivity_help() {
    redisctl()
        .arg("cloud")
        .arg("connectivity")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Network connectivity"))
        .stdout(predicate::str::contains("vpc-peering"))
        .stdout(predicate::str::contains("psc"))
        .stdout(predicate::str::contains("tgw"));
}

#[test]
fn test_cloud_fixed_database_help() {
    redisctl()
        .arg("cloud")
        .arg("fixed-database")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fixed database operations"));
}

#[test]
fn test_cloud_fixed_subscription_help() {
    redisctl()
        .arg("cloud")
        .arg("fixed-subscription")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fixed subscription operations"));
}

#[test]
fn test_cloud_workflow_help() {
    redisctl()
        .arg("cloud")
        .arg("workflow")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Workflow operations"));
}

// === ENTERPRISE SUBCOMMAND HELP TESTS ===

#[test]
fn test_enterprise_cluster_help() {
    redisctl()
        .arg("enterprise")
        .arg("cluster")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cluster operations"));
}

#[test]
fn test_enterprise_cluster_get_help() {
    redisctl()
        .arg("enterprise")
        .arg("cluster")
        .arg("get")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Get cluster configuration"));
}

#[test]
fn test_enterprise_database_help() {
    redisctl()
        .arg("enterprise")
        .arg("database")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Database operations"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn test_enterprise_database_list_help() {
    redisctl()
        .arg("enterprise")
        .arg("database")
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List all databases"));
}

#[test]
fn test_enterprise_node_help() {
    redisctl()
        .arg("enterprise")
        .arg("node")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Node operations"));
}

#[test]
fn test_enterprise_user_help() {
    redisctl()
        .arg("enterprise")
        .arg("user")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("User operations"));
}

#[test]
fn test_enterprise_role_help() {
    redisctl()
        .arg("enterprise")
        .arg("role")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Role operations"));
}

#[test]
fn test_enterprise_acl_help() {
    redisctl()
        .arg("enterprise")
        .arg("acl")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("ACL operations"));
}

#[test]
fn test_enterprise_license_help() {
    redisctl()
        .arg("enterprise")
        .arg("license")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("License management"));
}

#[test]
fn test_enterprise_support_package_help() {
    redisctl()
        .arg("enterprise")
        .arg("support-package")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Support package"));
}

#[test]
fn test_enterprise_support_package_cluster_help() {
    redisctl()
        .arg("enterprise")
        .arg("support-package")
        .arg("cluster")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Generate full cluster support package",
        ));
}

#[test]
fn test_enterprise_workflow_help() {
    redisctl()
        .arg("enterprise")
        .arg("workflow")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Workflow operations"));
}

#[test]
fn test_enterprise_workflow_init_cluster_help() {
    redisctl()
        .arg("enterprise")
        .arg("workflow")
        .arg("init-cluster")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Initialize a Redis Enterprise cluster",
        ));
}

#[test]
fn test_enterprise_crdb_help() {
    redisctl()
        .arg("enterprise")
        .arg("crdb")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Active-Active database"));
}

#[test]
fn test_enterprise_proxy_help() {
    redisctl()
        .arg("enterprise")
        .arg("proxy")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Proxy management"));
}

#[test]
fn test_enterprise_module_help() {
    redisctl()
        .arg("enterprise")
        .arg("module")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Module management"));
}
