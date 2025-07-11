use alloy::providers::Provider;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::{
    core::{
        config::Config, coordination::scheduler::MessageScheduler,
        event_processor::processors::EventProcessor,
    },
    error::Result,
};

use super::poller::BlockPoller;

/// Start polling-based event intake with the given components
pub async fn start_polling_mode<P: Provider + Clone + 'static>(
    config: Config,
    provider: Arc<P>,
    _scheduler: Option<Arc<MessageScheduler<P>>>,
    mut event_processor: EventProcessor<P>,
    shutdown: broadcast::Receiver<()>,
) -> Result<()> {
    info!("Starting event intake in POLLING mode");

    // Create event channel for communication between poller and processor
    let (event_tx, event_rx) = broadcast::channel(config.channel_size);

    // Create and configure the block poller
    let mut poller = BlockPoller::new(
        Arc::clone(&provider),
        config.clone(),
        shutdown.resubscribe(),
    );

    // Start both poller and processor concurrently
    let poller_handle = {
        let event_tx = event_tx.clone();
        tokio::spawn(async move {
            if let Err(e) = poller.start_polling(event_tx).await {
                error!("Block poller failed: {}", e);
            }
        })
    };

    let processor_handle = {
        tokio::spawn(async move {
            if let Err(e) = event_processor.process_gateway_events(event_rx).await {
                error!("Event processor failed: {}", e);
            }
        })
    };

    // Wait for either task to complete (or fail)
    tokio::select! {
        result = poller_handle => {
            match result {
                Ok(_) => info!("Block poller completed"),
                Err(e) => error!("Block poller task failed: {}", e),
            }
        }
        result = processor_handle => {
            match result {
                Ok(_) => info!("Event processor completed"),
                Err(e) => error!("Event processor task failed: {}", e),
            }
        }
    }

    Ok(())
}

/// Start event intake system - currently only supports polling mode
/// WebSocket mode with proper backpressure is not yet implemented
pub async fn start_event_intake<P: Provider + Clone + 'static>(
    config: Config,
    provider: Arc<P>,
    scheduler: Option<Arc<MessageScheduler<P>>>,
    event_processor: EventProcessor<P>,
    shutdown: broadcast::Receiver<()>,
) -> Result<()> {
    if config.use_polling_mode {
        info!("Starting event intake in POLLING mode");
        start_polling_mode(config, provider, scheduler, event_processor, shutdown).await
    } else {
        warn!("WebSocket mode is not yet implemented with proper backpressure control");
        warn!("Falling back to polling mode for production safety");

        // Force polling mode for safety
        let mut polling_config = config;
        polling_config.use_polling_mode = true;

        start_polling_mode(
            polling_config,
            provider,
            scheduler,
            event_processor,
            shutdown,
        )
        .await
    }
}
