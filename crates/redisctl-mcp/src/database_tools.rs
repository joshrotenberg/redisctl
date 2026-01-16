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

    // ========== WRITE OPERATIONS ==========

    /// Set a string value (SET command).
    ///
    /// Optionally set expiration in seconds (EX) or milliseconds (PX).
    /// Use NX to only set if key doesn't exist, XX to only set if key exists.
    pub async fn set(
        &self,
        key: &str,
        value: &str,
        ex: Option<u64>,
        px: Option<u64>,
        nx: bool,
        xx: bool,
    ) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("SET");
        cmd.arg(key).arg(value);

        if let Some(seconds) = ex {
            cmd.arg("EX").arg(seconds);
        } else if let Some(millis) = px {
            cmd.arg("PX").arg(millis);
        }

        if nx {
            cmd.arg("NX");
        } else if xx {
            cmd.arg("XX");
        }

        // SET returns OK on success, nil on NX/XX failure
        let result: Value = cmd.query_async(&mut conn).await?;
        Ok(!matches!(result, Value::Nil))
    }

    /// Delete one or more keys (DEL command).
    ///
    /// Returns the number of keys that were deleted.
    pub async fn del(&self, keys: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.del(keys).await
    }

    /// Set a key's expiration in seconds (EXPIRE command).
    ///
    /// Returns true if the timeout was set, false if key doesn't exist.
    pub async fn expire(&self, key: &str, seconds: i64) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        conn.expire(key, seconds).await
    }

    /// Remove a key's expiration (PERSIST command).
    ///
    /// Returns true if the timeout was removed, false if key doesn't exist or has no timeout.
    pub async fn persist(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        conn.persist(key).await
    }

    /// Increment a key's integer value by 1 (INCR command).
    ///
    /// If the key doesn't exist, it's set to 0 before incrementing.
    /// Returns the new value after incrementing.
    pub async fn incr(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.incr(key, 1i64).await
    }

    /// Decrement a key's integer value by 1 (DECR command).
    ///
    /// If the key doesn't exist, it's set to 0 before decrementing.
    /// Returns the new value after decrementing.
    pub async fn decr(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.decr(key, 1i64).await
    }

    /// Increment a key's integer value by a specific amount (INCRBY command).
    ///
    /// If the key doesn't exist, it's set to 0 before incrementing.
    /// Returns the new value after incrementing.
    pub async fn incrby(&self, key: &str, increment: i64) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.incr(key, increment).await
    }

    // ========== HASH WRITE OPERATIONS ==========

    /// Set a field in a hash (HSET command).
    ///
    /// Creates the hash if it doesn't exist.
    /// Returns the number of fields that were added (0 if field was updated, 1 if new).
    pub async fn hset(&self, key: &str, field: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.hset(key, field, value).await
    }

    /// Set multiple fields in a hash (HSET with multiple field-value pairs).
    ///
    /// Creates the hash if it doesn't exist.
    /// Returns the number of fields that were added.
    pub async fn hset_multiple(&self, key: &str, fields: &[(String, String)]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.hset_multiple(key, fields).await
    }

    /// Delete one or more fields from a hash (HDEL command).
    ///
    /// Returns the number of fields that were removed.
    pub async fn hdel(&self, key: &str, fields: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.hdel(key, fields).await
    }

    /// Get a specific field from a hash (HGET command).
    pub async fn hget(&self, key: &str, field: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.clone();
        conn.hget(key, field).await
    }

    // ========== LIST WRITE OPERATIONS ==========

    /// Push values to the left (head) of a list (LPUSH command).
    ///
    /// Creates the list if it doesn't exist.
    /// Returns the length of the list after the push.
    pub async fn lpush(&self, key: &str, values: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.lpush(key, values).await
    }

    /// Push values to the right (tail) of a list (RPUSH command).
    ///
    /// Creates the list if it doesn't exist.
    /// Returns the length of the list after the push.
    pub async fn rpush(&self, key: &str, values: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.rpush(key, values).await
    }

    /// Pop a value from the left (head) of a list (LPOP command).
    ///
    /// Returns None if the list is empty or doesn't exist.
    pub async fn lpop(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.clone();
        conn.lpop(key, None).await
    }

    /// Pop a value from the right (tail) of a list (RPOP command).
    ///
    /// Returns None if the list is empty or doesn't exist.
    pub async fn rpop(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.clone();
        conn.rpop(key, None).await
    }

    /// Get an element from a list by index (LINDEX command).
    ///
    /// Negative indices count from the end (-1 is the last element).
    /// Returns None if the index is out of range.
    pub async fn lindex(&self, key: &str, index: isize) -> RedisResult<Option<String>> {
        let mut conn = self.conn.clone();
        conn.lindex(key, index).await
    }

    /// Set an element in a list by index (LSET command).
    ///
    /// Returns an error if the index is out of range.
    pub async fn lset(&self, key: &str, index: isize, value: &str) -> RedisResult<()> {
        let mut conn = self.conn.clone();
        conn.lset(key, index, value).await
    }

    // ========== SET WRITE OPERATIONS ==========

    /// Add members to a set (SADD command).
    ///
    /// Creates the set if it doesn't exist.
    /// Returns the number of members that were added (not already present).
    pub async fn sadd(&self, key: &str, members: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.sadd(key, members).await
    }

    /// Remove members from a set (SREM command).
    ///
    /// Returns the number of members that were removed.
    pub async fn srem(&self, key: &str, members: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.srem(key, members).await
    }

    /// Check if a member exists in a set (SISMEMBER command).
    pub async fn sismember(&self, key: &str, member: &str) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        conn.sismember(key, member).await
    }

    // ========== SORTED SET OPERATIONS ==========

    /// Add members with scores to a sorted set (ZADD command).
    ///
    /// Creates the sorted set if it doesn't exist.
    /// Returns the number of members that were added (not updated).
    pub async fn zadd(&self, key: &str, members: &[(f64, String)]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.zadd_multiple(key, members).await
    }

    /// Remove members from a sorted set (ZREM command).
    ///
    /// Returns the number of members that were removed.
    pub async fn zrem(&self, key: &str, members: &[String]) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        conn.zrem(key, members).await
    }

    /// Get the score of a member in a sorted set (ZSCORE command).
    ///
    /// Returns None if the member doesn't exist.
    pub async fn zscore(&self, key: &str, member: &str) -> RedisResult<Option<f64>> {
        let mut conn = self.conn.clone();
        conn.zscore(key, member).await
    }

    /// Get the rank of a member in a sorted set (ZRANK command).
    ///
    /// Rank is 0-based with the lowest score at rank 0.
    /// Returns None if the member doesn't exist.
    pub async fn zrank(&self, key: &str, member: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.clone();
        conn.zrank(key, member).await
    }

    /// Get the reverse rank of a member in a sorted set (ZREVRANK command).
    ///
    /// Rank is 0-based with the highest score at rank 0.
    /// Returns None if the member doesn't exist.
    pub async fn zrevrank(&self, key: &str, member: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.clone();
        conn.zrevrank(key, member).await
    }

    /// Get a range of members from a sorted set in reverse order (ZREVRANGE command).
    ///
    /// Members are ordered from highest to lowest score.
    pub async fn zrevrange(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        conn.zrevrange(key, start, stop).await
    }

    /// Get members from a sorted set by score range (ZRANGEBYSCORE command).
    ///
    /// Returns members with scores between min and max (inclusive).
    /// Use f64::NEG_INFINITY for -inf and f64::INFINITY for +inf.
    pub async fn zrangebyscore(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        conn.zrangebyscore(key, min, max).await
    }

    /// Increment a member's score in a sorted set (ZINCRBY command).
    ///
    /// Creates the sorted set and member if they don't exist.
    /// Returns the new score.
    pub async fn zincrby(&self, key: &str, increment: f64, member: &str) -> RedisResult<f64> {
        let mut conn = self.conn.clone();
        conn.zincr(key, member, increment).await
    }

    /// Get a range of members with scores from a sorted set (ZRANGE WITHSCORES).
    pub async fn zrange_withscores(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.clone();
        conn.zrange_withscores(key, start, stop).await
    }

    /// Get a range of members with scores in reverse order (ZREVRANGE WITHSCORES).
    pub async fn zrevrange_withscores(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.clone();
        conn.zrevrange_withscores(key, start, stop).await
    }

    /// Rename a key (RENAME command).
    ///
    /// Returns an error if the source key doesn't exist.
    pub async fn rename(&self, key: &str, new_key: &str) -> RedisResult<()> {
        let mut conn = self.conn.clone();
        conn.rename(key, new_key).await
    }

    // ========== REDISEARCH OPERATIONS ==========

    /// Search an index (FT.SEARCH command).
    ///
    /// Returns documents matching the query from the specified index.
    pub async fn ft_search(
        &self,
        index: &str,
        query: &str,
        options: &FtSearchOptions,
    ) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("FT.SEARCH");
        cmd.arg(index).arg(query);

        if options.nocontent {
            cmd.arg("NOCONTENT");
        }
        if options.verbatim {
            cmd.arg("VERBATIM");
        }
        if options.withscores {
            cmd.arg("WITHSCORES");
        }
        if let Some(ref fields) = options.return_fields {
            cmd.arg("RETURN").arg(fields.len());
            for field in fields {
                cmd.arg(field);
            }
        }
        if let Some(ref sortby) = options.sortby {
            cmd.arg("SORTBY").arg(sortby);
            if options.sortby_desc {
                cmd.arg("DESC");
            } else {
                cmd.arg("ASC");
            }
        }
        if let Some(offset) = options.limit_offset {
            cmd.arg("LIMIT")
                .arg(offset)
                .arg(options.limit_num.unwrap_or(10));
        }
        if let Some(ref highlight_fields) = options.highlight_fields {
            cmd.arg("HIGHLIGHT")
                .arg("FIELDS")
                .arg(highlight_fields.len());
            for field in highlight_fields {
                cmd.arg(field);
            }
            if let (Some(open), Some(close)) =
                (&options.highlight_tags_open, &options.highlight_tags_close)
            {
                cmd.arg("TAGS").arg(open).arg(close);
            }
        }
        if let Some(ref language) = options.language {
            cmd.arg("LANGUAGE").arg(language);
        }
        if let Some(slop) = options.slop {
            cmd.arg("SLOP").arg(slop);
        }
        if options.inorder {
            cmd.arg("INORDER");
        }
        if let Some(timeout) = options.timeout {
            cmd.arg("TIMEOUT").arg(timeout);
        }
        if let Some(dialect) = options.dialect {
            cmd.arg("DIALECT").arg(dialect);
        }

        cmd.query_async(&mut conn).await
    }

    /// Aggregate query results (FT.AGGREGATE command).
    ///
    /// Performs aggregations on search results with grouping, sorting, and transformations.
    pub async fn ft_aggregate(
        &self,
        index: &str,
        query: &str,
        options: &FtAggregateOptions,
    ) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("FT.AGGREGATE");
        cmd.arg(index).arg(query);

        if options.verbatim {
            cmd.arg("VERBATIM");
        }
        if let Some(ref load_fields) = options.load {
            if load_fields.is_empty() {
                cmd.arg("LOAD").arg("*");
            } else {
                cmd.arg("LOAD").arg(load_fields.len());
                for field in load_fields {
                    cmd.arg(field);
                }
            }
        }
        for groupby in &options.groupby {
            cmd.arg("GROUPBY").arg(groupby.properties.len());
            for prop in &groupby.properties {
                cmd.arg(prop);
            }
            for reduce in &groupby.reducers {
                cmd.arg("REDUCE")
                    .arg(&reduce.function)
                    .arg(reduce.args.len());
                for arg in &reduce.args {
                    cmd.arg(arg);
                }
                if let Some(ref alias) = reduce.alias {
                    cmd.arg("AS").arg(alias);
                }
            }
        }
        for apply in &options.apply {
            cmd.arg("APPLY")
                .arg(&apply.expression)
                .arg("AS")
                .arg(&apply.alias);
        }
        if let Some(ref sortby) = options.sortby {
            let count = sortby.len() * 2;
            cmd.arg("SORTBY").arg(count);
            for (field, order) in sortby {
                cmd.arg(field).arg(order);
            }
            if let Some(max) = options.sortby_max {
                cmd.arg("MAX").arg(max);
            }
        }
        if let Some(ref filter) = options.filter {
            cmd.arg("FILTER").arg(filter);
        }
        if let Some(offset) = options.limit_offset {
            cmd.arg("LIMIT")
                .arg(offset)
                .arg(options.limit_num.unwrap_or(10));
        }
        if let Some(timeout) = options.timeout {
            cmd.arg("TIMEOUT").arg(timeout);
        }
        if let Some(dialect) = options.dialect {
            cmd.arg("DIALECT").arg(dialect);
        }

        cmd.query_async(&mut conn).await
    }

    /// Get index information (FT.INFO command).
    pub async fn ft_info(&self, index: &str) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        redis::cmd("FT.INFO")
            .arg(index)
            .query_async(&mut conn)
            .await
    }

    /// List all indexes (FT._LIST command).
    pub async fn ft_list(&self) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.clone();
        redis::cmd("FT._LIST").query_async(&mut conn).await
    }

    // ========== REDISJSON OPERATIONS ==========

    /// Get JSON value at path (JSON.GET command).
    pub async fn json_get(
        &self,
        key: &str,
        paths: &[String],
        indent: Option<&str>,
        newline: Option<&str>,
        space: Option<&str>,
    ) -> RedisResult<Option<String>> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.GET");
        cmd.arg(key);

        if let Some(indent) = indent {
            cmd.arg("INDENT").arg(indent);
        }
        if let Some(newline) = newline {
            cmd.arg("NEWLINE").arg(newline);
        }
        if let Some(space) = space {
            cmd.arg("SPACE").arg(space);
        }

        if paths.is_empty() {
            cmd.arg("$");
        } else {
            for path in paths {
                cmd.arg(path);
            }
        }

        cmd.query_async(&mut conn).await
    }

    /// Set JSON value at path (JSON.SET command).
    pub async fn json_set(
        &self,
        key: &str,
        path: &str,
        value: &str,
        nx: bool,
        xx: bool,
    ) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.SET");
        cmd.arg(key).arg(path).arg(value);

        if nx {
            cmd.arg("NX");
        } else if xx {
            cmd.arg("XX");
        }

        let result: Value = cmd.query_async(&mut conn).await?;
        Ok(!matches!(result, Value::Nil))
    }

    /// Delete JSON value at path (JSON.DEL command).
    pub async fn json_del(&self, key: &str, path: Option<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.DEL");
        cmd.arg(key);
        if let Some(p) = path {
            cmd.arg(p);
        }
        cmd.query_async(&mut conn).await
    }

    /// Get JSON value type at path (JSON.TYPE command).
    pub async fn json_type(&self, key: &str, path: Option<&str>) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.TYPE");
        cmd.arg(key);
        if let Some(p) = path {
            cmd.arg(p);
        } else {
            cmd.arg("$");
        }
        cmd.query_async(&mut conn).await
    }

    /// Append values to JSON array (JSON.ARRAPPEND command).
    pub async fn json_arrappend(
        &self,
        key: &str,
        path: &str,
        values: &[String],
    ) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.ARRAPPEND");
        cmd.arg(key).arg(path);
        for value in values {
            cmd.arg(value);
        }
        cmd.query_async(&mut conn).await
    }

    /// Get JSON array length (JSON.ARRLEN command).
    pub async fn json_arrlen(&self, key: &str, path: Option<&str>) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.ARRLEN");
        cmd.arg(key);
        if let Some(p) = path {
            cmd.arg(p);
        } else {
            cmd.arg("$");
        }
        cmd.query_async(&mut conn).await
    }

    /// Increment JSON number (JSON.NUMINCRBY command).
    pub async fn json_numincrby(&self, key: &str, path: &str, value: f64) -> RedisResult<String> {
        let mut conn = self.conn.clone();
        redis::cmd("JSON.NUMINCRBY")
            .arg(key)
            .arg(path)
            .arg(value)
            .query_async(&mut conn)
            .await
    }

    /// Get string length at JSON path (JSON.STRLEN command).
    pub async fn json_strlen(&self, key: &str, path: Option<&str>) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("JSON.STRLEN");
        cmd.arg(key);
        if let Some(p) = path {
            cmd.arg(p);
        } else {
            cmd.arg("$");
        }
        cmd.query_async(&mut conn).await
    }

    // ========== REDISTIMESERIES OPERATIONS ==========

    /// Add a sample to a time series (TS.ADD command).
    pub async fn ts_add(
        &self,
        key: &str,
        timestamp: &str,
        value: f64,
        options: &TsAddOptions,
    ) -> RedisResult<i64> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("TS.ADD");
        cmd.arg(key).arg(timestamp).arg(value);

        if let Some(retention) = options.retention {
            cmd.arg("RETENTION").arg(retention);
        }
        if let Some(ref encoding) = options.encoding {
            cmd.arg("ENCODING").arg(encoding);
        }
        if let Some(chunk_size) = options.chunk_size {
            cmd.arg("CHUNK_SIZE").arg(chunk_size);
        }
        if let Some(ref on_duplicate) = options.on_duplicate {
            cmd.arg("ON_DUPLICATE").arg(on_duplicate);
        }
        if let Some(ref labels) = options.labels {
            cmd.arg("LABELS");
            for (label, value) in labels {
                cmd.arg(label).arg(value);
            }
        }

        cmd.query_async(&mut conn).await
    }

    /// Get the last sample from a time series (TS.GET command).
    pub async fn ts_get(&self, key: &str) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        redis::cmd("TS.GET").arg(key).query_async(&mut conn).await
    }

    /// Query a range of samples (TS.RANGE command).
    pub async fn ts_range(
        &self,
        key: &str,
        from: &str,
        to: &str,
        options: &TsRangeOptions,
    ) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("TS.RANGE");
        cmd.arg(key).arg(from).arg(to);

        if options.latest {
            cmd.arg("LATEST");
        }
        if let Some(ref timestamps) = options.filter_by_ts {
            cmd.arg("FILTER_BY_TS");
            for ts in timestamps {
                cmd.arg(ts);
            }
        }
        if let (Some(min), Some(max)) = (options.filter_by_value_min, options.filter_by_value_max) {
            cmd.arg("FILTER_BY_VALUE").arg(min).arg(max);
        }
        if let Some(count) = options.count {
            cmd.arg("COUNT").arg(count);
        }
        if let Some(ref aggregation) = options.aggregation {
            if let Some(ref align) = options.align {
                cmd.arg("ALIGN").arg(align);
            }
            cmd.arg("AGGREGATION")
                .arg(&aggregation.aggregator)
                .arg(aggregation.bucket_duration);
            if let Some(ref bt) = aggregation.bucket_timestamp {
                cmd.arg("BUCKETTIMESTAMP").arg(bt);
            }
            if aggregation.empty {
                cmd.arg("EMPTY");
            }
        }

        cmd.query_async(&mut conn).await
    }

    /// Get time series information (TS.INFO command).
    pub async fn ts_info(&self, key: &str) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        redis::cmd("TS.INFO").arg(key).query_async(&mut conn).await
    }

    /// Create a new time series (TS.CREATE command).
    pub async fn ts_create(&self, key: &str, options: &TsCreateOptions) -> RedisResult<()> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("TS.CREATE");
        cmd.arg(key);

        if let Some(retention) = options.retention {
            cmd.arg("RETENTION").arg(retention);
        }
        if let Some(ref encoding) = options.encoding {
            cmd.arg("ENCODING").arg(encoding);
        }
        if let Some(chunk_size) = options.chunk_size {
            cmd.arg("CHUNK_SIZE").arg(chunk_size);
        }
        if let Some(ref duplicate_policy) = options.duplicate_policy {
            cmd.arg("DUPLICATE_POLICY").arg(duplicate_policy);
        }
        if let Some(ref labels) = options.labels {
            cmd.arg("LABELS");
            for (label, value) in labels {
                cmd.arg(label).arg(value);
            }
        }

        cmd.query_async(&mut conn).await
    }

    // ========== REDISBLOOM OPERATIONS ==========

    /// Create a Bloom filter (BF.RESERVE command).
    pub async fn bf_reserve(
        &self,
        key: &str,
        error_rate: f64,
        capacity: u64,
        expansion: Option<u32>,
        nonscaling: bool,
    ) -> RedisResult<()> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("BF.RESERVE");
        cmd.arg(key).arg(error_rate).arg(capacity);

        if let Some(exp) = expansion {
            cmd.arg("EXPANSION").arg(exp);
        }
        if nonscaling {
            cmd.arg("NONSCALING");
        }

        cmd.query_async(&mut conn).await
    }

    /// Add an item to a Bloom filter (BF.ADD command).
    pub async fn bf_add(&self, key: &str, item: &str) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        redis::cmd("BF.ADD")
            .arg(key)
            .arg(item)
            .query_async(&mut conn)
            .await
    }

    /// Add multiple items to a Bloom filter (BF.MADD command).
    pub async fn bf_madd(&self, key: &str, items: &[String]) -> RedisResult<Vec<bool>> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("BF.MADD");
        cmd.arg(key);
        for item in items {
            cmd.arg(item);
        }
        cmd.query_async(&mut conn).await
    }

    /// Check if an item exists in a Bloom filter (BF.EXISTS command).
    pub async fn bf_exists(&self, key: &str, item: &str) -> RedisResult<bool> {
        let mut conn = self.conn.clone();
        redis::cmd("BF.EXISTS")
            .arg(key)
            .arg(item)
            .query_async(&mut conn)
            .await
    }

    /// Check if multiple items exist in a Bloom filter (BF.MEXISTS command).
    pub async fn bf_mexists(&self, key: &str, items: &[String]) -> RedisResult<Vec<bool>> {
        let mut conn = self.conn.clone();
        let mut cmd = redis::cmd("BF.MEXISTS");
        cmd.arg(key);
        for item in items {
            cmd.arg(item);
        }
        cmd.query_async(&mut conn).await
    }

    /// Get Bloom filter information (BF.INFO command).
    pub async fn bf_info(&self, key: &str) -> RedisResult<Value> {
        let mut conn = self.conn.clone();
        redis::cmd("BF.INFO").arg(key).query_async(&mut conn).await
    }
}

