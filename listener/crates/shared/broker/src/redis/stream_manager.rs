use redis::{AsyncCommands, Value};
use tracing::{debug, info, warn};

use super::{
    config::StreamTopology, connection::RedisConnectionManager, error::RedisConsumerError,
};

/// A consumer group as reported by `XINFO GROUPS`.
///
/// This is the single typed view of that reply. The trimmer and the queue
/// inspector each used to parse their own subset of it; both now go through
/// [`parse_group_statuses`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupStatus {
    /// Group name.
    pub name: String,
    /// Highest ID delivered to any consumer in the group. Frozen when nothing
    /// is draining the group, which is what makes it a liveness signal.
    pub last_delivered_id: String,
    /// Size of the pending-entries list.
    pub pending: u64,
    /// Entries added but never delivered. `None` on Redis < 7.0, where the
    /// field is absent from the reply.
    pub lag: Option<u64>,
}

/// Manages Redis stream and consumer group setup.
///
/// Analogous to `ExchangeManager` in the RMQ broker — handles
/// stream creation, consumer group creation, and topology setup.
pub struct StreamManager {
    connection: RedisConnectionManager,
}

impl StreamManager {
    /// Create a new StreamManager with the given connection manager.
    pub fn new(connection: RedisConnectionManager) -> Self {
        Self { connection }
    }

    /// Ensure a stream exists. If it doesn't exist, creates it via
    /// `XADD ... MAXLEN 0 * _init _init` then immediately trims.
    /// This is a no-op if the stream already exists.
    pub async fn ensure_stream(&self, stream: &str) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // Check if stream exists via XLEN (returns 0 for non-existent or empty)
        let exists: bool = redis::cmd("EXISTS")
            .arg(stream)
            .query_async(&mut conn)
            .await
            .map_err(RedisConsumerError::Connection)?;

        if !exists {
            // Create stream with a sentinel entry, then delete it
            let id: String = redis::cmd("XADD")
                .arg(stream)
                .arg("*")
                .arg("_init")
                .arg("1")
                .query_async(&mut conn)
                .await
                .map_err(|e| RedisConsumerError::StreamRead {
                    stream: stream.to_string(),
                    source: e,
                })?;

            // Remove the sentinel entry
            let _: i64 = redis::cmd("XDEL")
                .arg(stream)
                .arg(&id)
                .query_async(&mut conn)
                .await
                .map_err(|e| RedisConsumerError::StreamRead {
                    stream: stream.to_string(),
                    source: e,
                })?;

            info!(stream = %stream, "Stream created");
        } else {
            debug!(stream = %stream, "Stream already exists");
        }

