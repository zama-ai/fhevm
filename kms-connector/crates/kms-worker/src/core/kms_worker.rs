use crate::{
    core::{
        KmsResponsePublisher,
        config::{Config, HostSettings},
        event_picker::{DbEventPicker, EventPicker},
        event_processor::{
            CiphertextManager, DbContextManager, DbEventProcessor, DecryptionProcessor,
            EventProcessor, HostChain, HostDecryptionVerifier, HostRpcClient,
            KMSGenerationProcessor, KmsClient, ProcessingError, ProcessingErrorKind,
            ProtocolConfigProcessor,
        },
        kms_response_publisher::DbKmsResponsePublisher,
        solana::{SolanaHost, proof::CoprocessorProofClient, snapshot::SolanaRpcClient},
    },
    monitoring::{
        health::{KmsHealthClient, State},
        metrics::register_event_latency,
    },
};
use anyhow::anyhow;
use connector_utils::{
    conn::{DefaultProvider, connect_to_db, connect_to_rpc_node, connect_to_rpc_node_with_bounds},
    tasks::spawn_with_limit,
    types::{KmsResponse, ProtocolEvent, ProtocolEventKind, db::RequestSource},
};
use fhevm_host_bindings::acl::ACL;
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Struct processing stored Gateway's events.
pub struct KmsWorker<E, Proc> {
    /// The entity responsible for picking events to process.
    event_picker: E,

    /// The entity responsible for processing events.
    event_processor: Proc,

    /// The entity responsible for publishing KMS Core's responses.
    response_publisher: DbKmsResponsePublisher,

