//! Database tools for direct Redis database connections.
//!
//! This module provides tools for querying, inspecting, and diagnosing Redis databases
//! through direct connections (not via Cloud/Enterprise APIs).

use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisResult, Value, cmd};
use redisctl_config::Config;
use tracing::debug;

/// Tools for interacting with Redis databases directly.
#[derive(Clone)]
pub struct DatabaseTools {
    conn: ConnectionManager,
}

impl DatabaseTools {
    /// Create a new DatabaseTools instance from a profile.
    ///
    /// If no profile is specified, uses the default database profile.
    pub async fn new(profile: Option<&str>) -> anyhow::Result<Self> {
        let config = Config::load()?;

        let profile_name = match profile {
            Some(name) => name.to_string(),
            None => config.resolve_database_profile(None)?,
        };

        debug!(profile = %profile_name, "Loading Database client from profile");

        let profile_config = config
            .profiles
            .get(&profile_name)
            .ok_or_else(|| anyhow::anyhow!("Database profile '{}' not found", profile_name))?;

        let (host, port, password, tls, username, db) = profile_config
            .resolve_database_credentials()?
            .ok_or_else(|| {
                anyhow::anyhow!("Profile '{}' is not a Database profile", profile_name)
            })?;

        // Build redis connection URL
        // Format: redis[s]://[[username:]password@]host[:port][/database]
        let scheme = if tls { "rediss" } else { "redis" };
        let auth = match (&password, username.as_str()) {
            (Some(pwd), "default") => format!(":{}@", urlencoding::encode(pwd)),
            (Some(pwd), user) => {
                format!(
                    "{}:{}@",
                    urlencoding::encode(user),
                    urlencoding::encode(pwd)
                )
            }
            (None, _) => String::new(),
        };
        let url = format!("{scheme}://{auth}{host}:{port}/{db}");

        debug!(host = %host, port = %port, tls = %tls, "Connecting to Redis");

        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;

        Ok(Self { conn })
    }

    /// Execute an arbitrary Redis command.
    ///
    /// This is the generic execute function that can run any Redis command.
    pub async fn execute(&self, command: &str, args: &[String]) -> RedisResult<Value> {
        let mut redis_cmd = cmd(command);
        for arg in args {
            redis_cmd.arg(arg);
        }
        redis_cmd.query_async(&mut self.conn.clone()).await
    }

    /// Get Redis server information (INFO command).
    pub async fn info(&self, section: Option<&str>) -> RedisResult<String> {
        let mut conn = self.conn.clone();
        match section {
            Some(s) => redis::cmd("INFO").arg(s).query_async(&mut conn).await,
            None => redis::cmd("INFO").query_async(&mut conn).await,
        }
    }

