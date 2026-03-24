use crate::{
    core::{Config, publish::update_last_block_polled, publish_event},
    monitoring::metrics::EVENT_RECEIVED_COUNTER,
};
use alloy::{
    network::Ethereum,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEventInterface,
};
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tasks::spawn_with_limit,
    types::{GatewayEvent, GatewayEventKind, db::EventType},
};
use fhevm_gateway_bindings::{
    decryption::Decryption::DecryptionEvents, kms_generation::KMSGeneration::KMSGenerationEvents,
};
use sqlx::{Pool, Postgres, Row};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, trace, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

const DECRYPTION_EVENT_TYPES: [EventType; 2] = [
    EventType::PublicDecryptionRequest,
    EventType::UserDecryptionRequest,
];

const KMS_GENERATION_EVENT_TYPES: [EventType; 5] = [
    EventType::PrepKeygenRequest,
    EventType::KeygenRequest,
    EventType::CrsgenRequest,
    EventType::PrssInit,
    EventType::KeyReshareSameSet,
];

/// Identifies which contract is being polled.
#[derive(Clone, Copy)]
enum MonitoredContract {
    Decryption,
    KmsGeneration,
}

impl std::fmt::Display for MonitoredContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitoredContract::Decryption => write!(f, "Decryption"),
            MonitoredContract::KmsGeneration => write!(f, "KmsGeneration"),
        }
    }
}

