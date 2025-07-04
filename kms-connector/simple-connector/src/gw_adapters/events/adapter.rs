use alloy::rpc::types::eth::Log as EthLog;
use alloy::{primitives::Address, providers::Provider};
use anyhow::{Result, anyhow};
use fhevm_gateway_rust_bindings::{decryption::Decryption, kmsmanagement::KmsManagement};
use std::{
    fmt::Debug,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};

/// Default event processing timeout
const EVENT_TIMEOUT: Duration = Duration::from_secs(60);

/// Events that can be processed by the KMS Core
#[derive(Clone, Debug)]
pub enum KmsCoreEvent {
    /// Public decryption request
    PublicDecryptionRequest(Decryption::PublicDecryptionRequest),
    /// Public decryption response
    PublicDecryptionResponse(Decryption::PublicDecryptionResponse),
    /// User decryption request
    UserDecryptionRequest(Decryption::UserDecryptionRequest),
    /// User decryption response
    UserDecryptionResponse(Decryption::UserDecryptionResponse),
    /// Preprocess keygen request
    PreprocessKeygenRequest(KmsManagement::PreprocessKeygenRequest),
    /// Preprocess keygen response
    PreprocessKeygenResponse(KmsManagement::PreprocessKeygenResponse),
    /// Preprocess kskgen request
    PreprocessKskgenRequest(KmsManagement::PreprocessKskgenRequest),
    /// Preprocess kskgen response
    PreprocessKskgenResponse(KmsManagement::PreprocessKskgenResponse),
    /// Keygen request
    KeygenRequest(KmsManagement::KeygenRequest),
    /// Keygen response
    KeygenResponse(KmsManagement::KeygenResponse),
    /// CRS generation request
    CrsgenRequest(KmsManagement::CrsgenRequest),
    /// CRS generation response
    CrsgenResponse(KmsManagement::CrsgenResponse),
    /// KSK generation request
    KskgenRequest(KmsManagement::KskgenRequest),
    /// KSK generation response
    KskgenResponse(KmsManagement::KskgenResponse),
}

/// Adapter for handling Gateway events
#[derive(Debug)]
pub struct EventsAdapter<P> {
    provider: Arc<P>,
    decryption: Address,
    gateway_config: Address,
    event_tx: mpsc::Sender<KmsCoreEvent>,
    running: Arc<AtomicBool>,
    handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl<P: Provider + Clone + 'static> EventsAdapter<P> {
    /// Create a new events adapter
    pub fn new(
        provider: Arc<P>,
        decryption: Address,
        gateway_config: Address,
        event_tx: mpsc::Sender<KmsCoreEvent>,
    ) -> Self {
        Self {
            provider,
            decryption,
            gateway_config,
            event_tx,
            running: Arc::new(AtomicBool::new(true)),
            handles: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Initialize event subscriptions
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing event subscriptions...");

        let provider = Arc::clone(&self.provider);
        let decryption = self.decryption;
        let gateway_config = self.gateway_config;
        let event_tx = self.event_tx.clone();
        let running = self.running.clone();

        tokio::spawn(schedule_log_queue_capacity(event_tx.clone()));
        let handle = tokio::spawn(async move {
            while running.load(Ordering::SeqCst) {
                Self::subscribe_to_events(
                    Arc::clone(&provider),
                    decryption,
                    gateway_config,
                    event_tx.clone(),
                    running.clone(),
                )
                .await
            }
            info!("Subscription loop terminated");
        });

        self.store_handle(handle);
        Ok(())
    }

    /// Subscribe to events
    async fn subscribe_to_events(
        provider: Arc<P>,
        decryption: Address,
        gateway_config: Address,
        event_tx: mpsc::Sender<KmsCoreEvent>,
        running: Arc<AtomicBool>,
    ) {
        let mut tasks = vec![
            tokio::spawn(Self::subscribe_to_decryption_events(
                decryption,
                Arc::clone(&provider),
                event_tx.clone(),
                running.clone(),
            )),
            tokio::spawn(Self::subscribe_to_gateway_config_events(
                gateway_config,
                provider,
                event_tx,
                running.clone(),
            )),
        ];

        // Create a stream from the running flag for graceful shutdown
        let mut shutdown = tokio::time::interval(Duration::from_millis(100));
        let running_check = running.clone();

        // Wait for any task to complete or fail, or for shutdown signal
        while !tasks.is_empty() {
            tokio::select! {
                _ = shutdown.tick() => {
                    if !running_check.load(Ordering::SeqCst) {
                        debug!("Received shutdown signal, stopping tasks");
                        for task in &tasks {
                            task.abort();
                        }
                        return;
                    }
                }
                result = futures::future::select_all(tasks.iter_mut()) => {
                    let (result, idx, _) = result;
                    match result {
                        Ok(Ok(_)) => {
                            tasks.remove(idx);
                            if !tasks.is_empty() {
                                info!("One task completed, {} remaining", tasks.len());
                            }
                        }
                        Ok(Err(e)) => {
                            // Abort other tasks
                            for task in &tasks {
                                task.abort();
                            }
                            error!("Task {} failed: {}", idx, e);
                        }
                        Err(e) => {
                            // Abort other tasks
                            for task in &tasks {
                                task.abort();
                            }
                            return error!("Task {} panicked: {}", idx, e);
                        }
                    }
                }
            }
        }
    }

    /// Stop event subscriptions and clean up resources
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping event subscriptions...");

        // 1. Signal stop to all running tasks
        self.running.store(false, Ordering::SeqCst);

        // 2. Take all handles first to avoid holding MutexGuard across await points
        let handles = {
            if let Ok(mut handles) = self.handles.lock() {
                handles.drain(..).collect::<Vec<_>>()
            } else {
                error!("Failed to acquire lock for subscription handles");
                return Ok(()); // Return OK since we've signaled shutdown
            }
        };

        // 3. Wait for all tasks with timeout
        let mut errors = Vec::new();
        for handle in handles {
            match tokio::time::timeout(Duration::from_secs(5), handle).await {
                Ok(result) => {
                    if let Err(e) = result {
                        errors.push(format!("Task failed: {e}"));
                    }
                }
                Err(_) => {
                    errors.push("Task timed out".to_string());
                }
            }
        }

        // Log any errors that occurred during shutdown
        if !errors.is_empty() {
            error!(
                "Errors during event subscription shutdown: {}",
                errors.join(", ")
            );
        }

        info!("Event subscriptions stopped");
        Ok(())
    }