        Ok(())
    }

    /// Ensure a consumer group exists on a stream.
    ///
    /// Uses `XGROUP CREATE ... MKSTREAM` which is idempotent — catches
    /// the BUSYGROUP error if the group already exists.
    ///
    /// `start_id` is typically `"0"` to read from the beginning or `"$"` to
    /// read only new messages.
    pub async fn ensure_consumer_group(
        &self,
        stream: &str,
        group: &str,
        start_id: &str,
    ) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Result<String, redis::RedisError> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(stream)
            .arg(group)
            .arg(start_id)
            .arg("MKSTREAM")
            .query_async(&mut conn)
            .await;

        match result {
            Ok(_) => {
                info!(
                    stream = %stream,
                    group = %group,
                    start_id = %start_id,
                    "Consumer group created"
                );
                Ok(())
            }
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("BUSYGROUP") {
                    debug!(
                        stream = %stream,
                        group = %group,
                        "Consumer group already exists"
                    );
                    Ok(())
                } else {
                    Err(RedisConsumerError::GroupCreation {
                        stream: stream.to_string(),
                        group: group.to_string(),
                        source: e,
                    })
                }
            }
        }
    }

    /// Ensure the full stream topology for a chain: main stream + dead-letter stream.
    ///
    /// This is an **infrastructure-level** operation — it only creates the streams.
    /// Consumer groups are automatically created when consumers start (via `XGROUP CREATE ... MKSTREAM`)
    /// and when the dead-letter stream receives its first message (via `XADD`).
    ///
    /// No explicit consumer group setup is needed — the system handles it transparently.
    pub async fn ensure_topology(
        &self,
        topology: &StreamTopology,
    ) -> Result<(), RedisConsumerError> {
        self.ensure_stream(&topology.main).await?;
        self.ensure_stream(&topology.dead).await?;

        info!(
            main = %topology.main,
            dead = %topology.dead,
            "Stream topology ensured (streams only)"
        );

        Ok(())
    }

    /// Delete a consumer from a group.
    /// Useful for cleaning up stale consumers.
    pub async fn delete_consumer(
        &self,
        stream: &str,
        group: &str,
        consumer_name: &str,
    ) -> Result<u64, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let pending_count: u64 = redis::cmd("XGROUP")
            .arg("DELCONSUMER")
            .arg(stream)
            .arg(group)
            .arg(consumer_name)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::GroupCreation {
                stream: stream.to_string(),
                group: group.to_string(),
                source: e,
            })?;

        if pending_count > 0 {
            warn!(
                stream = %stream,
                group = %group,
                consumer = %consumer_name,
                pending_count = %pending_count,
                "Deleted consumer with pending messages"
            );
        } else {
            info!(
                stream = %stream,
                group = %group,
                consumer = %consumer_name,
                "Consumer deleted"
            );
        }

        Ok(pending_count)
    }

    /// Get stream info via `XINFO STREAM`.
    /// Returns the raw Redis value for flexible inspection.
    pub async fn stream_info(&self, stream: &str) -> Result<redis::Value, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let info: redis::Value = redis::cmd("XINFO")
            .arg("STREAM")
            .arg(stream)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            })?;

        Ok(info)
    }

    /// List every consumer group on a stream, typed.
    ///
    /// A stream that does not exist has no groups, so that case returns an
    /// empty vector rather than an error — callers that are cleaning up after
    /// something cannot distinguish "already gone" from "never existed", and
    /// should not have to.
    pub async fn list_groups(&self, stream: &str) -> Result<Vec<GroupStatus>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Result<Value, _> = redis::cmd("XINFO")
            .arg("GROUPS")
            .arg(stream)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(value) => Ok(parse_group_statuses(&value)),
            Err(e) if is_missing_key(&e) => {
                debug!(stream = %stream, "Stream does not exist, reporting no groups");
                Ok(Vec::new())
            }
            Err(e) => Err(RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            }),
        }
    }

    /// The highest entry ID the stream has ever issued, via `XINFO STREAM`.
    ///
    /// This is the signal for "has anybody written since I last looked".
    /// Length cannot answer that: a stream being trimmed as fast as it is
    /// written holds its length steady while entries keep arriving. The last
    /// generated ID only ever moves forwards, and trimming does not move it.
    ///
    /// A stream that does not exist returns `None`, for the same reason
    /// [`list_groups`](Self::list_groups) returns an empty vector.
    pub async fn last_generated_id(
        &self,
        stream: &str,
    ) -> Result<Option<String>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Result<Value, _> = redis::cmd("XINFO")
            .arg("STREAM")
            .arg(stream)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(Value::Array(fields)) => {
                Ok(flat_array_to_map(&fields).get("last-generated-id").cloned())
            }
            Ok(_) => Ok(None),
            Err(e) if is_missing_key(&e) => {
                debug!(stream = %stream, "Stream does not exist, reporting no write position");
                Ok(None)
            }
            Err(e) => Err(RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            }),
        }
    }

    /// The oldest entry a group was handed but never acknowledged, via
    /// `XPENDING`.
    ///
    /// `last_delivered_id` says how far a group has read, not how far it has
    /// got; the difference between the two is the pending-entries list. A
    /// consumer killed mid-batch leaves its share of that list behind for
    /// good, because nothing here reclaims another consumer's pending entries
    /// and the host listener takes no part in its own shutdown. A successor
    /// seeded from `last_delivered_id` therefore starts past work that nobody
    /// finished.
    ///
    /// `None` means the group has nothing outstanding, or is already gone —
    /// a caller that read the group from [`list_groups`](Self::list_groups)
    /// can be raced by a retiring replica destroying it, which is not an
    /// error worth propagating.
    pub async fn oldest_pending_id(
        &self,
        stream: &str,
        group: &str,
    ) -> Result<Option<String>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // Summary form: [count, min_id, max_id, [[consumer, count], ...]].
        let result: Result<Value, _> = redis::cmd("XPENDING")
            .arg(stream)
            .arg(group)
            .query_async(&mut conn)
            .await;

        let value = match result {
            Ok(value) => value,
            // Matched on the code rather than the kind: a missing key and a
            // missing group both answer NOGROUP, and redis-rs files unknown
            // codes under a catch-all rather than giving them a kind of their
            // own. Matching the kind here would swallow unrelated failures.
            Err(e) if e.code() == Some("NOGROUP") => {
                debug!(
                    stream = %stream,
                    group = %group,
                    "Group is gone, reporting nothing pending"
                );
                return Ok(None);
            }
            Err(e) => {
                return Err(RedisConsumerError::StreamRead {
                    stream: stream.to_string(),
                    source: e,
                });
            }
        };

        let Value::Array(parts) = value else {
            return Ok(None);
        };

        // A group holding nothing answers [0, nil, nil, nil], so the ID coming
        // back as a string is itself the "has outstanding work" signal.
        match parts.get(1) {
            Some(Value::BulkString(id)) => Ok(Some(String::from_utf8_lossy(id).to_string())),
            _ => Ok(None),
        }
    }

    /// Destroy a consumer group via `XGROUP DESTROY`.
    ///
    /// Returns whether a group was actually removed, so a caller racing
    /// another replica can log precisely without treating the loss as an
    /// error. Destroying a group discards its pending-entries list; the stream
    /// and its entries are untouched.
    pub async fn destroy_group(
        &self,
        stream: &str,
        group: &str,
    ) -> Result<bool, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Result<i64, _> = redis::cmd("XGROUP")
            .arg("DESTROY")
            .arg(stream)
            .arg(group)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(removed) => {
                if removed > 0 {
                    info!(stream = %stream, group = %group, "Consumer group destroyed");
                } else {
                    debug!(stream = %stream, group = %group, "Consumer group already absent");
                }
                Ok(removed > 0)
            }
            // ERR no such key — the stream is gone, so the group is too.
            Err(e) if e.kind() == redis::ErrorKind::ResponseError => {
                debug!(stream = %stream, group = %group, "Stream does not exist, nothing to destroy");
                Ok(false)
            }
            Err(e) => Err(RedisConsumerError::GroupCreation {
                stream: stream.to_string(),
                group: group.to_string(),
                source: e,
            }),
        }
    }

    /// Delete a stream outright via `DEL`, taking its consumer groups with it.
    ///
    /// Returns whether a key was removed. This is destructive and unconditional
    /// — anything still consuming from the stream will see it vanish, and a
    /// publisher gated on stream existence will stop routing to it. Callers are
    /// responsible for establishing that nobody is using it first.
    pub async fn delete_stream(&self, stream: &str) -> Result<bool, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let removed: i64 = redis::cmd("DEL")
            .arg(stream)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            })?;

        if removed > 0 {
            info!(stream = %stream, "Stream deleted");
        } else {
            debug!(stream = %stream, "Stream already absent");
        }

        Ok(removed > 0)
    }

    /// Get consumer group info via `XINFO GROUPS`.
    /// Returns the raw Redis value for flexible inspection.
    pub async fn group_info(&self, stream: &str) -> Result<redis::Value, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let info: redis::Value = redis::cmd("XINFO")
            .arg("GROUPS")
            .arg(stream)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            })?;

        Ok(info)
    }

    /// Get stream length via `XLEN`.
    pub async fn stream_len(&self, stream: &str) -> Result<u64, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let len: u64 = conn
            .xlen(stream)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: stream.to_string(),
                source: e,
            })?;

        Ok(len)
    }
}

