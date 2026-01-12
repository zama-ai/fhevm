use crate::{
    core::{
        KmsResponsePublisher,
        config::Config,
        event_picker::{DbEventPicker, EventPicker},
        event_processor::{
            AclChecker, CoprocessorApi, DbEventProcessor, DecryptionProcessor, EventProcessor,
            KMSGenerationProcessor, KmsClient,
        },
        kms_response_publisher::DbKmsResponsePublisher,
    },
    api::ApiState,
    monitoring::health::{KmsHealthClient, State},
};
use alloy::{primitives::U256, sol_types::Eip712Domain, transports::http::reqwest};
use anyhow::anyhow;
use connector_utils::{
    conn::{GatewayProvider, connect_to_db, connect_to_gateway},
    tasks::spawn_with_limit,
    types::{GatewayEvent, KmsResponse},
};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Struct processing stored Gateway's events.
pub struct KmsWorker<E, Proc> {
    /// The entity responsible for picking events to process.
    event_picker: E,

    /// The entity responsible for processing events.
    event_processor: Proc,

    /// The entity responsible for publishing KMS Core's responses.
    response_publisher: DbKmsResponsePublisher,
}

impl<E, Proc> KmsWorker<E, Proc>
where
    E: EventPicker<Event = GatewayEvent>,
    Proc: EventProcessor<Event = GatewayEvent> + Clone + Send + 'static,
{
    /// Creates a new `KmsWorker<E, Proc>`.
    pub fn new(
        event_picker: E,
        event_processor: Proc,
        response_publisher: DbKmsResponsePublisher,
    ) -> Self {
        Self {
            event_picker,
            event_processor,
            response_publisher,
        }
    }

    /// Starts the `KmsWorker`.
    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting KmsWorker");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("Stopping KmsWorker"),
            _ = self.run() => (),
        }
    }

    /// Runs the event processing loop of the `KmsWorker`.
    async fn run(mut self) {
        loop {
            match self.event_picker.pick_events().await {
                Ok(events) => self.spawn_event_processing_tasks(events).await,
                Err(e) => break error!("Event picker is broken: {e}"),
            };
        }
    }

    /// Spawns a new task to process each event.
    async fn spawn_event_processing_tasks(&self, events: Vec<GatewayEvent>) {
        for event in events {
            let event_processor = self.event_processor.clone();
            let response_publisher = self.response_publisher.clone();

            spawn_with_limit(async move {
                Self::handle_event(event_processor, response_publisher, event).await
            })
            .await;
        }
    }

    /// Processes an event coming from the Gateway.
    #[tracing::instrument(skip(event_processor, response_publisher), fields(event = % event.kind))]
    async fn handle_event(
        mut event_processor: Proc,
        response_publisher: DbKmsResponsePublisher,
        mut event: GatewayEvent,
    ) {
        let otlp_context = event.otlp_context.clone();
        tracing::Span::current().set_parent(otlp_context.extract());

        let Some(response_kind) = event_processor.process(&mut event).await else {
            return;
        };

        let response = KmsResponse::new(response_kind, otlp_context);
        if let Err(e) = response_publisher.publish_response(response).await {
            response_publisher.mark_event_as_pending(event).await;
            error!("Failed to publish response: {e}");
        }
    }
}

impl KmsWorker<DbEventPicker, DbEventProcessor<GatewayProvider>> {
    /// Creates a new `KmsWorker` instance from a valid `Config`.
    pub async fn from_config(
        config: Config,
    ) -> anyhow::Result<(Self, State<GatewayProvider>, ApiState)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let provider = connect_to_gateway(config.gateway_url.clone(), config.chain_id).await?;
        let kms_client = KmsClient::connect(&config).await?;
        let kms_health_client = KmsHealthClient::connect(&config.kms_core_endpoints).await?;
        let http_client = reqwest::Client::builder()
            .connect_timeout(config.s3_connect_timeout)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        let event_picker = DbEventPicker::connect(db_pool.clone(), &config).await?;

        let gateway_config_contract = fhevm_gateway_bindings::gateway_config::GatewayConfig::new(
            config.gateway_config_contract.address,
            provider.clone(),
        );
        let acl_checker = AclChecker::new(gateway_config_contract.clone(), build_host_chain_providers(&config).await?, provider.clone())
            .await?;
        let domain = Eip712Domain::new(
            Some(config.input_verification_contract.domain_name.clone().into()),
            Some(config.input_verification_contract.domain_version.clone().into()),
            Some(U256::from(config.chain_id)),
            Some(config.input_verification_contract.address),
            None,
        );
        let coprocessor_api = CoprocessorApi::new(gateway_config_contract, http_client, domain);
        let decryption_processor = DecryptionProcessor::new(&config, acl_checker, coprocessor_api, db_pool.clone());
        let kms_generation_processor = KMSGenerationProcessor::new(&config);
        let event_processor = DbEventProcessor::new(
            kms_client.clone(),
            decryption_processor,
            kms_generation_processor,
            config.max_decryption_attempts,
            db_pool.clone(),
        );
        let response_publisher = DbKmsResponsePublisher::new(db_pool.clone());

        let state = State::new(
            db_pool,
            provider,
            kms_health_client,
            config.healthcheck_timeout,
        );
        let api_state = ApiState::new(
            state.db_pool().clone(),
            config.signer_address,
            config.share_index,
        );
        let kms_worker = KmsWorker::new(event_picker, event_processor, response_publisher);
        Ok((kms_worker, state, api_state))
    }
}

async fn build_host_chain_providers(
    config: &Config,
) -> anyhow::Result<HashMap<u64, GatewayProvider>> {
    let mut providers = HashMap::new();
    for host_chain in config.host_chain_urls.values() {
        let provider =
            connect_to_gateway(host_chain.rpc_url.clone(), host_chain.chain_id).await?;
        providers.insert(host_chain.chain_id, provider);
    }
    Ok(providers)
}
