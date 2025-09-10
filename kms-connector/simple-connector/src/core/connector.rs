use super::{event_processor::processors::EventProcessor, polling::BlockPoller};
use crate::{
    core::{config::Config, decryption::handler::DecryptionHandler},
    error::Result,
    gw_adapters::{
        decryption::DecryptionAdapter,
        events::{EventsAdapter, KmsCoreEvent},
    },
    kms_core_adapters::service::KmsServiceImpl,
};
use alloy::providers::Provider;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};

/// Core KMS connector that handles all interactions with the Gateway
pub struct KmsCoreConnector<P> {
    events: Option<EventsAdapter<P>>,
    poller: Option<BlockPoller<P>>,
    event_processor: EventProcessor<P>,
    kms_client: Arc<KmsServiceImpl>,
    shutdown: Option<broadcast::Receiver<()>>,
}

impl<P: Provider + Clone + 'static> KmsCoreConnector<P> {
    /// Creates a new KMS Core connector
    pub async fn new(
        provider: Arc<P>,
        config: Config,
        kms_client: Arc<KmsServiceImpl>,
        shutdown: broadcast::Receiver<()>,
    ) -> Result<(Self, mpsc::Receiver<KmsCoreEvent>)> {
        let (event_tx, event_rx) = mpsc::channel(config.channel_size);

        // Create broadcast channel for shutdown signaling to polling system
        let (shutdown_tx, _) = broadcast::channel(1);

        // Possible gas limit
        let decryption = DecryptionAdapter::new(config.decryption_address, provider.clone());

        let decryption_handler =
            DecryptionHandler::new(decryption.clone(), kms_client.clone(), config.clone());

        let event_processor = EventProcessor::new(
            decryption_handler.clone(),
            config.clone(),
            provider.clone(),
            shutdown.resubscribe(),
        )
        .await?;

        // Create either WebSocket events adapter or polling system based on config
        let (events, poller) = if config.use_polling_mode {
            info!("Using polling mode for blockchain events");

            // Get backpressure receiver from event processor for polling integration
            let backpressure_rx = event_processor.get_backpressure_receiver();

            let poller = BlockPoller::new(
                Arc::clone(&provider),
                Arc::new(config.clone()),
                event_tx,
                shutdown_tx.clone(),
                backpressure_rx,
            );
            (None, Some(poller))
        } else {
            info!("Using WebSocket mode for blockchain events");
            let events = EventsAdapter::new(
                Arc::clone(&provider),
                config.decryption_address,
                config.gateway_config_address,
                event_tx,
            );
            (Some(events), None)
        };

        let connector = Self {
            events,
            poller,
            event_processor,
            kms_client,
            shutdown: Some(shutdown),
        };

        Ok((connector, event_rx))
    }

    /// Start the connector
    pub async fn start(&mut self, event_rx: mpsc::Receiver<KmsCoreEvent>) -> Result<()> {
        info!("Starting KMS Core Connector...");

        // Initialize event intake system (WebSocket or polling)
        if let Some(events) = &self.events {
            info!("Initializing WebSocket event subscriptions...");
            events.initialize().await?;
        } else if let Some(poller) = &self.poller {
            info!("Starting blockchain polling system...");
            let poller_handle = {
                let poller = poller.clone();
                tokio::spawn(async move {
                    if let Err(e) = poller.start().await {
                        error!("Critical polling system error: {}", e);
                        // Force exit with error code for restart
                        std::process::exit(1);
                    }
                })
            };

            // Store the handle for cleanup (we could add this to the struct if needed)
            // For now, the poller will handle its own shutdown via broadcast channel
            std::mem::forget(poller_handle); // Let it run independently
        } else {
            return Err(crate::Error::Config(
                "Neither WebSocket nor polling mode configured".to_string(),
            ));
        }

        // Keep trying to initialize KMS client
        loop {
            match self.kms_client.initialize().await {
                Ok(_) => {
                    info!("Successfully connected to KMS-core");
                    break;
                }
                Err(e) => {
                    error!("Failed to connect to KMS-core: {}, retrying...", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }

            // Check for shutdown signal
            if let Some(shutdown) = &self.shutdown
                && shutdown.resubscribe().try_recv().is_ok()
            {
                info!("Received shutdown signal while trying to connect to KMS-core");
                return Ok(());
            }
        }

        // Process events
        self.event_processor
            .process_gateway_events(event_rx)
            .await?;

        Ok(())
    }

    /// Stop the connector and clean up resources
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping KMS Core Connector...");

        // 1. Signal shutdown through broadcast channel first to stop new events
        if let Some(shutdown) = self.shutdown.take() {
            drop(shutdown);
        }

        // 2. Stop KMS client to prevent new operations
        self.kms_client.stop();

        // 3. Stop event intake system (WebSocket or polling)
        if let Some(events) = &self.events
            && let Err(e) = events.stop().await
        {
            error!("Error during event adapter shutdown: {}", e);
            // Continue shutdown process despite error
        }

        // Note: BlockPoller handles its own shutdown via broadcast channel
        // so no explicit cleanup needed for polling mode
        if self.poller.is_some() {
            info!("Polling system will shut down via broadcast signal");
        }

        info!("KMS Core Connector stopped");
        Ok(())
    }
}