/// Whether an error is Redis saying the key is not there.
///
/// `XINFO` answers a missing stream with a bare `ERR no such key`, and a bare
/// `ERR` is the one reply carrying no distinguishing code of its own: in
/// `redis-rs` every other server error — `WRONGTYPE`, `NOPERM`, `NOGROUP` —
/// keeps its code, so only `ERR` collides. Matching the error kind alone would
/// therefore read *any* bare `ERR` as "the stream does not exist", including a
/// malformed command, and the caller would carry on against a stream it had
/// been told nothing about. The detail string is the only thing that separates
/// them.
fn is_missing_key(e: &redis::RedisError) -> bool {
    e.kind() == redis::ErrorKind::ResponseError
        && e.detail()
            .is_some_and(|detail| detail.contains("no such key"))
}

/// The largest stream ID strictly below `id`.
///
/// `XGROUP CREATE` is told the last ID a group should treat as already seen,
/// so asking a group to start *at* an entry means naming the one before it.
/// IDs are `<milliseconds>-<sequence>` ordered on the pair, which makes the
/// predecessor of sequence zero the previous millisecond at the highest
/// sequence that millisecond could hold.
///
/// Anything that is not a well-formed ID comes back unchanged. The only
/// caller hands the result straight to Redis, which rejects a malformed ID
/// with a better message than this could invent.
pub fn id_before(id: &str) -> String {
    let Some((ms, seq)) = id.split_once('-') else {
        return id.to_string();
    };
    let (Ok(ms), Ok(seq)) = (ms.parse::<u64>(), seq.parse::<u64>()) else {
        return id.to_string();
    };

    match seq.checked_sub(1) {
        Some(previous) => format!("{ms}-{previous}"),
        None => match ms.checked_sub(1) {
            Some(previous) => format!("{previous}-{}", u64::MAX),
            // `0-0` is the only ID with nothing below it, and Redis will not
            // issue it for a real entry, so this is unreachable in practice.
            None => "0-0".to_string(),
        },
    }
}

