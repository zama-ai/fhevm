use std::sync::Arc;

use lapin::options::ExchangeDeclareOptions;
use lapin::types::{FieldTable, ShortString};
use tracing::info;

use super::{config::ExchangeTopology, connection::ConnectionManager, error::ExchangeError};

/// Manages RabbitMQ exchange declarations.
pub struct ExchangeManager {
    connection: Arc<ConnectionManager>,
}

impl ExchangeManager {
    /// Create a new ExchangeManager with a shared connection manager.
    pub fn new(connection: Arc<ConnectionManager>) -> Self {
        Self { connection }
    }

    /// Create a new ExchangeManager with a fresh connection manager.
    pub fn with_addr(addr: impl Into<String>) -> Self {
        Self {
            connection: Arc::new(ConnectionManager::new(addr)),
        }
    }

    /// Declare a single exchange.
    pub async fn declare_exchange(&self, name: &str) -> Result<(), ExchangeError> {
        let channel = self.connection.create_channel().await?;

        channel
            .exchange_declare(
                ShortString::from(name),
                lapin::ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| ExchangeError::Declaration {
                name: name.to_string(),
                source: e,
            })?;

        info!(exchange = %name, "Exchange declared");
        Ok(())
    }

    /// Declare all exchanges for a chain topology (main, retry, dlx).
    ///
    /// NOTE: This method may be deprecated in the future as exchange management
    /// moves to infrastructure-as-code or separate tooling.
    pub async fn declare_topology(&self, topology: &ExchangeTopology) -> Result<(), ExchangeError> {
        self.declare_exchange(&topology.main).await?;
        self.declare_exchange(&topology.retry).await?;
        self.declare_exchange(&topology.dlx).await?;

        info!(
            main = %topology.main,
            retry = %topology.retry,
            dlx = %topology.dlx,
            "Exchange topology declared"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_manager_new() {
        let connection = Arc::new(ConnectionManager::new("amqp://localhost:5672"));
        let _manager = ExchangeManager::new(connection);
    }

    #[test]
    fn test_exchange_manager_with_addr() {
        let _manager = ExchangeManager::with_addr("amqp://localhost:5672");
    }

    // Integration tests requiring RabbitMQ
    #[tokio::test]
    #[ignore]
    async fn test_declare_exchange() {
        let manager = ExchangeManager::with_addr("amqp://localhost:5672");
        let result = manager.declare_exchange("test.exchange").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_declare_topology() {
        let manager = ExchangeManager::with_addr("amqp://localhost:5672");
        let topology = ExchangeTopology::from_prefix("test.events");
        let result = manager.declare_topology(&topology).await;
        assert!(result.is_ok());
    }
}