    /// Store a subscription handle for cleanup
    fn store_handle(&self, handle: JoinHandle<()>) {
        if let Ok(mut handles) = self.handles.lock() {
            handles.push(handle);
        }
    }

    /// Subscribe to decryption events
    async fn subscribe_to_decryption_events(
        decryption: Address,
        provider: Arc<P>,
        event_tx: mpsc::Sender<KmsCoreEvent>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        info!("Starting Decryption event subscriptions...");

        let contract = Decryption::new(decryption, provider);
        let public_filter = contract.PublicDecryptionRequest_filter().watch().await?;
        info!("✓ Subscribed to PublicDecryptionRequest events");

        let user_filter = contract.UserDecryptionRequest_filter().watch().await?;
        info!("✓ Subscribed to UserDecryptionRequest events");

        let mut public_stream = public_filter.into_stream();
        let mut user_stream = user_filter.into_stream();

        info!("Successfully subscribed to all Decryption events");

        loop {
            if !running.load(Ordering::SeqCst) {
                info!("Event subscription stopping due to shutdown signal");
                break;
            }

            tokio::select! {
                result = public_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::PublicDecryptionRequest, "PublicDecryptionRequest".to_string()).await?,
                result = user_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::UserDecryptionRequest, "UserDecryptionRequest".to_string()).await?,
            }
        }

        Ok(())
    }

    /// Subscribe to GatewayConfig events
    async fn subscribe_to_gateway_config_events(
        address: Address,
        provider: Arc<P>,
        event_tx: mpsc::Sender<KmsCoreEvent>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        info!("Starting KmsManagement event subscriptions...");

        let contract = KmsManagement::new(address, provider);
        let preprocess_keygen_request_filter =
            contract.PreprocessKeygenRequest_filter().watch().await?;
        info!("✓ Subscribed to PreprocessKeygenRequest events");

        let preprocess_kskgen_request_filter =
            contract.PreprocessKskgenRequest_filter().watch().await?;
        info!("✓ Subscribed to PreprocessKskgenRequest events");

        let keygen_request_filter = contract.KeygenRequest_filter().watch().await?;
        info!("✓ Subscribed to KeygenRequest events");

        let crsgen_request_filter = contract.CrsgenRequest_filter().watch().await?;
        info!("✓ Subscribed to CrsgenRequest events");

        let kskgen_request_filter = contract.KskgenRequest_filter().watch().await?;
        info!("✓ Subscribed to KskgenRequest events");

        // Convert filters to streams
        let mut preprocess_keygen_request_stream = preprocess_keygen_request_filter.into_stream();
        let mut preprocess_kskgen_request_stream = preprocess_kskgen_request_filter.into_stream();
        let mut keygen_request_stream = keygen_request_filter.into_stream();
        let mut crsgen_request_stream = crsgen_request_filter.into_stream();
        let mut kskgen_request_stream = kskgen_request_filter.into_stream();

        info!("Successfully subscribed to all KmsManagement events");

        loop {
            if !running.load(Ordering::SeqCst) {
                info!("KmsManagement event subscription stopping due to shutdown signal");
                break;
            }

            tokio::select! {
                result = preprocess_keygen_request_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::PreprocessKeygenRequest, "PreprocessKeygenRequest".to_string()).await?,
                result = preprocess_kskgen_request_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::PreprocessKskgenRequest, "PreprocessKskgenRequest".to_string()).await?,
                result = keygen_request_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::KeygenRequest, "KeygenRequest".to_string()).await?,
                result = crsgen_request_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::CrsgenRequest, "CrsgenRequest".to_string()).await?,
                result = kskgen_request_stream.next() => Self::handle_event(result, event_tx.clone(), KmsCoreEvent::KskgenRequest, "KskgenRequest".to_string()).await?,
            }
        }

        Ok(())
    }

    /// Helper function to handle event stream results
    async fn handle_event<T: Debug>(
        result: Option<Result<(T, EthLog), alloy::sol_types::Error>>,
        event_tx: mpsc::Sender<KmsCoreEvent>,
        event_constructor: fn(T) -> KmsCoreEvent,
        event_name: String,
    ) -> Result<()> {
        let event = match result {
            Some(Ok((event, log))) => {
                info!("[EVENT] 🔒 Received {} event:", event_name);
                info!(
                    "  Block: {}, Tx: {}, LogIdx: {}",
                    log.block_number
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    log.transaction_hash
                        .map(|h| format!("0x{h}"))
                        .unwrap_or_else(|| "N/A".to_string()),
                    log.log_index
                        .map(|i| i.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                );
                info!("  Topics: {:?}", log.topics());
                debug!("  Raw Data: {:?}", log.data());
                debug!("  Decoded Event: {:#?}", event);
                let core_event = event_constructor(event);
                debug!("🔎 Event processed: {:#?}", core_event);
                core_event
            }
            Some(Err(e)) => {
                error!("Failed to decode {}: {}", event_name, e);
                return Err(anyhow!("Failed to decode {}: {}", event_name, e));
            }
            None => {
                warn!("Event stream ended for {}", event_name);
                return Err(anyhow!("Event stream ended for {}", event_name));
            }
        };

        // Simple timeout for event sending
        match tokio::time::timeout(EVENT_TIMEOUT, event_tx.send(event.clone())).await {
            Ok(Ok(_)) => {
                debug!("Successfully sent {} event", event_name);
                Ok(())
            }
            Ok(Err(e)) => {
                error!("Failed to send {}: {}", event_name, e);
                Err(anyhow!("Failed to send {}: {}", event_name, e))
            }
            Err(_) => {
                warn!(
                    "Event send timeout for {}. Re-sending the event without timeout",
                    event_name
                );
                event_tx.send(event).await.map_err(anyhow::Error::from)
            }
        }
    }
}

