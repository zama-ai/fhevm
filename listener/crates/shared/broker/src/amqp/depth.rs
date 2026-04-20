//! AMQP queue depth introspection via passive queue_declare.

use crate::traits::depth::{QueueDepths, QueueInspector};
use async_trait::async_trait;
use lapin::options::QueueDeclareOptions;
use lapin::protocol::AMQPErrorKind;
use lapin::protocol::AMQPSoftError;
use lapin::types::FieldTable;

use super::connection::ConnectionManager;
use super::error::ConsumerError;

/// Inspects AMQP queue depth using passive `queue_declare`.
///
/// Derives queue names from the logical name:
/// - principal: `{name}`
/// - retry: `{name}.retry`
/// - dead-letter: `{name}.error`
///
/// Passive declare does NOT create queues — it only queries existing ones.
/// If a queue does not exist, the depth is reported as 0.
pub struct AmqpQueueInspector {
    connection: ConnectionManager,
}

impl AmqpQueueInspector {
    /// Create a new inspector from an existing connection manager.
    pub fn new(connection: ConnectionManager) -> Self {
        Self { connection }
    }

    /// Query the message count of a single queue via passive declare.
    ///
    /// Returns 0 if the queue does not exist (lapin returns a channel error
    /// for passive declare on a non-existent queue, which we catch here).
    async fn queue_message_count(&self, queue: &str) -> Result<u64, ConsumerError> {
        let channel = self.connection.create_channel().await?;
        let result = channel
            .queue_declare(
                queue.into(),
                QueueDeclareOptions {
                    passive: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await;

        match result {
            Ok(queue_state) => Ok(queue_state.message_count() as u64),
            Err(e) => {
                // Passive declare on a non-existent queue closes the channel
                // with a NOT_FOUND error. Treat as 0 messages.
                if is_not_found(&e) {
                    Ok(0)
                } else {
                    Err(ConsumerError::QueueDeclaration {
                        queue: queue.to_string(),
                        source: e,
                    })
                }
            }
        }
    }

    /// Returns whether a queue exists (passive declare succeeds).
    async fn queue_exists(&self, queue: &str) -> Result<bool, ConsumerError> {
        let channel = self.connection.create_channel().await?;
        let result = channel
            .queue_declare(
                queue.into(),
                QueueDeclareOptions {
                    passive: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if is_not_found(&e) {
                    Ok(false)
                } else {
                    Err(ConsumerError::QueueDeclaration {
                        queue: queue.to_string(),
                        source: e,
                    })
                }
            }
        }
    }
}

/// Check if a lapin error indicates a queue-not-found condition (AMQP 404).
fn is_not_found(err: &lapin::Error) -> bool {
    if let lapin::ErrorKind::ProtocolError(amqp_err) = err.kind() {
        amqp_err.kind() == &AMQPErrorKind::Soft(AMQPSoftError::NOTFOUND)
    } else {
        false
    }
}

#[async_trait]
impl QueueInspector for AmqpQueueInspector {
    type Error = ConsumerError;

    async fn queue_depths(
        &self,
        name: &str,
        _group: Option<&str>,
    ) -> Result<QueueDepths, Self::Error> {
        let retry_queue = format!("{name}.retry");
        let error_queue = format!("{name}.error");

        let (principal, retry, dead_letter) = tokio::try_join!(
            self.queue_message_count(name),
            self.queue_message_count(&retry_queue),
            self.queue_message_count(&error_queue),
        )?;

        // AMQP `message_count()` already returns ready (unconsumed) messages,
        // so `principal` is effectively the consumer-visible depth.
        // `pending`/`lag` are Redis-specific concepts — left as None.
        Ok(QueueDepths {
            principal,
            retry: Some(retry),
            dead_letter,
            pending: None,
            lag: None,
        })
    }

    /// Single passive `queue_declare` round-trip: returns `true` when the
    /// principal queue has zero ready messages (or does not exist).
    async fn is_empty(&self, name: &str, _group: &str) -> Result<bool, Self::Error> {
        let count = self.queue_message_count(name).await?;
        Ok(count == 0)
    }

    /// Equivalent to [`is_empty`](Self::is_empty) — AMQP has no pending/lag
    /// distinction, so this returns `true` when `message_count == 0`.
    async fn is_empty_or_pending(&self, name: &str, _group: &str) -> Result<bool, Self::Error> {
        let count = self.queue_message_count(name).await?;
        Ok(count == 0)
    }

    async fn exists(&self, name: &str) -> Result<bool, Self::Error> {
        self.queue_exists(name).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn amqp_queue_naming() {
        let name = "ethereum.indexer";
        assert_eq!(format!("{name}.retry"), "ethereum.indexer.retry");
        assert_eq!(format!("{name}.error"), "ethereum.indexer.error");
    }
}
