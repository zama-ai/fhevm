//! fhevm Relayer Library
//!
//! fhevm relayer service acts as a bridge between fhevm and gateway blockchains,
//! specifically handling FHE keys related operations. The service:
//!
//! 1. Listens for requests to from fhevm blockchain events to http endpoint.
//! 2. Forwards requests to the gateway blockchain for processing
//! 3. Receives responses from gateway blockchain 4. Relay the result back to source (fhevm blockchain or HTTP caller).
//!
//! # Architecture
//!
//! The system consists of several key components:
//! - [`Orchestrator`]: Manages event flow and dispatch
//! - [`GatewayHandler`]: Manages gateway interactions
//! - [`TransactionService`]: Handles blockchain transactions (for both fhevm and gateway)
//!
//! # Configuration
//!
//! The service is configured via:
//! - Environment variables
//! - Configuration files in the `config/` directory
//! - Command-line arguments
//!
//! See [`Settings`] for detailed configuration options.

use crate::gateway::{
    readiness_checker::ReadinessChecker, InputProofGatewayHandler, PublicDecryptGatewayHandler,
    UserDecryptGatewayHandler,
};
use crate::store::BlockNumberStore;
use alloy::primitives::Address;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::{str::FromStr, sync::Arc};
use tokio_util::sync::CancellationToken;
use tracing::{info, span, Level};

