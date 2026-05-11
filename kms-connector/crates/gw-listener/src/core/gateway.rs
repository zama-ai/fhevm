use crate::{
    core::{Config, publish::publish_batch},
    monitoring::metrics::{EVENT_LISTENING_ERRORS, EVENT_RECEIVED_COUNTER},
};
use alloy::{
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
use fhevm_gateway_bindings::decryption::Decryption::DecryptionEvents;
use sqlx::{Pool, Postgres};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, info_span, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

const DECRYPTION_EVENT_TYPES: [EventType; 2] = [
    EventType::PublicDecryptionRequest,
    EventType::UserDecryptionRequest,
];

/// Struct monitoring and storing Gateway's decryption events.
pub struct GatewayListener<P>
where
    P: Provider,
{
    /// The database pool for storing Gateway's events.
    db_pool: Pool<Postgres>,

    /// The Gateway RPC Provider.
    provider: P,

    /// The configuration of the `GatewayListener`.
    config: Config,

    /// The cancellation token to handle the graceful shutdown of the listener.
    cancel_token: CancellationToken,
}

impl<P> GatewayListener<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Creates a new `GatewayListener` instance.
    pub fn new(
        db_pool: Pool<Postgres>,
        provider: P,
        config: &Config,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            db_pool,
            provider,
            config: config.clone(),
            cancel_token,
        }
    }

    /// Starts the `GatewayListener`.
    ///
    /// Polls for Decryption events on the Gateway chain.
    pub async fn start(self) {
        select! {
            biased;
            _ = self.cancel_token.cancelled() => info!("Decryption polling cancelled..."),
            result = self.run_poll_loop() => if let Err(e) = result {
                error!("Decryption polling failed: {e}");
            }
        }
        self.cancel_token.cancel();
        info!("GatewayListener stopped successfully!");
    }

    /// Polling loop to listen to [`Decryption`] contract events.
    async fn run_poll_loop(&self) -> anyhow::Result<()> {
        let contract_address = self.config.decryption_contract.address;
        let poll_interval = self.config.decryption_polling;
        let from_block_config = self.config.decryption_from_block_number;
        let event_types = DECRYPTION_EVENT_TYPES.as_slice();

        let event_signatures = event_types
            .iter()
            .map(|e| e.signature_hash())
            .collect::<Vec<_>>();
        let base_filter = Filter::new()
            .address(contract_address)
            .event_signature(event_signatures);

        let mut from_block = self.get_start_block(from_block_config, event_types).await?;
<<<<<<< HEAD
        info!("Started {contract} polling from block {from_block}");
=======
        info!("Started Decryption polling from block {from_block}");
>>>>>>> release/0.13.x

        let mut ticker = tokio::time::interval(poll_interval);
        let max_errors = self.config.max_consecutive_polling_errors;
        let mut consecutive_errors: usize = 0;
        loop {
            ticker.tick().await;
            match self
<<<<<<< HEAD
                .fetch_and_publish(contract, base_filter.clone(), event_types, from_block)
=======
                .fetch_and_publish(base_filter.clone(), event_types, from_block)
>>>>>>> release/0.13.x
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
<<<<<<< HEAD
                        .with_label_values(&[contract.to_string().to_lowercase()])
                        .inc();
                    consecutive_errors = consecutive_errors.saturating_add(1);
                    warn!("{contract} listening error: {e} ({consecutive_errors}/{max_errors})");
                    if consecutive_errors >= max_errors {
                        anyhow::bail!("Too many consecutive errors for {contract}");
=======
                        .with_label_values(&["decryption"])
                        .inc();
                    consecutive_errors = consecutive_errors.saturating_add(1);
                    warn!("Decryption listening error: {e} ({consecutive_errors}/{max_errors})");
                    if consecutive_errors >= max_errors {
                        anyhow::bail!("Too many consecutive errors for Decryption");
>>>>>>> release/0.13.x
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
<<<<<<< HEAD
        contract: MonitoredContract,
=======
>>>>>>> release/0.13.x
        base_filter: Filter,
        event_types: &[EventType],
        from_block: u64,
    ) -> anyhow::Result<(u64, bool)> {
        let current_block = self.provider.get_block_number().await?;

        if from_block > current_block {
            return Ok((from_block, false));
        }

        let to_block = std::cmp::min(
            from_block.saturating_add(self.config.get_logs_batch_size.saturating_sub(1)),
            current_block,
        );

        let filter = base_filter.from_block(from_block).to_block(to_block);

        let logs = self.provider.get_logs(&filter).await?;
<<<<<<< HEAD
        let events = Self::prepare_events(contract, logs)?;
        publish_batch(&self.db_pool, events, event_types, to_block).await?;

        Ok((to_block.saturating_add(1), to_block < current_block))
    }

    /// Decodes a log into a `ProtocolEventKind`.
    fn decode_log(contract: MonitoredContract, log: &Log) -> anyhow::Result<ProtocolEventKind> {
        match contract {
            MonitoredContract::Decryption => {
                let event = DecryptionEvents::decode_log(&log.inner)
                    .map_err(|e| anyhow!("Failed to decode Decryption event: {e}"))?;
                match event.data {
                    DecryptionEvents::PublicDecryptionRequest(e) => Ok(e.into()),
                    DecryptionEvents::UserDecryptionRequest(e) => Ok(e.into()),
                    _ => Err(anyhow!("Unexpected Decryption event: {log:?}")),
                }
            }
            MonitoredContract::KmsGeneration => {
                let event = KMSGenerationEvents::decode_log(&log.inner)
                    .map_err(|e| anyhow!("Failed to decode KMSGeneration event: {e}"))?;
                match event.data {
                    KMSGenerationEvents::PrepKeygenRequest(e) => Ok(e.into()),
                    KMSGenerationEvents::KeygenRequest(e) => Ok(e.into()),
                    KMSGenerationEvents::CrsgenRequest(e) => Ok(e.into()),
                    KMSGenerationEvents::PRSSInit(e) => Ok(e.into()),
                    KMSGenerationEvents::KeyReshareSameSet(e) => Ok(e.into()),
                    _ => Err(anyhow!("Unexpected KMSGeneration event: {log:?}")),
                }
            }
        }
    }

    /// Decodes logs and prepares `ProtocolEvent` structs with OTLP context and metrics.
    fn prepare_events(
        contract: MonitoredContract,
        logs: Vec<Log>,
    ) -> anyhow::Result<Vec<ProtocolEvent>> {
        let mut events = Vec::with_capacity(logs.len());
        for log in logs {
            let event_kind = Self::decode_log(contract, &log)?;
            EVENT_RECEIVED_COUNTER
                .with_label_values(&[EventType::from(&event_kind).as_str()])
                .inc();

            let span = info_span!("handle_gateway_event", event = %event_kind);
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
            info!("Found configured from_block_number ({from_block}) for polling");
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
            None => Ok(self.provider.get_block_number().await?),
        }
=======
        let events = Self::prepare_events(logs)?;
        publish_batch(&self.db_pool, events, event_types, to_block).await?;

        Ok((to_block.saturating_add(1), to_block < current_block))
>>>>>>> release/0.13.x
    }

    /// Decodes logs and prepares `ProtocolEvent` structs with OTLP context and metrics.
    fn prepare_events(logs: Vec<Log>) -> anyhow::Result<Vec<ProtocolEvent>> {
        let mut events = Vec::with_capacity(logs.len());
        for log in logs {
            let event_kind = DecryptionEvents::decode_log(&log.inner)
                .map_err(|e| anyhow!("Failed to decode Decryption event: {e}"))?
                .data
                .try_into()?;
            EVENT_RECEIVED_COUNTER
                .with_label_values(&[EventType::from(&event_kind).as_str()])
                .inc();

            let span = info_span!("handle_gateway_event", event = %event_kind);
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
            info!("Found configured from_block_number ({from_block}) for polling");
            return Ok(from_block);
        }

        info!("Fetching min last block polled from DB for {event_types:?}...");
        let min_block = sqlx::query_scalar!(
            "SELECT MIN(block_number) FROM last_block_polled WHERE event_type = ANY($1::event_type[])",
            event_types as &[EventType],
        )
        .fetch_one(&self.db_pool)
        .await?;

        match min_block {
            Some(last_block_polled) => Ok(last_block_polled as u64 + 1),
            None => {
                info!("No block polled yet. Listening from latest block number instead...");
                Ok(self.provider.get_block_number().await?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::{
        Identity, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        mock::Asserter,
    };
    use alloy::rpc::json_rpc::ErrorPayload;
    use connector_utils::tests::setup::{TestInstance, TestInstanceBuilder};
    use std::time::Duration;

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_consecutive_get_logs_error_stops_listener() {
        let (_test_instance, asserter, gw_listener) = test_setup(None).await;

        // Initial get_block_number succeeds
        asserter.push_success(&100_u64);

        for _ in 0..MAX_CONSECUTIVE_POLLING_ERRORS {
            // Loop get_block_number succeeds
            asserter.push_success(&101_u64);

            // get_logs fails
            asserter.push_failure(ErrorPayload {
                code: -32000,
                message: "get logs error".into(),
                data: None,
            });
        }

<<<<<<< HEAD
        gw_listener.poll_events(MonitoredContract::Decryption).await;
=======
        gw_listener.start().await;
>>>>>>> release/0.13.x
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_listener_ended_by_cancel_token() {
        let (mut test_instance, _asserter, gw_listener) = test_setup(None).await;

        gw_listener.cancel_token.cancel();

        gw_listener.start().await;
        test_instance
            .wait_for_log("GatewayListener stopped successfully")
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
        decryption_from_block_number: Option<u64>,
    ) -> (TestInstance, Asserter, GatewayListener<MockProvider>) {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();

        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        let config = Config {
            decryption_polling: Duration::from_millis(500),
<<<<<<< HEAD
            key_management_polling: Duration::from_millis(500),
            kms_operation_from_block_number,
=======
            decryption_from_block_number,
>>>>>>> release/0.13.x
            max_consecutive_polling_errors: MAX_CONSECUTIVE_POLLING_ERRORS,
            ..Default::default()
        };
        let listener = GatewayListener::new(
            test_instance.db().clone(),
            mock_provider,
            &config,
            CancellationToken::new(),
        );
        (test_instance, asserter, listener)
    }
}
