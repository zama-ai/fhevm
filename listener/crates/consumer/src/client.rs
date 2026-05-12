use alloy_primitives::Address;
use async_trait::async_trait;
pub use broker::{AckDecision, Broker, HandlerError};
use broker::{BrokerError, CancellationToken, Consumer, Handler, Message, Topic};
use primitives::event::{
    BlockPayload, CatchupPayload, FilterCommand, FilterCommandValidationError,
};
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
///
/// # Cancellation
///
/// Three tokens cooperate:
///
/// - [`cancel_token`](Self::cancel_token) — parent. Cancelling it stops both
///   the live and catchup flows.
/// - `live_cancel` — child of `cancel_token`. Wired into the live consumer
///   and the live handler. Cancel via [`cancel_live`](Self::cancel_live) to
///   stop only the live flow.
/// - `catchup_cancel` — child of `cancel_token`. Wired into the catchup
///   consumer and the catchup handler. Cancel via
///   [`cancel_catchup`](Self::cancel_catchup) to stop only catchup
///   (typical use: stop the bounded backfill once it has drained while the
///   live stream keeps running).
///
/// Cancelling a child does *not* propagate up — the parent and the sibling
/// keep going. Cancelling the parent cancels every child.
#[derive(Clone)]
pub struct ListenerConsumer {
    broker: Broker,
    chain_id: u64,
    consumer_id: String,
    /// Parent cancellation token. Cancelling this stops both flows.
    pub cancel_token: CancellationToken,
    /// Child token: cancels only the live flow.
    live_cancel: CancellationToken,
    /// Child token: cancels only the catchup flow.
    catchup_cancel: CancellationToken,
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
        let cancel_token = CancellationToken::new();
        let live_cancel = cancel_token.child_token();
        let catchup_cancel = cancel_token.child_token();
        Self {
            broker: broker.clone(),
            chain_id,
            consumer_id: consumer_id_trimmed.into(),
            cancel_token,
            live_cancel,
            catchup_cancel,
        }
    }

    /// Cancel both the live and catchup flows.
    ///
    /// Equivalent to cancelling [`cancel_token`](Self::cancel_token) directly.
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// Cancel only the live flow. The catchup flow keeps running.
    pub fn cancel_live(&self) {
        self.live_cancel.cancel();
    }

    /// Cancel only the catchup flow. The live flow keeps running.
    pub fn cancel_catchup(&self) {
        self.catchup_cancel.cancel();
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

    /// Request a historical catch-up over `[block_start, block_end]` (inclusive).
    ///
    /// Publishes a [`CatchupPayload`] to the chain-namespaced
    /// `routing::CATCHUP` control plane. The listener fans the range out to
    /// its `range-catchup` workers; the resulting events are delivered on
    /// `{consumer_id}.catchup-event` and consumed via
    /// [`consume_catchup`](Self::consume_catchup).
    pub async fn request_catchup(
        &self,
        block_start: u64,
        block_end: u64,
    ) -> Result<(), ConsumerError> {
        let mut payload = CatchupPayload {
            consumer_id: self.consumer_id.clone(),
            block_start,
            block_end,
        };
        payload.validate()?;
        let namespace = chain_id_to_namespace(self.chain_id);
        let publisher = self.broker.publisher(&namespace).await?;
        publisher.publish(routing::CATCHUP, &payload).await?;
        Ok(())
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

    pub fn catchup_consumer_topic(&self) -> Topic {
        let routing = routing::consumer_catchup_event_routing(self.consumer_id.clone());
        Topic::new(routing)
    }

    fn broker_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.consumer_topic();
        let cancel = self.live_cancel.clone();
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

    fn broker_catchup_consumer(&self) -> Result<Consumer, BrokerError> {
        let topic = self.catchup_consumer_topic();
        let cancel = self.catchup_cancel.clone();
        self.broker
            .consumer(&topic)
            .group(topic.to_string())
            .prefetch(10)
            .with_cancellation(cancel)
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

    /// Ensure the catchup consumer topology is set up in the broker.
    pub async fn ensure_catchup_consumer(&self) -> Result<(), BrokerError> {
        self.broker_catchup_consumer()?.ensure_topology().await
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
                cancel: client.live_cancel.clone(),
            };
            consumer.run(handler).await?;
            Ok(())
        }
    }

    /// Start consuming catchup messages with the provided handler function.
    ///
    /// Same shape and ownership as [`consume`](Self::consume); only differs
    /// by subscribing to `{consumer_id}.catchup-event` instead of
    /// `{consumer_id}.new-event`.
    pub fn consume_catchup<F, Fut>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send + 'static
    where
        F: Fn(BlockPayload, CancellationToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<AckDecision, HandlerError>> + Send + 'static,
    {
        let client = self.clone();
        async move {
            let consumer = client.broker_catchup_consumer()?;
            let handler = ConsumerHandler {
                call: Arc::new(f),
                cancel: client.catchup_cancel.clone(),
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

    #[tokio::test]
    #[ignore = "requires Docker"]
    async fn test_consumer_catchup_happy_path() {
        let broker_url = "amqp://user:pass@localhost:5672";
        let broker = Broker::amqp(broker_url).build().await.unwrap();
        let chain_id = 1;
        let consumer_id = unique_name("copro-1-host-eth-catchup");
        let consumer = ListenerConsumer::new(&broker, chain_id, &consumer_id);
        consumer.ensure_catchup_consumer().await.unwrap();
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let consumer_task = consumer.consume_catchup(|payload, cancel| async move {
            let v = COUNTER.fetch_add(1, Ordering::Relaxed);
            eprintln!("Received catchup payload: {:?} {v}", payload);
            if v + 1 >= 2 {
                cancel.cancel();
                println!("Cancel after receiving 2 catchup payloads");
            }
            Ok(AckDecision::Ack)
        });
        let consumer_run = tokio::spawn(consumer_task);
        eprintln!("Catchup consumer task spawned, waiting for messages or timeout...");
        let routing_key = consumer.catchup_consumer_topic();
        assert_eq!(
            routing_key.to_string(),
            format!("{consumer_id}.catchup-event"),
            "catchup_consumer_topic must derive from consumer_catchup_event_routing"
        );
        let publisher = RmqPublisher::connect(broker_url, "main").await;
        let fake_block = BlockPayload {
            flow: BlockFlow::Catchup,
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
        eprintln!("Catchup consumer task completed or timed out: {with_timeout:?}");
        consumer.cancel();
        assert!(
            with_timeout.is_ok(),
            "Catchup consumer should have cancel and not timeout"
        );
        assert_eq!(COUNTER.fetch_add(0, Ordering::Relaxed), 2);
    }

    #[test]
    fn catchup_consumer_topic_uses_catchup_event_routing() {
        let consumer_id = "copro-1-host-eth";
        let expected = format!("{consumer_id}.{}", routing::CATCHUP_EVENT);
        assert_eq!(
            routing::consumer_catchup_event_routing(consumer_id.into()),
            expected,
        );
        assert_ne!(
            routing::consumer_catchup_event_routing(consumer_id.into()),
            routing::consumer_new_event_routing(consumer_id.into()),
            "catchup and new-event routings must not collide",
        );
    }

    /// Locks in the parent/child cancellation contract that `ListenerConsumer`
    /// relies on. If `tokio_util` ever changes child-token semantics, this
    /// test fails before any consumer-lib regression reaches a user.
    #[test]
    fn parent_child_cancellation_semantics() {
        let parent = CancellationToken::new();
        let live = parent.child_token();
        let catchup = parent.child_token();

        // Cancelling a child must not propagate to siblings or to the parent.
        live.cancel();
        assert!(live.is_cancelled());
        assert!(!catchup.is_cancelled(), "live cancel must not stop catchup");
        assert!(!parent.is_cancelled(), "child cancel must not stop parent");

        // Cancelling the parent must cascade to every remaining child.
        parent.cancel();
        assert!(parent.is_cancelled());
        assert!(catchup.is_cancelled(), "parent cancel must stop catchup");
    }
}
