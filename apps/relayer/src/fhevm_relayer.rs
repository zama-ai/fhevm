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
    http::{server::run_http_server, HealthCheck, HealthChecker},
    metrics,
    orchestrator::{
        traits::{EventHandler, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
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
    mut settings: Settings,
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
    let mut task_set = tokio::task::JoinSet::new();

    let tx_engine_gateway = GatewayTransactionEngine::new(
        settings.gateway.blockchain_rpc.clone(),
        settings.gateway.tx_engine.clone(),
    );

    // === Intialize the orchestrator.
    let orchestrator = Orchestrator::new(Arc::new(TokioEventDispatcher::<RelayerEvent>::new()));

    // Initialize SQL repositories
    let repositories = Arc::new(Repositories::new(settings.storage.clone()).await);
    info!("Initialized SQL repositories");

    // let gateway_tx_config = GatewayTxConfig::from(settings.transaction.clone());
    let gateway_tx_helper = Arc::new(GatewayTransactionHelper::new(
        settings.gateway.clone(),
        tx_engine_gateway.clone().into(),
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
    task_set.spawn(arbitrum_listener(
        subscription_gateway,
        Arc::clone(&orchestrator),
        repositories.block_number.clone(),
    ));

    // Setup KeyUrl gateway handler
    let keyurl_gateway_handler =
        setup_keyurl_gateway_handler(&orchestrator, settings.keyurl.clone())?;

    // Initialize KeyUrl cache with config data
    keyurl_gateway_handler.initialize().await;

    // HTTP endpoint
    if let Some(_http_endpoint) = settings.http.endpoint.clone() {
        info!("Starting Relayer HTTP server");

        // Set up health checker with composable health checks
        let mut health_checker = HealthChecker::new();

        // Add Gateway RPC health check (using transaction helper directly)
        health_checker.add_health_check(
            "gateway_rpc".to_string(),
            gateway_tx_helper.clone() as Arc<dyn HealthCheck>,
        );

        // Add Gateway WebSocket health check (using WebSocket client directly)
        health_checker.add_health_check(
            "gateway_websocket".to_string(),
            listener_client_ws.clone() as Arc<dyn HealthCheck>,
        );

        // Add Database health check (using Repositories)
        health_checker.add_health_check(
            "database".to_string(),
            repositories.clone() as Arc<dyn HealthCheck>,
        );

        let health_checker = Arc::new(health_checker);

        let actual_http_addr = run_http_server(
            &settings.http,
            Arc::clone(&orchestrator),
            health_checker,
            repositories.clone(),
        )
        .await;
        // Update settings with the actual bound address
        settings.http.endpoint = Some(actual_http_addr.to_string());
        info!("HTTP server bound to actual address: {}", actual_http_addr);
    };

    // Run metrics server
    let actual_metrics_addr =
        metrics::server::run_metrics_server(registry_clone, metrics_endpoint).await;
    // Update settings with the actual bound address
    settings.metrics.endpoint = actual_metrics_addr.to_string();
    info!(
        "Metrics server bound to actual address: {}",
        actual_metrics_addr
    );

    // Initialize KeyUrl handler with config data
    // Must be after http server init, so that http handler can catch the event.
    keyurl_gateway_handler.initialize().await;

    drop(setup_span);

    // Perform self-check to ensure servers are ready
    wait_for_servers_ready(
        settings.http.endpoint.as_deref(),
        &settings.metrics.endpoint,
    )
    .await?;
    info!("All servers are ready and responding");

    // Send settings through the channel if provided (for tests)
    if let Some(sender) = settings_sender {
        let _ = sender.send(settings.clone());
        info!("Settings sent to test setup");
    }

    // === Wait for shutdown signal via cancellation token
    shutdown_token.cancelled().await;
    task_set.shutdown().await;

    info!("Relayer shutdown complete");

    Ok(())
}

/// Wait for servers to be ready by performing health checks
async fn wait_for_servers_ready(
    http_endpoint: Option<&str>,
    metrics_endpoint: &str,
) -> eyre::Result<()> {
    use std::time::Duration;

    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY: Duration = Duration::from_millis(200);

    // Check HTTP server if configured
    if let Some(http_endpoint) = http_endpoint {
        let url = format!("http://{}/liveness", http_endpoint);
        let mut retries = 0;

        loop {
            match reqwest::get(&url).await {
                Ok(response) if response.status().is_success() => {
                    info!("HTTP server health check passed");
                    break;
                }
                _ => {
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        return Err(eyre::eyre!("HTTP server failed to start within timeout"));
                    }
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

    // Check metrics server
    {
        let url = format!("http://{}/health", metrics_endpoint);
        let mut retries = 0;

        loop {
            match reqwest::get(&url).await {
                Ok(response) if response.status().is_success() => {
                    info!("Metrics server health check passed");
                    break;
                }
                _ => {
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        return Err(eyre::eyre!("Metrics server failed to start within timeout"));
                    }
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

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
