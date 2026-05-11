use redis::AsyncCommands;
use tracing::{debug, info, warn};

use super::{
    config::StreamTopology, connection::RedisConnectionManager, error::RedisConsumerError,
};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
