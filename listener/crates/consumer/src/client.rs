use alloy_primitives::Address;
use async_trait::async_trait;
pub use broker::{AckDecision, Broker, HandlerError};
use broker::{BrokerError, CancellationToken, Consumer, Handler, Message, Topic};
use primitives::event::{BlockPayload, FilterCommand, FilterCommandValidationError};
use primitives::routing;
use primitives::utils::chain_id_to_namespace;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::warn;

pub use crate::error::ConsumerError;

/// Chain & Consumer-scoped client for the consumer library.
///
/// Downstream services instantiate this once with their broker connection and
/// target chain, then call instance methods for watch/unwatch operations.
#[derive(Clone)]
pub struct ListenerConsumer {
    broker: Broker,
    chain_id: u64,
    consumer_id: String,
    pub cancel_token: CancellationToken,
}

impl ListenerConsumer {
    /// Create a new consumer bound to a broker and chain ID.
    pub fn new(broker: &Broker, chain_id: u64, consumer_id: &str) -> Self {
        let consumer_id_trimmed = consumer_id.trim();
        if consumer_id != consumer_id_trimmed {
            warn!(
                "Consumer ID has leading or trailing whitespace, which may cause issues with routing. Consider trimming it before passing to ListenerConsumer::new."
            );
        }
        Self {
            broker: broker.clone(),
            chain_id,
            consumer_id: consumer_id_trimmed.into(),
            cancel_token: CancellationToken::new(),
        }
    }

    /// Cancel via the cancel token passed to new
    /// This is useful for stopping the consumer from another task.
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// Return the chain ID this client publishes into.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Return the consumer ID this client publishes into.
    pub fn consumer_id(&self) -> &str {
        &self.consumer_id
    }

    pub fn create_filter_on_log_address(&self, contract: Address) -> FilterCommand {
        FilterCommand {
            consumer_id: self.consumer_id.clone(),
            from: None,
            to: None,
            log_address: Some(contract),
        }
    }

    /// Publish a filter removal command to the unwatch topic.
    pub async fn unregister_filter(&self, command: &FilterCommand) -> Result<(), ConsumerError> {
        self.publish_filter_command(command, routing::UNWATCH).await
    }

    /// Publish a filter registration command to the watch topic.
    pub async fn register_filter(&self, command: &FilterCommand) -> Result<(), ConsumerError> {
        self.publish_filter_command(command, routing::WATCH).await
    }

    async fn publish_filter_command(
        &self,
        command: &FilterCommand,
        routing_key: &'static str,
    ) -> Result<(), ConsumerError> {
        let mut command = command.clone();
        if command.consumer_id != self.consumer_id {
            return Err(ConsumerError::InconsistentConsumerId(
                command.consumer_id.clone(),
                self.consumer_id.clone(),
            ));
        }
        command.validate()?;

        let namespace = chain_id_to_namespace(self.chain_id);
        let publisher = self.broker.publisher(&namespace).await?;
        publisher.publish(routing_key, &command).await?;
        Ok(())
    }

    pub fn consumer_topic(&self) -> Topic {
        let routing = routing::consumer_new_event_routing(self.consumer_id.clone());
        Topic::new(routing)
    }

    fn broker_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.consumer_topic();
        let cancel = self.cancel_token.clone();
        let builder = self
            .broker
            .consumer(&topic)
            .group(topic.to_string())
            .prefetch(1)
            .max_retries(3)
            .circuit_breaker(3, Duration::from_secs(30))
            .with_cancellation(cancel);

