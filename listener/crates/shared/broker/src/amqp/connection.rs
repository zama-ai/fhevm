use lapin::{Channel, Connection, ConnectionProperties};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};
use tracing::{error, info};

use super::error::ConnectionError;

/// Manages RabbitMQ connections with automatic reconnection.
#[derive(Clone)]
pub struct ConnectionManager {
    addr: String,
    connection: Arc<RwLock<Option<Arc<Connection>>>>,
    pub(crate) reconnect_delay: Duration,
}

impl ConnectionManager {
    /// Create a new ConnectionManager with the given address.
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            connection: Arc::new(RwLock::new(None)),
            reconnect_delay: Duration::from_secs(5),
        }
    }

    /// Create a new ConnectionManager with custom reconnect delay.
    pub fn with_reconnect_delay(addr: impl Into<String>, reconnect_delay: Duration) -> Self {
        Self {
            addr: addr.into(),
            connection: Arc::new(RwLock::new(None)),
            reconnect_delay,
        }
    }

    /// Get the RabbitMQ address.
    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// Get or create a connection.
    async fn get_or_create_connection(&self) -> Result<Arc<Connection>, ConnectionError> {
        // Check if we have a healthy connection
        {
            let guard = self.connection.read().await;
            if let Some(ref conn) = *guard
                && conn.status().connected()
            {
                return Ok(Arc::clone(conn));
            }
        }

        // Need to create a new connection
        let mut guard = self.connection.write().await;

        // Double-check after acquiring write lock
        if let Some(ref conn) = *guard
            && conn.status().connected()
        {
            return Ok(Arc::clone(conn));
        }

        // Create new connection
        let conn = Connection::connect(&self.addr, ConnectionProperties::default()).await?;
        let conn = Arc::new(conn);
        *guard = Some(Arc::clone(&conn));
        info!("ConnectionManager: Connected to RabbitMQ at {}", self.addr);

        Ok(conn)
    }

    /// Create a new channel from the shared connection.
    pub async fn create_channel(&self) -> Result<Channel, ConnectionError> {
        let conn = self.get_or_create_connection().await?;
        conn.create_channel()
            .await
            .map_err(|e| ConnectionError::Channel(e.to_string()))
    }

    /// Create a channel with infinite retry on failure.
    /// Use this when the consumer must eventually succeed.
    pub async fn create_channel_with_retry(&self) -> Channel {
        loop {
            match self.create_channel().await {
                Ok(channel) => return channel,
                Err(e) => {
                    error!(
                        "Failed to create channel: {}. Retrying in {:?}...",
                        e, self.reconnect_delay
                    );
                    // Clear the connection so next attempt creates a fresh one
                    {
                        let mut guard = self.connection.write().await;
                        *guard = None;
                    }
                    sleep(self.reconnect_delay).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_manager_new() {
        let manager = ConnectionManager::new("amqp://localhost:5672");
        assert_eq!(manager.addr(), "amqp://localhost:5672");
        assert_eq!(manager.reconnect_delay, Duration::from_secs(5));
    }

    #[test]
    fn test_connection_manager_with_custom_delay() {
        let manager = ConnectionManager::with_reconnect_delay(
            "amqp://localhost:5672",
            Duration::from_secs(10),
        );
        assert_eq!(manager.reconnect_delay, Duration::from_secs(10));
    }

    // Integration tests requiring RabbitMQ
    #[tokio::test]
    #[ignore]
    async fn test_connection_manager_create_channel() {
        let manager = ConnectionManager::new("amqp://localhost:5672");
        let channel = manager.create_channel().await;
        assert!(channel.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_connection_manager_reconnect() {
        let manager = ConnectionManager::new("amqp://localhost:5672");

        // Create first channel
        let channel1 = manager.create_channel().await.unwrap();
        assert!(channel1.status().connected());

        // Create second channel (should reuse connection)
        let channel2 = manager.create_channel().await.unwrap();
        assert!(channel2.status().connected());
    }
}
