//! Redis Streams depth introspection via XLEN and XINFO GROUPS.
//!
//! When a consumer `group` is specified, also queries `XINFO GROUPS` to
//! populate `pending` (PEL count) and `lag` (undelivered entries, Redis 7.0+).

use crate::traits::depth::{QueueDepths, QueueInspector};
use async_trait::async_trait;
use redis::{AsyncCommands, Value};
use tracing::debug;

use super::connection::RedisConnectionManager;
use super::error::RedisConsumerError;

/// Inspects Redis Streams depth using XLEN and XINFO GROUPS.
///
/// Derives stream names from the logical name:
/// - principal: `{name}`
/// - dead-letter: `{name}:dead`
/// - retry: `None` (Redis retry is PEL-based, not a separate stream)
///
/// When a consumer group is specified, also queries:
/// - `pending`: PEL count from `XINFO GROUPS`
/// - `lag`: undelivered entries from `XINFO GROUPS` (Redis 7.0+)
pub struct RedisQueueInspector {
    connection: RedisConnectionManager,
}

impl RedisQueueInspector {
    /// Create a new inspector from an existing connection manager.
    pub fn new(connection: RedisConnectionManager) -> Self {
        Self { connection }
    }

    /// Query the length of a single stream, returning 0 if the stream does not exist.
    ///
    /// Redis XLEN natively returns 0 for non-existent keys, so errors are
    /// propagated directly — no special-casing needed.
    async fn stream_len(&self, stream: &str) -> Result<u64, RedisConsumerError> {
        let mut conn = self.connection.get_connection();
        let len: u64 = conn.xlen(stream).await?;
        Ok(len)
    }

    /// Query `XINFO GROUPS {stream}` and extract `pending` and `lag` for
    /// the specified consumer group.
    ///
    /// Returns `(pending, lag)` where:
    /// - `pending` = PEL count (always available when the group exists)
    /// - `lag` = undelivered entries (Redis 7.0+; `None` on older versions)
    ///
    /// If the stream or group does not exist, returns `(Some(0), Some(0))`.
    async fn group_info(
        &self,
        stream: &str,
        group: &str,
    ) -> Result<(Option<u64>, Option<u64>), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Result<Value, _> = redis::cmd("XINFO")
            .arg("GROUPS")
            .arg(stream)
            .query_async(&mut conn)
            .await;

        let value = match result {
            Ok(v) => v,
            Err(e) => {
                // ERR no such key — stream doesn't exist → no work.
                if e.kind() == redis::ErrorKind::ResponseError {
                    debug!(
                        stream = stream,
                        group = group,
                        "Stream does not exist, reporting pending=0 lag=0"
                    );
                    return Ok((Some(0), Some(0)));
                }
                return Err(RedisConsumerError::StreamRead {
                    stream: stream.to_string(),
                    source: e,
                });
            }
        };

        let info = parse_group_depth_info(&value, group);
        match info {
            Some((pending, lag)) => Ok((Some(pending), lag)),
            // Group not found in XINFO response → no PEL, no lag.
            None => {
                debug!(
                    stream = stream,
                    group = group,
                    "Consumer group not found, reporting pending=0 lag=0"
                );
                Ok((Some(0), Some(0)))
            }
        }
    }
}

#[async_trait]
impl QueueInspector for RedisQueueInspector {
    type Error = RedisConsumerError;

    async fn queue_depths(
        &self,
        name: &str,
        group: Option<&str>,
    ) -> Result<QueueDepths, Self::Error> {
        let dead_stream = format!("{name}:dead");

        let (principal, dead_letter) =
            tokio::try_join!(self.stream_len(name), self.stream_len(&dead_stream))?;

        let (pending, lag) = match group {
            Some(g) => self.group_info(name, g).await?,
            None => (None, None),
        };

        Ok(QueueDepths {
            principal,
            retry: None, // Redis retry is PEL-based, no separate stream
            dead_letter,
            pending,
            lag,
        })
    }

    async fn exists(&self, name: &str) -> Result<bool, Self::Error> {
        let mut conn = self.connection.get_connection();
        let key_type: String = redis::cmd("TYPE")
            .arg(name)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: name.to_string(),
                source: e,
            })?;
        Ok(key_type == "stream")
    }

    /// Single `XINFO GROUPS` round-trip: returns `true` when the consumer
    /// group has zero pending (PEL) and zero lag (undelivered) entries.
    async fn is_empty(&self, name: &str, group: &str) -> Result<bool, Self::Error> {
        let (pending, lag) = self.group_info(name, group).await?;

        let pending_zero = pending.unwrap_or(0) == 0;
        let lag_zero = lag.unwrap_or(0) == 0;

        Ok(pending_zero && lag_zero)
    }

    /// Single `XINFO GROUPS` round-trip: returns `true` when the consumer group
    /// has at most one pending (PEL) entry and zero lag (undelivered) entries.
    ///
    /// One pending message means either an in-flight delivery or a stuck entry
    /// from a transient error — in both cases the consumer is already handling it.
    async fn is_empty_or_pending(&self, name: &str, group: &str) -> Result<bool, Self::Error> {
        let (pending, lag) = self.group_info(name, group).await?;

        let pending_zero = pending.unwrap_or(0) == 0 || pending.unwrap_or(0) == 1;
        let lag_zero = lag.unwrap_or(0) == 0;

        Ok(pending_zero && lag_zero)
    }
}

