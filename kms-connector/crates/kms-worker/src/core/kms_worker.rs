use crate::{
    core::{
        KmsResponsePublisher,
        config::{Config, HostChainKind},
        event_picker::{DbEventPicker, EventPicker},
        event_processor::{
            CiphertextManager, DbContextManager, DbEventProcessor, DecryptionProcessor,
            EventProcessor, KMSGenerationProcessor, KmsClient, ProtocolConfigProcessor,
        },
        kms_response_publisher::DbKmsResponsePublisher,
    },
    monitoring::{
        health::{KmsHealthClient, State},
        metrics::register_event_latency,
    },
};
use alloy::transports::http::reqwest;
use anyhow::anyhow;
use connector_utils::{
    conn::{DefaultProvider, connect_to_db, connect_to_rpc_node},
    tasks::spawn_with_limit,
    types::{KmsResponse, ProtocolEvent},
};
use fhevm_host_bindings::acl::ACL;
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
    E: EventPicker<Event = ProtocolEvent>,
    Proc: EventProcessor<Event = ProtocolEvent> + Clone + Send + 'static,
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
    async fn spawn_event_processing_tasks(&self, events: Vec<ProtocolEvent>) {
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
        mut event: ProtocolEvent,
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
        } else {
            register_event_latency(&event);
        }
    }
}

impl
    KmsWorker<
        DbEventPicker,
        DbEventProcessor<DefaultProvider, DefaultProvider, DbContextManager<DefaultProvider>>,
    >
{
    /// Creates a new `KmsWorker` instance from a valid `Config`.
    pub async fn from_config(
        config: Config,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<(Self, State<DefaultProvider>)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;

        let gateway_provider =
            connect_to_rpc_node(config.gateway_url.clone(), config.gateway_chain_id).await?;
        let ethereum_provider =
            connect_to_rpc_node(config.ethereum_url.clone(), config.ethereum_chain_id).await?;

        let mut acl_contracts = HashMap::new();
        for host_chain in &config.host_chains {
            if host_chain.chain_kind == HostChainKind::Solana {
                if host_chain.solana_host_program_id.is_some() {
                    info!(
                        "Configured Solana host chain {} with solana_host_program_id. Gateway \
                        decryption ACL checks for Solana still fail closed; native-v0 Solana \
                        request processing is handled outside the Gateway event processor.",
                        host_chain.chain_id
                    );
                } else {
                    info!(
                        "Configured Solana host chain {} without solana_host_program_id. The KMS \
                        connector will reject decryption ACL checks for this chain fail-closed.",
                        host_chain.chain_id
                    );
                }
                continue;
            }
            let provider = connect_to_rpc_node(host_chain.url.clone(), host_chain.chain_id).await?;
            let acl_address = host_chain.acl_address.ok_or_else(|| {
                anyhow!(
                    "EVM host chain {} requires acl_address (the ACL contract to gate decryptions)",
                    host_chain.chain_id
                )
            })?;
            let acl_contract = ACL::new(acl_address, provider);
            let host_chain_id = host_chain.chain_id;
            if acl_contracts.insert(host_chain_id, acl_contract).is_some() {
                return Err(anyhow!(
                    "Duplicate host chain in config for chain ID {host_chain_id}"
                ));
            };
        }

        let kms_client = KmsClient::connect(&config).await?;
        let kms_health_client = KmsHealthClient::connect(&config.kms_core_endpoints).await?;
        let s3_client = reqwest::Client::builder()
            .connect_timeout(config.s3_connect_timeout)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        let event_picker = DbEventPicker::connect(db_pool.clone(), &config).await?;

        let context_manager =
            DbContextManager::new(db_pool.clone(), &config, ethereum_provider.clone());
        let ciphertext_manager =
            CiphertextManager::connect(gateway_provider.clone(), s3_client, &config, cancel_token)
                .await?;
        let decryption_processor = DecryptionProcessor::new(
            &config,
            context_manager.clone(),
            gateway_provider.clone(),
            acl_contracts,
            ciphertext_manager,
        );
        let kms_generation_processor = KMSGenerationProcessor::new(&config, context_manager);
        let protocol_config_processor = ProtocolConfigProcessor::new(&config, ethereum_provider);
        let event_processor = DbEventProcessor::new(
            kms_client.clone(),
            decryption_processor,
            kms_generation_processor,
            protocol_config_processor,
            config.max_decryption_attempts,
            db_pool.clone(),
        );
        let response_publisher = DbKmsResponsePublisher::new(db_pool.clone());

        let state = State::new(
            db_pool,
            gateway_provider,
            // TODO: add ethereum_provider (and each host-chain providers?)
            // Tracking issue: https://github.com/zama-ai/fhevm-internal/issues/1465
            kms_health_client,
            config.healthcheck_timeout,
        );
        let kms_worker = KmsWorker::new(event_picker, event_processor, response_publisher);
        Ok((kms_worker, state))
    }
}
