use alloy::primitives::U256;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use futures::future;
use std::{cmp::Ordering, collections::BinaryHeap, sync::Arc, time::Duration};
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    time::Instant,
};
use tracing::{debug, error, info, warn};

use crate::{
    core::{config::Config, decryption::handler::DecryptionHandler},
    error::Result,
};

/// Priority queue entry for efficient time-based scheduling
#[derive(Debug, Clone)]
struct ScheduledMessage {
    send_time: DateTime<Utc>,
    message_id: String,
}

impl PartialEq for ScheduledMessage {
    fn eq(&self, other: &Self) -> bool {
        self.send_time == other.send_time
    }
}

impl Eq for ScheduledMessage {}

impl PartialOrd for ScheduledMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior (earliest time first)
        other.send_time.cmp(&self.send_time)
    }
}

/// A message that needs to be sent at a coordinated time
#[derive(Debug, Clone)]
pub struct CoordinatedMessage {
    pub request_id: U256,
    pub key_id_hex: String,
    pub sns_ciphertext_materials: Vec<(Vec<u8>, Vec<u8>)>,
    pub client_addr: Option<alloy::primitives::Address>,
    pub public_key: Option<alloy::primitives::Bytes>,
    pub block_timestamp: u64,     // Block timestamp in seconds
    pub send_time: DateTime<Utc>, // Calculated send time (block_time + delta)
    /// Whether this message uses fixed interval scheduling (vs block-time-based)
    pub is_fixed_interval: bool,
    /// When this message was created (for TTL cleanup)
    pub created_at: DateTime<Utc>,
}

/// Coordinates message sending across multiple connector instances
pub struct MessageScheduler<P> {
    config: Config,
    decryption_handler: DecryptionHandler<P>,
    // DashMap for lock-free concurrent access to pending messages
    pending_messages: Arc<DashMap<String, CoordinatedMessage>>,
    sender: mpsc::UnboundedSender<CoordinatedMessage>,
}

