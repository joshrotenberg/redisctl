//! Integration tests for raw API access commands

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_enterprise_api_get_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["enterprise", "api", "GET", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute GET request"))
            .stdout(predicate::str::contains("API path"));
    }

    #[test]
    fn test_cloud_api_get_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["cloud", "api", "GET", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute GET request"))
            .stdout(predicate::str::contains("API path"));
    }

    #[test]
    fn test_enterprise_api_post_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["enterprise", "api", "POST", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute POST request"))
            .stdout(predicate::str::contains("--data"));
    }

    #[test]
    fn test_cloud_api_post_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["cloud", "api", "POST", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute POST request"))
            .stdout(predicate::str::contains("--data"));
    }

    #[test]
    fn test_api_delete_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["enterprise", "api", "DELETE", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute DELETE request"));
    }

    #[test]
    fn test_api_put_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["cloud", "api", "PUT", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute PUT request"))
            .stdout(predicate::str::contains("--data"));
    }

    #[test]
    fn test_api_patch_help() {
        let mut cmd = Command::cargo_bin("redisctl").unwrap();
        cmd.args(["enterprise", "api", "PATCH", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Execute PATCH request"))
            .stdout(predicate::str::contains("--data"));
    }
}