    /// Get the number of keys in the database (DBSIZE command).
    pub async fn dbsize(&self) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        redis::cmd("DBSIZE").query_async(&mut conn).await
    }

    /// Scan keys matching a pattern.
    ///
    /// Uses SCAN internally to avoid blocking. Returns up to `count` keys.
    pub async fn scan(&self, pattern: &str, count: usize) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        let mut keys = Vec::new();
        let mut cursor: u64 = 0;

        loop {
            let (new_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100) // Batch size per SCAN call
                .query_async(&mut conn)
                .await?;

            keys.extend(batch);
            cursor = new_cursor;

            // Stop if we've collected enough keys or finished scanning
            if cursor == 0 || keys.len() >= count {
                break;
            }
        }

        // Truncate to requested count
        keys.truncate(count);
        Ok(keys)
    }

    /// Get the type of a key (TYPE command).
    pub async fn key_type(&self, key: &str) -> RedisResult<String> {
        let mut conn = self.conn.clone();
        redis::cmd("TYPE").arg(key).query_async(&mut conn).await
    }

    /// Get the TTL of a key in seconds (TTL command).
    ///
    /// Returns -1 if the key exists but has no expiration.
    /// Returns -2 if the key does not exist.
    pub async fn ttl(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.ttl(key).await
    }

    /// Get the TTL of a key in milliseconds (PTTL command).
    pub async fn pttl(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.pttl(key).await
    }

    /// Get memory usage of a key (MEMORY USAGE command).
    pub async fn memory_usage(&self, key: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.clone();
        redis::cmd("MEMORY")
            .arg("USAGE")
            .arg(key)
            .query_async(&mut conn)
            .await
    }

    /// Get slow log entries (SLOWLOG GET command).
    pub async fn slowlog_get(&self, count: Option<usize>) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let count = count.unwrap_or(10);
        redis::cmd("SLOWLOG")
            .arg("GET")
            .arg(count)
            .query_async(&mut conn)
            .await
    }

    /// Get the slow log length (SLOWLOG LEN command).
    pub async fn slowlog_len(&self) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        redis::cmd("SLOWLOG")
            .arg("LEN")
            .query_async(&mut conn)
            .await
    }

    /// Get connected clients (CLIENT LIST command).
    pub async fn client_list(&self) -> RedisResult<String> {
        let mut conn = self.conn.clone();
        redis::cmd("CLIENT")
            .arg("LIST")
            .query_async(&mut conn)
            .await
    }

    /// Get configuration values (CONFIG GET command).
    pub async fn config_get(&self, pattern: &str) -> RedisResult<Vec<(String, String)>> {
        let mut conn = self.conn.clone();
        redis::cmd("CONFIG")
            .arg("GET")
            .arg(pattern)
            .query_async(&mut conn)
            .await
    }

    /// List loaded modules (MODULE LIST command).
    pub async fn module_list(&self) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        redis::cmd("MODULE")
            .arg("LIST")
            .query_async(&mut conn)
            .await
    }

    /// Ping the server.
    pub async fn ping(&self) -> RedisResult<String> {
        let mut conn = self.conn.clone();
        redis::cmd("PING").query_async(&mut conn).await
    }

    /// Get a string value (GET command).
    pub async fn get(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.clone();
        conn.get(key).await
    }

    /// Check if a key exists (EXISTS command).
    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        conn.exists(key).await
    }

    /// Get all fields and values of a hash (HGETALL command).
    pub async fn hgetall(&self, key: &str) -> RedisResult<Vec<(String, String)>> {
        let mut conn = self.conn.clone();
        conn.hgetall(key).await
    }

    /// Get a range of elements from a list (LRANGE command).
    pub async fn lrange(&self, key: &str, start: isize, stop: isize) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        conn.lrange(key, start, stop).await
    }

    /// Get all members of a set (SMEMBERS command).
    pub async fn smembers(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        conn.smembers(key).await
    }

    /// Get a range of elements from a sorted set (ZRANGE command).
    pub async fn zrange(&self, key: &str, start: isize, stop: isize) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        conn.zrange(key, start, stop).await
    }

    /// Get the length of a list (LLEN command).
    pub async fn llen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.llen(key).await
    }

    /// Get the cardinality of a set (SCARD command).
    pub async fn scard(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.scard(key).await
    }

    /// Get the cardinality of a sorted set (ZCARD command).
    pub async fn zcard(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.zcard(key).await
    }

    /// Get the number of fields in a hash (HLEN command).
    pub async fn hlen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.hlen(key).await
    }
}

/// Convert a Redis Value to a JSON-friendly representation.
pub fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Nil => serde_json::Value::Null,
        Value::Int(i) => serde_json::json!(i),
        Value::BulkString(bytes) => {
            // Try to convert to string, fall back to base64
            match String::from_utf8(bytes.clone()) {
                Ok(s) => serde_json::json!(s),
                Err(_) => serde_json::json!({
                    "type": "binary",
                    "base64": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
                }),
            }
        }
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(value_to_json).collect()),
        Value::SimpleString(s) => serde_json::json!(s),
        Value::Okay => serde_json::json!("OK"),
        Value::Map(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .filter_map(|(k, v)| {
                    // Try to convert key to string
                    let key_str = match k {
                        Value::BulkString(bytes) => String::from_utf8(bytes.clone()).ok(),
                        Value::SimpleString(s) => Some(s.clone()),
                        _ => None,
                    };
                    key_str.map(|k| (k, value_to_json(v)))
                })
                .collect();
            serde_json::Value::Object(obj)
        }
        Value::Attribute {
            data,
            attributes: _,
        } => value_to_json(data),
        Value::Set(set) => serde_json::Value::Array(set.iter().map(value_to_json).collect()),
        Value::Double(d) => serde_json::json!(d),
        Value::Boolean(b) => serde_json::json!(b),
        Value::VerbatimString { format: _, text } => serde_json::json!(text),
        Value::BigNumber(n) => serde_json::json!(n.to_string()),
        Value::Push { kind: _, data } => {
            serde_json::Value::Array(data.iter().map(value_to_json).collect())
        }
        Value::ServerError(err) => serde_json::json!({
            "error": format!("{:?}", err)
        }),
    }
}

