use crate::{
    core::{Config, publish::publish_batch},
    monitoring::metrics::{EVENT_LISTENING_ERRORS, EVENT_RECEIVED_COUNTER},
};
use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEventInterface,
};
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{ProtocolEvent, db::EventType},
};
use fhevm_host_bindings::kms_generation::KMSGeneration::KMSGenerationEvents;
use fhevm_host_bindings::kms_verifier::KMSVerifier::{self, KMSVerifierInstance};
use sqlx::{Pool, Postgres, Row};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, info_span, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::publish::publish_context_id;

const KMS_GENERATION_EVENT_TYPES: [EventType; 3] = [
    EventType::PrepKeygenRequest,
    EventType::KeygenRequest,
    EventType::CrsgenRequest,
];

/// Struct monitoring and storing Ethereum's keygen events.
#[derive(Clone)]
pub struct EthereumListener<P> {
    /// The database pool for storing Ethereum's events.
    db_pool: Pool<Postgres>,

    /// The Ethereum RPC Provider.
    provider: P,

    /// The configuration of the `EthereumListener`.
    config: Config,

    /// The cancellation token to handle the graceful shutdown of the listener.
    cancel_token: CancellationToken,

    /// The `KMSVerifier` contract instance on Ethereum.
    kms_verifier_contract: KMSVerifierInstance<P>,
}