// ========== OPTION STRUCTS FOR MODULE COMMANDS ==========

/// Options for FT.SEARCH command.
#[derive(Debug, Default, Clone)]
pub struct FtSearchOptions {
    pub nocontent: bool,
    pub verbatim: bool,
    pub withscores: bool,
    pub return_fields: Option<Vec<String>>,
    pub sortby: Option<String>,
    pub sortby_desc: bool,
    pub limit_offset: Option<i64>,
    pub limit_num: Option<i64>,
    pub highlight_fields: Option<Vec<String>>,
    pub highlight_tags_open: Option<String>,
    pub highlight_tags_close: Option<String>,
    pub language: Option<String>,
    pub slop: Option<i64>,
    pub inorder: bool,
    pub timeout: Option<i64>,
    pub dialect: Option<i32>,
}

/// Options for FT.AGGREGATE command.
#[derive(Debug, Default, Clone)]
pub struct FtAggregateOptions {
    pub verbatim: bool,
    pub load: Option<Vec<String>>,
    pub groupby: Vec<FtGroupBy>,
    pub apply: Vec<FtApply>,
    pub sortby: Option<Vec<(String, String)>>,
    pub sortby_max: Option<i64>,
    pub filter: Option<String>,
    pub limit_offset: Option<i64>,
    pub limit_num: Option<i64>,
    pub timeout: Option<i64>,
    pub dialect: Option<i32>,
}