/// Parse an `XINFO GROUPS` reply into one [`GroupStatus`] per group.
///
/// The reply is an array of groups, each a flat `[key, value, ...]` array.
/// Unknown keys are ignored and missing keys fall back to the value Redis
/// would have reported for an untouched group, so a reply from an older
/// server parses as far as it goes rather than failing.
pub(super) fn parse_group_statuses(value: &Value) -> Vec<GroupStatus> {
    let Value::Array(items) = value else {
        return Vec::new();
    };

    items
        .iter()
        .filter_map(|item| {
            let Value::Array(fields) = item else {
                return None;
            };
            let map = flat_array_to_map(fields);

            Some(GroupStatus {
                name: map.get("name").cloned().unwrap_or_default(),
                last_delivered_id: map
                    .get("last-delivered-id")
                    .cloned()
                    .unwrap_or_else(|| "0-0".to_string()),
                pending: map
                    .get("pending")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0),
                // Absent before Redis 7.0.
                lag: map.get("lag").and_then(|v| v.parse::<u64>().ok()),
            })
        })
        .collect()
}

/// Convert a flat Redis field array `[key, value, key, value, ...]` into a map.
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
    fn parse_group_statuses_empty() {
        assert!(parse_group_statuses(&Value::Array(vec![])).is_empty());
    }

    #[test]
    fn parse_group_statuses_extracts_every_field() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"my-group".to_vec()),
            Value::BulkString(b"consumers".to_vec()),
            Value::Int(2),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(5),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"1234567890-0".to_vec()),
            Value::BulkString(b"lag".to_vec()),
            Value::Int(42),
        ])]);

        let groups = parse_group_statuses(&value);

        assert_eq!(
            groups,
            vec![GroupStatus {
                name: "my-group".to_string(),
                last_delivered_id: "1234567890-0".to_string(),
                pending: 5,
                lag: Some(42),
            }]
        );
    }

    /// Before Redis 7.0 the reply carries no `lag` field. Everything else
    /// still parses; `lag` reports as unknown rather than as zero, so callers
    /// can tell "nothing outstanding" apart from "cannot tell".
    #[test]
    fn parse_group_statuses_without_lag() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"name".to_vec()),
            Value::BulkString(b"my-group".to_vec()),
            Value::BulkString(b"pending".to_vec()),
            Value::Int(3),
            Value::BulkString(b"last-delivered-id".to_vec()),
            Value::BulkString(b"5-0".to_vec()),
        ])]);

        let groups = parse_group_statuses(&value);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].pending, 3);
        assert_eq!(groups[0].lag, None);
    }

    #[test]
    fn parse_group_statuses_reads_all_groups() {
        let group = |name: &str, id: &str| {
            Value::Array(vec![
                Value::BulkString(b"name".to_vec()),
                Value::BulkString(name.as_bytes().to_vec()),
                Value::BulkString(b"last-delivered-id".to_vec()),
                Value::BulkString(id.as_bytes().to_vec()),
            ])
        };
        let value = Value::Array(vec![group("blue", "10-0"), group("green", "20-0")]);

        let groups = parse_group_statuses(&value);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "blue");
        assert_eq!(groups[1].name, "green");
    }

    // Integration tests requiring Redis
    #[tokio::test]
    #[ignore]
    async fn test_ensure_consumer_group() {
        let conn = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let manager = StreamManager::new(conn);

        let result = manager
            .ensure_consumer_group("test.stream", "test-group", "0")
            .await;
        assert!(result.is_ok());

        // Idempotent — calling again should succeed
        let result = manager
            .ensure_consumer_group("test.stream", "test-group", "0")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_ensure_topology() {
        let conn = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let manager = StreamManager::new(conn);

        let topology = StreamTopology::from_prefix("test.events");

        // Topology only creates streams, consumer groups are created automatically by consumers
        let result = manager.ensure_topology(&topology).await;
        assert!(result.is_ok());

        // Idempotent — calling again should succeed
        let result = manager.ensure_topology(&topology).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_multiple_consumer_groups() {
        let conn = RedisConnectionManager::new("redis://localhost:6379")
            .await
            .unwrap();
        let manager = StreamManager::new(conn);

        let topology = StreamTopology::from_prefix("test.events");
        manager.ensure_topology(&topology).await.unwrap();

        // Multiple apps can create their own consumer groups on the same stream
        // Each uses ensure_consumer_group directly (or it's auto-created by RedisConsumer)
        let result = manager
            .ensure_consumer_group(&topology.main, "app-a-group", "0")
            .await;
        assert!(result.is_ok());

        let result = manager
            .ensure_consumer_group(&topology.main, "app-b-group", "0")
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn id_before_steps_back_one_sequence() {
        assert_eq!(id_before("1700000000000-5"), "1700000000000-4");
        assert_eq!(id_before("1700000000000-1"), "1700000000000-0");
    }

    #[test]
    fn id_before_crosses_into_the_previous_millisecond() {
        assert_eq!(
            id_before("1700000000000-0"),
            format!("1699999999999-{}", u64::MAX),
            "the predecessor of sequence zero is the millisecond before it, at \
             the highest sequence that millisecond could have held"
        );
    }

    #[test]
    fn id_before_leaves_malformed_ids_alone() {
        // Redis rejects these with a better message than this could invent.
        assert_eq!(id_before("$"), "$");
        assert_eq!(id_before("not-an-id"), "not-an-id");
        assert_eq!(id_before("1700000000000"), "1700000000000");
        assert_eq!(id_before("0-0"), "0-0");
    }
}