use crate::{
    config::settings::Settings,
    core::event::{
        GatewayChainEventId, InputProofEventId, PublicDecryptEventId, RelayerEvent,
        UserDecryptEventId,
    },
    gateway::arbitrum::{
        listener::arbitrum_listener,
        transaction::{
            helper::GatewayTransactionEngine, TransactionHelper as GatewayTransactionHelper,
        },
        ArbitrumJsonRPCWsClient,
    },
    http::http_server::run_http_server,
    metrics,
    orchestrator::{
        hooks::EventPersistenceHook,
        traits::{EventHandler, HandlerRegistry, HookRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    store::{
        key_value_db::RocksDBKVStore,
        sql::{
            client::PgClient,
            repositories::{
                input_proof_repo::InputProofRepository,
                public_decrypt_repo::PublicDecryptRepository,
                user_decrypt_repo::UserDecryptRepository,
            },
        },
        EventStore,
    },
};
use prometheus::Registry;
use std::sync::OnceLock;

// Global singleton registry for metrics
static GLOBAL_REGISTRY: OnceLock<Registry> = OnceLock::new();

/// Main library function for the FHE Event Relayer service.
///
/// This function performs the following initialization steps:
/// 1. Loads and validates configuration
/// 2. Sets up transaction services for fhevm and gateway
/// 3. Creates and configures event handlers
/// 4. Starts event listeners
/// 5. Waits for shutdown signal
///
/// TODO: properly shutdown tasks
pub async fn run_fhevm_relayer(
    settings: Settings,
    shutdown_token: CancellationToken,
) -> eyre::Result<()> {
    // 0. Print settings
    info!("Starting relayer with configuration: {:?}", settings);

    // 2. Init logging
    //
    let main_span = span!(Level::INFO, "main-span"); // Add other relevant top-level details
    let setup_span = span!(parent: &main_span, Level::INFO, "setup-span");

    // 1. Init metrics
    //
    let metrics_registry = ensure_global_init(&settings)?;

    // === Use the singleton registry for metrics endpoint
    let metrics_endpoint = settings.metrics.endpoint.clone();
    let registry_clone = metrics_registry.clone();

    // 3. Validate settings
    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;

    // 4. Init handlers
    //
    //
    let mut task_set = tokio::task::JoinSet::new();

    let tx_engine_gateway = GatewayTransactionEngine::new(
        settings.gateway.blockchain_rpc.clone(),
        settings.gateway.tx_engine.clone(),
    );

    // === Intialize the orchestrator.
    let orchestrator = Orchestrator::new(Arc::new(TokioEventDispatcher::<RelayerEvent>::new()));

    // Create root storage
    let path_rocks_db = PathBuf::from(settings.storage.db_path_rocksdb);
    let kv_store = RocksDBKVStore::open(path_rocks_db.clone())
        .map_err(|e| eyre::eyre!("Failed to open RocksDB: {}", e))?;
    info!("using rocks db databse at: {}", path_rocks_db.display());
    let kv_store = Arc::new(kv_store);

    // Initialize PostgreSQL client and repositories
    let pg_client = PgClient::new(
        settings.storage.sql_database_url.clone(),
        settings.storage.sql_max_connections,
    )
    .await;
    let pg_client = Arc::new(pg_client);
    info!("Initialized PostgreSQL client");

    // Create SQL repositories
    let input_proof_repo = Arc::new(InputProofRepository::new((*pg_client).clone()));
    let public_decrypt_repo = Arc::new(PublicDecryptRepository::new((*pg_client).clone()));
    let user_decrypt_repo = Arc::new(UserDecryptRepository::new((*pg_client).clone()));

    // Init and register event persistence hook
    let event_store = Arc::new(EventStore::<RelayerEvent>::new(kv_store.clone()));
    orchestrator.register_pre_dispatch_hook(EventPersistenceHook::<RelayerEvent>::new(
        event_store.clone(),
    ));

    // let gateway_tx_config = GatewayTxConfig::from(settings.transaction.clone());
    let gateway_tx_helper = Arc::new(GatewayTransactionHelper::new(
        tx_engine_gateway.clone().into(),
        settings.gateway.blockchain_rpc.chain_id,
    ));

    // Create ReadinessChecker once to be shared by both decrypt handlers
    let readiness_checker = Arc::new(ReadinessChecker::new(&settings.gateway)?);

    // Parse decryption address once
    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invalid decryption address"))?;

    setup_input_proof_gateway_handler(
        &orchestrator,
        gateway_tx_helper.clone(),
        settings.gateway.contracts.clone(),
        input_proof_repo.clone(),
    )?;

    setup_public_decrypt_gateway_handler(
        &orchestrator,
        gateway_tx_helper.clone(),
        readiness_checker.clone(),
        decryption_address,
        public_decrypt_repo.clone(),
    )?;

    setup_user_decrypt_gateway_handler(
        &orchestrator,
        gateway_tx_helper.clone(),
        readiness_checker.clone(),
        decryption_address,
        settings.gateway.contracts.user_decrypt_shares_threshold as usize,
        user_decrypt_repo.clone(),
    )?;

    // === Initialize gateway listener with reconnection configuration
    let listener_client_ws = ArbitrumJsonRPCWsClient::new(
        &settings.gateway.blockchain_rpc.ws_url,
        settings.gateway.listener.ws_reconnect_config.max_attempts,
        settings
            .gateway
            .listener
            .ws_reconnect_config
            .retry_interval_ms,
    )
    .await
    .map_err(|e| eyre::eyre!("Failed to create event handler for gateway: {}", e))?;
    let listener_client_ws = Arc::new(listener_client_ws);

    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invalid decryption contract address"))?;

    let input_verification_address =
        Address::from_str(&settings.gateway.contracts.input_verification_address)
            .map_err(|_| eyre::eyre!("Invalid InputVerification address"))?;

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let gateway_contract_addresses = vec![decryption_address, input_verification_address];

    let gateway_block_store = Arc::new(BlockNumberStore::new(
        kv_store.clone(),
        "gateway".to_string(),
    ));
    let latest_block_gateway = match settings.gateway.listener.last_block_number {
        Some(block_number) => Some(block_number),
        None => gateway_block_store
            .get_last_block_number()
            .await
            .map_err(|e| eyre::eyre!("Error getting last block number: {}", e))?,
    };
    info!(
        "start listening from block \"{}\" on gateway chain",
        latest_block_gateway
            .map(|b| b.to_string())
            .unwrap_or("latest".to_string())
    );
    let subscription_gateway = listener_client_ws
        .new_subscription(gateway_contract_addresses, latest_block_gateway)
        .await?;
    info!("Starting Relayer Gateway Listener");
    task_set.spawn(arbitrum_listener(
        subscription_gateway,
        Arc::clone(&orchestrator),
        Arc::clone(&gateway_block_store),
    ));

    // HTTP endpoint
    if let Some(http_endpoint) = settings.http.endpoint {
        info!("Starting Relayer HTTP server");
        let addr: SocketAddr = http_endpoint
            .parse()
            .expect("Invalid http-endpoint address");
        task_set.spawn(run_http_server(
            addr,
            Arc::clone(&orchestrator),
            settings.keyurl,
            settings.gateway.blockchain_rpc.http_url,
            settings.http.rate_limit_post_endpoints,
            input_proof_repo.clone(),
            public_decrypt_repo.clone(),
            user_decrypt_repo.clone(),
        ));
    };

    // Run metrics server
    task_set.spawn(async move {
        metrics::server::run_metrics_server(registry_clone, metrics_endpoint).await;
    });

    drop(setup_span);

    // === Wait for shutdown signal via cancellation token
    shutdown_token.cancelled().await;
    task_set.shutdown().await;

    info!("Relayer shutdown complete");

    Ok(())
}

/// Helper function to register a handler for multiple events
fn register_handler_for_events(
    orchestrator: &Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>,
    handler: Arc<dyn EventHandler<RelayerEvent>>,
    event_ids: &[u8],
) {
    for event_id in event_ids {
        orchestrator.register_handler(*event_id, Arc::clone(&handler));
    }
}

/// Initialize all global state exactly once
fn ensure_global_init(settings: &Settings) -> eyre::Result<&'static Registry> {
    let registry = GLOBAL_REGISTRY.get_or_init(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to install AWS-LC crypto provider");

        let registry = Registry::new();
        metrics::init_blockchain_metrics(&registry);
        metrics::init_http_metrics(&registry, &settings.http.metrics);
        metrics::init_cache_metrics(&registry);
        metrics::init_transaction_metrics(&registry);

        registry
    });

    Ok(registry)
}

