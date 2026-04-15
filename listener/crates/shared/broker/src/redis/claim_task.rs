use redis::Value;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use super::{
    config::RedisRetryConfig, connection::RedisConnectionManager, error::RedisConsumerError,
};

const CLASS_TRANSIENT: &str = "transient";
const CLASS_PERMANENT: &str = "permanent";

/// Background task that sweeps for idle pending messages and either
/// re-claims them for reprocessing or routes them to the dead-letter stream.
///
/// Runs as a supervised loop — restarts after panics with a delay.
pub struct ClaimSweeper {
    connection: RedisConnectionManager,
    config: RedisRetryConfig,
    classification_paused: Option<Arc<AtomicBool>>,
}

/// A pending entry returned by XPENDING with detail.
#[derive(Debug)]
struct PendingEntry {
    id: String,
    _consumer: String,
    idle_ms: u64,
    delivery_count: u64,
}

impl ClaimSweeper {
    pub fn new(connection: RedisConnectionManager, config: RedisRetryConfig) -> Self {
        Self {
            connection,
            config,
            classification_paused: None,
        }
    }

    pub fn new_with_classification_pause(
        connection: RedisConnectionManager,
        config: RedisRetryConfig,
        classification_paused: Arc<AtomicBool>,
    ) -> Self {
        Self {
            connection,
            config,
            classification_paused: Some(classification_paused),
        }
    }

    /// Run the claim sweeper as a supervisor loop.
    ///
    /// Periodically calls `sweep_once()` at `config.claim_interval`.
    /// On panic/error: logs, waits, and restarts.
    /// Stops when the cancellation token is cancelled.
    pub async fn run(&self, cancel: CancellationToken) {
        info!(
            stream = %self.config.base.stream,
            group = %self.config.base.group_name,
            interval = ?self.config.claim_interval,
            "ClaimSweeper started"
        );

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    info!("ClaimSweeper: cancellation requested, stopping");
                    return;
                }
                _ = sleep(self.config.claim_interval) => {
                    if self.is_classification_paused() {
                        debug!(
                            stream = %self.config.base.stream,
                            group = %self.config.base.group_name,
                            "ClaimSweeper paused: waiting for classification writes to recover"
                        );
                        continue;
                    }
                    match self.sweep_once().await {
                        Ok(()) => {}
                        Err(e) => {
                            if e.is_connection_error() {
                                self.connection.force_reconnect().await;
                            }
                            error!(
                                error = %e,
                                "ClaimSweeper: sweep_once failed, will retry next interval"
                            );
                            // Extra delay after error to avoid tight error loops
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        }
    }

    fn is_classification_paused(&self) -> bool {
        self.classification_paused
            .as_ref()
            .is_some_and(|paused| paused.load(Ordering::Relaxed))
    }

    /// Perform a single sweep: find idle pending messages, claim or DLQ them.
    async fn sweep_once(&self) -> Result<(), RedisConsumerError> {
        let pending_entries = self.get_pending_entries().await?;

        if pending_entries.is_empty() {
            debug!(
                stream = %self.config.base.stream,
                group = %self.config.base.group_name,
                "ClaimSweeper: no idle pending messages"
            );
            return Ok(());
        }

        for entry in &pending_entries {
            if entry.idle_ms < self.config.claim_min_idle.as_millis() as u64 {
                continue;
            }

            let classification = match self.get_classification(&entry.id).await {
                Ok(classification) => classification,
                Err(e) => {
                    // Unknown marker classification must never trigger unsafe DLQ.
                    warn!(
                        error = %e,
                        stream_id = %entry.id,
                        "Classification lookup failed; defaulting to transient path"
                    );
                    None
                }
            };

            if classification.as_deref() == Some(CLASS_TRANSIENT) {
                // Infinite transient retry: reclaim without increasing delivery count.
                self.claim_message(entry, Some(entry.delivery_count))
                    .await?;
                continue;
            }

            if classification.as_deref() != Some(CLASS_PERMANENT) {
                // No explicit permanent classification => fail-safe transient handling.
                self.claim_message(entry, Some(entry.delivery_count))
                    .await?;
            } else if entry.delivery_count >= self.config.max_retries as u64 {
                // Explicitly permanent and retry budget exhausted => move to dead stream.
                self.move_to_dead_letter(entry).await?;
            } else {
                // Explicitly permanent and still under budget => bounded retry path.
                self.claim_message(entry, None).await?;
            }
        }

        Ok(())
    }

    /// Get pending entries via `XPENDING {stream} {group} - + 100`.
    async fn get_pending_entries(&self) -> Result<Vec<PendingEntry>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Value = redis::cmd("XPENDING")
            .arg(&self.config.base.stream)
            .arg(&self.config.base.group_name)
            .arg("-")
            .arg("+")
            .arg(100)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::Claim {
                stream: self.config.base.stream.clone(),
                source: e,
            })?;

