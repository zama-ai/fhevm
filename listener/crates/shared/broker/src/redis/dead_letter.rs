use redis::Value;
use tracing::{debug, info};

use super::{connection::RedisConnectionManager, error::RedisConsumerError};

/// A message in the dead-letter stream.
#[derive(Debug, Clone)]
pub struct DeadMessage {
    /// The dead-letter stream entry ID
    pub dead_id: String,
    /// The original stream entry ID
    pub original_id: String,
    /// The original stream name
    pub original_stream: String,
    /// How many times the message was delivered before DLQ
    pub delivery_count: u64,
    /// The raw payload data
    pub data: Vec<u8>,
}

/// Processor for dead-letter stream operations.
///
/// Provides list, replay, purge, and count operations on a dead-letter
/// stream, allowing operators to inspect and recover failed messages.
pub struct DeadLetterProcessor {
    connection: RedisConnectionManager,
}

impl DeadLetterProcessor {
    pub fn new(connection: RedisConnectionManager) -> Self {
        Self { connection }
    }

    /// List dead-letter messages.
    ///
    /// Returns up to `count` messages from the dead-letter stream.
    pub async fn list(
        &self,
        dead_stream: &str,
        count: usize,
    ) -> Result<Vec<DeadMessage>, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let result: Value = redis::cmd("XRANGE")
            .arg(dead_stream)
            .arg("-")
            .arg("+")
            .arg("COUNT")
            .arg(count)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: dead_stream.to_string(),
                source: e,
            })?;

        let messages = Self::parse_dead_messages(result);
        debug!(
            dead_stream = %dead_stream,
            count = messages.len(),
            "Listed dead-letter messages"
        );

        Ok(messages)
    }

    /// Replay a dead-letter message back to the main stream.
    ///
    /// Reads the message from the dead-letter stream, publishes it
    /// to the main stream via XADD, and deletes it from the dead-letter stream.
    ///
    /// Returns the new message ID in the main stream.
    pub async fn replay(
        &self,
        dead_stream: &str,
        main_stream: &str,
        dead_msg_id: &str,
    ) -> Result<String, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // Read the specific message from dead-letter stream
        let result: Value = redis::cmd("XRANGE")
            .arg(dead_stream)
            .arg(dead_msg_id)
            .arg(dead_msg_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: dead_stream.to_string(),
                source: e,
            })?;

        let messages = Self::parse_dead_messages(result);
        let msg = messages.into_iter().next().ok_or_else(|| {
            RedisConsumerError::Configuration(format!(
                "Dead-letter message not found: {} in {}",
                dead_msg_id, dead_stream
            ))
        })?;

        // Publish to main stream
        let new_id: String = redis::cmd("XADD")
            .arg(main_stream)
            .arg("*")
            .arg("data")
            .arg(&msg.data)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: main_stream.to_string(),
                source: e,
            })?;

        // Delete from dead-letter stream
        let _: i64 = redis::cmd("XDEL")
            .arg(dead_stream)
            .arg(dead_msg_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: dead_stream.to_string(),
                source: e,
            })?;

        info!(
            dead_stream = %dead_stream,
            main_stream = %main_stream,
            dead_msg_id = %dead_msg_id,
            new_id = %new_id,
            "Replayed dead-letter message"
        );

        Ok(new_id)
    }

    /// Purge old dead-letter messages.
    ///
    /// Reads messages from the dead-letter stream and deletes those
    /// whose stream ID timestamp is older than `older_than` duration.
    /// Returns the number of purged messages.
    pub async fn purge(
        &self,
        dead_stream: &str,
        older_than: std::time::Duration,
    ) -> Result<usize, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        // Calculate the cutoff timestamp in milliseconds
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let cutoff_ms = now_ms.saturating_sub(older_than.as_millis() as u64);

        // Read all entries up to the cutoff
        let cutoff_id = format!("{}-0", cutoff_ms);
        let result: Value = redis::cmd("XRANGE")
            .arg(dead_stream)
            .arg("-")
            .arg(&cutoff_id)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: dead_stream.to_string(),
                source: e,
            })?;

        let ids = Self::extract_entry_ids(result);
        if ids.is_empty() {
            debug!(dead_stream = %dead_stream, "No messages to purge");
            return Ok(0);
        }

        // Delete in batches
        let mut deleted = 0usize;
        for chunk in ids.chunks(100) {
            let mut cmd = redis::cmd("XDEL");
            cmd.arg(dead_stream);
            for id in chunk {
                cmd.arg(id);
            }
            let count: i64 =
                cmd.query_async(&mut conn)
                    .await
                    .map_err(|e| RedisConsumerError::DeadLetter {
                        stream: dead_stream.to_string(),
                        source: e,
                    })?;
            deleted += count as usize;
        }

        info!(
            dead_stream = %dead_stream,
            purged = deleted,
            older_than = ?older_than,
            "Purged dead-letter messages"
        );

        Ok(deleted)
    }

    /// Get the count of messages in the dead-letter stream.
    pub async fn count(&self, dead_stream: &str) -> Result<u64, RedisConsumerError> {
        let mut conn = self.connection.get_connection();

        let len: u64 = redis::cmd("XLEN")
            .arg(dead_stream)
            .query_async(&mut conn)
            .await
            .map_err(|e| RedisConsumerError::DeadLetter {
                stream: dead_stream.to_string(),
                source: e,
            })?;

        Ok(len)
    }

    /// Parse XRANGE response into DeadMessage structs.
    fn parse_dead_messages(value: Value) -> Vec<DeadMessage> {
        let mut messages = Vec::new();

        if let Value::Array(entries) = value {
            for entry in entries {
                if let Value::Array(parts) = entry {
                    if parts.len() < 2 {
                        continue;
                    }

                    let dead_id = match &parts[0] {
                        Value::BulkString(b) => String::from_utf8_lossy(b).to_string(),
                        _ => continue,
                    };

                    if let Value::Array(fields) = &parts[1] {
                        let field_map = Self::fields_to_map(fields);

                        let original_id = field_map.get("original_id").cloned().unwrap_or_default();
                        let original_stream = field_map
                            .get("original_stream")
                            .cloned()
                            .unwrap_or_default();
                        let delivery_count: u64 = field_map
                            .get("delivery_count")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);
                        let data = field_map
                            .get("data")
                            .map(|s| s.as_bytes().to_vec())
                            .unwrap_or_default();

                        messages.push(DeadMessage {
                            dead_id,
                            original_id,
                            original_stream,
                            delivery_count,
                            data,
                        });
                    }
                }
            }
        }

        messages
    }

    /// Convert a flat field array [key, value, key, value, ...] to a map.
    fn fields_to_map(fields: &[Value]) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        let mut iter = fields.iter();
        while let Some(key) = iter.next() {
            if let (Value::BulkString(k), Some(Value::BulkString(v))) = (key, iter.next()) {
                map.insert(
                    String::from_utf8_lossy(k).to_string(),
                    String::from_utf8_lossy(v).to_string(),
                );
            }
        }
        map
    }

    /// Extract entry IDs from XRANGE response.
    fn extract_entry_ids(value: Value) -> Vec<String> {
        let mut ids = Vec::new();

        if let Value::Array(entries) = value {
            for entry in entries {
                if let Value::Array(parts) = entry
                    && let Some(Value::BulkString(b)) = parts.first()
                {
                    ids.push(String::from_utf8_lossy(b).to_string());
                }
            }
        }

        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dead_messages_empty() {
        let value = Value::Array(vec![]);
        let messages = DeadLetterProcessor::parse_dead_messages(value);
        assert!(messages.is_empty());
    }

    #[test]
    fn test_parse_dead_messages_with_data() {
        let value = Value::Array(vec![Value::Array(vec![
            Value::BulkString(b"9999-0".to_vec()),
            Value::Array(vec![
                Value::BulkString(b"original_id".to_vec()),
                Value::BulkString(b"1234-0".to_vec()),
                Value::BulkString(b"original_stream".to_vec()),
                Value::BulkString(b"ethereum.events".to_vec()),
                Value::BulkString(b"delivery_count".to_vec()),
                Value::BulkString(b"3".to_vec()),
                Value::BulkString(b"data".to_vec()),
                Value::BulkString(b"{\"block\":42}".to_vec()),
            ]),
        ])]);

        let messages = DeadLetterProcessor::parse_dead_messages(value);
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].dead_id, "9999-0");
        assert_eq!(messages[0].original_id, "1234-0");
        assert_eq!(messages[0].original_stream, "ethereum.events");
        assert_eq!(messages[0].delivery_count, 3);
        assert_eq!(messages[0].data, b"{\"block\":42}");
    }

    #[test]
    fn test_extract_entry_ids() {
        let value = Value::Array(vec![
            Value::Array(vec![
                Value::BulkString(b"1-0".to_vec()),
                Value::Array(vec![]),
            ]),
            Value::Array(vec![
                Value::BulkString(b"2-0".to_vec()),
                Value::Array(vec![]),
            ]),
        ]);

        let ids = DeadLetterProcessor::extract_entry_ids(value);
        assert_eq!(ids, vec!["1-0", "2-0"]);
    }
}
