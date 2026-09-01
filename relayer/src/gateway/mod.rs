pub mod arbitrum;
pub mod ciphertext_checker;
pub mod handled_events;
pub mod input_handlers;
pub mod public_decrypt_handler;
pub mod throttlers;
pub mod user_decrypt_handler;
pub mod utils;

pub use input_handlers::InputProofGatewayHandler;
pub use public_decrypt_handler::GatewayHandler as PublicDecryptGatewayHandler;
pub use user_decrypt_handler::GatewayHandler as UserDecryptGatewayHandler;

use crate::config::settings::{ListenerType, Settings};
use crate::gateway::arbitrum::transaction::tx_processor::GatewayTxProcessor;
use crate::gateway::handled_events::HandledEvents;
use crate::gateway::throttlers::GatewayThrottlers;
use crate::host::{HostAclChecker, ThresholdResolver};
use crate::orchestrator::{HealthCheck, Orchestrator};
use crate::readiness::{
    checker::ReadinessChecker, public_decrypt_processor::PublicDecryptReadinessProcessor,
    user_decrypt_processor::UserDecryptReadinessProcessor,
};
use crate::store::sql::repositories::Repositories;
use alloy::primitives::Address;
use arbitrum::{
    transaction::{
        helper::GatewayTransactionEngine, TransactionHelper as GatewayTransactionHelper,
    },
    ArbitrumListener, PollingListener, WsRecycleStagger,
};
use std::{str::FromStr, sync::Arc, time::Duration};
use tracing::{error, info};

/// Initialize all gateway components including handlers and listeners.
pub async fn initialize_gateway(
    orchestrator: Arc<Orchestrator>,
    settings: &Settings,
    repositories: Arc<Repositories>,
    gateway_throttlers: GatewayThrottlers,
) -> anyhow::Result<()> {
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

    let threshold_resolver = Arc::new(
        ThresholdResolver::new(
            &settings.protocol_config,
            settings.gateway.contracts.user_decrypt_shares_threshold, // u32
            10_000,
        )
        .await?,
    );

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
        repositories.user_decrypt.clone(),
        user_decrypt_handler::UserDecryptHandlerConfig {
            decryption_address,
            gw_event_retry: settings.gateway.gw_event_not_found_retry.clone(),
        },
        threshold_resolver,
    );

    // Register transaction helper with orchestrator for health checks
    orchestrator.add_health_check(
        "gateway_http".to_string(),
        gateway_tx_helper.clone() as Arc<dyn HealthCheck>,
    );

    // Every listener hands its logs to one place, so a log seen by several instances is
    // dispatched once and the cursor only passes events that have been handled.
    let pool_config = &settings.gateway.listener_pool;
    let handled_events = Arc::new(HandledEvents::new(
        Duration::from_secs(pool_config.dedup_ttl_seconds),
        pool_config.dedup_max_capacity as u64,
        repositories.chain_cursor.clone(),
        orchestrator.clone(),
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

    // Counts subscription listeners only, so it staggers their recycles without gaps. Reaches
    // the listener inside a `WsRecycleStagger` and never as its identity - see that type.
    let mut ws_stagger_index = 0;

    // Initialize and spawn listeners based on their type
    for (pool_index, listener_config) in pool_config.listeners.iter().enumerate() {
        let url = &listener_config.url;

        match listener_config.listener_type {
            ListenerType::Subscription => {
                info!(
                    instance_id = pool_index,
                    ws_stagger_index = ws_stagger_index,
                    url = %url,
                    "Initializing WebSocket subscription listener"
                );

                let listener = Arc::new(
                    ArbitrumListener::new(
                        settings.gateway.clone(),
                        handled_events.clone(),
                        pool_index,
                        url.clone(),
                        WsRecycleStagger {
                            index: ws_stagger_index,
                            total: num_ws_listeners,
                        },
                    )
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to initialize subscription listener {}: {}",
                            pool_index,
                            e
                        )
                    })?,
                );

                let task_name = format!("gateway_listener_{}", pool_index);

                // Register health check
                orchestrator.add_health_check(
                    format!("gateway_listener_{}", pool_index),
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
                                error!("Subscription listener {} failed: {}", pool_index, e);
                            }
                        },
                        async move { health_listener.check().await },
                    )
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to start subscription listener {}: {}",
                            pool_index,
                            e
                        )
                    })?;

                ws_stagger_index += 1;
            }
            ListenerType::Polling => {
                info!(
                    instance_id = pool_index,
                    url = %url,
                    "Initializing HTTP polling listener"
                );

                let listener = Arc::new(
                    PollingListener::new(
                        settings.gateway.clone(),
                        repositories.chain_cursor.clone(),
                        handled_events.clone(),
                        pool_index,
                        url.clone(),
                    )
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to initialize polling listener {}: {}",
                            pool_index,
                            e
                        )
                    })?,
                );

                let task_name = format!("gateway_listener_{}", pool_index);

                // Register health check
                orchestrator.add_health_check(
                    format!("gateway_listener_{}", pool_index),
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
                                error!("Polling listener {} failed: {}", pool_index, e);
                            }
                        },
                        async move { health_listener.check().await },
                    )
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to start polling listener {}: {}", pool_index, e)
                    })?;
            }
        }
    }

    Ok(())
}
