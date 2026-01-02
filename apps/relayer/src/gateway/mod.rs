pub mod arbitrum;
pub mod input_handlers;
pub mod keyurl_handler;
pub mod public_decrypt_handler;
pub mod readiness_checker;
pub mod user_decrypt_handler;
pub mod utils;

pub use input_handlers::InputProofGatewayHandler;
pub use keyurl_handler::KeyUrlGatewayHandler;
pub use public_decrypt_handler::GatewayHandler as PublicDecryptGatewayHandler;
pub use readiness_checker::ReadinessChecker;
pub use user_decrypt_handler::GatewayHandler as UserDecryptGatewayHandler;

use crate::config::settings::Settings;
use crate::core::event::RelayerEvent;
use crate::gateway::arbitrum::transaction::pool::{GatewayTask, Mempool};
use crate::gateway::arbitrum::transaction::processor::GatewayTxProcessor;
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

    // Create mempool
    // TODO: Change with config settings.
    let mempool = Arc::new(Mempool::<GatewayTask>::new(20));

    // Spawn gateway task
    GatewayTxProcessor::spawn(
        mempool.clone(),
        gateway_tx_helper.clone(),
        orchestrator.clone(),
    );

    // Create ReadinessChecker to be shared by decrypt handlers
    let readiness_checker = Arc::new(ReadinessChecker::new(&settings.gateway)?);

    // Parse addresses for handlers (listener parses its own from config)
    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;

    // Initialize all gateway components (each handles its own orchestrator registration)
    InputProofGatewayHandler::new(
        orchestrator.clone(),
        mempool.clone(),
        settings.gateway.contracts.clone(),
        repositories.input_proof.clone(),
    );

    PublicDecryptGatewayHandler::new(
        orchestrator.clone(),
        mempool.clone(),
        readiness_checker.clone(),
        decryption_address,
        repositories.public_decrypt.clone(),
    );

    UserDecryptGatewayHandler::new(
        orchestrator.clone(),
        mempool.clone(),
        readiness_checker,
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
        let listener = Arc::new(
            ArbitrumListener::new(
                settings.gateway.clone(),
                orchestrator.clone(),
                repositories.block_number.clone(),
                deduplicator.clone(),
                instance_id,
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
