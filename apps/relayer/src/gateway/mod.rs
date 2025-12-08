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
use crate::orchestrator::{HealthCheck, Orchestrator, TokioEventDispatcher};
use crate::store::sql::repositories::Repositories;
use alloy::primitives::Address;
use arbitrum::{
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

    // Create ReadinessChecker to be shared by decrypt handlers
    let readiness_checker = Arc::new(ReadinessChecker::new(&settings.gateway)?);

    // Parse addresses for handlers (listener parses its own from config)
    let decryption_address = Address::from_str(&settings.gateway.contracts.decryption_address)
        .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;

    // Initialize all gateway components (each handles its own orchestrator registration)
    InputProofGatewayHandler::new(
        orchestrator.clone(),
        gateway_tx_helper.clone(),
        settings.gateway.contracts.clone(),
        repositories.input_proof.clone(),
    );

    PublicDecryptGatewayHandler::new(
        orchestrator.clone(),
        gateway_tx_helper.clone(),
        readiness_checker.clone(),
        decryption_address,
        repositories.public_decrypt.clone(),
    );

    UserDecryptGatewayHandler::new(
        orchestrator.clone(),
        gateway_tx_helper.clone(),
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

    // Initialize gateway blockchain listener (spawned as task)
    let listener_for_health = Arc::new(
        ArbitrumListener::new(
            settings.gateway.clone(),
            orchestrator.clone(),
            repositories.block_number.clone(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize gateway listener: {}", e))?,
    );

    // Register listener for health monitoring
    orchestrator.add_health_check(
        "gateway_ws".to_string(),
        listener_for_health.clone() as Arc<dyn HealthCheck>,
    );

    // Create listener for running (run() consumes self)
    let listener_for_run = ArbitrumListener::new(
        settings.gateway.clone(),
        orchestrator.clone(),
        repositories.block_number.clone(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Failed to initialize gateway listener: {}", e))?;

    orchestrator
        .spawn_task_and_wait_ready(
            "gateway_listener",
            async move {
                if let Err(e) = listener_for_run.run().await {
                    error!("Gateway listener failed: {}", e);
                }
            },
            async move { listener_for_health.check().await }, // Real readiness check
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to start gateway listener: {}", e))?;

    // Create KeyUrl handler (but don't initialize yet - that happens after HTTP server)
    let keyurl_handler =
        KeyUrlGatewayHandler::new(Arc::clone(&orchestrator), settings.keyurl.clone());

    Ok(keyurl_handler)
}
