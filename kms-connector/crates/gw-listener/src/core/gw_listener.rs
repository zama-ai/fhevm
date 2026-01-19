use super::Config;
use crate::{
    core::{publish::update_last_block_polled, publish_event},
    monitoring::{
        health::State,
        metrics::{EVENT_RECEIVED_COUNTER, EVENT_RECEIVED_ERRORS},
    },
};
use alloy::{
    contract::{Event, EventPoller},
    network::Ethereum,
    primitives::LogData,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use anyhow::anyhow;
use connector_utils::{
    conn::{DefaultProvider, connect_to_db, connect_to_rpc_node},
    monitoring::otlp::PropagationContext,
    tasks::spawn_with_limit,
    types::{GatewayEvent, GatewayEventKind, db::EventType},
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    kms_generation::KMSGeneration::{self, KMSGenerationInstance},
};
use sqlx::{Pool, Postgres, Row};
use std::time::Duration;
use tokio::{select, task::JoinSet, time::timeout};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Struct monitoring and storing Gateway's events.
#[derive(Clone)]
pub struct GatewayListener<P>
where
    P: Provider,
{
    /// The database pool for storing Gateway's events.
    db_pool: Pool<Postgres>,

    /// The Gateway's `Decryption` contract instance which is monitored.
    decryption_contract: DecryptionInstance<P>,

    /// The Gateway's `KMSGeneration` contract instance which is monitored.
    kms_generation_contract: KMSGenerationInstance<P>,

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
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, provider.clone());
        let kms_generation_contract =
            KMSGeneration::new(config.kms_generation_contract.address, provider);

        Self {
            db_pool,
            decryption_contract,
            kms_generation_contract,
            config: config.clone(),
            cancel_token,
        }
    }

    /// Starts the `GatewayListener`.
    ///
    /// Spawns and joins the `GatewayListener` event monitoring tasks.
    pub async fn start(self) {
        let mut tasks = JoinSet::new();

        tasks.spawn(self.clone().subscribe(EventType::PublicDecryptionRequest));
        tasks.spawn(self.clone().subscribe(EventType::UserDecryptionRequest));
        tasks.spawn(self.clone().subscribe(EventType::PrepKeygenRequest));
        tasks.spawn(self.clone().subscribe(EventType::KeygenRequest));
        tasks.spawn(self.clone().subscribe(EventType::CrsgenRequest));
        tasks.spawn(self.clone().subscribe(EventType::PrssInit));
        tasks.spawn(self.subscribe(EventType::KeyReshareSameSet));

        while let Some(res) = tasks.join_next().await {
            if let Err(e) = res {
                error!("{e}");
            }
        }
        info!("GatewayListener stopped successfully!");
    }

    /// Subscribes to a particular set of events.
    ///
    /// Each event received from the `event_filer` is then published in the DB.
    pub async fn subscribe(self, event_type: EventType) {
        let polling = match &event_type {
            EventType::PublicDecryptionRequest | EventType::UserDecryptionRequest => {
                self.config.decryption_polling
            }
            _ => self.config.key_management_polling,
        };

        let result = match &event_type {
            EventType::PublicDecryptionRequest => {
                let filter = self.decryption_contract.PublicDecryptionRequest_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
            EventType::UserDecryptionRequest => {
                let filter = self.decryption_contract.UserDecryptionRequest_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
            EventType::PrepKeygenRequest => {
                let filter = self.kms_generation_contract.PrepKeygenRequest_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
            EventType::KeygenRequest => {
                let filter = self.kms_generation_contract.KeygenRequest_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
            EventType::CrsgenRequest => {
                let filter = self.kms_generation_contract.CrsgenRequest_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
            EventType::PrssInit => {
                let filter = self.kms_generation_contract.PRSSInit_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
            EventType::KeyReshareSameSet => {
                let filter = self.kms_generation_contract.KeyReshareSameSet_filter();
                self.subscribe_inner(event_type, filter, polling).await
            }
        };
        self.cancel_token.cancel(); // Cancel other event subscription tasks

        if let Err(e) = result {
            error!("{e}");
        }
    }

    async fn subscribe_inner<E>(
        &self,
        event_type: EventType,
        event_filter: Event<&'_ P, E>,
        poll_interval: Duration,
    ) -> anyhow::Result<()>
    where
        E: Into<GatewayEventKind> + SolEvent + Send + Sync + 'static,
    {
        let mut last_block_polled = self.get_last_block_polled(event_type).await?;
        let mut event_poller = event_filter
            .watch()
            .await
            .map_err(|e| anyhow!("Failed to subscribe to {event_type} events: {e}"))?;
        event_poller.poller = event_poller.poller.with_poll_interval(poll_interval);
        info!("✓ Subscribed to {event_type} events");

        let _ = self
            .catchup_past_events::<E>(&mut last_block_polled, event_type)
            .await
            .inspect_err(|e| warn!("Failed to catch up past {event_type} events: {e}"));

        select! {
            _ = self.process_events(event_type, event_poller, &mut last_block_polled) => (),
            _ = self.cancel_token.cancelled() => info!("{event_type} subscription cancelled..."),
        }

        // Use a timeout to ensure we are not preventing the `GatewayListener` from being shutdown
        // if the `last_block_polled` update get stuck for some reason.
        timeout(
            LAST_BLOCK_POLLED_UPDATE_TIMEOUT,
            update_last_block_polled(&self.db_pool, event_type, last_block_polled),
        )
        .await??;
        Ok(())
    }

    /// Catches events created before the event filter using `eth_getFilterLogs`.
    async fn catchup_past_events<E>(
        &self,
        last_block_polled: &mut Option<u64>,
        event_type: EventType,
    ) -> anyhow::Result<()>
    where
        E: Into<GatewayEventKind> + SolEvent + Send + Sync + 'static,
    {
        let catchup_from_block = match last_block_polled {
            None => {
                info!(
                    "No previously polled block for {event_type}; skipping catchup of past events."
                );
                return Ok(());
            }
            Some(block) => *block,
        };

        let contract_address = match event_type {
            EventType::PublicDecryptionRequest | EventType::UserDecryptionRequest => {
                self.decryption_contract.address()
            }
            _ => self.kms_generation_contract.address(),
        };

        let filter = Filter::new()
            .address(*contract_address)
            .event_signature(E::SIGNATURE_HASH)
            .from_block(catchup_from_block);
        let provider = self.decryption_contract.provider();

        info!("Catching up {event_type} from {catchup_from_block}...");
        let mut event_count = 0;
        let event_filter_id = provider.new_filter(&filter).await?;
        let past_events = provider
            .get_filter_logs(event_filter_id)
            .await?
            .into_iter()
            .map(|log| {
                decode_log::<E>(&log).map(|event| {
                    event_count += 1;
                    (event, log)
                })
            });

        for event in past_events {
            self.spawn_event_handling(event_type, event, last_block_polled)
                .await;
        }

        info!(
            "Successfully caught {event_count} {event_type} events from block {catchup_from_block}!"
        );
        if let Err(e) = provider.uninstall_filter(event_filter_id).await {
            warn!("Failed to uninstall {event_type} event catchup filter: {e}");
        }
        Ok(())
    }

    /// Event processing loop.
    async fn process_events<E>(
        &self,
        event_type: EventType,
        event_poller: EventPoller<E>,
        last_block_polled: &mut Option<u64>,
    ) where
        E: Into<GatewayEventKind> + SolEvent + Send + Sync + 'static,
    {
        let mut events = event_poller.into_stream();
        loop {
            info!("Waiting for next {event_type}...");
            match events.next().await {
                Some(event) => {
                    self.spawn_event_handling(event_type, event, last_block_polled)
                        .await
                }
                None => break error!("Alloy Provider was dropped for {event_type}"),
            }
        }
    }

    async fn spawn_event_handling<E>(
        &self,
        event_type: EventType,
        event: alloy::sol_types::Result<(E, Log)>,
        last_block: &mut Option<u64>,
    ) where
        E: Into<GatewayEventKind> + SolEvent + Send + Sync + 'static,
    {
        match event {
            Ok((event, log)) => {
                *last_block = log.block_number;
                EVENT_RECEIVED_COUNTER
                    .with_label_values(&[event_type.as_str()])
                    .inc();

                let db = self.db_pool.clone();
                spawn_with_limit(handle_gateway_event(db, event.into(), log.block_number)).await;
            }
            Err(err) => {
                error!("Error while listening for {event_type} events: {err}");
                EVENT_RECEIVED_ERRORS
                    .with_label_values(&[event_type.as_str()])
                    .inc();
            }
        }
    }

    /// Get the last block polled from config or DB.
    async fn get_last_block_polled(&self, event_type: EventType) -> anyhow::Result<Option<u64>> {
        let from_block_number = match event_type {
            EventType::PublicDecryptionRequest | EventType::UserDecryptionRequest => {
                self.config.decryption_from_block_number
            }
            _ => self.config.kms_operation_from_block_number,
        };

        let last_block_polled = match from_block_number {
            // Start polling event from the configured `from_block_number` if set
            Some(from_block) => {
                info!(
                    "Found configured `from_block_number` ({from_block}) for {event_type} subscriptions!"
                );
                Some(from_block)
            }
            // Start from `last_block_polled` stored in DB + 1 if not configured
            None => self
                .get_last_block_polled_from_db(event_type)
                .await?
                .map(|n| n + 1),
        };

        info!(
            "Starting {} subscriptions from block {}...",
            event_type,
            last_block_polled
                .map(|b| b.to_string())
                .unwrap_or_else(|| "latest".into())
        );

        Ok(last_block_polled)
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
async fn handle_gateway_event(
    db_pool: Pool<Postgres>,
    event_kind: GatewayEventKind,
    block_number: Option<u64>,
) {
    let event = GatewayEvent::new(
        event_kind,
        PropagationContext::inject(&tracing::Span::current().context()),
    );
    if let Err(err) = publish_event(&db_pool, event, block_number).await {
        error!("Failed to publish event: {err}");
    }
}

fn decode_log<E: SolEvent>(log: &Log) -> alloy::sol_types::Result<E> {
    let log_data: &LogData = log.as_ref();
    E::decode_raw_log(log_data.topics().iter().copied(), &log_data.data)
}

impl GatewayListener<DefaultProvider> {
    /// Creates a new `GatewayListener` instance from a valid `Config`.
    pub async fn from_config(
        config: Config,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<(Self, State<DefaultProvider>)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let provider =
            connect_to_rpc_node(config.gateway_url.clone(), config.gateway_chain_id).await?;

        let state = State::new(
            db_pool.clone(),
            provider.clone(),
            config.healthcheck_timeout,
        );

        let gw_listener = GatewayListener::new(db_pool, provider, &config, cancel_token);
        Ok((gw_listener, state))
    }
}

/// The timeout we allow for the listener to store the last block polled in DB.
const LAST_BLOCK_POLLED_UPDATE_TIMEOUT: Duration = Duration::from_mins(5);

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::Address,
        providers::{
            Identity, ProviderBuilder, RootProvider,
            fillers::{
                BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            },
            mock::Asserter,
        },
        rpc::json_rpc::ErrorPayload,
    };
    use connector_utils::tests::setup::{TestInstance, TestInstanceBuilder};

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_reset_filter_stops_listener() {
        let (_test_instance, asserter, gw_listener) = test_setup(None).await;

        asserter.push_failure(ErrorPayload {
            code: -32000,
            message: "filter not found".into(),
            data: None,
        });

        gw_listener.subscribe(EventType::KeygenRequest).await;
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_failed_catchup_does_not_stop_listener() {
        let (mut test_instance, asserter, gw_listener) = test_setup(Some(0)).await;

        asserter.push_failure(ErrorPayload {
            code: -32002,
            message: "request timed out".into(),
            data: None,
        });

        let event_type = EventType::KeygenRequest;
        tokio::spawn(gw_listener.subscribe(event_type));
        test_instance.wait_for_log("Failed to catch up").await;
        test_instance
            .wait_for_log(&format!("Waiting for next {event_type}"))
            .await;
    }

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_listener_ended_by_end_of_any_task() {
        let (mut test_instance, _asserter, gw_listener) = test_setup(None).await;

        // Will stop because some subcription tasks will not be able to init their event filter
        gw_listener.start().await;

        test_instance.wait_for_log("Failed to subscribe to").await;
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

        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        // Used to mock response of `filter.watch()` operation
        let mocked_eth_get_filter_changes_result = Address::default();
        asserter.push_success(&mocked_eth_get_filter_changes_result);

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