/// GROUPBY clause for FT.AGGREGATE.
#[derive(Debug, Default, Clone)]
pub struct FtGroupBy {
    pub properties: Vec<String>,
    pub reducers: Vec<FtReducer>,
}

/// REDUCE clause for FT.AGGREGATE.
#[derive(Debug, Default, Clone)]
pub struct FtReducer {
    pub function: String,
    pub args: Vec<String>,
    pub alias: Option<String>,
}

/// APPLY clause for FT.AGGREGATE.
#[derive(Debug, Default, Clone)]
pub struct FtApply {
    pub expression: String,
    pub alias: String,
}

/// Options for TS.ADD command.
#[derive(Debug, Default, Clone)]
pub struct TsAddOptions {
    pub retention: Option<i64>,
    pub encoding: Option<String>,
    pub chunk_size: Option<i64>,
    pub on_duplicate: Option<String>,
    pub labels: Option<Vec<(String, String)>>,
}

/// Options for TS.CREATE command.
#[derive(Debug, Default, Clone)]
pub struct TsCreateOptions {
    pub retention: Option<i64>,
    pub encoding: Option<String>,
    pub chunk_size: Option<i64>,
    pub duplicate_policy: Option<String>,
    pub labels: Option<Vec<(String, String)>>,
}

/// Options for TS.RANGE command.
#[derive(Debug, Default, Clone)]
pub struct TsRangeOptions {
    pub latest: bool,
    pub filter_by_ts: Option<Vec<i64>>,
    pub filter_by_value_min: Option<f64>,
    pub filter_by_value_max: Option<f64>,
    pub count: Option<i64>,
    pub align: Option<String>,
    pub aggregation: Option<TsAggregation>,
}

/// Aggregation options for TS.RANGE.
#[derive(Debug, Default, Clone)]
pub struct TsAggregation {
    pub aggregator: String,
    pub bucket_duration: i64,
    pub bucket_timestamp: Option<String>,
    pub empty: bool,
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
