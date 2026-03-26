pub mod arbitrum;
pub mod ciphertext_checker;
pub mod input_handlers;
pub mod keyurl_handler;
pub mod public_decrypt_handler;
pub mod throttlers;
pub mod user_decrypt_handler;
pub mod utils;

pub use input_handlers::InputProofGatewayHandler;
pub use keyurl_handler::KeyUrlGatewayHandler;
pub use public_decrypt_handler::GatewayHandler as PublicDecryptGatewayHandler;
pub use user_decrypt_handler::GatewayHandler as UserDecryptGatewayHandler;

use crate::config::settings::{ListenerType, Settings};
use crate::gateway::arbitrum::transaction::tx_processor::GatewayTxProcessor;
use crate::gateway::throttlers::GatewayThrottlers;
use crate::host::HostAclChecker;
use crate::orchestrator::{HealthCheck, Orchestrator};
use crate::readiness::{
    checker::ReadinessChecker,
    delegated_user_decrypt_processor::DelegatedUserDecryptReadinessProcessor,
    public_decrypt_processor::PublicDecryptReadinessProcessor,
    user_decrypt_processor::UserDecryptReadinessProcessor,
};
use crate::store::sql::repositories::Repositories;
use alloy::primitives::Address;
use arbitrum::{
    event_deduplicator::EventDeduplicator,
    transaction::{
        helper::GatewayTransactionEngine, TransactionHelper as GatewayTransactionHelper,
    },
    ArbitrumListener, PollingListener,
};
use std::{str::FromStr, sync::Arc};
use tracing::{error, info};

/// Initialize all gateway components including handlers, listener, and KeyUrl handler
pub async fn initialize_gateway(
    orchestrator: Arc<Orchestrator>,
    settings: &Settings,
    repositories: Arc<Repositories>,
    gateway_throttlers: GatewayThrottlers,
) -> anyhow::Result<KeyUrlGatewayHandler> {
    info!("Initializing gateway components");

    // Create transaction engine and helper
    let tx_engine_gateway = GatewayTransactionEngine::new(
        settings.gateway.blockchain_rpc.clone(),
        settings.gateway.tx_engine.clone(),
    )
    .await?;

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

    // Create ReadinessChecker (host ACL + gateway ciphertext) to be shared by decrypt handlers
    let host_acl_checker = HostAclChecker::new(
        &settings.host_chains,
        settings
            .gateway
            .readiness_checker
            .host_acl_check
            .retry
            .clone(),
    )?;
    let readiness_checker = Arc::new(ReadinessChecker::new(host_acl_checker, &settings.gateway)?);

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

    DelegatedUserDecryptReadinessProcessor::orchestrator_spawn_task(
        gateway_throttlers
            .readiness_throttlers
            .delegated_user_decrypt_readiness_worker,
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
        settings.gateway.gw_event_not_found_retry.clone(),
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
        settings.gateway.gw_event_not_found_retry.clone(),
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
        gateway_throttlers
            .readiness_throttlers
            .delegated_user_decrypt_readiness_throttler
            .clone(),
        repositories.user_decrypt.clone(),
        user_decrypt_handler::UserDecryptHandlerConfig {
            decryption_address,
            shares_threshold: settings.gateway.contracts.user_decrypt_shares_threshold as usize,
            gw_event_retry: settings.gateway.gw_event_not_found_retry.clone(),
        },
    );

    // Register transaction helper with orchestrator for health checks
    orchestrator.add_health_check(
        "gateway_http".to_string(),
        gateway_tx_helper.clone() as Arc<dyn HealthCheck>,
    );

    // Create shared event deduplicator
    let pool_config = &settings.gateway.listener_pool;
    let deduplicator = Arc::new(EventDeduplicator::new(
        pool_config.dedup_ttl_seconds,
        pool_config.dedup_max_capacity,
    ));

    // Count only WebSocket listeners for stagger calculation
    // Staggered recycling is only needed for WS connections to prevent all listeners
    // from recycling at the same time. Polling listeners don't need staggering.
    let num_ws_listeners = pool_config
        .listeners
        .iter()
        .filter(|l| matches!(l.listener_type, ListenerType::Subscription))
        .count();
    let num_listeners = pool_config.listeners.len();
    info!(
        "Initializing {} gateway listeners from pool ({} WebSocket, {} polling)",
        num_listeners,
        num_ws_listeners,
        num_listeners - num_ws_listeners
    );

    // Track WS-specific index for stagger calculation
    let mut ws_instance_idx = 0;

    // Initialize and spawn listeners based on their type
    for (instance_id, listener_config) in pool_config.listeners.iter().enumerate() {
        let url = &listener_config.url;

        match listener_config.listener_type {
            ListenerType::Subscription => {
                info!(
                    instance_id = instance_id,
                    ws_instance_idx = ws_instance_idx,
                    url = %url,
                    "Initializing WebSocket subscription listener"
                );

                let listener = Arc::new(
                    ArbitrumListener::new(
                        settings.gateway.clone(),
                        orchestrator.clone(),
                        repositories.block_number.clone(),
                        deduplicator.clone(),
                        ws_instance_idx,
                        url.clone(),
                        num_ws_listeners,
                    )
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to initialize subscription listener {}: {}",
                            instance_id,
                            e
                        )
                    })?,
                );

                let task_name = format!("gateway_listener_{}", instance_id);

                // Register health check
                orchestrator.add_health_check(
                    format!("gateway_listener_{}", instance_id),
                    listener.clone() as Arc<dyn HealthCheck>,
                );

                // Spawn listener and wait for it to be ready
                let listener_clone = listener.clone();
                let health_listener = listener.clone();
                orchestrator
                    .spawn_task_and_wait_ready(
                        &task_name,
                        async move {
                            if let Err(e) = listener_clone.run().await {
                                error!("Subscription listener {} failed: {}", instance_id, e);
                            }
                        },
                        async move { health_listener.check().await },
                    )
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to start subscription listener {}: {}",
                            instance_id,
                            e
                        )
                    })?;

                ws_instance_idx += 1;
            }
            ListenerType::Polling => {
                info!(
                    instance_id = instance_id,
                    url = %url,
                    "Initializing HTTP polling listener"
                );

                let listener = Arc::new(
                    PollingListener::new(
                        settings.gateway.clone(),
                        orchestrator.clone(),
                        repositories.block_number.clone(),
                        deduplicator.clone(),
                        instance_id,
                        url.clone(),
                    )
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to initialize polling listener {}: {}",
                            instance_id,
                            e
                        )
                    })?,
                );

                let task_name = format!("gateway_listener_{}", instance_id);

                // Register health check
                orchestrator.add_health_check(
                    format!("gateway_listener_{}", instance_id),
                    listener.clone() as Arc<dyn HealthCheck>,
                );

                // Spawn listener and wait for it to be ready
                let listener_clone = listener.clone();
                let health_listener = listener.clone();
                orchestrator
                    .spawn_task_and_wait_ready(
                        &task_name,
                        async move {
                            if let Err(e) = listener_clone.run().await {
                                error!("Polling listener {} failed: {}", instance_id, e);
                            }
                        },
                        async move { health_listener.check().await },
                    )
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to start polling listener {}: {}", instance_id, e)
                    })?;
            }
        }
    }

    // Create KeyUrl handler (but don't initialize yet - that happens after HTTP server)
    let keyurl_handler =
        KeyUrlGatewayHandler::new(Arc::clone(&orchestrator), settings.keyurl.clone());

    Ok(keyurl_handler)
}
