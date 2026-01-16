pub mod arbitrum;
pub mod input_handlers;
pub mod keyurl_handler;
pub mod public_decrypt_handler;
pub mod readiness_check;
pub mod throttlers;
pub mod user_decrypt_handler;
pub mod utils;

pub use input_handlers::InputProofGatewayHandler;
pub use keyurl_handler::KeyUrlGatewayHandler;
pub use public_decrypt_handler::GatewayHandler as PublicDecryptGatewayHandler;
pub use user_decrypt_handler::GatewayHandler as UserDecryptGatewayHandler;

use crate::config::settings::Settings;
use crate::core::event::RelayerEvent;
use crate::gateway::arbitrum::transaction::tx_processor::GatewayTxProcessor;
use crate::gateway::readiness_check::public_decrypt_processor::PublicDecryptReadinessProcessor;
use crate::gateway::readiness_check::readiness_checker::ReadinessChecker;
use crate::gateway::readiness_check::user_decrypt_processor::UserDecryptReadinessProcessor;
use crate::gateway::throttlers::GatewayThrottlers;
use crate::orchestrator::{HealthCheck, Orchestrator, TokioEventDispatcher};
use crate::store::sql::repositories::Repositories;
use alloy::primitives::Address;
use arbitrum::{
    event_deduplicator::EventDeduplicator,
    transaction::{
        helper::GatewayTransactionEngine, TransactionHelper as GatewayTransactionHelper,
    },
    ArbitrumListener,
};
use std::{str::FromStr, sync::Arc};
use tracing::{error, info};

/// Initialize all gateway components including handlers, listener, and KeyUrl handler
pub async fn initialize_gateway(
    orchestrator: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    settings: &Settings,
    repositories: Arc<Repositories>,
    gateway_throttlers: GatewayThrottlers,
) -> anyhow::Result<KeyUrlGatewayHandler> {
    info!("Initializing gateway components");

    // Create transaction engine and helper
    let tx_engine_gateway = GatewayTransactionEngine::new(
        settings.gateway.blockchain_rpc.clone(),
        settings.gateway.tx_engine.clone(),
    );

    let gateway_tx_helper = Arc::new(GatewayTransactionHelper::new(
        settings.gateway.clone(),
        tx_engine_gateway.into(),
    ));

    // Spawn gateway task for input proof throttler.
    GatewayTxProcessor::orchestrator_spawn_task(
        gateway_throttlers.tx_throttlers.input_proof_tx_worker,
        gateway_tx_helper.clone(),
        orchestrator.clone(),
    )
    .await?;

    // Spawn gateway task for public decrypt throttler.
    GatewayTxProcessor::orchestrator_spawn_task(
        gateway_throttlers.tx_throttlers.public_decrypt_tx_worker,
        gateway_tx_helper.clone(),
        orchestrator.clone(),
    )
    .await?;

    // Spawn gateway task for user decrypt throttler.
    GatewayTxProcessor::orchestrator_spawn_task(
        gateway_throttlers.tx_throttlers.user_decrypt_tx_worker,
        gateway_tx_helper.clone(),
        orchestrator.clone(),
    )
    .await?;

    // Create ReadinessChecker to be shared by decrypt handlers
    let readiness_checker = Arc::new(ReadinessChecker::new(&settings.gateway)?);

    PublicDecryptReadinessProcessor::orchestrator_spawn_task(
        gateway_throttlers
            .readiness_throttlers
            .public_decrypt_readiness_worker,
        readiness_checker.clone(),
        orchestrator.clone(),
    )
    .await?;

    UserDecryptReadinessProcessor::orchestrator_spawn_task(
        gateway_throttlers
            .readiness_throttlers
            .user_decrypt_readiness_worker,
        readiness_checker.clone(),
        orchestrator.clone(),
    )
    .await?;

    // Parse addresses for handlers (listener parses its own from config)
    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;

    // Initialize all gateway components (each handles its own orchestrator registration)
    InputProofGatewayHandler::new(
        orchestrator.clone(),
        gateway_throttlers
            .tx_throttlers
            .input_proof_tx_throttler
            .clone(),
        settings.gateway.contracts.clone(),
        repositories.input_proof.clone(),
    );

    PublicDecryptGatewayHandler::new(
        orchestrator.clone(),
        gateway_throttlers
            .tx_throttlers
            .public_decrypt_tx_throttler
            .clone(),
        gateway_throttlers
            .readiness_throttlers
            .public_decrypt_readiness_throttler
            .clone(),
        decryption_address,
        repositories.public_decrypt.clone(),
    );

    UserDecryptGatewayHandler::new(
        orchestrator.clone(),
        gateway_throttlers
            .tx_throttlers
            .user_decrypt_tx_throttler
            .clone(),
        gateway_throttlers
            .readiness_throttlers
            .user_decrypt_readiness_throttler
            .clone(),
        decryption_address,
        settings.gateway.contracts.user_decrypt_shares_threshold as usize,
        repositories.user_decrypt.clone(),
    );

    // Register transaction helper with orchestrator for health checks
    orchestrator.add_health_check(
        "gateway_http".to_string(),
        gateway_tx_helper.clone() as Arc<dyn HealthCheck>,
    );

    // Create shared event deduplicator
    let deduplicator = Arc::new(EventDeduplicator::new(
        settings.gateway.listener.dedup_ttl_seconds,
        settings.gateway.listener.dedup_max_capacity,
    ));

    // Get number of listener instances from configuration
    let listener_instances = settings.gateway.listener.listener_instances;
    info!(
        "Initializing {} gateway listener instances",
        listener_instances
    );

    // Initialize and spawn multiple listener instances
    for instance_id in 0..listener_instances {
        // Get per-instance WebSocket URL (falls back to default if not configured)
        let ws_url = settings
            .gateway
            .listener
            .get_ws_url_for_instance(instance_id, &settings.gateway.blockchain_rpc);

        info!("Listener {} using ws_url: {}", instance_id, ws_url);

        let listener = Arc::new(
            ArbitrumListener::new(
                settings.gateway.clone(),
                orchestrator.clone(),
                repositories.block_number.clone(),
                deduplicator.clone(),
                instance_id,
                ws_url,
                listener_instances,
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to initialize gateway listener {}: {}",
                    instance_id,
                    e
                )
            })?,
        );

        let task_name = format!("gateway_listener_{}", instance_id);

        // Register THIS listener's health check
        orchestrator.add_health_check(
            format!("gateway_ws_{}", instance_id),
            listener.clone() as Arc<dyn HealthCheck>,
        );

        // Spawn listener and wait for it to be ready (verifies gateway connection)
        let listener_clone = listener.clone();
        let health_listener = listener.clone();
        orchestrator
            .spawn_task_and_wait_ready(
                &task_name,
                async move {
                    if let Err(e) = listener_clone.run().await {
                        error!("Gateway listener {} failed: {}", instance_id, e);
                    }
                },
                async move { health_listener.check().await },
            )
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to start gateway listener {}: {}", instance_id, e)
            })?;
    }

    // Create KeyUrl handler (but don't initialize yet - that happens after HTTP server)
    let keyurl_handler =
        KeyUrlGatewayHandler::new(Arc::clone(&orchestrator), settings.keyurl.clone());

    Ok(keyurl_handler)
}