/// Logs the status of the queue every 5 seconds.
async fn schedule_log_queue_capacity(event_tx: mpsc::Sender<KmsCoreEvent>) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        info!("Capacity of the event queue: {}", event_tx.capacity());
    }
}

impl<P> Drop for EventsAdapter<P> {
    fn drop(&mut self) {
        // Set running to false to signal all tasks to stop
        self.running.store(false, Ordering::SeqCst);

        // Abort all handles immediately without trying to create a runtime
        if let Ok(handles) = self.handles.lock() {
            for handle in handles.iter() {
                handle.abort();
            }
        }
    }
}

impl<P> EventsAdapter<P> {
    /// Graceful shutdown with timeout
    pub async fn shutdown(&mut self, timeout: Duration) -> Result<()> {
        // Signal shutdown
        self.running.store(false, Ordering::SeqCst);

        // Take handles out to avoid deadlock with Drop
        let handles = {
            if let Ok(mut handles) = self.handles.lock() {
                handles.drain(..).collect::<Vec<_>>()
            } else {
                return Err(anyhow!(
                    "Failed to acquire lock for handles during shutdown"
                ));
            }
        };

        if handles.is_empty() {
            return Ok(());
        }

        // Create a future that completes when all handles are done
        let shutdown_future = async {
            for handle in handles {
                if let Err(e) = handle.await {
                    warn!("Task failed during shutdown: {}", e);
                }
            }
        };

        // Wait for handles with timeout
        match tokio::time::timeout(timeout, shutdown_future).await {
            Ok(_) => {
                debug!("All event handlers shut down gracefully");
                Ok(())
            }
            Err(_) => {
                warn!("Shutdown timed out, forcing abort");
                // No need to abort handles here as Drop will handle it
                Err(anyhow!("Shutdown timed out"))
            }
        }
    }
}