impl<P: alloy::providers::Provider + Clone + 'static> MessageScheduler<P> {
    /// Create a new message scheduler
    pub fn new(
        config: Config,
        decryption_handler: DecryptionHandler<P>,
        shutdown: broadcast::Receiver<()>,
    ) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let pending_messages = Arc::new(DashMap::new());
        let schedule_queue = Arc::new(Mutex::new(BinaryHeap::new()));

        let shutdown_for_task = shutdown.resubscribe();

        let scheduler = Self {
            config: config.clone(),
            decryption_handler: decryption_handler.clone(),
            pending_messages: pending_messages.clone(),
            sender,
        };

        // Start the background task to process scheduled messages
        let scheduler_task = Self::start_scheduler_task(
            config,
            decryption_handler,
            receiver,
            pending_messages.clone(),
            schedule_queue.clone(),
            shutdown_for_task,
        );

        tokio::spawn(scheduler_task);

        scheduler
    }

    /// Schedule a message to be sent at the coordinated time
    pub async fn schedule_message(
        &self,
        request_id: U256,
        key_id_hex: String,
        sns_ciphertext_materials: Vec<(Vec<u8>, Vec<u8>)>,
        client_addr: Option<alloy::primitives::Address>,
        public_key: Option<alloy::primitives::Bytes>,
        block_timestamp: u64,
    ) -> Result<()> {
        if !self.config.enable_coordinated_sending {
            // If coordinated sending is disabled, send immediately
            return self
                .decryption_handler
                .handle_decryption_request_response(
                    request_id,
                    key_id_hex,
                    sns_ciphertext_materials,
                    client_addr,
                    public_key,
                )
                .await;
        }

        // Calculate the send time: block_timestamp + delta
        let send_time = DateTime::from_timestamp(
            block_timestamp as i64 + (self.config.message_send_delta.as_millis() as i64 / 1000),
            ((self.config.message_send_delta.as_millis() % 1000) * 1_000_000) as u32,
        )
        .ok_or_else(|| {
            crate::error::Error::Config("Invalid block timestamp for scheduling".to_string())
        })?;

        // Determine if we should use fixed interval scheduling
        let use_fixed_interval = self.config.fixed_send_interval_ms > 0;

        // Calculate send time based on scheduling mode
        let send_time = if use_fixed_interval {
            // For fixed interval: calculate next interval boundary
            let now = Utc::now();
            let interval_ms = self.config.fixed_send_interval_ms as i64;
            let millis_since_epoch = now.timestamp_millis();
            let next_interval = ((millis_since_epoch / interval_ms) + 1) * interval_ms;
            DateTime::from_timestamp_millis(next_interval).ok_or_else(|| {
                crate::error::Error::Config("Invalid fixed interval calculation".to_string())
            })?
        } else {
            // Use the original block-time-based calculation
            send_time
        };

        let coordinated_message = CoordinatedMessage {
            request_id,
            key_id_hex,
            sns_ciphertext_materials,
            client_addr,
            public_key,
            block_timestamp,
            send_time,
            is_fixed_interval: use_fixed_interval,
            created_at: Utc::now(),
        };

        let message_id = format!("{request_id:x}");

        if use_fixed_interval {
            info!(
                "Scheduling message {} to be sent at {} (fixed interval: {}ms)",
                message_id,
                send_time.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                self.config.fixed_send_interval_ms
            );
        } else {
            info!(
                "Scheduling message {} to be sent at {} (block_time: {}, delta: {}ms)",
                message_id,
                send_time.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                DateTime::from_timestamp(block_timestamp as i64, 0)
                    .unwrap_or_default()
                    .format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                self.config.message_send_delta.as_millis()
            );
        }

        // Check queue size limit to prevent memory exhaustion
        const MAX_PENDING_MESSAGES: usize = 10000;
        if self.pending_messages.len() >= MAX_PENDING_MESSAGES {
            warn!(
                "Message queue at capacity ({}), dropping oldest messages",
                MAX_PENDING_MESSAGES
            );
            self.cleanup_old_messages().await;
        }

        // Store the message in pending messages
        self.pending_messages
            .insert(message_id.clone(), coordinated_message.clone());

        // Note: In this simplified implementation, we rely on the background task
        // to process messages based on their send_time stored in the DashMap

        // Send the message to the background task
        if let Err(e) = self.sender.send(coordinated_message) {
            error!("Failed to send message to scheduler task: {}", e);
            // Remove from pending messages if we failed to queue it
            self.pending_messages.remove(&message_id);
            return Err(crate::error::Error::Channel(
                "Failed to queue message for scheduling".to_string(),
            ));
        }

        Ok(())
    }

    /// Background task that processes scheduled messages
    async fn start_scheduler_task(
        _config: Config,
        decryption_handler: DecryptionHandler<P>,
        mut receiver: mpsc::UnboundedReceiver<CoordinatedMessage>,
        pending_messages: Arc<DashMap<String, CoordinatedMessage>>,
        schedule_queue: Arc<Mutex<BinaryHeap<ScheduledMessage>>>,
        mut shutdown: broadcast::Receiver<()>,
    ) {
        info!("Starting coordinated message scheduler task");
        let mut next_wake_time: Option<Instant> = None;

        loop {
            // Calculate when to wake up next based on the earliest scheduled message
            let sleep_future = if let Some(wake_time) = next_wake_time {
                tokio::time::sleep_until(wake_time)
            } else {
                // If no messages scheduled, sleep for a reasonable interval
                tokio::time::sleep(Duration::from_millis(1000))
            };

            tokio::select! {
                // Receive new messages to schedule
                message = receiver.recv() => {
                    match message {
                        Some(msg) => {
                            let message_id = format!("{:x}", msg.request_id);

                            // Insert into DashMap (lock-free)
                            pending_messages.insert(message_id.clone(), msg.clone());

                            // CRITICAL FIX: Also add to priority queue for timing-based processing
                            {
                                let mut queue = schedule_queue.lock().await;
                                queue.push(ScheduledMessage {
                                    send_time: msg.send_time,
                                    message_id: message_id.clone(),
                                });
                            }

                            debug!("Added message {} to pending queue, scheduled for {:?}", message_id, msg.send_time);
                        }
                        None => {
                            warn!("Message scheduler channel closed");
                            break;
                        }
                    }
                }

                // Process messages that are ready to send
                _ = sleep_future => {
                    next_wake_time = Self::process_ready_messages(
                        &_config,
                        &decryption_handler,
                        &pending_messages,
                        &schedule_queue,
                    ).await;
                }

                // Handle shutdown
                _ = shutdown.recv() => {
                    info!("Received shutdown signal in message scheduler");
                    break;
                }
            }
        }

        // Send any remaining messages before shutdown
        info!("Processing remaining messages before shutdown");
        Self::process_all_remaining_messages(
            &decryption_handler,
            &pending_messages,
            &schedule_queue,
        )
        .await;
        info!("Message scheduler task stopped");
    }

    /// Process messages that are ready to be sent
    /// Returns the next wake time (Instant) if there are more messages to process
    async fn process_ready_messages(
        config: &Config,
        decryption_handler: &DecryptionHandler<P>,
        pending_messages: &Arc<DashMap<String, CoordinatedMessage>>,
        schedule_queue: &Arc<Mutex<BinaryHeap<ScheduledMessage>>>,
    ) -> Option<Instant> {
        let now = Utc::now();
        let mut ready_message_ids = Vec::with_capacity(512); // Optimized for 100 TPS with 2s delay + 10ms spacing: handles 400+ message batches
        let mut next_wake_time: Option<Instant> = None;

        // OPTIMIZATION: Minimize lock hold time - only extract message IDs
        {
            let mut queue = schedule_queue.lock().await;

            // Extract ready message IDs (minimal work while holding lock)
            while let Some(scheduled) = queue.peek() {
                if scheduled.send_time <= now {
                    let scheduled = queue.pop().unwrap();
                    ready_message_ids.push(scheduled.message_id);
                } else {
                    // Calculate next wake time based on the earliest remaining message
                    let time_until_ready = scheduled.send_time - now;
                    let duration_until_ready =
                        Duration::from_secs(time_until_ready.num_seconds().max(0) as u64)
                            + Duration::from_millis(
                                (time_until_ready.num_milliseconds() % 1000).max(0) as u64,
                            );
                    next_wake_time = Some(Instant::now() + duration_until_ready);
                    break;
                }
            }
        } // Lock released here - much faster!

        // Process messages outside the lock to avoid contention
        if !ready_message_ids.is_empty() {
            let mut ready_messages = Vec::with_capacity(ready_message_ids.len());

            // Retrieve messages from DashMap (lock-free operations)
            for message_id in ready_message_ids {
                if let Some((_, message)) = pending_messages.remove(&message_id) {
                    ready_messages.push(message);
                    debug!("Message {} is ready to send", message_id);
                } else {
                    warn!("Message {} not found in pending messages", message_id);
                }
            }

            // Send ready messages sequentially with proper spacing
            if !ready_messages.is_empty() {
                let message_count = ready_messages.len();
                info!(
                    "Sending {} coordinated messages sequentially",
                    message_count
                );

                for (i, message) in ready_messages.into_iter().enumerate() {
                    Self::send_message(decryption_handler, message).await;

                    // Apply message spacing between sends (except for the last message)
                    if i < message_count - 1 {
                        let spacing = Duration::from_millis(config.message_spacing_ms);
                        if !spacing.is_zero() {
                            tokio::time::sleep(spacing).await;
                        }
                    }
                }
            }
        }

        next_wake_time
    }

    /// Send an individual coordinated message
    async fn send_message(decryption_handler: &DecryptionHandler<P>, message: CoordinatedMessage) {
        let message_id = format!("{:x}", message.request_id);
        let now = Utc::now();

        debug!(
            "Sending coordinated message {} (scheduled: {}, actual: {})",
            message_id,
            message.send_time.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
            now.format("%Y-%m-%d %H:%M:%S%.3f UTC")
        );

        if let Err(e) = decryption_handler
            .handle_decryption_request_response(
                message.request_id,
                message.key_id_hex,
                message.sns_ciphertext_materials,
                message.client_addr,
                message.public_key,
            )
            .await
        {
            error!("Failed to send coordinated message {}: {}", message_id, e);
        } else {
            info!("Successfully sent coordinated message {}", message_id);
        }
    }

    /// Clean up old messages to prevent memory leaks
    /// OPTIMIZED: Single-pass cleanup without temporary allocations
    async fn cleanup_old_messages(&self) {
        const TTL_HOURS: i64 = 1; // Messages older than 1 hour are considered stale
        let cutoff_time = Utc::now() - chrono::Duration::hours(TTL_HOURS);
        let mut removed_count = 0;

        // OPTIMIZATION: Single-pass removal using retain pattern
        self.pending_messages.retain(|message_id, message| {
            if message.created_at < cutoff_time {
                warn!("Removed stale message {} (TTL exceeded)", message_id);
                removed_count += 1;
                false // Remove this entry
            } else {
                true // Keep this entry
            }
        });

        if removed_count > 0 {
            info!("TTL cleanup removed {} stale messages", removed_count);
        }
    }

    /// Process all remaining messages during shutdown
    async fn process_all_remaining_messages(
        decryption_handler: &DecryptionHandler<P>,
        pending_messages: &Arc<DashMap<String, CoordinatedMessage>>,
        _schedule_queue: &Arc<Mutex<BinaryHeap<ScheduledMessage>>>,
    ) {
        // Collect all remaining messages from DashMap
        let remaining_messages: Vec<_> = pending_messages
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();

        // Clear the DashMap
        pending_messages.clear();

        if !remaining_messages.is_empty() {
            warn!(
                "Sending {} remaining messages during shutdown",
                remaining_messages.len()
            );

            // Send remaining messages concurrently
            let send_futures: Vec<_> = remaining_messages
                .into_iter()
                .map(|(_message_id, message)| {
                    let handler = decryption_handler.clone();
                    async move {
                        warn!(
                            "Sending remaining message during shutdown (was scheduled for: {})",
                            message.send_time.format("%Y-%m-%d %H:%M:%S%.3f UTC")
                        );
                        Self::send_message(&handler, message).await;
                    }
                })
                .collect();

            future::join_all(send_futures).await;
        }
    }

    /// Get the number of pending messages (for monitoring)
    pub fn pending_count(&self) -> usize {
        self.pending_messages.len()
    }
}
