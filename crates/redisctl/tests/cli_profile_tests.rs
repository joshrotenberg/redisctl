use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Helper to create a test command with isolated config
fn test_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("redisctl").unwrap();
    let config_file = temp_dir.path().join("config.toml");
    cmd.arg("--config-file").arg(config_file);
    cmd
}

/// Helper to get redisctl command without temp dir
fn redisctl() -> Command {
    Command::cargo_bin("redisctl").unwrap()
}

#[test]
fn test_profile_list() {
    let temp_dir = TempDir::new().unwrap();

    // Should succeed even with no profiles
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_profile_set_cloud() {
    let temp_dir = TempDir::new().unwrap();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("test-cloud")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("test-key")
        .arg("--api-secret")
        .arg("test-secret")
        .assert()
        .success();

    // Verify profile was created
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-cloud"));
}

#[test]
fn test_profile_set_enterprise() {
    let temp_dir = TempDir::new().unwrap();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("test-enterprise")
        .arg("--deployment")
        .arg("enterprise")
        .arg("--url")
        .arg("https://localhost:9443")
        .arg("--username")
        .arg("admin@redis.local")
        .arg("--password")
        .arg("password123")
        .assert()
        .success();

    // Verify profile was created
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-enterprise"));
}

#[test]
fn test_profile_get_nonexistent() {
    let temp_dir = TempDir::new().unwrap();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("show")
        .arg("nonexistent")
        .assert()
        .failure();
}

#[test]
fn test_profile_get_existing() {
    let temp_dir = TempDir::new().unwrap();

    // Create profile first
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("myprofile")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key123")
        .arg("--api-secret")
        .arg("secret456")
        .assert()
        .success();

    // Get profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("show")
        .arg("myprofile")
        .assert()
        .success()
        .stdout(predicate::str::contains("myprofile"));
}

#[test]
fn test_profile_delete() {
    let temp_dir = TempDir::new().unwrap();

    // Create profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("to-delete")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .success();

    // Delete profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("remove")
        .arg("to-delete")
        .write_stdin("y\n")
        .assert()
        .success();

    // Verify it's gone
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("to-delete").not());
}

#[test]
fn test_profile_delete_nonexistent() {
    let temp_dir = TempDir::new().unwrap();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("remove")
        .arg("nonexistent")
        .assert()
        .failure();
}

#[test]
fn test_profile_set_default_subcommand() {
    let temp_dir = TempDir::new().unwrap();

    // Create profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("myprofile")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .success();

    // Set as default using subcommand
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("default-cloud")
        .arg("myprofile")
        .assert()
        .success();
}

#[test]
fn test_profile_list_shows_multiple() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple profiles
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("profile1")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key1")
        .arg("--api-secret")
        .arg("secret1")
        .assert()
        .success();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("profile2")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key2")
        .arg("--api-secret")
        .arg("secret2")
        .assert()
        .success();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("profile3")
        .arg("--deployment")
        .arg("enterprise")
        .arg("--url")
        .arg("https://localhost:9443")
        .arg("--username")
        .arg("admin@redis.local")
        .arg("--password")
        .arg("pass")
        .assert()
        .success();

    // List should show all three
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("profile1"))
        .stdout(predicate::str::contains("profile2"))
        .stdout(predicate::str::contains("profile3"));
}

#[test]
fn test_profile_list_json_output() {
    let temp_dir = TempDir::new().unwrap();

    // Create a profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("json-test")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .success();

    // Get list in JSON format
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .arg("-o")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("json-test"));
}

#[test]
fn test_profile_set_with_config_file_flag() {
    let temp_dir = TempDir::new().unwrap();
    let custom_config = temp_dir.path().join("custom-config.toml");

    let mut cmd = Command::cargo_bin("redisctl").unwrap();
    cmd.arg("--config-file")
        .arg(&custom_config)
        .arg("profile")
        .arg("set")
        .arg("custom-file-profile")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .success();

    // Verify custom config file was created
    assert!(
        custom_config.exists(),
        "Custom config file should be created"
    );

    // Verify we can read it back
    let mut cmd2 = Command::cargo_bin("redisctl").unwrap();
    cmd2.arg("--config-file")
        .arg(&custom_config)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("custom-file-profile"));
}

#[test]
fn test_profile_update_existing() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("updateme")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("old-key")
        .arg("--api-secret")
        .arg("old-secret")
        .assert()
        .success();

    // Update with new credentials
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("updateme")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("new-key")
        .arg("--api-secret")
        .arg("new-secret")
        .assert()
        .success();
}

