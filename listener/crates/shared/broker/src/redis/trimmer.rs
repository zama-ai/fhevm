use redis::Value;
use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use super::{connection::RedisConnectionManager, error::RedisConsumerError};

/// Configuration for the stream trimmer.
#[derive(Debug, Clone)]
pub struct StreamTrimmerConfig {
    /// Stream to trim
    pub stream: String,
    /// Interval between trim cycles
    pub interval: Duration,
    /// Fallback MAXLEN if no consumer groups are found
    pub fallback_maxlen: Option<usize>,
}

/// Server-side stream trimmer that safely trims a Redis stream based on
/// all consumer groups' progress.
///
/// Runs on the listener core (publisher side), NOT on clients.
/// Uses `XTRIM ... MINID ~` to only trim what ALL groups have fully processed.
pub struct StreamTrimmer {
    connection: RedisConnectionManager,
    config: StreamTrimmerConfig,
}

impl StreamTrimmer {
    pub fn new(connection: RedisConnectionManager, config: StreamTrimmerConfig) -> Self {
        Self { connection, config }
    }

    /// Run the trimmer as a supervised loop.
    ///
    /// Periodically calls `trim_once()` at the configured interval.
    /// On error: logs, waits, and retries.
    /// Stops when the cancellation token is cancelled.
    pub async fn run(&self, cancel: CancellationToken) {
        info!(
            stream = %self.config.stream,
            interval = ?self.config.interval,
            "StreamTrimmer started"
        );

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    info!("StreamTrimmer: cancellation requested, stopping");
                    return;
                }
                _ = sleep(self.config.interval) => {
                    match self.trim_once().await {
                        Ok(trimmed) => {
                            if trimmed > 0 {
                                info!(
                                    stream = %self.config.stream,
                                    trimmed = trimmed,
                                    "StreamTrimmer: trimmed entries"
                                );
                            }
                        }
                        Err(e) => {
                            error!(
                                error = %e,
                                stream = %self.config.stream,
                                "StreamTrimmer: trim_once failed, will retry next interval"
                            );
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        }
    }

    /// Perform a single trim operation.
    ///
    /// 1. `XINFO GROUPS {stream}` to get all consumer groups
    /// 2. For each group: find the oldest pending message (or last-delivered-id)
    /// 3. Compute `safe_min_id` = minimum across all groups
    /// 4. `XTRIM {stream} MINID ~ {safe_min_id}` — trim only what's safe
    ///
    /// Returns the number of trimmed entries.
    pub async fn trim_once(&self) -> Result<u64, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // Get all consumer groups
        let groups_value: Value = redis::cmd("XINFO")
            .arg("GROUPS")
            .arg(&self.config.stream)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: self.config.stream.clone(),
                source: e,
            })?;

        let groups = Self::parse_groups(groups_value);

        if groups.is_empty() {
            // No consumer groups — use fallback MAXLEN if configured
            if let Some(maxlen) = self.config.fallback_maxlen {
                let trimmed: u64 = redis::cmd("XTRIM")
                    .arg(&self.config.stream)
                    .arg("MAXLEN")
                    .arg("~")
                    .arg(maxlen)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| RedisConsumerError::StreamRead {
                        stream: self.config.stream.clone(),
                        source: e,
                    })?;
                return Ok(trimmed);
            }
            debug!(
                stream = %self.config.stream,
                "No consumer groups and no fallback MAXLEN, skipping trim"
            );
            return Ok(0);
        }

        // For each group, find the safe minimum ID
        let mut safe_min_id: Option<String> = None;

        for group in &groups {
            let group_min = self.get_group_min_id(&group.name).await?;

            if let Some(ref min_id) = group_min {
                safe_min_id = Some(match safe_min_id {
                    None => min_id.clone(),
                    Some(ref current) => Self::min_stream_id(current, min_id),
                });
            }
            // If a group has no min ID (no pending AND no delivered), skip trimming
            // to be safe — don't trim anything
        }

        let Some(min_id) = safe_min_id else {
            debug!(
                stream = %self.config.stream,
                "No safe min ID found, skipping trim"
            );
            return Ok(0);
        };

        // XTRIM with MINID ~
        let trimmed: u64 = redis::cmd("XTRIM")
            .arg(&self.config.stream)
            .arg("MINID")
            .arg("~")
            .arg(&min_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: self.config.stream.clone(),
                source: e,
            })?;

        Ok(trimmed)
    }

    /// Get the minimum safe ID for a consumer group.
    ///
    /// First checks XPENDING for the oldest pending message.
    /// If no pending messages, uses the group's last-delivered-id.
    async fn get_group_min_id(&self, group: &str) -> Result<Option<String>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // XPENDING summary: [count, min_id, max_id, [[consumer, count], ...]]
        let result: Value = redis::cmd("XPENDING")
            .arg(&self.config.stream)
            .arg(group)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: self.config.stream.clone(),
                source: e,
            })?;

        // Parse XPENDING summary
        if let Value::Array(ref parts) = result
            && parts.len() >= 2
        {
            // parts[0] = count of pending
            let pending_count = match &parts[0] {
                Value::Int(n) => *n,
                _ => 0,
            };

            if pending_count > 0 {
                // parts[1] = min pending ID
                if let Value::BulkString(b) = &parts[1] {
                    return Ok(Some(String::from_utf8_lossy(b).to_string()));
                }
            }
        }

        // No pending messages — get last-delivered-id from XINFO GROUPS
        let groups_value: Value = redis::cmd("XINFO")
            .arg("GROUPS")
            .arg(&self.config.stream)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: self.config.stream.clone(),
                source: e,
            })?;

        let groups = Self::parse_groups(groups_value);
        for g in &groups {
            if g.name == group {
                return Ok(Some(g.last_delivered_id.clone()));
            }
        }

        Ok(None)
    }

    /// Compare two stream IDs and return the smaller one.
    ///
    /// Stream IDs are formatted as "{timestamp}-{sequence}".
    fn min_stream_id(a: &str, b: &str) -> String {
        let parse_id = |id: &str| -> (u64, u64) {
            let parts: Vec<&str> = id.splitn(2, '-').collect();
            let ts = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
            let seq = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            (ts, seq)
        };

        let (a_ts, a_seq) = parse_id(a);
        let (b_ts, b_seq) = parse_id(b);

        if (a_ts, a_seq) <= (b_ts, b_seq) {
            a.to_string()
        } else {
            b.to_string()
        }
    }

    /// Parse consumer group info from XINFO GROUPS response.
    fn parse_groups(value: Value) -> Vec<GroupInfo> {
        let mut groups = Vec::new();

        if let Value::Array(items) = value {
            for item in items {
                if let Value::Array(fields) = item {
                    let map = Self::flat_array_to_map(&fields);
                    let name = map.get("name").cloned().unwrap_or_default();
                    let last_delivered_id = map
                        .get("last-delivered-id")
                        .cloned()
                        .unwrap_or_else(|| "0-0".to_string());

                    groups.push(GroupInfo {
                        name,
                        last_delivered_id,
                    });
                }
            }
        }

        groups
    }

    /// Convert a flat Redis field array [key, value, key, value, ...] to a HashMap.
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
                // Skip non-bulk-string keys (shouldn't happen in XINFO output)
                let _ = iter.next();
            }
        }
        map
    }
}

/// Internal representation of a consumer group from XINFO GROUPS.
struct GroupInfo {
    name: String,
    last_delivered_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stream_id() {
        assert_eq!(StreamTrimmer::min_stream_id("100-0", "200-0"), "100-0");
        assert_eq!(StreamTrimmer::min_stream_id("200-0", "100-0"), "100-0");
        assert_eq!(StreamTrimmer::min_stream_id("100-1", "100-2"), "100-1");
        assert_eq!(StreamTrimmer::min_stream_id("100-0", "100-0"), "100-0");
    }

    #[test]
    fn test_parse_groups_empty() {
        let value = Value::Array(vec![]);
        let groups = StreamTrimmer::parse_groups(value);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_parse_groups_with_data() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"my-group".to_vec()),
            Value::BulkString(b"consumers".to_vec()),
            Value::Int(2),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(5),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"1234567890-0".to_vec()),
        ])]);

        let groups = StreamTrimmer::parse_groups(value);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "my-group");
        assert_eq!(groups[0].last_delivered_id, "1234567890-0");
    }
}