        let entries = Self::parse_pending_entries(result);
        Ok(entries)
    }

    /// Parse the XPENDING detail reply into structured entries.
    ///
    /// XPENDING returns an array of arrays:
    /// `[[id, consumer, idle_ms, delivery_count], ...]`
    fn parse_pending_entries(value: Value) -> Vec<PendingEntry> {
        let mut entries = Vec::new();

        if let Value::Array(items) = value {
            for item in items {
                if let Value::Array(fields) = item
                    && fields.len() >= 4
                {
                    let id = match &fields[0] {
                        Value::BulkString(b) => String::from_utf8_lossy(b).to_string(),
                        _ => continue,
                    };
                    let consumer = match &fields[1] {
                        Value::BulkString(b) => String::from_utf8_lossy(b).to_string(),
                        _ => continue,
                    };
                    let idle_ms = match &fields[2] {
                        Value::Int(n) => *n as u64,
                        _ => continue,
                    };
                    let delivery_count = match &fields[3] {
                        Value::Int(n) => *n as u64,
                        _ => continue,
                    };

                    entries.push(PendingEntry {
                        id,
                        _consumer: consumer,
                        idle_ms,
                        delivery_count,
                    });
                }
            }
        }

        entries
    }

    /// Claim a message for this consumer via XCLAIM.
    ///
    /// When `retry_count_override` is provided, the claim uses `RETRYCOUNT` to
    /// preserve that delivery count (used for infinite transient retries).
    async fn claim_message(
        &self,
        entry: &PendingEntry,
        retry_count_override: Option<u64>,
    ) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let mut cmd = redis::cmd("XCLAIM");
        cmd.arg(&self.config.base.stream)
            .arg(&self.config.base.group_name)
            .arg(&self.config.base.consumer_name)
            .arg(self.config.claim_min_idle.as_millis() as u64)
            .arg(&entry.id);

        if let Some(retry_count) = retry_count_override {
            cmd.arg("RETRYCOUNT").arg(retry_count);
        }

        let _: Value = cmd
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::Claim {
                stream: self.config.base.stream.clone(),
                source: e,
            })?;

        debug!(
            stream = %self.config.base.stream,
            id = %entry.id,
            delivery_count = %entry.delivery_count,
            idle_ms = %entry.idle_ms,
            "Claimed idle message for reprocessing"
        );

        Ok(())
    }

    /// Move a message to the dead-letter stream and acknowledge from the main stream.
    async fn move_to_dead_letter(&self, entry: &PendingEntry) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // First, read the message data so we can copy it to DLQ
        let data = self.read_message_data(&entry.id).await?;

        // XADD to dead-letter stream with metadata
        let _: String = redis::cmd("XADD")
            .arg(&self.config.dead_stream)
            .arg("*")
            .arg("original_id")
            .arg(&entry.id)
            .arg("original_stream")
            .arg(&self.config.base.stream)
            .arg("delivery_count")
            .arg(entry.delivery_count)
            .arg("data")
            .arg(&data)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: self.config.dead_stream.clone(),
                source: e,
            })?;

        // XACK from the main stream
        let _: i64 = redis::cmd("XACK")
            .arg(&self.config.base.stream)
            .arg(&self.config.base.group_name)
            .arg(&entry.id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::Acknowledge {
                stream: self.config.base.stream.clone(),
                source: e,
            })?;

        // Clear stale classification marker now that the message is terminal.
        self.clear_classification(&entry.id).await?;

        warn!(
            stream = %self.config.base.stream,
            dead_stream = %self.config.dead_stream,
            id = %entry.id,
            delivery_count = %entry.delivery_count,
            "Message moved to dead-letter stream after max retries"
        );

        Ok(())
    }

    async fn get_classification(
        &self,
        stream_id: &str,
    ) -> Result<Option<String>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();
        let marker_key = self.config.classification_marker_key();
        redis::cmd("HGET")
            .arg(&marker_key)
            .arg(stream_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::TransientMarker {
                key: marker_key,
                source: e,
            })
    }

    async fn clear_classification(&self, stream_id: &str) -> Result<(), RedisConsumerError> {
        let mut conn = self.connection.get_connection();
        let marker_key = self.config.classification_marker_key();
        let _: i64 = redis::cmd("HDEL")
            .arg(&marker_key)
            .arg(stream_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::TransientMarker {
                key: marker_key,
                source: e,
            })?;
        Ok(())
    }

    /// Read message data from the stream by ID via XRANGE.
    async fn read_message_data(&self, id: &str) -> Result<Vec<u8>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Value = redis::cmd("XRANGE")
            .arg(&self.config.base.stream)
            .arg(id)
            .arg(id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::StreamRead {
                stream: self.config.base.stream.clone(),
                source: e,
            })?;

        // Parse XRANGE response: [[id, [field, value, field, value, ...]], ...]
        if let Value::Array(entries) = result
            && let Some(Value::Array(entry)) = entries.into_iter().next()
            && entry.len() >= 2
            && let Value::Array(fields) = &entry[1]
        {
            // Find the "data" field
            let mut iter = fields.iter();
            while let Some(key) = iter.next() {
                if let Value::BulkString(k) = key
                    && k == b"data"
                    && let Some(Value::BulkString(v)) = iter.next()
                {
                    return Ok(v.clone());
                }
                // Skip value if key didn't match
                let _ = iter.next();
            }
        }

        // If we can't find the data, return empty bytes
        // This can happen if the message was already deleted
        warn!(
            stream = %self.config.base.stream,
            id = %id,
            "Could not read message data for DLQ, using empty payload"
        );
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pending_entries_empty() {
        let value = Value::Array(vec![]);
        let entries = ClaimSweeper::parse_pending_entries(value);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_parse_pending_entries_with_data() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"1234567890-0".to_vec()),
            Value::BulkString(b"consumer-1".to_vec()),
            Value::Int(60000),
            Value::Int(3),
        ])]);

        let entries = ClaimSweeper::parse_pending_entries(value);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "1234567890-0");
        assert_eq!(entries[0]._consumer, "consumer-1");
        assert_eq!(entries[0].idle_ms, 60000);
        assert_eq!(entries[0].delivery_count, 3);
    }

    #[test]
    fn test_parse_pending_entries_multiple() {
        let value = Value::Array(vec![
            Value::Array(vec![
                Value::BulkString(b"1-0".to_vec()),
                Value::BulkString(b"c1".to_vec()),
                Value::Int(10000),
                Value::Int(1),
            ]),
            Value::Array(vec![
                Value::BulkString(b"2-0".to_vec()),
                Value::BulkString(b"c2".to_vec()),
                Value::Int(90000),
                Value::Int(5),
            ]),
        ]);

        let entries = ClaimSweeper::parse_pending_entries(value);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[1].delivery_count, 5);
    }
}