    /// The maximum number of decryption attempts.
    max_decryption_attempts: u16,
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
        max_decryption_attempts: u16,
    ) -> Self {
        Self {
            event_picker,
            event_processor,
            response_publisher,
            max_decryption_attempts,
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
            let max_decryption_attempts = self.max_decryption_attempts;

            spawn_with_limit(async move {
                Self::handle_event(
                    event_processor,
                    response_publisher,
                    event,
                    max_decryption_attempts,
                )
                .await
            })
            .await;
        }
    }

    /// Processes an event coming from the Gateway.
    #[tracing::instrument(skip(event_processor, response_publisher, max_decryption_attempts), fields(event = % event.kind))]
    async fn handle_event(
        mut event_processor: Proc,
        response_publisher: DbKmsResponsePublisher,
        mut event: ProtocolEvent,
        max_decryption_attempts: u16,
    ) {
        let otlp_context = event.otlp_context.clone();
        tracing::Span::current().set_parent(otlp_context.extract());

        info!("Starting to process {:?}...", event.kind);
        let response_kind = match event_processor.process(&mut event).await {
            Ok(response_kind) => response_kind,
            Err(error) => {
                return Self::handle_processing_error(
                    &response_publisher,
                    event,
                    error,
                    max_decryption_attempts,
                )
                .await;
            }
        };
        info!("Event successfully processed!");
        let Some(response_kind) = response_kind else {
            return;
        };

        let response = KmsResponse::new(response_kind, otlp_context, event.source);
        if let Err(e) = response_publisher.publish_response(response).await {
            event.error_counter += 1;
            Self::handle_processing_error(
                &response_publisher,
                event,
                ProcessingError::transient(anyhow!("Failed to publish response: {e}")),
                max_decryption_attempts,
            )
            .await;
        } else {
            register_event_latency(&event);
        }
    }

    async fn handle_processing_error(
        response_publisher: &DbKmsResponsePublisher,
        event: ProtocolEvent,
        error: ProcessingError,
        max_decryption_attempts: u16,
    ) {
        // For HTTP-sourced requests: the caller is waiting on a held connection, so any failure
        // is stored as an error response row instead of being retried internally.
        if event.source == RequestSource::Http {
            return Self::reject_http_decryption(response_publisher, &event, error).await;
        }

        match (error.kind, &event.kind) {
            (ProcessingErrorKind::Irrecoverable, _) => {
                error!("{error}");
                if let Err(e) = response_publisher.mark_event_as_failed(&event).await {
                    warn!("{e}");
                }
            }
            (ProcessingErrorKind::Aborted, _) => {
                warn!("{error}");
                if let Err(e) = response_publisher.mark_event_as_aborted(&event).await {
                    warn!("{e}");
                }
            }
            // For now, we only check the error counter for public and user decryptions as they are
            // the most frequent operations, and we want to avoid infinite retry loop for them.
            // For key management operations, as they are not frequent at all, we currently rely on
            // a manual cleanup of the DB in such case. We want to avoid to "accidentally" remove a
            // key management operation at all cost.
            (
                ProcessingErrorKind::Recoverable,
                ProtocolEventKind::PublicDecryption(_)
                | ProtocolEventKind::UserDecryption(_)
                | ProtocolEventKind::UserDecryptionV2(_)
                | ProtocolEventKind::SolanaUserDecryptionV1(_),
            ) if event.error_counter as u16 >= max_decryption_attempts => {
                error!(
                    "Processing failed with irrecoverable error: {:#}. Maximum number of \
                     decryption attempts reached: {}",
                    error.source, event.error_counter
                );
                if let Err(e) = response_publisher.mark_event_as_failed(&event).await {
                    warn!("{e}");
                }
            }
            (ProcessingErrorKind::Recoverable, _) => {
                error!("{error}");
                if let Err(e) = response_publisher.mark_event_as_pending(&event).await {
                    warn!("{e}");
                }
            }
        }
    }

    /// Stores the failure of an HTTP-sourced decryption request as an error response row.
    async fn reject_http_decryption(
        response_publisher: &DbKmsResponsePublisher,
        event: &ProtocolEvent,
        error: ProcessingError,
    ) {
        error!(
            "{error}. Storing `{}` error response for the HTTP-sourced request...",
            error.code.as_str()
        );

        let details = format!("{error:#}");
        let result = match &event.kind {
            ProtocolEventKind::PublicDecryption(req) => {
                response_publisher
                    .publish_public_decryption_error(
                        req.decryptionId,
                        error.code,
                        &details,
                        &req.extraData,
                        &event.otlp_context,
                    )
                    .await
            }
            ProtocolEventKind::UserDecryptionV2(req) => {
                response_publisher
                    .publish_user_decryption_error(
                        req.decryptionId,
                        error.code,
                        &details,
                        &req.payload.extraData,
                        &event.otlp_context,
                    )
                    .await
            }
            ProtocolEventKind::SolanaUserDecryptionV1(req) => {
                response_publisher
                    .publish_user_decryption_error(
                        req.decryption_id,
                        error.code,
                        &details,
                        &req.extra_data(),
                        &event.otlp_context,
                    )
                    .await
            }
            kind => {
                error!(
                    "Unexpected HTTP-sourced {kind}: only decryption requests can be HTTP-sourced"
                );
                if let Err(e) = response_publisher.mark_event_as_failed(event).await {
                    warn!("{e}");
                }
                return;
            }
        };

        if let Err(e) = result {
            error!("Failed to store the error response: {e}");
            if let Err(e) = response_publisher.mark_event_as_failed(event).await {
                warn!("{e}");
            }
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
        let hosts = register_host_chains(&config).await?;
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;

        let gateway_provider =
            connect_to_rpc_node(config.gateway_url.clone(), config.gateway_chain_id).await?;
        let ethereum_provider =
            connect_to_rpc_node(config.ethereum_url.clone(), config.ethereum_chain_id).await?;

        let kms_client = KmsClient::connect(&config).await?;
        let kms_health_client = KmsHealthClient::connect(&config.kms_core_endpoints).await?;

        let event_picker = DbEventPicker::connect(db_pool.clone(), &config).await?;

        let context_manager =
            DbContextManager::new(db_pool.clone(), &config, ethereum_provider.clone());
        let ciphertext_manager =
            CiphertextManager::connect(gateway_provider.clone(), &config, cancel_token).await?;
        let decryption_processor =
            DecryptionProcessor::new(&config, gateway_provider.clone(), ciphertext_manager);
        let host_verifier = HostDecryptionVerifier::new(&config, hosts);
        let kms_generation_processor = KMSGenerationProcessor::new(&config);
        let protocol_config_processor = ProtocolConfigProcessor::new(&config, ethereum_provider);
        let event_processor = DbEventProcessor::new(
            kms_client.clone(),
            context_manager,
            decryption_processor,
            host_verifier,
            kms_generation_processor,
            protocol_config_processor,
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
        let kms_worker = KmsWorker::new(
            event_picker,
            event_processor,
            response_publisher,
            config.max_decryption_attempts,
        );
        Ok((kms_worker, state))
    }
}

async fn register_host_chains(
    config: &Config,
) -> anyhow::Result<HashMap<u64, HostChain<DefaultProvider>>> {
    let mut hosts = HashMap::with_capacity(config.host_chains.len());
    // The workspace `reqwest`, not alloy's re-export: alloy now vendors a different major, and
    // the Solana readers are typed against the workspace crate.
    let proof_client = ::reqwest::Client::builder()
        .connect_timeout(config.host_rpc_call_timeout)
        .timeout(config.host_rpc_call_timeout)
        .build()?;

    for host_chain in &config.host_chains {
        let host = match &host_chain.host {
            HostSettings::Evm { acl_address } => {
                let provider = connect_to_rpc_node_with_bounds(
                    host_chain.url.clone(),
                    host_chain.chain_id,
                    config.host_rpc_max_concurrent_calls,
                    config.host_rpc_call_timeout,
                )
                .await?;
                HostChain::Evm(HostRpcClient::new(
                    host_chain.chain_id,
                    ACL::new(*acl_address, provider),
                ))
            }
            HostSettings::Solana(solana) => HostChain::Solana(Box::new(SolanaHost {
                program_id: solana.host_program_id,
                reader: SolanaRpcClient::new(
                    host_chain.url.clone(),
                    config.host_rpc_call_timeout,
                    config.host_rpc_max_concurrent_calls,
                ),
                proofs: CoprocessorProofClient::new(&solana.proof_routes, proof_client.clone()),
            })),
        };
        hosts.insert(host_chain.chain_id, host);
    }

    Ok(hosts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{HostChainConfig, SolanaHostSettings, solana_host_chain_id};

    fn evm_chain(chain_id: u64) -> HostChainConfig {
        HostChainConfig {
            chain_id,
            ..Config::default().host_chains.remove(0)
        }
    }

    fn solana_chain(cluster_tag: u64, endpoint: &str) -> HostChainConfig {
        HostChainConfig {
            url: endpoint.parse().unwrap(),
            chain_id: solana_host_chain_id(cluster_tag),
            host: HostSettings::Solana(SolanaHostSettings {
                host_program_id: [7; 32],
                proof_routes: vec![
                    serde_json::from_value(serde_json::json!({
                        "url": endpoint, "api_key": "test-key"
                    }))
                    .unwrap(),
                ],
            }),
        }
    }

    #[tokio::test]
    async fn registers_evm_and_solana_hosts_once() {
        let config = Config {
            host_chains: vec![
                evm_chain(1),
                solana_chain(2, "http://coprocessor.test:8080"),
            ],
            ..Default::default()
        };
        let hosts = register_host_chains(&config)
            .await
            .expect("valid host chains should register");

        assert!(matches!(hosts.get(&1), Some(HostChain::Evm(_))));
        match hosts.get(&solana_host_chain_id(2)) {
            Some(HostChain::Solana(host)) => {
                assert_eq!(host.program_id, [7; 32])
            }
            _ => panic!("the Solana host chain should use the Solana host"),
        }
    }

    #[tokio::test]
    async fn registered_solana_hosts_bound_rpc_and_proof_requests() {
        use crate::core::solana::{
            proof::{HostProofReader, LeafKind, LeafQuery},
            snapshot::HostStateReader,
        };
        use std::time::Duration;
        use tokio::{net::TcpListener, time::timeout};

        // The listening socket permits TCP connections but never answers HTTP requests.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let config = Config {
            host_chains: vec![solana_chain(2, &endpoint)],
            host_rpc_call_timeout: Duration::from_millis(250),
            ..Default::default()
        };
        let hosts = register_host_chains(&config).await.unwrap();
        let HostChain::Solana(host) = &hosts[&solana_host_chain_id(2)] else {
            panic!("expected Solana host")
        };
        let keys = [[1; 32]];
        let queries = [LeafQuery {
            encrypted_store: [1; 32],
            handle: [2; 32],
            kind: LeafKind::Public,
        }];
        let (rpc, proofs) = tokio::join!(
            timeout(
                Duration::from_secs(3),
                host.reader.read_accounts(&keys, None)
            ),
            timeout(Duration::from_secs(3), host.proofs.read_proofs(0, &queries)),
        );
        assert!(rpc.expect("registered RPC client must time out").is_err());
        assert!(
            proofs
                .expect("registered proof client must time out")
                .is_err()
        );
        drop(listener);
    }
}
