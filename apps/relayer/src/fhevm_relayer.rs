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
//! - [`FhevmHandler`]: Processes fhevm blockchain events and responses
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
//!
//! # Event Flow
//!
//! ```text
//! [fhevm] → [fhevm listener] → [Orchestrator] → [gateway Handler]
//!                                       ↓
//! [fhevm] ← [fhevm Handler] ← [Orchestrator] ← [gateway Listener]
//! ```

use crate::store::{
    BlockNumberStore, PublicDecryptRequestCacheStore, PublicDecryptResponseCacheStore,
    UserDecryptRequestCacheStore, UserDecryptResponseCacheStore, UserDecryptResponseStore,
};
use alloy::primitives::Address;
use alloy::signers::Signer;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::{str::FromStr, sync::Arc};
use tokio_util::sync::CancellationToken;
use tracing::{info, span, Level};

use crate::{
    blockchain::{
        fhevm::ethereum::listener::ethereum_listener as fhevm_ethereum_listener,
        fhevm::ethereum::{
            parse_private_key, ChainName, ContractAndTopicsFilter, EthereumJsonRPCWsClient,
        },
        gateway::arbitrum::listener::ethereum_listener as gateway_ethereum_listener,
        InputProofGatewayHandler, PublicDecryptFhevmHandler, PublicDecryptGatewayHandler,
        UserDecryptGatewayHandler,
    },
    blockchain::{
        fhevm::ethereum::transaction::{
            TransactionService as FhevmTransactionService, TxConfig as FhevmTxConfig,
        },
        gateway::arbitrum::transaction::{
            TransactionService as GatewayTransactionService, TxConfig as GatewayTxConfig,
        },
    },
    config::settings::Settings,
    core::event::{
        GatewayChainEventId, HostChainEventId, InputProofEventId, PublicDecryptEventId,
        RelayerEvent, UserDecryptEventId,
    },
    http::http_server::run_http_server,
    metrics,
    orchestrator::{
        hooks::EventPersistenceHook,
        traits::{EventHandler, HandlerRegistry, HookRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    store::{key_value_db::RocksDBKVStore, EventStore},
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
    let metrics_endpoint = settings.metrics_endpoint.clone();
    let registry_clone = metrics_registry.clone();

    // 3. Validate settings
    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;

    // 4. Init handlers
    //
    //
    let mut task_set = tokio::task::JoinSet::new();

    // 4.1 Transaction servicve for fhevm
    //
    // TODO: change this to accomodate generic signers
    // as it should already be supported in the lib
    let mut fhevm_signer = parse_private_key(&settings.transaction.private_key_fhevm)?;
    fhevm_signer.set_chain_id(Some(settings.networks.fhevm.chain_id));

    // Prepare tx service for fhevm
    let tx_service_host =
        FhevmTransactionService::new(&settings.networks.fhevm.ws_url, Arc::new(fhevm_signer))
            .await
            .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    Arc::clone(&tx_service_host).spawn_maintenance_tasks(
        tokio::time::Duration::from_secs(5),
        tokio::time::Duration::from_secs(10),
    );

    let decryption_oracle_address =
        Address::from_str(&settings.contracts.decryption_oracle_address)
            .map_err(|_| eyre::eyre!("Invalid decryption oracle address"))?;

    // 4.2 Gateway settings
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .map_err(|e| eyre::eyre!("Failed to get gateway settings: {}", e))?;

    let mut gateway_signer = parse_private_key(&settings.transaction.private_key_gateway)?;
    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    // Prepare tx service for gateway
    let tx_service_gateway =
        GatewayTransactionService::new(&gateway_settings.http_url, Arc::new(gateway_signer))
            .await
            .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    Arc::clone(&tx_service_gateway).spawn_maintenance_tasks(
        tokio::time::Duration::from_secs(5),
        tokio::time::Duration::from_secs(10),
    );

    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invalid decryption contract address"))?;

    let input_verification_address =
        Address::from_str(&settings.contracts.input_verification_address)
            .map_err(|_| eyre::eyre!("Invalid InputVerification address"))?;

    // === Intialize the orchestrator.
    let orchestrator = Orchestrator::new(Arc::new(TokioEventDispatcher::<RelayerEvent>::new()));

    // Create root storage
    let path_rocks_db = PathBuf::from(settings.db_path_rocksdb);
    let kv_store = RocksDBKVStore::open(path_rocks_db.clone())
        .map_err(|e| eyre::eyre!("Failed to open RocksDB: {}", e))?;
    info!("using rocks db databse at: {}", path_rocks_db.display());
    let kv_store = Arc::new(kv_store);

    // Init and register event persistence hook
    let event_store = Arc::new(EventStore::<RelayerEvent>::new(kv_store.clone()));
    orchestrator.register_pre_dispatch_hook(EventPersistenceHook::<RelayerEvent>::new(
        event_store.clone(),
    ));

    // === Register the event handlers
    let fhevm_tx_config = FhevmTxConfig::from(settings.transaction.clone());
    setup_public_decrypt_fhevm_handler(
        &orchestrator,
        tx_service_host.clone(),
        fhevm_tx_config,
        settings.networks.fhevm.chain_id,
    )?;

    let gateway_tx_config = GatewayTxConfig::from(settings.transaction.clone());
    setup_input_proof_gateway_handler(
        &orchestrator,
        tx_service_gateway.clone(),
        gateway_tx_config.clone(),
        settings.contracts.clone(),
        settings.networks.gateway.chain_id,
    )?;

    setup_public_decrypt_gateway_handler(
        &orchestrator,
        kv_store.clone(),
        tx_service_gateway.clone(),
        gateway_tx_config.clone(),
        settings.contracts.clone(),
        gateway_settings.http_url.clone(),
        settings.transaction.clone().ciphertext_check_retry.clone(),
        gateway_settings.chain_id,
    )?;

    setup_user_decrypt_gateway_handler(
        &orchestrator,
        kv_store.clone(),
        tx_service_gateway,
        gateway_tx_config,
        settings.contracts,
        gateway_settings.http_url.clone(),
        settings.transaction.clone().ciphertext_check_retry.clone(),
        settings.networks.gateway.chain_id,
    )?;

    // === Initialize fhevm listener
    let fhevm = EthereumJsonRPCWsClient::new(ChainName::Fhevm, &settings.networks.fhevm.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler: {}", e))?;
    let fhevm = Arc::new(fhevm);

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let filter_fhevm = ContractAndTopicsFilter::new(vec![decryption_oracle_address], vec![]);
    let fhevm_block_store = Arc::new(BlockNumberStore::new(kv_store.clone(), "fhevm".to_string()));
    let latest_block_fhevm = match settings.networks.fhevm.last_block_number {
        Some(block_number) => Some(block_number),
        None => fhevm_block_store
            .get_last_block_number()
            .await
            .map_err(|e| eyre::eyre!("Error getting last block number: {}", e))?,
    };
    info!(
        "start listening from block \"{}\" on host chain",
        latest_block_fhevm
            .map(|b| b.to_string())
            .unwrap_or("latest".to_string())
    );
    let subscription_fhevm = fhevm
        .new_subscription(filter_fhevm, latest_block_fhevm)
        .await?;
    info!("Starting Relayer fhevm Listener");
    task_set.spawn(fhevm_ethereum_listener(
        subscription_fhevm,
        Arc::clone(&orchestrator),
        Arc::clone(&fhevm_block_store),
    ));

    // === Initialize gateway listener
    let gateway = EthereumJsonRPCWsClient::new(ChainName::Gateway, &gateway_settings.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler for gateway: {}", e))?;
    let gateway = Arc::new(gateway);

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let filter_gateway =
        ContractAndTopicsFilter::new(vec![decryption_address, input_verification_address], vec![]);

    let gateway_block_store = Arc::new(BlockNumberStore::new(
        kv_store.clone(),
        "gateway".to_string(),
    ));
    let latest_block_gateway = match settings.networks.gateway.last_block_number {
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
    let subscription_gateway = gateway
        .new_subscription(filter_gateway, latest_block_gateway)
        .await?;
    info!("Starting Relayer Gateway Listener");
    task_set.spawn(gateway_ethereum_listener(
        subscription_gateway,
        Arc::clone(&orchestrator),
        Arc::clone(&gateway_block_store),
    ));

    // HTTP endpoint
    if let Some(http_config) = settings.http_endpoint {
        info!("Starting Relayer HTTP server");
        let addr: SocketAddr = http_config.parse().expect("Invalid http-endpoint address");
        task_set.spawn(run_http_server(
            addr,
            Arc::clone(&orchestrator),
            settings.keyurl,
            gateway_settings.ws_url,
            settings.networks.fhevm.ws_url,
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
        metrics::init_http_metrics(&registry, &settings.http_metrics);
        metrics::init_cache_metrics(&registry);
        metrics::init_transaction_metrics(&registry);

        registry
    });

    Ok(registry)
}

/// Setup PublicDecryptFhevmHandler and register its events
fn setup_public_decrypt_fhevm_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_service: Arc<FhevmTransactionService>,
    tx_config: FhevmTxConfig,
    chain_id: u64,
) -> eyre::Result<()> {
    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(PublicDecryptFhevmHandler::new(
        Arc::clone(orchestrator),
        tx_service,
        tx_config,
        chain_id,
    ));

    let event_ids = [
        HostChainEventId::EventLogRcvd.into(),
        PublicDecryptEventId::RespRcvdFromGw.into(),
        PublicDecryptEventId::RespSentToFhevm.into(),
    ];

    register_handler_for_events(orchestrator, handler, &event_ids);
    Ok(())
}

/// Setup InputProofGatewayHandler and register its events
fn setup_input_proof_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_service: Arc<GatewayTransactionService>,
    tx_config: GatewayTxConfig,
    contracts: crate::config::settings::ContractConfig,
    chain_id: u64,
) -> eyre::Result<()> {
    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(InputProofGatewayHandler::new(
        Arc::clone(orchestrator),
        tx_service,
        tx_config,
        contracts,
        chain_id,
    ));

    let event_ids = [
        InputProofEventId::ReqRcvdFromUser.into(),
        InputProofEventId::ReqSentToGw.into(),
        InputProofEventId::RespRcvdFromGw.into(),
        GatewayChainEventId::EventLogRcvd.into(),
    ];

    register_handler_for_events(orchestrator, handler, &event_ids);
    Ok(())
}

/// Setup PublicDecryptGatewayHandler and register its events
fn setup_public_decrypt_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    kv_store: Arc<RocksDBKVStore>,
    tx_service: Arc<GatewayTransactionService>,
    tx_config: GatewayTxConfig,
    contracts: crate::config::settings::ContractConfig,
    http_url: String,
    retry_config: crate::config::settings::RetrySettings,
    chain_id: u64,
) -> eyre::Result<()> {
    let public_decrypt_responses_cache =
        Arc::new(PublicDecryptResponseCacheStore::new(kv_store.clone()));
    let public_decrypt_requests_cache = Arc::new(PublicDecryptRequestCacheStore::new(kv_store));
    let public_decrypt_caches =
        crate::blockchain::gateway::public_decrypt_handler::PublicDecryptCaches {
            responses: public_decrypt_responses_cache,
            requests: public_decrypt_requests_cache,
        };

    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(PublicDecryptGatewayHandler::new(
        Arc::clone(orchestrator),
        public_decrypt_caches,
        tx_service,
        tx_config,
        contracts,
        http_url,
        retry_config,
        chain_id,
    ));

    let event_ids = [
        PublicDecryptEventId::ReqRcvdFromUser.into(),
        PublicDecryptEventId::ReqRcvdFromFhevm.into(),
        PublicDecryptEventId::ReqSentToGw.into(),
        GatewayChainEventId::EventLogRcvd.into(),
    ];

    register_handler_for_events(orchestrator, handler, &event_ids);
    Ok(())
}

/// Setup UserDecryptGatewayHandler and register its events
fn setup_user_decrypt_gateway_handler(
    orchestrator: &Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    kv_store: Arc<RocksDBKVStore>,
    tx_service: Arc<GatewayTransactionService>,
    tx_config: GatewayTxConfig,
    contracts: crate::config::settings::ContractConfig,
    http_url: String,
    retry_config: crate::config::settings::RetrySettings,
    chain_id: u64,
) -> eyre::Result<()> {
    let user_decrypt_responses_cache =
        Arc::new(UserDecryptResponseCacheStore::new(kv_store.clone()));
    let user_decrypt_requests_cache = Arc::new(UserDecryptRequestCacheStore::new(kv_store));

    let user_decrypt_shares_threshold = contracts.user_decrypt_shares_threshold;
    let user_decrypt_response_store =
        Arc::new(UserDecryptResponseStore::new(user_decrypt_shares_threshold));

    let handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(UserDecryptGatewayHandler::new(
        Arc::clone(orchestrator),
        user_decrypt_responses_cache,
        user_decrypt_requests_cache,
        user_decrypt_response_store,
        tx_service,
        tx_config,
        contracts,
        http_url,
        retry_config,
        chain_id,
    ));

    let event_ids = [
        UserDecryptEventId::ReqRcvdFromUser.into(),
        UserDecryptEventId::ReqSentToGw.into(),
        GatewayChainEventId::EventLogRcvd.into(),
    ];

    register_handler_for_events(orchestrator, handler, &event_ids);
    Ok(())
}
