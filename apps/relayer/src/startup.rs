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

use crate::gateway::{self, throttlers::init_throttlers};
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use tracing::{info, span, Level};

use crate::{
    config::settings::Settings,
    core::event::RelayerEvent,
    http::server::run_http_server,
    metrics,
    orchestrator::{HealthCheck, Orchestrator, TokioEventDispatcher},
    store::sql::repositories::Repositories,
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
    // === Setup Phase ===
    // Initialize logging, metrics, and validate configuration
    info!("Starting relayer with configuration: {:?}", settings);

    let main_span = span!(Level::INFO, "main-span"); // Add other relevant top-level details
    let setup_span = span!(parent: &main_span, Level::INFO, "setup-span");
    let metrics_registry = ensure_global_init(&settings)?;
    let metrics_endpoint = settings.metrics.endpoint.clone();
    let registry_clone = metrics_registry.clone();

    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;
    settings
        .validate_listener_config()
        .map_err(|e| eyre::eyre!("Listener configuration validation failed: {}", e))?;

    // === Orchestration Phase ===
    // Create orchestrator, repositories, and gateway components
    let orchestrator = Orchestrator::new(Arc::new(TokioEventDispatcher::<RelayerEvent>::new()));

    // Initialize SQL repositories
    let repositories = Arc::new(
        Repositories::new(settings.storage.clone())
            .await
            .map_err(|e| eyre::eyre!("Failed to initialize SQL repositories: {}", e))?,
    );
    info!("Initialized SQL repositories");

    if !settings.global.test_mock {
        // Register background workers with orchestrator (timeout, expiry cron jobs, and DB pool monitor)
        repositories
            .register_background_workers(&orchestrator, settings.storage.cron.clone())
            .await
            .map_err(|e| eyre::eyre!("Failed to register background workers: {}", e))?;
    }

    // Register database with orchestrator for health checks
    orchestrator.add_health_check(
        "database".to_string(),
        repositories.clone() as Arc<dyn HealthCheck>,
    );

    let (gateway_throttlers, bouncer_throttlers) = init_throttlers(&settings);

    // Initialize all gateway components
    let gateway_handler = gateway::initialize_gateway(
        orchestrator.clone(),
        &settings,
        repositories.clone(),
        gateway_throttlers,
    )
    .await
    .map_err(|e| eyre::eyre!("Failed to initialize gateway: {}", e))?;

    let mut settings = settings;

    // === Services Phase ===
    // Start HTTP server, metrics server, and initialize handlers
    if settings.http.endpoint.is_some() {
        info!("Starting Relayer HTTP server");

        let addr = run_http_server(
            &settings.http,
            Arc::clone(&orchestrator),
            repositories.clone(),
            settings.gateway.contracts.user_decrypt_shares_threshold,
            bouncer_throttlers,
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

    // Initialize KeyUrl handler after HTTP server is up
    gateway_handler.initialize().await;

    drop(setup_span);

    info!("All servers are ready and responding");

    // Send settings through the channel if provided (for tests)
    if let Some(sender) = settings_sender {
        let _ = sender.send(settings.clone());
        info!("Settings sent to test setup with actual server addresses");
    }

    // === Runtime Phase ===
    // Wait for shutdown signal and shutdown all tasks via orchestrator
    orchestrator
        .run_until_shutdown(shutdown_token)
        .await
        .map_err(|e| eyre::eyre!("Failed during shutdown: {}", e))?;

    // Ensure pools close cleanly before exit.
    repositories.close_pools().await;

    info!("Relayer shutdown complete");

    Ok(())
}

/// Initialize all global state exactly once
fn ensure_global_init(settings: &Settings) -> eyre::Result<&'static Registry> {
    let registry = GLOBAL_REGISTRY.get_or_init(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to install AWS-LC crypto provider");

        let registry = Registry::new();
        metrics::init_http_metrics(&registry, &settings.http.metrics);
        metrics::init_transaction_metrics(&registry, settings.metrics.clone());
        metrics::init_statuses_metrics(&registry, settings.metrics.clone());
        metrics::init_db_metrics(&registry, settings.metrics.clone());
        metrics::init_queue_metrics(&registry);

        registry
    });

    Ok(registry)
}
