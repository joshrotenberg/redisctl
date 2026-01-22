//! Integration tests using docker-wrapper for automatic container lifecycle management.
//!
//! These tests demonstrate using docker-wrapper's ContainerGuard for RAII-style
//! container management. Containers are automatically started before tests and
//! cleaned up afterward, even on panic.
//!
//! Currently uses basic Redis (not Enterprise) to validate the testing infrastructure.
//! TODO: Add RedisEnterpriseTemplate support to docker-wrapper, then update these tests.
//!
//! Run with:
//! ```bash
//! cargo test --test docker_wrapper_tests -- --ignored --nocapture
//! ```
//!
//! For faster iteration during development, use reuse_if_running:
//! ```bash
//! REUSE_CONTAINERS=1 cargo test --test docker_wrapper_tests -- --ignored --nocapture
//! ```

use docker_wrapper::template::redis::RedisTemplate;
use docker_wrapper::testing::ContainerGuardBuilder;
use std::time::Duration;
use tokio::sync::OnceCell;

// Shared container state for test reuse
static REDIS_GUARD: OnceCell<RedisTestContext> = OnceCell::const_new();

/// Context for Redis tests - holds the guard and connection info
struct RedisTestContext {
    _guard: docker_wrapper::testing::ContainerGuard<RedisTemplate>,
    port: u16,
}

// Safety: The guard is only accessed from async context and we control access via OnceCell
unsafe impl Send for RedisTestContext {}
unsafe impl Sync for RedisTestContext {}

/// Get or create the shared Redis container
async fn get_redis() -> anyhow::Result<&'static RedisTestContext> {
    REDIS_GUARD
        .get_or_try_init(|| async {
            let reuse = std::env::var("REUSE_CONTAINERS").is_ok();

            // Use docker-wrapper's built-in Redis template with a non-conflicting port
            let template = RedisTemplate::new("redisctl-dw-test").port(16379);

            let guard = ContainerGuardBuilder::new(template)
                .stop_on_drop(!reuse)
                .remove_on_drop(!reuse)
                .reuse_if_running(reuse)
                .keep_on_panic(true)
                .capture_logs(true)
                .wait_for_ready(true)
                .stop_timeout(Duration::from_secs(10))
                .start()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start container: {}", e))?;

            // Query using container port (6379), docker-wrapper returns the mapped host port
            let port = guard
                .host_port(6379)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get port: {}", e))?;

            Ok(RedisTestContext {
                _guard: guard,
                port,
            })
        })
        .await
}

// =============================================================================
// TESTS USING DOCKER-WRAPPER WITH BASIC REDIS
// These validate the docker-wrapper integration pattern.
// =============================================================================

#[tokio::test]
#[ignore = "Requires Docker - run with --ignored"]
async fn test_dw_redis_ping() {
    let ctx = get_redis().await.expect("Failed to get Redis container");

    // Use redis-cli via docker exec or direct connection
    let client = redis::Client::open(format!("redis://localhost:{}", ctx.port)).unwrap();
    let mut con = client.get_connection().unwrap();
    let pong: String = redis::cmd("PING").query(&mut con).unwrap();
    assert_eq!(pong, "PONG");
}

#[tokio::test]
#[ignore = "Requires Docker - run with --ignored"]
async fn test_dw_redis_set_get() {
    let ctx = get_redis().await.expect("Failed to get Redis container");

    let client = redis::Client::open(format!("redis://localhost:{}", ctx.port)).unwrap();
    let mut con = client.get_connection().unwrap();

    // SET and GET
    let _: () = redis::cmd("SET")
        .arg("docker_wrapper_test_key")
        .arg("hello from docker-wrapper!")
        .query(&mut con)
        .unwrap();

    let value: String = redis::cmd("GET")
        .arg("docker_wrapper_test_key")
        .query(&mut con)
        .unwrap();

    assert_eq!(value, "hello from docker-wrapper!");
}

#[tokio::test]
#[ignore = "Requires Docker - run with --ignored"]
async fn test_dw_redis_info() {
    let ctx = get_redis().await.expect("Failed to get Redis container");

    let client = redis::Client::open(format!("redis://localhost:{}", ctx.port)).unwrap();
    let mut con = client.get_connection().unwrap();

    let info: String = redis::cmd("INFO").arg("server").query(&mut con).unwrap();
    assert!(info.contains("redis_version"));
}
