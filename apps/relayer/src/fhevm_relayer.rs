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
    readiness_checker::ReadinessChecker, InputProofGatewayHandler, KeyUrlGatewayHandler,
    PublicDecryptGatewayHandler, UserDecryptGatewayHandler,
};
use alloy::primitives::Address;
use std::{str::FromStr, sync::Arc};
use tokio::sync::oneshot;
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
    http::server::run_http_server,
    metrics,
    orchestrator::{
        traits::{EventHandler, HandlerRegistry},
        HealthCheck, Orchestrator, TokioEventDispatcher,
    },
    store::sql::repositories::{
        input_proof_repo::InputProofRepository, public_decrypt_repo::PublicDecryptRepository,
        user_decrypt_repo::UserDecryptRepository, Repositories,
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
    settings_sender: Option<oneshot::Sender<Settings>>,
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

    let tx_engine_gateway = GatewayTransactionEngine::new(
        settings.gateway.blockchain_rpc.clone(),
        settings.gateway.tx_engine.clone(),
    );

    // === Intialize the orchestrator.
    let orchestrator = Orchestrator::new(Arc::new(TokioEventDispatcher::<RelayerEvent>::new()));

    // Initialize SQL repositories
    let repositories = Arc::new(Repositories::new(settings.storage.clone()).await);
    info!("Initialized SQL repositories");

    // Register database with orchestrator for health checks
    orchestrator.add_health_check(
        "database".to_string(),
        repositories.clone() as Arc<dyn HealthCheck>,
    );

    // let gateway_tx_config = GatewayTxConfig::from(settings.transaction.clone());
    let gateway_tx_helper = Arc::new(GatewayTransactionHelper::new(
        settings.gateway.clone(),
        tx_engine_gateway.clone().into(),
    ));

    // Register gateway transaction helper with orchestrator for health checks
    orchestrator.add_health_check(
        "gateway_http".to_string(),
        gateway_tx_helper.clone() as Arc<dyn HealthCheck>,
    );

    // Create ReadinessChecker once to be shared by both decrypt handlers
    let readiness_checker = Arc::new(ReadinessChecker::new(&settings.gateway)?);

    // Parse decryption address once
    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invalid decryption address"))?;

    setup_input_proof_gateway_handler(
        &orchestrator,
        gateway_tx_helper.clone(),
        settings.gateway.contracts.clone(),
        repositories.input_proof.clone(),
    )?;

    setup_public_decrypt_gateway_handler(
        &orchestrator,
        gateway_tx_helper.clone(),
        readiness_checker.clone(),
        decryption_address,
        repositories.public_decrypt.clone(),
    )?;

    setup_user_decrypt_gateway_handler(
        &orchestrator,
        gateway_tx_helper.clone(),
        readiness_checker.clone(),
        decryption_address,
        settings.gateway.contracts.user_decrypt_shares_threshold as usize,
        repositories.user_decrypt.clone(),
    )?;

    // === Initialize gateway listener with reconnection configuration
    let listener_client_ws = ArbitrumJsonRPCWsClient::new(
        settings.gateway.blockchain_rpc.clone(),
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

    // Register gateway websocket client with orchestrator for health checks
    orchestrator.add_health_check(
        "gateway_ws".to_string(),
        listener_client_ws.clone() as Arc<dyn HealthCheck>,
    );

    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invalid decryption contract address"))?;

    let input_verification_address =
        Address::from_str(&settings.gateway.contracts.input_verification_address)
            .map_err(|_| eyre::eyre!("Invalid InputVerification address"))?;

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let gateway_contract_addresses = vec![decryption_address, input_verification_address];

    let latest_block_gateway = match settings.gateway.listener.last_block_number {
        Some(block_number) => Some(block_number),
        None => repositories
            .block_number
            .get_last_block_info()
            .await
            .map_err(|e| eyre::eyre!("Error getting last block number: {}", e))?
            .map(|info| info.block_number),
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
    orchestrator
        .spawn_task_and_wait_ready(
            "gateway_listener",
            arbitrum_listener(
                subscription_gateway,
                Arc::clone(&orchestrator),
                repositories.block_number.clone(),
            ),
            async { Ok(()) }, // Gateway listener doesn't have a specific readiness check
        )
        .await
        .map_err(|e| eyre::eyre!("Failed to start gateway listener: {}", e))?;

    // Setup KeyUrl gateway handler
    let keyurl_gateway_handler =
        setup_keyurl_gateway_handler(&orchestrator, settings.keyurl.clone())?;

    // Initialize KeyUrl cache with config data
    keyurl_gateway_handler.initialize().await;

    let mut settings = settings;

    // HTTP endpoint
    if settings.http.endpoint.is_some() {
        info!("Starting Relayer HTTP server");

        let addr = run_http_server(
            &settings.http,
            Arc::clone(&orchestrator),
            repositories.clone(),
        )
        .await;

        info!("HTTP server bound to actual address: {}", addr);
        settings.http.endpoint = Some(addr.to_string());
    };

    // Run metrics server
    info!("Starting Relayer metrics server");
    let actual_metrics_addr = metrics::server::run_metrics_server(
        registry_clone,
        metrics_endpoint,
        Arc::clone(&orchestrator),
    )
    .await;
    info!(
        "Metrics server bound to actual address: {}",
        actual_metrics_addr
    );
    settings.metrics.endpoint = actual_metrics_addr.to_string();

    // Initialize KeyUrl handler with config data
    // Must be after http server init, so that http handler can catch the event.
    keyurl_gateway_handler.initialize().await;

    drop(setup_span);

    info!("All servers are ready and responding");

    // Send settings through the channel if provided (for tests)
    if let Some(sender) = settings_sender {
        let _ = sender.send(settings.clone());
        info!("Settings sent to test setup with actual server addresses");
    }

    // === Wait for shutdown signal and shutdown all tasks via orchestrator
    orchestrator
        .run_until_shutdown(shutdown_token)
        .await
        .map_err(|e| eyre::eyre!("Failed during shutdown: {}", e))?;

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

/// Setup KeyUrl gateway handler - emits events through orchestrator.
fn setup_keyurl_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    config: crate::config::settings::KeyUrl,
) -> eyre::Result<KeyUrlGatewayHandler> {
    let handler = KeyUrlGatewayHandler::new(Arc::clone(orchestrator), config);

    Ok(handler)
}
