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
        .stderr(predicate::str::contains("--deployment"));
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
fn test_enterprise_database_upgrade_help() {
    redisctl()
        .arg("enterprise")
        .arg("database")
        .arg("upgrade")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Upgrade database Redis version"))
        .stdout(predicate::str::contains("--version"))
        .stdout(predicate::str::contains("--preserve-roles"));
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