        match &self.broker {
            Broker::Redis { .. } => builder.redis_claim_min_idle(60).redis_claim_interval(1),
            Broker::Amqp { .. } => builder,
        }
        .build()
    }

    /// Publish filters registration command to watch contracts.
    pub async fn register_contracts(&self, contracts: &[Address]) -> Result<(), ConsumerError> {
        if contracts.is_empty() {
            return Err(ConsumerError::InvalidParameter(
                "contracts array cannot be empty".into(),
            ));
        }
        for contract in contracts {
            self.register_filter(&self.create_filter_on_log_address(*contract))
                .await?;
        }
        Ok(())
    }

    /// Publish filters removal command to unwatch contracts.
    pub async fn unregister_contracts(&self, contracts: &[Address]) -> Result<(), ConsumerError> {
        if contracts.is_empty() {
            return Err(ConsumerError::InvalidFilterCommand(
                FilterCommandValidationError::MissingContractAddresses,
            ));
        }
        for contract in contracts {
            self.unregister_filter(&self.create_filter_on_log_address(*contract))
                .await?;
        }
        Ok(())
    }

    /// Ensure the consumer topology is set up in the broker.
    pub async fn ensure_consumer(&self) -> Result<(), BrokerError> {
        // TODO: start core listener
        self.broker_consumer()?.ensure_topology().await
    }

    /// Start consuming messages with the provided handler function.
    ///
    /// The returned future owns an internal clone of the client, so it can be
    /// spawned without forcing the caller to clone `ListenerConsumer` first.
    pub fn consume<F, Fut>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + 'static
    where
        F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
    {
        let client = self.clone();
        async move {
            let consumer = client.broker_consumer()?;
            let handler = ConsumerHandler {
                call: Arc::new(f),
                cancel: client.cancel_token.clone(),
            };
            consumer.run(handler).await?;
            Ok(())
        }
    }
}

struct ConsumerHandler<F> {
    call: Arc<F>,
    cancel: CancellationToken,
}

impl<F> Clone for ConsumerHandler<F> {
    fn clone(&self) -> Self {
        Self {
            call: Arc::clone(&self.call),
            cancel: self.cancel.clone(),
        }
    }
}

#[async_trait]
impl<F, Fut> Handler for ConsumerHandler<F>
where
    F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
{
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let payload: BlockPayload = serde_json::from_slice(&msg.payload)?;
        (self.call)(payload, self.cancel.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use broker::{amqp::RmqPublisher, traits::Publisher};
    use primitives::event::BlockFlow;

    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static TEST_ID: AtomicU64 = AtomicU64::new(0);

    fn unique_name(prefix: &str) -> String {
        format!("{prefix}-{}", TEST_ID.fetch_add(1, Ordering::Relaxed))
    }

    #[tokio::test]
    #[ignore = "requires Docker"]
    async fn test_consumer_happy_path() {
        let broker_url = "amqp://user:pass@localhost:5672";
        let broker = Broker::amqp(broker_url).build().await.unwrap();
        let chain_id = 1;
        let consumer_id = unique_name("copro-1-host-eth");
        let consumer = ListenerConsumer::new(&broker, chain_id, &consumer_id);
        let contracts = vec![Address::ZERO];
        consumer.register_contracts(&contracts).await.unwrap();
        consumer.ensure_consumer().await.unwrap();
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let consumer_task = consumer.consume(|payload, cancel| async move {
            let v = COUNTER.fetch_add(1, Ordering::Relaxed);
            eprintln!("Received payload: {:?} {v}", payload);
            if v + 1 >= 2 {
                cancel.cancel();
                println!("Cancel after receiving 2 payloads");
            }
            Ok(AckDecision::Ack)
        });
        let consumer_run = tokio::spawn(consumer_task);
        eprintln!("Consumer task spawned, waiting for messages or timeout...");
        let routing_key = consumer.consumer_topic();
        let publisher = RmqPublisher::connect(broker_url, "main").await;
        let fake_block = BlockPayload {
            flow: BlockFlow::Live,
            chain_id,
            block_number: 0,
            block_hash: B256::ZERO,
            parent_hash: B256::ZERO,
            timestamp: 0,
            transactions: vec![],
        };
        for _ in 1..=2 {
            publisher
                .publish(&routing_key.to_string(), &fake_block)
                .await
                .unwrap();
        }
        let with_timeout =
            tokio::time::timeout(std::time::Duration::from_secs(5), consumer_run).await;
        eprintln!("Consumer task completed or timed out: {with_timeout:?}");
        consumer.cancel();
        consumer.unregister_contracts(&contracts).await.unwrap();
        assert!(
            with_timeout.is_ok(),
            "Consumer should have cancel and not timeout"
        );
        assert_eq!(COUNTER.fetch_add(0, Ordering::Relaxed), 2);
    }
}