/// Struct monitoring and storing Gateway's events.
#[derive(Clone)]
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
    /// Spawns two polling tasks: one for Decryption events and one for KMSGeneration events.
    pub async fn start(self) {
        let mut tasks = JoinSet::new();
        tasks.spawn(self.clone().poll_events(MonitoredContract::Decryption));
        tasks.spawn(self.poll_events(MonitoredContract::KmsGeneration));

        while let Some(res) = tasks.join_next().await {
            if let Err(e) = res {
                error!("{e}");
            }
        }
        info!("GatewayListener stopped successfully!");
    }

    /// Polls a contract group for events using `eth_getLogs`.
    ///
    /// Cancels all other tasks on failure.
    async fn poll_events(self, contract: MonitoredContract) {
        if let Err(e) = self.run_poll_loop(contract).await {
            error!("{contract} polling failed: {e}");
        }
        self.cancel_token.cancel();
    }

    /// Shared `eth_getLogs` polling loop.
    async fn run_poll_loop(&self, contract: MonitoredContract) -> anyhow::Result<()> {
        let (contract_address, poll_interval, from_block_config, event_types) = match contract {
            MonitoredContract::Decryption => (
                self.config.decryption_contract.address,
                self.config.decryption_polling,
                self.config.decryption_from_block_number,
                &DECRYPTION_EVENT_TYPES[..],
            ),
            MonitoredContract::KmsGeneration => (
                self.config.kms_generation_contract.address,
                self.config.key_management_polling,
                self.config.kms_operation_from_block_number,
                &KMS_GENERATION_EVENT_TYPES[..],
            ),
        };

        let mut last_processed_block = self.get_start_block(from_block_config, event_types).await?;
        let mut ticker = tokio::time::interval(poll_interval);

        info!(
            "Started {contract} polling from block {}",
            last_processed_block
                .map(|b| (b + 1).to_string())
                .unwrap_or_else(|| "latest".into())
        );

        loop {
            tokio::select! {
                biased;
                _ = self.cancel_token.cancelled() => break info!("{contract} polling cancelled..."),
                _ = ticker.tick() => {
                    let current_block = match self.provider.get_block_number().await {
                        Ok(block_number) => block_number,
                        Err(e) => {
                            warn!("Failed to get block number: {e}");
                            continue;
                        }
                    };

                    let from_block = match last_processed_block {
                        Some(last) if last >= current_block => continue,
                        Some(last) => last + 1,
                        None => current_block,
                    };

                    let to_block = std::cmp::min(
                        from_block.saturating_add(self.config.get_logs_batch_size.saturating_sub(1)),
                        current_block,
                    );

                    let filter = Filter::new()
                        .address(contract_address)
                        .from_block(from_block)
                        .to_block(to_block);

                    let logs = match self.provider.get_logs(&filter).await {
                        Ok(logs) => logs,
                        Err(e) => {
                            // TODO: if too many errors, stop the loop?
                            warn!("Failed to get logs for {contract}: {e}");
                            continue;
                        }
                    };

                    for log in logs {
                        self.process_log(contract, log).await;
                    }

                    last_processed_block = Some(to_block);
                    self.update_block_tracking(event_types, Some(to_block)).await?;

                    if to_block < current_block {
                        ticker.reset_immediately();
                    }
                }
            }
        }
        Ok(())
    }

    /// Decodes a log and dispatches it to the appropriate event handler.
    async fn process_log(&self, group: MonitoredContract, log: Log) {
        let event: GatewayEventKind = match group {
            MonitoredContract::Decryption => match DecryptionEvents::decode_log(&log.inner) {
                Ok(event) => match event.data {
                    DecryptionEvents::PublicDecryptionRequest(e) => e.into(),
                    DecryptionEvents::UserDecryptionRequest(e) => e.into(),
                    _ => return trace!("Ignoring Decryption event: {log:?}"),
                },
                Err(e) => return warn!("Failed to decode Decryption event: {e}"),
            },
            MonitoredContract::KmsGeneration => match KMSGenerationEvents::decode_log(&log.inner) {
                Ok(event) => match event.data {
                    KMSGenerationEvents::PrepKeygenRequest(e) => e.into(),
                    KMSGenerationEvents::KeygenRequest(e) => e.into(),
                    KMSGenerationEvents::CrsgenRequest(e) => e.into(),
                    KMSGenerationEvents::PRSSInit(e) => e.into(),
                    KMSGenerationEvents::KeyReshareSameSet(e) => e.into(),
                    _ => return trace!("Ignoring KMSGeneration event: {log:?}"),
                },
                Err(e) => return warn!("Failed to decode KMSGeneration event: {e}"),
            },
        };

        EVENT_RECEIVED_COUNTER
            .with_label_values(&[EventType::from(&event).as_str()])
            .inc();
        let db = self.db_pool.clone();
        spawn_with_limit(handle_gateway_event(db, event, log)).await;
    }

    /// Updates block tracking for all event types in a group.
    async fn update_block_tracking(
        &self,
        event_types: &[EventType],
        block_number: Option<u64>,
    ) -> anyhow::Result<()> {
        for &event_type in event_types {
            // TODO: update as group?
            update_last_block_polled(&self.db_pool, event_type, block_number).await?;
        }
        Ok(())
    }

    /// Determines the last processed block for a polling group from config or DB.
    ///
    /// Returns the last **completed** block, so that the polling loop starts from `last + 1`.
    async fn get_start_block(
        &self,
        from_block_config: Option<u64>,
        event_types: &[EventType],
    ) -> anyhow::Result<Option<u64>> {
        if let Some(from_block) = from_block_config {
            info!("Found configured from_block_number ({from_block}) for polling");
            // Subtract 1 because the polling loop will do `last + 1` to get the first block.
            return Ok(Some(from_block.saturating_sub(1)));
        }

        let mut min_block: Option<u64> = None;
        for &event_type in event_types {
            if let Some(block) = self.get_last_block_polled_from_db(event_type).await? {
                min_block = Some(match min_block {
                    Some(current_min) => std::cmp::min(current_min, block),
                    None => block,
                });
            }
        }

        // DB stores the last completed block — return as-is (loop adds +1).
        Ok(min_block)
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

/// Main function used to trace a single event handling across all Connector's services.
#[tracing::instrument(skip_all, fields(event = %event_kind))]
async fn handle_gateway_event(db_pool: Pool<Postgres>, event_kind: GatewayEventKind, log: Log) {
    let event = GatewayEvent::new(
        event_kind,
        log.transaction_hash,
        PropagationContext::inject(&tracing::Span::current().context()),
    );
    if let Err(err) = publish_event(&db_pool, event).await {
        error!("Failed to publish event: {err}");
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
    async fn test_get_logs_error_stops_listener() {
        let (_test_instance, asserter, gw_listener) = test_setup(None).await;

        // get_block_number succeeds
        asserter.push_success(&100_u64);
        // get_logs fails
        asserter.push_failure(ErrorPayload {
            code: -32000,
            message: "get logs error".into(),
            data: None,
        });

        gw_listener.poll_events(MonitoredContract::Decryption).await;
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_listener_ended_by_end_of_any_task() {
        let (mut test_instance, asserter, gw_listener) = test_setup(None).await;

        // Both tasks will fail on get_block_number or get_logs
        for _ in 0..2 {
            asserter.push_success(&100_u64);
            asserter.push_failure(ErrorPayload {
                code: -32000,
                message: "rpc error".into(),
                data: None,
            });
        }

        gw_listener.start().await;
        test_instance.wait_for_log("polling failed").await;
    }

    type MockProvider = FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >;

    async fn test_setup(
        kms_operation_from_block_number: Option<u64>,
    ) -> (TestInstance, Asserter, GatewayListener<MockProvider>) {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();

        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        let config = Config {
            decryption_polling: Duration::from_millis(500),
            key_management_polling: Duration::from_millis(500),
            kms_operation_from_block_number,
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
