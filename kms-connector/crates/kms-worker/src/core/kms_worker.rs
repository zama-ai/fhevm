use crate::{
    core::{
        KmsResponsePublisher,
        config::{Config, HostChainConfig, HostChainKind, SOLANA_CHAIN_TYPE_BIT},
        event_picker::{DbEventPicker, EventPicker},
        event_processor::{
            CiphertextManager, DbContextManager, DbEventProcessor, DecryptionProcessor,
            EventProcessor, HostChainAclBackend, KMSGenerationProcessor, KmsClient,
            ProcessingError, ProcessingErrorKind, ProtocolConfigProcessor,
            solana_public_decrypt::SolanaHost,
        },
        kms_response_publisher::DbKmsResponsePublisher,
        solana::{deployment::DeploymentIdentity, snapshot::RpcHostStateReader},
        solana_v2_fetcher::SolanaV2Fetcher,
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
use std::collections::{HashMap, HashSet};
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
                | ProtocolEventKind::UserDecryptionV3(_),
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
        let host_chain_backends = register_host_chain_backends(&config).await?;
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
        let decryption_processor = DecryptionProcessor::new(
            &config,
            context_manager.clone(),
            gateway_provider.clone(),
            host_chain_backends,
            ciphertext_manager,
        );
        let kms_generation_processor = KMSGenerationProcessor::new(&config);
        let protocol_config_processor = ProtocolConfigProcessor::new(&config, ethereum_provider);
        let event_processor = DbEventProcessor::new(
            kms_client.clone(),
            context_manager,
            decryption_processor,
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

async fn register_host_chain_backends(
    config: &Config,
) -> anyhow::Result<HashMap<u64, HostChainAclBackend<DefaultProvider>>> {
    validate_host_chain_configs(&config.host_chains)?;

    let mut backends = HashMap::with_capacity(config.host_chains.len());
    // The workspace `reqwest`, not alloy's re-export: alloy now vendors a different major, and
    // `SolanaV2Fetcher` is typed against the workspace crate.
    let solana_client = ::reqwest::Client::new();

    for host_chain in &config.host_chains {
        let backend = match host_chain.chain_kind {
            HostChainKind::Evm => {
                let acl_address = host_chain.acl_address.ok_or_else(|| {
                    anyhow!(
                        "EVM host chain {} requires acl_address (the ACL contract to gate decryptions)",
                        host_chain.chain_id
                    )
                })?;
                let provider = connect_to_rpc_node_with_bounds(
                    host_chain.url.clone(),
                    host_chain.chain_id,
                    config.host_rpc_max_concurrent_calls,
                    config.host_rpc_call_timeout,
                )
                .await?;
                HostChainAclBackend::Evm(ACL::new(acl_address, provider))
            }
            HostChainKind::Solana => {
                let program_id = host_chain.solana_host_program_id.ok_or_else(|| {
                    anyhow!(
                        "Solana host chain {} requires solana_host_program_id",
                        host_chain.chain_id
                    )
                })?;
                let deployment = DeploymentIdentity::resolve(program_id, host_chain.chain_id)
                    .map_err(|e| {
                        anyhow!(
                            "Solana host chain {} has an invalid deployment identity: {e}",
                            host_chain.chain_id
                        )
                    })?;
                HostChainAclBackend::Solana(Box::new(SolanaHost {
                    deployment,
                    reader: RpcHostStateReader::new(host_chain.url.clone(), solana_client.clone()),
                    fetcher: SolanaV2Fetcher::new(host_chain.url.clone(), solana_client.clone()),
                }))
            }
        };

        backends.insert(host_chain.chain_id, backend);
    }

    Ok(backends)
}

fn validate_host_chain_configs(host_chains: &[HostChainConfig]) -> anyhow::Result<()> {
    let mut chain_ids = HashSet::with_capacity(host_chains.len());
    for host_chain in host_chains {
        if !chain_ids.insert(host_chain.chain_id) {
            return Err(anyhow!(
                "Duplicate host chain in config for chain ID {}",
                host_chain.chain_id
            ));
        }

        let has_chain_type_bit = host_chain.chain_id & SOLANA_CHAIN_TYPE_BIT != 0;
        match host_chain.chain_kind {
            HostChainKind::Evm => {
                if has_chain_type_bit {
                    return Err(anyhow!(
                        "EVM host chain {} must not set the RFC-021 Solana chain-type high bit (bit 63)",
                        host_chain.chain_id
                    ));
                }
                if host_chain.acl_address.is_none() {
                    return Err(anyhow!(
                        "EVM host chain {} requires acl_address (the ACL contract to gate decryptions)",
                        host_chain.chain_id
                    ));
                }
                if host_chain.solana_host_program_id.is_some() {
                    return Err(anyhow!(
                        "EVM host chain {} must not set solana_host_program_id",
                        host_chain.chain_id
                    ));
                }
            }
            HostChainKind::Solana => {
                if !has_chain_type_bit {
                    return Err(anyhow!(
                        "Solana host chain {} must set the RFC-021 Solana chain-type high bit (bit 63)",
                        host_chain.chain_id
                    ));
                }
                if host_chain.solana_host_program_id.is_none() {
                    return Err(anyhow!(
                        "Solana host chain {} requires solana_host_program_id",
                        host_chain.chain_id
                    ));
                }
                if host_chain.acl_address.is_some() {
                    return Err(anyhow!(
                        "Solana host chain {} must not set acl_address",
                        host_chain.chain_id
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;

    /// Builds a host-chain config for the given logical id and kind, applying the
    /// RFC-021 chain-type high bit to Solana ids so the fixture satisfies the
    /// invariant enforced by `validate_host_chain_configs`.
    fn host_chain(chain_id: u64, chain_kind: HostChainKind) -> HostChainConfig {
        let mut host_chain = Config::default().host_chains.remove(0);
        host_chain.chain_kind = chain_kind;
        match chain_kind {
            HostChainKind::Evm => {
                host_chain.chain_id = chain_id;
                host_chain.solana_host_program_id = None;
            }
            HostChainKind::Solana => {
                host_chain.chain_id = SOLANA_CHAIN_TYPE_BIT | chain_id;
                host_chain.acl_address = None;
                host_chain.solana_host_program_id = Some([7; 32]);
            }
        }
        host_chain
    }

    fn validation_error(host_chains: &[HostChainConfig]) -> String {
        validate_host_chain_configs(host_chains)
            .expect_err("host-chain validation should fail")
            .to_string()
    }

    #[tokio::test]
    async fn registers_evm_and_solana_backends_once() {
        let config = Config {
            host_chains: vec![
                host_chain(1, HostChainKind::Evm),
                host_chain(2, HostChainKind::Solana),
            ],
            ..Default::default()
        };
        let backends = register_host_chain_backends(&config)
            .await
            .expect("valid host chains should register");

        assert!(matches!(
            backends.get(&1),
            Some(HostChainAclBackend::Evm(_))
        ));
        match backends.get(&(SOLANA_CHAIN_TYPE_BIT | 2)) {
            Some(HostChainAclBackend::Solana(host)) => {
                assert_eq!(host.deployment.program_id(), [7; 32])
            }
            _ => panic!("the Solana host chain should use the Solana backend"),
        }
    }

    #[test]
    fn rejects_duplicate_chain_ids() {
        // Under the RFC-021 invariant an EVM id and a Solana id can never collide
        // (different high bit), so duplicates are only possible within one kind.
        for kind in [HostChainKind::Evm, HostChainKind::Solana] {
            let duplicate_id = host_chain(7, kind).chain_id;
            let error = validation_error(&[host_chain(7, kind), host_chain(7, kind)]);
            assert!(
                error.contains(&format!(
                    "Duplicate host chain in config for chain ID {duplicate_id}"
                )),
                "unexpected error: {error}"
            );
        }
    }

    #[test]
    fn rejects_chain_id_inconsistent_with_chain_kind() {
        // A Solana host chain that leaves the high bit clear.
        let mut solana_without_bit = host_chain(9, HostChainKind::Solana);
        solana_without_bit.chain_id = 9;
        // An EVM host chain that sets the high bit.
        let mut evm_with_bit = host_chain(9, HostChainKind::Evm);
        evm_with_bit.chain_id = SOLANA_CHAIN_TYPE_BIT | 9;

        let cases = [
            (
                solana_without_bit,
                "must set the RFC-021 Solana chain-type high bit",
            ),
            (
                evm_with_bit,
                "must not set the RFC-021 Solana chain-type high bit",
            ),
        ];
        for (host_chain, expected) in cases {
            let error = validation_error(&[host_chain]);
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn rejects_missing_or_contradictory_backend_fields() {
        let mut evm_missing_acl = host_chain(1, HostChainKind::Evm);
        evm_missing_acl.acl_address = None;
        let mut evm_with_program_id = host_chain(2, HostChainKind::Evm);
        evm_with_program_id.solana_host_program_id = Some([7; 32]);
        let mut solana_missing_program_id = host_chain(3, HostChainKind::Solana);
        solana_missing_program_id.solana_host_program_id = None;
        let mut solana_with_acl = host_chain(4, HostChainKind::Solana);
        solana_with_acl.acl_address = Some(Address::ZERO);

        let cases = [
            (evm_missing_acl, "requires acl_address"),
            (evm_with_program_id, "must not set solana_host_program_id"),
            (solana_missing_program_id, "requires solana_host_program_id"),
            (solana_with_acl, "must not set acl_address"),
        ];
        for (host_chain, expected) in cases {
            let error = validation_error(&[host_chain]);
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }
}