/// Parse the `XINFO GROUPS` response to extract `pending` and `lag` for a
/// specific group.
///
/// The response is an array of groups, each represented as a flat
/// `[key, value, key, value, ...]` array. We scan for the group matching
/// `target_group` and extract:
/// - `pending` (integer, always present)
/// - `lag` (integer, Redis 7.0+ only; `None` on older versions)
///
/// Returns `None` if the target group is not found.
fn parse_group_depth_info(value: &Value, target_group: &str) -> Option<(u64, Option<u64>)> {
    let groups = match value {
        Value::Array(items) => items,
        _ => return None,
    };

    for group_value in groups {
        let fields = match group_value {
            Value::Array(f) => f,
            _ => continue,
        };

        let map = flat_array_to_map(fields);

        let name = map.get("name")?;
        if name != target_group {
            continue;
        }

        let pending = map
            .get("pending")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        // `lag` is only available on Redis 7.0+.
        let lag = map.get("lag").and_then(|v| v.parse::<u64>().ok());

        return Some((pending, lag));
    }

    None
}

/// Convert a flat Redis field array `[key, value, key, value, ...]` to a
/// `HashMap<String, String>`.
///
/// Same pattern used by `StreamTrimmer::flat_array_to_map`.
fn flat_array_to_map(fields: &[Value]) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let mut iter = fields.iter();
    while let Some(key) = iter.next() {
        if let Value::BulkString(k) = key {
            if let Some(val) = iter.next() {
                let v = match val {
                    Value::BulkString(b) => String::from_utf8_lossy(b).to_string(),
                    Value::Int(n) => n.to_string(),
                    _ => continue,
                };
                map.insert(String::from_utf8_lossy(k).to_string(), v);
            }
        } else {
            // Skip non-bulk-string keys (shouldn't happen in XINFO output).
            let _ = iter.next();
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redis_queue_inspector_dead_stream_naming() {
        let name = "ethereum.blocks";
        let dead = format!("{name}:dead");
        assert_eq!(dead, "ethereum.blocks:dead");
    }

    #[test]
    fn parse_group_info_extracts_pending_and_lag() {
        // Simulate XINFO GROUPS response (Redis 7.0+):
        // [ [name, "mygroup", consumers, 1, pending, 5, last-delivered-id, "1-0", entries-read, 10, lag, 3] ]
        let response = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"mygroup".to_vec()),
            Value::BulkString(b"consumers".to_vec()),
            Value::Int(1),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(5),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"1-0".to_vec()),
            Value::BulkString(b"entries-read".to_vec()),
            Value::Int(10),
            Value::BulkString(b"lag".to_vec()),
            Value::Int(3),
        ])]);

        let result = parse_group_depth_info(&response, "mygroup");
        assert_eq!(result, Some((5, Some(3))));
    }

    #[test]
    fn parse_group_info_without_lag_redis_6() {
        // Simulate XINFO GROUPS response (Redis < 7.0, no lag/entries-read):
        let response = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"mygroup".to_vec()),
            Value::BulkString(b"consumers".to_vec()),
            Value::Int(1),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(2),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"100-0".to_vec()),
        ])]);

        let result = parse_group_depth_info(&response, "mygroup");
        assert_eq!(result, Some((2, None)));
    }

    #[test]
    fn parse_group_info_group_not_found() {
        let response = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"other-group".to_vec()),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(0),
        ])]);

        let result = parse_group_depth_info(&response, "mygroup");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_group_info_empty_response() {
        let response = Value::Array(vec![]);
        let result = parse_group_depth_info(&response, "mygroup");
        assert_eq!(result, None);
    }

    /// Validates the `is_empty_or_pending` logic: pending=1, lag=0 should
    /// be treated as caught-up (the single entry is already in-flight).
    #[test]
    fn is_empty_or_pending_allows_single_pending() {
        let response = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"mygroup".to_vec()),
            Value::BulkString(b"consumers".to_vec()),
            Value::Int(1),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(1),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"42-0".to_vec()),
            Value::BulkString(b"entries-read".to_vec()),
            Value::Int(42),
            Value::BulkString(b"lag".to_vec()),
            Value::Int(0),
        ])]);

        let (pending, lag) = parse_group_depth_info(&response, "mygroup").unwrap();

        // Mirror the logic from is_empty_or_pending
        let pending_ok = pending == 0 || pending == 1;
        let lag_ok = lag.unwrap_or(0) == 0;

        assert!(
            pending_ok && lag_ok,
            "pending=1 lag=0 must be treated as caught-up"
        );
    }

    /// pending=2 with lag=0 should NOT be treated as caught-up.
    #[test]
    fn is_empty_or_pending_rejects_multiple_pending() {
        let response = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"mygroup".to_vec()),
            Value::BulkString(b"consumers".to_vec()),
            Value::Int(1),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(2),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"42-0".to_vec()),
            Value::BulkString(b"entries-read".to_vec()),
            Value::Int(42),
            Value::BulkString(b"lag".to_vec()),
            Value::Int(0),
        ])]);

        let (pending, lag) = parse_group_depth_info(&response, "mygroup").unwrap();

        let pending_ok = pending == 0 || pending == 1;
        let lag_ok = lag.unwrap_or(0) == 0;

        assert!(
            !(pending_ok && lag_ok),
            "pending=2 must NOT be treated as caught-up"
        );
    }
}
