use crate::{
    core::{
        Config,
        publish::{ChainName, publish_batch, publish_context_and_epoch},
    },
    monitoring::metrics::{EVENT_LISTENING_ERRORS, EVENT_RECEIVED_COUNTER},
};
use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::B256,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::{SolEvent, SolEventInterface},
};
use anyhow::anyhow;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    types::{KMS_CONTEXT_COUNTER_BASE, ProtocolEvent, db::EventType},
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::{
        AbortCrsgen, AbortKeygen, CrsgenRequest, KMSGenerationEvents, KeygenRequest,
        PrepKeygenRequest,
    },
    protocol_config::ProtocolConfig::{
        self, KmsContextDestroyed, KmsEpochDestroyed, NewKmsContext, NewKmsEpoch,
        ProtocolConfigEvents, ProtocolConfigInstance,
    },
};
use sqlx::{Pool, Postgres};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, info_span, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Ethereum-side events signatures polled by `EthereumListener`.
/// Used to build the multi-address `eth_getLogs` filter.
const ETHEREUM_EVENT_SIGNATURES: [B256; 9] = [
    PrepKeygenRequest::SIGNATURE_HASH,
    KeygenRequest::SIGNATURE_HASH,
    CrsgenRequest::SIGNATURE_HASH,
    AbortKeygen::SIGNATURE_HASH,
    AbortCrsgen::SIGNATURE_HASH,
    NewKmsContext::SIGNATURE_HASH,
    NewKmsEpoch::SIGNATURE_HASH,
    KmsContextDestroyed::SIGNATURE_HASH,
    KmsEpochDestroyed::SIGNATURE_HASH,
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

    /// The `ProtocolConfig` contract instance on Ethereum.
    protocol_config_contract: ProtocolConfigInstance<P>,
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
        let protocol_config_contract =
            ProtocolConfig::new(config.protocol_config_contract.address, provider.clone());
        Self {
            db_pool,
            provider,
            config: config.clone(),
            cancel_token,
            protocol_config_contract,
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

    /// Stores the active KMS context and epoch found on-chain in the database.
    ///
    /// Only the currently active pair is seeded. Other valid pairs are validated on demand by the
    /// kms-worker against `ProtocolConfig` and cached back in the `kms_context` table.
    pub async fn store_on_chain_context(&self) -> anyhow::Result<()> {
        let active = self
            .protocol_config_contract
            .getCurrentKmsContextAndEpoch()
            .call()
            .await?;

        publish_context_and_epoch(&self.db_pool, active.contextId, active.epochId).await
    }

    /// Polling loop to listen to [`KMSGeneration`] and [`ProtocolConfig`] events on Ethereum.
    async fn run_poll_loop(&self) -> anyhow::Result<()> {
        let base_filter = Filter::new()
            .address(vec![
                self.config.kms_generation_contract.address,
                self.config.protocol_config_contract.address,
            ])
            .event_signature(ETHEREUM_EVENT_SIGNATURES.to_vec());

        let mut from_block = match self.config.kms_operation_from_block_number {
            Some(from_block) => {
                info!("Found configured from_block_number ({from_block}) for Ethereum polling");
                from_block
            }
            None => self.fetch_start_block().await?,
        };
        info!("Started Ethereum polling from block {from_block}");

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
                        .with_label_values(&["ethereum"])
                        .inc();
                    consecutive_errors = consecutive_errors.saturating_add(1);
                    warn!("Ethereum listening error: {e} ({consecutive_errors}/{max_errors})");
                    if consecutive_errors >= max_errors {
                        anyhow::bail!("Too many consecutive errors for Ethereum polling");
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
        let events = self.prepare_events(logs)?;
        publish_batch(&self.db_pool, events, ChainName::Ethereum, to_block).await?;

        Ok((to_block.saturating_add(1), to_block < finalized_block))
    }

    /// Decodes logs into the [`ProtocolEvent`]s to store in DB, for the kms-worker to forward
    /// them to the KMS Core.
    fn prepare_events(&self, logs: Vec<Log>) -> anyhow::Result<Vec<ProtocolEvent>> {
        let kms_generation_address = self.config.kms_generation_contract.address;
        let protocol_config_address = self.config.protocol_config_contract.address;

        let mut events = Vec::with_capacity(logs.len());
        for log in logs {
            let log_address = log.inner.address;
            let event_kind = if log_address == kms_generation_address {
                KMSGenerationEvents::decode_log(&log.inner)
                    .map_err(|e| anyhow!("Failed to decode KMSGeneration event: {e}"))?
                    .data
                    .try_into()?
            } else if log_address == protocol_config_address {
                let protocol_config_event = ProtocolConfigEvents::decode_log(&log.inner)
                    .map_err(|e| anyhow!("Failed to decode ProtocolConfig event: {e}"))?
                    .data;

                // Skip the genesis / re-init `NewKmsContext` event. That context has no
                // predecessor to reshare from, so there is nothing for the connector to do.
                // Drop it here rather than storing an event the kms-worker could never process.
                if let ProtocolConfigEvents::NewKmsContext(e) = &protocol_config_event
                    && e.previousContextId == KMS_CONTEXT_COUNTER_BASE
                {
                    info!(
                        "Skipping genesis/re-init NewKmsContext #{} (sentinel previousContextId)",
                        e.contextId,
                    );
                    continue;
                }

                protocol_config_event.try_into()?
            } else {
                warn!("Skipping log from unexpected address: {log_address}");
                continue;
            };
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
    async fn fetch_start_block(&self) -> anyhow::Result<u64> {
        let chain = ChainName::Ethereum.as_str();
        info!("Fetching last block polled from DB for chain {chain}...");
        let last_block_polled = sqlx::query_scalar!(
            "SELECT block_number FROM last_block_polled_by_chain WHERE chain_name = $1",
            chain,
        )
        .fetch_one(&self.db_pool)
        .await?;

        match last_block_polled {
            Some(block_i64) => {
                let block = u64::try_from(block_i64).expect("block_number should be a valid u64");
                Ok(block.checked_add(1).expect("block < u64::MAX"))
            }
            None => {
                info!("No block polled yet. Listening from finalized block number instead...");
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Address, U256},
        providers::{
            Identity, ProviderBuilder, RootProvider,
            fillers::{
                BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            },
            mock::Asserter,
        },
        sol_types::{SolEvent, SolValue},
    };
    use connector_utils::{
        tests::setup::{TestInstance, TestInstanceBuilder},
        types::ProtocolEventKind,
    };
    use std::time::Duration;

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_store_active_context_and_epoch() {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();
        let context_id = U256::from(79_u64);
        let epoch_id = U256::from(3_u64);

        let asserter = Asserter::new();
        // `getActiveKmsContextAndEpoch()` returns the tuple `(contextId, epochId)`.
        asserter.push_success(&(context_id, epoch_id).abi_encode_sequence());

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

        let context_valid: bool = sqlx::query_scalar!(
            "SELECT is_valid FROM kms_context WHERE id = $1",
            context_id.as_le_slice()
        )
        .fetch_one(test_instance.db())
        .await
        .unwrap();
        assert!(context_valid);

        let epoch_row = sqlx::query!(
            "SELECT context_id, is_valid FROM kms_epoch WHERE id = $1",
            epoch_id.as_le_slice()
        )
        .fetch_one(test_instance.db())
        .await
        .unwrap();
        assert!(epoch_row.is_valid);
        assert_eq!(epoch_row.context_id, Some(context_id.to_le_bytes_vec()));
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_prepare_events_skips_genesis_new_kms_context() {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();

        // Distinct addresses so logs are routed to the ProtocolConfig decode branch.
        let protocol_config_address = Address::from([0x11; 20]);
        let kms_generation_address = Address::from([0x22; 20]);
        let mut config = Config::default();
        config.protocol_config_contract.address = protocol_config_address;
        config.kms_generation_contract.address = kms_generation_address;

        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new());
        let listener = EthereumListener::new(
            test_instance.db().clone(),
            mock_provider,
            &config,
            CancellationToken::new(),
        );

        let genesis = NewKmsContext {
            contextId: KMS_CONTEXT_COUNTER_BASE + U256::ONE,
            previousContextId: KMS_CONTEXT_COUNTER_BASE,
            ..Default::default()
        };
        let normal = NewKmsContext {
            contextId: KMS_CONTEXT_COUNTER_BASE + U256::from(2),
            previousContextId: KMS_CONTEXT_COUNTER_BASE + U256::ONE,
            ..Default::default()
        };
        let make_log = |event: &NewKmsContext| Log {
            inner: alloy::primitives::Log {
                address: protocol_config_address,
                data: event.encode_log_data(),
            },
            ..Default::default()
        };

        let events = listener
            .prepare_events(vec![make_log(&genesis), make_log(&normal)])
            .unwrap();

        assert_eq!(events.len(), 1, "genesis NewKmsContext should be skipped");
        match &events[0].kind {
            ProtocolEventKind::NewKmsContext(e) => assert_eq!(
                e.previousContextId,
                KMS_CONTEXT_COUNTER_BASE + U256::ONE,
                "only the non-genesis context switch should be kept"
            ),
            other => panic!("unexpected event kind: {other:?}"),
        }
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