#[test]
fn test_profile_validate() {
    let temp_dir = TempDir::new().unwrap();

    // Create a valid profile
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("valid-profile")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .success();

    // Validate should succeed
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("validate")
        .assert()
        .success();
}

#[test]
fn test_profile_set_enterprise_with_insecure() {
    let temp_dir = TempDir::new().unwrap();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("insecure-test")
        .arg("--deployment")
        .arg("enterprise")
        .arg("--url")
        .arg("https://localhost:9443")
        .arg("--username")
        .arg("admin@redis.local")
        .arg("--password")
        .arg("pass")
        .arg("--insecure")
        .assert()
        .success();
}

#[test]
fn test_profile_set_cloud_with_custom_url() {
    let temp_dir = TempDir::new().unwrap();

    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("custom-url")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .arg("--api-url")
        .arg("https://custom-api.example.com/v1")
        .assert()
        .success();
}

// === NEW TESTS FOR --type FLAG AND HELP TEXT ACCURACY ===

#[test]
fn test_profile_set_with_type_flag_cloud() {
    let temp_dir = TempDir::new().unwrap();

    // Test new --type flag
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("type-test-cloud")
        .arg("--type")
        .arg("cloud")
        .arg("--api-key")
        .arg("test-key")
        .arg("--api-secret")
        .arg("test-secret")
        .assert()
        .success();

    // Verify profile was created
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("type-test-cloud"));
}

#[test]
fn test_profile_set_with_type_flag_enterprise() {
    let temp_dir = TempDir::new().unwrap();

    // Test new --type flag
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("type-test-enterprise")
        .arg("--type")
        .arg("enterprise")
        .arg("--url")
        .arg("https://localhost:9443")
        .arg("--username")
        .arg("admin@redis.local")
        .arg("--password")
        .arg("password123")
        .assert()
        .success();

    // Verify profile was created
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("type-test-enterprise"));
}

#[test]
fn test_profile_set_deployment_alias_still_works() {
    let temp_dir = TempDir::new().unwrap();

    // Verify --deployment alias still works for backward compatibility
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("alias-test")
        .arg("--deployment")
        .arg("cloud")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .success();
}

#[test]
fn test_profile_set_help_shows_type_flag() {
    // Verify help shows --type as the primary flag
    redisctl()
        .arg("profile")
        .arg("set")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--type <TYPE>"))
        .stdout(predicate::str::contains("Platform type"));
}

#[test]
fn test_profile_set_help_shows_deployment_alias() {
    // Verify help shows --deployment as an alias
    redisctl()
        .arg("profile")
        .arg("set")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--deployment"));
}

#[test]
fn test_profile_set_help_examples_use_type_flag() {
    // Verify examples use --type, not bare positional argument
    redisctl()
        .arg("profile")
        .arg("set")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--type cloud"))
        .stdout(predicate::str::contains("--type enterprise"));
}

#[test]
fn test_profile_help_examples_accurate() {
    // Verify main profile help has accurate examples
    redisctl()
        .arg("profile")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("profile set mycloud --type cloud"))
        .stdout(predicate::str::contains(
            "profile set myenterprise --type enterprise",
        ));
}

#[test]
fn test_main_help_profile_examples_accurate() {
    // Verify main redisctl help has accurate profile examples
    redisctl()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("profile set mycloud --type cloud"))
        .stdout(predicate::str::contains(
            "profile set myenterprise --type enterprise",
        ));
}

#[test]
fn test_profile_set_requires_type_flag() {
    let temp_dir = TempDir::new().unwrap();

    // Should fail without --type flag
    test_cmd(&temp_dir)
        .arg("profile")
        .arg("set")
        .arg("test-profile")
        .arg("--api-key")
        .arg("key")
        .arg("--api-secret")
        .arg("secret")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--type"));
}

#[test]
fn test_profile_list_help() {
    redisctl()
        .arg("profile")
        .arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List all configured profiles"));
}

#[test]
fn test_profile_show_help() {
    redisctl()
        .arg("profile")
        .arg("show")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Show details of a specific profile",
        ));
}

#[test]
fn test_profile_validate_help() {
    redisctl()
        .arg("profile")
        .arg("validate")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Validate configuration file and profiles",
        ));
}

#[test]
fn test_profile_remove_help() {
    redisctl()
        .arg("profile")
        .arg("remove")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Remove a profile"));
}

#[test]
fn test_profile_path_help() {
    redisctl()
        .arg("profile")
        .arg("path")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Show the path to the configuration file",
        ));
}