impl<P> EthereumListener<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Creates a new `EthereumListener` instance.
    pub fn new(
        db_pool: Pool<Postgres>,
        provider: P,
        config: &Config,
        cancel_token: CancellationToken,
    ) -> Self {
        let kms_verifier_contract = KMSVerifier::new(config.kms_verifier_address, provider.clone());
        Self {
            db_pool,
            provider,
            config: config.clone(),
            cancel_token,
            kms_verifier_contract,
        }
    }

    /// Starts the `EthereumListener`.
    pub async fn start(self) {
        select! {
            biased;
            _ = self.cancel_token.cancelled() => info!("KMSGeneration polling cancelled..."),
            result = self.run_poll_loop() => if let Err(e) = result {
                error!("KMSGeneration polling failed: {e}");
            }
        }
        self.cancel_token.cancel();
        info!("EthereumListener stopped successfully!");
    }

    /// Stores the current context ID found on-chain in the database.
    pub async fn store_on_chain_context(&self) -> anyhow::Result<()> {
        let current_context_id = self
            .kms_verifier_contract
            .getCurrentKmsContextId()
            .call()
            .await?;

        publish_context_id(&self.db_pool, current_context_id).await
    }

    /// Polling loop to listen to [`KMSGeneration`] events on Ethereum.
    async fn run_poll_loop(&self) -> anyhow::Result<()> {
        let event_signatures = KMS_GENERATION_EVENT_TYPES
            .iter()
            .map(|e| e.signature_hash())
            .collect::<Vec<_>>();
        let base_filter = Filter::new()
            .address(self.config.kms_generation_contract.address)
            .event_signature(event_signatures);

        let mut from_block = self
            .get_start_block(
                self.config.kms_operation_from_block_number,
                &KMS_GENERATION_EVENT_TYPES,
            )
            .await?;
        info!("Started KMSGeneration polling from block {from_block}");

        let mut ticker = tokio::time::interval(self.config.key_management_polling);
        let max_errors = self.config.max_consecutive_polling_errors;
        let mut consecutive_errors: usize = 0;
        loop {
            ticker.tick().await;
            match self
                .fetch_and_publish(base_filter.clone(), from_block)
                .await
            {
                Ok((new_from_block, has_more)) => {
                    consecutive_errors = 0;
                    from_block = new_from_block;
                    if has_more {
                        ticker.reset_immediately();
                    }
                }
                Err(e) => {
                    EVENT_LISTENING_ERRORS
                        .with_label_values(&["kms_generation"])
                        .inc();
                    consecutive_errors = consecutive_errors.saturating_add(1);
                    warn!("KMSGeneration listening error: {e} ({consecutive_errors}/{max_errors})");
                    if consecutive_errors >= max_errors {
                        anyhow::bail!("Too many consecutive errors for KMSGeneration polling");
                    }
                }
            }
        }
    }

    /// Fetches logs for a block range, decodes them, and publishes them in a single transaction.
    ///
    /// Returns `(new_from_block, has_more_blocks)`.
    async fn fetch_and_publish(
        &self,
        base_filter: Filter,
        from_block: u64,
    ) -> anyhow::Result<(u64, bool)> {
        let finalized_block = self
            .provider
            .get_block_by_number(BlockNumberOrTag::Finalized)
            .await?
            .ok_or_else(|| anyhow!("Finalized block not available"))?
            .header
            .number;

        if from_block > finalized_block {
            return Ok((from_block, false));
        }

        let to_block = std::cmp::min(
            from_block.saturating_add(self.config.get_logs_batch_size.saturating_sub(1)),
            finalized_block,
        );

        let filter = base_filter.from_block(from_block).to_block(to_block);

        let logs = self.provider.get_logs(&filter).await?;
        let events = Self::prepare_events(logs)?;
        publish_batch(&self.db_pool, events, &KMS_GENERATION_EVENT_TYPES, to_block).await?;

        Ok((to_block.saturating_add(1), to_block < finalized_block))
    }

    /// Decodes logs and prepares `ProtocolEvent` structs with OTLP context and metrics.
    fn prepare_events(logs: Vec<Log>) -> anyhow::Result<Vec<ProtocolEvent>> {
        let mut events = Vec::with_capacity(logs.len());
        for log in logs {
            let event_kind = KMSGenerationEvents::decode_log(&log.inner)
                .map_err(|e| anyhow!("Failed to decode KMSGeneration event: {e}"))?
                .data
                .try_into()?;
            EVENT_RECEIVED_COUNTER
                .with_label_values(&[EventType::from(&event_kind).as_str()])
                .inc();

            let span = info_span!("handle_ethereum_event", event = %event_kind);
            let otlp_ctx = PropagationContext::inject(&span.context());
            events.push(ProtocolEvent::new(
                event_kind,
                log.transaction_hash,
                otlp_ctx,
            ));
        }
        Ok(events)
    }

    /// Determines the block to start event listening from.
    async fn get_start_block(
        &self,
        from_block_config: Option<u64>,
        event_types: &[EventType],
    ) -> anyhow::Result<u64> {
        if let Some(from_block) = from_block_config {
            info!("Found configured from_block_number ({from_block}) for KMSGeneration polling");
            return Ok(from_block);
        }

        let mut min_last_processed_block: Option<u64> = None;
        for &event_type in event_types {
            if let Some(last) = self.get_last_block_polled_from_db(event_type).await? {
                min_last_processed_block = match min_last_processed_block {
                    Some(current) => Some(std::cmp::min(current, last)),
                    None => Some(last),
                };
            }
        }

        match min_last_processed_block {
            Some(last_block_polled) => Ok(last_block_polled.saturating_add(1)),
            None => {
                let finalized = self
                    .provider
                    .get_block_by_number(BlockNumberOrTag::Finalized)
                    .await?
                    .ok_or_else(|| anyhow!("Finalized block not available"))?
                    .header
                    .number;
                Ok(finalized)
            }
        }
    }

    async fn get_last_block_polled_from_db(
        &self,
        event_type: EventType,
    ) -> anyhow::Result<Option<u64>> {
        info!("Fetching last block polled from DB for {event_type}...");
        let query_result =
            sqlx::query("SELECT block_number FROM last_block_polled WHERE event_type = $1")
                .bind(event_type)
                .fetch_one(&self.db_pool)
                .await?
                .try_get::<Option<i64>, _>("block_number")?;

        let Some(block_number) = query_result else {
            info!("No block number stored in DB yet for {event_type}");
            return Ok(None);
        };
        Ok(Some(block_number as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::U256,
        providers::{
            Identity, ProviderBuilder, RootProvider,
            fillers::{
                BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            },
            mock::Asserter,
        },
        sol_types::SolValue,
    };
    use connector_utils::tests::setup::{TestInstance, TestInstanceBuilder};
    use std::time::Duration;

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_store_current_context_id() {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();
        let context_id = U256::from(79_u64);

        let asserter = Asserter::new();
        asserter.push_success(&context_id.abi_encode());

        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);

        let config = Config::default();
        let listener = EthereumListener::new(
            test_instance.db().clone(),
            mock_provider,
            &config,
            CancellationToken::new(),
        );

        listener.store_on_chain_context().await.unwrap();

        let row = sqlx::query("SELECT is_valid FROM kms_context WHERE id = $1")
            .bind(context_id.as_le_slice())
            .fetch_one(test_instance.db())
            .await
            .unwrap();

        assert!(row.try_get::<bool, _>("is_valid").unwrap());
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_consecutive_get_logs_error_stops_listener() {
        let (_test_instance, asserter, eth_listener) = test_setup(None).await;

        // Initial get_block (finalized) succeeds — returns a full block response
        push_finalized_block(&asserter, 100);

        for _ in 0..MAX_CONSECUTIVE_POLLING_ERRORS {
            // Loop get_block (finalized) succeeds
            push_finalized_block(&asserter, 101);

            // get_logs fails
            asserter.push_failure(alloy::rpc::json_rpc::ErrorPayload {
                code: -32000,
                message: "get logs error".into(),
                data: None,
            });
        }

        eth_listener.start().await;
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_listener_ended_by_cancel_token() {
        let (mut test_instance, _asserter, eth_listener) = test_setup(None).await;

        eth_listener.cancel_token.cancel();

        eth_listener.start().await;
        test_instance
            .wait_for_log("EthereumListener stopped successfully")
            .await;
    }

    type MockProvider = FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >;

    const MAX_CONSECUTIVE_POLLING_ERRORS: usize = 2;

    async fn test_setup(
        kms_operation_from_block_number: Option<u64>,
    ) -> (TestInstance, Asserter, EthereumListener<MockProvider>) {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();

        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        let config = Config {
            key_management_polling: Duration::from_millis(500),
            kms_operation_from_block_number,
            max_consecutive_polling_errors: MAX_CONSECUTIVE_POLLING_ERRORS,
            ..Default::default()
        };
        let listener = EthereumListener::new(
            test_instance.db().clone(),
            mock_provider,
            &config,
            CancellationToken::new(),
        );
        (test_instance, asserter, listener)
    }

    /// Pushes a mock finalized block response onto the asserter.
    fn push_finalized_block(asserter: &Asserter, block_number: u64) {
        let block = alloy::rpc::types::Block::<alloy::consensus::TxEnvelope> {
            header: alloy::rpc::types::Header {
                inner: alloy::consensus::Header {
                    number: block_number,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        asserter.push_success(&block);
    }
}