/// Setup InputProofGatewayHandler and register its events
fn setup_input_proof_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_helper: Arc<GatewayTransactionHelper>,
    contracts: crate::config::settings::ContractConfig,
    input_proof_repo: Arc<InputProofRepository>,
) -> eyre::Result<()> {
    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(InputProofGatewayHandler::new(
        Arc::clone(orchestrator),
        tx_helper,
        contracts,
        input_proof_repo,
    ));

    register_handler_for_events(
        orchestrator,
        handler,
        &[
            InputProofEventId::ReqRcvdFromUser.into(),
            InputProofEventId::ReqSentToGw.into(),
            InputProofEventId::RespRcvdFromGw.into(),
            GatewayChainEventId::EventLogRcvd.into(),
        ],
    );
    Ok(())
}

/// Setup PublicDecryptGatewayHandler and register its events
fn setup_public_decrypt_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_helper: Arc<GatewayTransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
) -> eyre::Result<()> {
    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(PublicDecryptGatewayHandler::new(
        Arc::clone(orchestrator),
        tx_helper,
        readiness_checker,
        decryption_address,
        public_decrypt_repo,
    ));

    let event_ids = [
        PublicDecryptEventId::ReqRcvdFromUser.into(),
        PublicDecryptEventId::ReqSentToGw.into(),
        GatewayChainEventId::EventLogRcvd.into(),
    ];

    register_handler_for_events(orchestrator, handler, &event_ids);
    Ok(())
}

/// Setup UserDecryptGatewayHandler and register its events
fn setup_user_decrypt_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_helper: Arc<GatewayTransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
    user_decrypt_shares_threshold: usize,
    user_decrypt_repo: Arc<UserDecryptRepository>,
) -> eyre::Result<()> {
    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(UserDecryptGatewayHandler::new(
        Arc::clone(orchestrator),
        tx_helper,
        readiness_checker,
        decryption_address,
        user_decrypt_shares_threshold,
        user_decrypt_repo,
    ));

    let event_ids = [
        UserDecryptEventId::ReqRcvdFromUser.into(),
        UserDecryptEventId::ReqSentToGw.into(),
        GatewayChainEventId::EventLogRcvd.into(),
    ];

    register_handler_for_events(orchestrator, handler, &event_ids);
    Ok(())
}