/// List of Redis commands that are considered write operations.
pub const WRITE_COMMANDS: &[&str] = &[
    "SET",
    "SETNX",
    "SETEX",
    "PSETEX",
    "MSET",
    "MSETNX",
    "SETRANGE",
    "APPEND",
    "INCR",
    "INCRBY",
    "INCRBYFLOAT",
    "DECR",
    "DECRBY",
    "DEL",
    "UNLINK",
    "EXPIRE",
    "EXPIREAT",
    "PEXPIRE",
    "PEXPIREAT",
    "PERSIST",
    "RENAME",
    "RENAMENX",
    "COPY",
    "MOVE",
    "HSET",
    "HSETNX",
    "HMSET",
    "HINCRBY",
    "HINCRBYFLOAT",
    "HDEL",
    "LPUSH",
    "LPUSHX",
    "RPUSH",
    "RPUSHX",
    "LPOP",
    "RPOP",
    "LSET",
    "LINSERT",
    "LREM",
    "LTRIM",
    "SADD",
    "SREM",
    "SPOP",
    "SMOVE",
    "ZADD",
    "ZINCRBY",
    "ZREM",
    "ZREMRANGEBYRANK",
    "ZREMRANGEBYSCORE",
    "ZREMRANGEBYLEX",
    "PFADD",
    "PFMERGE",
    "XADD",
    "XDEL",
    "XTRIM",
    "XSETID",
    "GEOADD",
    "GEORADIUS",
    "GEORADIUSBYMEMBER",
    "BITOP",
    "BITFIELD",
    "SETBIT",
    "JSON.SET",
    "JSON.DEL",
    "JSON.MSET",
    "JSON.MERGE",
    "JSON.NUMINCRBY",
    "JSON.NUMMULTBY",
    "JSON.STRAPPEND",
    "JSON.ARRAPPEND",
    "JSON.ARRINSERT",
    "JSON.ARRPOP",
    "JSON.ARRTRIM",
    "TS.CREATE",
    "TS.DEL",
    "TS.ADD",
    "TS.MADD",
    "TS.INCRBY",
    "TS.DECRBY",
    "FT.CREATE",
    "FT.DROP",
    "FT.DROPINDEX",
    "FT.ALTER",
    "BF.ADD",
    "BF.MADD",
    "BF.INSERT",
    "BF.RESERVE",
    "CF.ADD",
    "CF.ADDNX",
    "CF.INSERT",
    "CF.INSERTNX",
    "CF.DEL",
    "CF.RESERVE",
    "CMS.INITBYDIM",
    "CMS.INITBYPROB",
    "CMS.INCRBY",
    "CMS.MERGE",
    "TOPK.RESERVE",
    "TOPK.ADD",
    "TOPK.INCRBY",
    "TDIGEST.CREATE",
    "TDIGEST.ADD",
    "TDIGEST.MERGE",
    "TDIGEST.RESET",
    "FLUSHDB",
    "FLUSHALL",
    "RESTORE",
    "DUMP",
    "EVAL",
    "EVALSHA",
    "SCRIPT",
    "CLUSTER",
    "READONLY",
    "READWRITE",
    "CONFIG SET",
    "DEBUG",
    "MIGRATE",
    "SLAVEOF",
    "REPLICAOF",
    "SHUTDOWN",
];

/// Check if a command is a write operation.
pub fn is_write_command(command: &str) -> bool {
    let cmd_upper = command.to_uppercase();
    WRITE_COMMANDS
        .iter()
        .any(|&w| cmd_upper == w || cmd_upper.starts_with(&format!("{} ", w)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_write_command() {
        assert!(is_write_command("SET"));
        assert!(is_write_command("set"));
        assert!(is_write_command("DEL"));
        assert!(is_write_command("FLUSHDB"));
        assert!(!is_write_command("GET"));
        assert!(!is_write_command("INFO"));
        assert!(!is_write_command("SCAN"));
        assert!(!is_write_command("DBSIZE"));
    }

    #[test]
    fn test_value_to_json() {
        assert_eq!(value_to_json(&Value::Nil), serde_json::Value::Null);
        assert_eq!(value_to_json(&Value::Int(42)), serde_json::json!(42));
        assert_eq!(value_to_json(&Value::Okay), serde_json::json!("OK"));
        assert_eq!(
            value_to_json(&Value::SimpleString("hello".to_string())),
            serde_json::json!("hello")
        );
    }

    /// Integration test that requires a local Redis instance.
    /// Run with: cargo test -p redisctl-mcp -- --ignored test_database_tools_integration
    #[tokio::test]
    #[ignore]
    async fn test_database_tools_integration() {
        // This test requires:
        // 1. A local Redis instance running on localhost:6379
        // 2. A profile named "local-redis" configured for it

        let tools = DatabaseTools::new(Some("local-redis"))
            .await
            .expect("Failed to connect to Redis - ensure local-redis profile exists");

        // Test ping
        let pong = tools.ping().await.expect("PING failed");
        assert_eq!(pong, "PONG");

        // Test info
        let info = tools.info(Some("server")).await.expect("INFO failed");
        assert!(info.contains("redis_version"));

        // Test dbsize (just check it doesn't error)
        let _dbsize = tools.dbsize().await.expect("DBSIZE failed");

        // Test execute with a simple command
        let result = tools
            .execute("ECHO", &["hello".to_string()])
            .await
            .expect("EXECUTE failed");
        assert_eq!(value_to_json(&result), serde_json::json!("hello"));

        println!("All database tools integration tests passed!");
    }
}
