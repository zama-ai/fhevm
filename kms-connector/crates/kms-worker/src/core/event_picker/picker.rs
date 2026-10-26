use crate::{
    core::{Config, event_picker::notifier::DbEventNotifier},
    monitoring::metrics::{EVENT_RECEIVED_COUNTER, EVENT_RECEIVED_ERRORS},
};
use anyhow::anyhow;
use connector_utils::types::{ProtocolEvent, db::EventType, event};
use sqlx::{Pool, Postgres};
use std::collections::VecDeque;
use tokio::sync::mpsc::{self, Receiver};
use tracing::{debug, info, warn};

/// Interface used to pick Gateway's events from some storage.
pub trait EventPicker {
    type Event;

    fn pick_events(&mut self) -> impl Future<Output = anyhow::Result<Vec<Self::Event>>>;
}

/// Struct that collects Gateway's events from a `Postgres` database.
pub struct DbEventPicker {
    /// The DB connection pool used to query events when notified.
    db_pool: Pool<Postgres>,

    /// The receiver channel used to receive event notification.
    notif_receiver: Receiver<EventType>,

    /// The event types that need to be picked, in FIFO order and without duplicates.
    pending_event_types: VecDeque<EventType>,

    /// The limit number of events to fetch from the database.
    events_batch_size: u8,
}

impl DbEventPicker {
    pub fn new(
        db_pool: Pool<Postgres>,
        notif_receiver: Receiver<EventType>,
        events_batch_size: u8,
    ) -> Self {
        Self {
            db_pool,
            notif_receiver,
            pending_event_types: VecDeque::new(),
            events_batch_size,
        }
    }

    pub async fn connect(db_pool: Pool<Postgres>, config: &Config) -> anyhow::Result<Self> {
        let (notif_sender, notif_receiver) = mpsc::channel(config.task_limit);
        let event_notifier =
            DbEventNotifier::connect(db_pool.clone(), notif_sender, config).await?;
        tokio::spawn(event_notifier.start());

        let event_picker = DbEventPicker::new(db_pool, notif_receiver, config.events_batch_size);
        Ok(event_picker)
    }
}

impl EventPicker for DbEventPicker {
    type Event = ProtocolEvent;

    /// Picks events from the database.
    ///
    /// Notifications are coalesced per event type to handle bursts:
    /// - `drain_notifications` reads the received notifications off the channel, but
    ///   `mark_pending` skips the types already queued, so 1000 notifications result in a single
    ///   pick query.
    /// - After a full batch, the type is re-queued at the back, as more events may be pending,
    ///   giving the other types a chance to be picked first.
    ///
    /// Should only return an error if the notification channel is closed, so the `kms_worker` can
    /// shutdown gracefully.
    /// If another error is encountered, it will just be logged with a warning and will wait for
    /// next events.
    async fn pick_events(&mut self) -> anyhow::Result<Vec<Self::Event>> {
        loop {
            if self.pending_event_types.is_empty() {
                let Some(notification) = self.notif_receiver.recv().await else {
                    return Err(anyhow!("notification channel was closed!"));
                };
                self.mark_pending(notification);
            }
            self.drain_notifications();

            let Some(event_type) = self.pending_event_types.pop_front() else {
                continue;
            };
            match self.pick_notified_events(&event_type).await {
                Err(e) => {
                    warn!("Error while picking events: {e}");
                    EVENT_RECEIVED_ERRORS
                        .with_label_values(&[event_type.as_str()])
                        .inc();
                    continue;
                }
                Ok(events) if events.is_empty() => {
                    debug!("Events have already been picked");
                    continue;
                }
                Ok(events) => {
                    // A full batch means more events may still be pending: re-queue the event type.
                    if events.len() >= self.batch_limit(&event_type) {
                        self.mark_pending(event_type);
                    }
                    info!("Picked {} {} successfully", events.len(), event_type);
                    EVENT_RECEIVED_COUNTER
                        .with_label_values(&[event_type.as_str()])
                        .inc_by(events.len() as u64);
                    return Ok(events);
                }
            }
        }
    }
}

impl DbEventPicker {
    /// Moves the notifications already received into the `pending_event_types` queue, without
    /// blocking.
    ///
    /// The drain is bounded by the channel capacity, so it cannot loop forever if notifications
    /// keep coming. The remaining ones will be drained on the next pick.
    ///
    /// A closed channel is ignored here, it will be reported by `recv` once `pending_event_types` is
    /// empty.
    fn drain_notifications(&mut self) {
        for _ in 0..self.notif_receiver.max_capacity() {
            let Ok(notification) = self.notif_receiver.try_recv() else {
                break;
            };
            self.mark_pending(notification);
        }
    }

    fn mark_pending(&mut self, event_type: EventType) {
        if !self.pending_event_types.contains(&event_type) {
            self.pending_event_types.push_back(event_type);
        }
    }

    /// The maximum number of events returned by a single pick query of the given event type.
    fn batch_limit(&self, event_type: &EventType) -> usize {
        match event_type {
            EventType::PublicDecryptionRequest | EventType::UserDecryptionRequest => {
                self.events_batch_size as usize
            }
            _ => 1,
        }
    }

    async fn pick_notified_events(
        &self,
        event_type: &EventType,
    ) -> anyhow::Result<Vec<ProtocolEvent>> {
        match event_type {
            EventType::PublicDecryptionRequest => self.pick_public_decryption_requests().await,
            EventType::UserDecryptionRequest => self.pick_user_decryption_requests().await,
            EventType::PrepKeygenRequest => self.pick_prep_keygen_requests().await,
            EventType::KeygenRequest => self.pick_keygen_requests().await,
            EventType::CrsgenRequest => self.pick_crsgen_requests().await,
            EventType::AbortKeygenRequest => self.pick_abort_keygen_requests().await,
            EventType::AbortCrsgenRequest => self.pick_abort_crsgen_requests().await,
            EventType::NewKmsContext => self.pick_new_kms_context_events().await,
            EventType::NewKmsEpoch => self.pick_new_kms_epoch_events().await,
            EventType::KmsContextDestroyed => self.pick_kms_context_destroyed_events().await,
            EventType::KmsEpochDestroyed => self.pick_kms_epoch_destroyed_events().await,
        }
    }

    async fn pick_public_decryption_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE public_decryption_requests
                SET status = 'under_process'
                FROM (
                    SELECT decryption_id
                    FROM public_decryption_requests
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE public_decryption_requests.decryption_id = req.decryption_id
                RETURNING req.decryption_id, ct_handles, extra_data,
                source, tx_hash, already_sent, error_counter, created_at, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_public_decryption_row)
        .collect()
    }

    async fn pick_user_decryption_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE user_decryption_requests
                SET status = 'under_process'
                FROM (
                    SELECT decryption_id
                    FROM user_decryption_requests
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE user_decryption_requests.decryption_id = req.decryption_id
                RETURNING req.decryption_id, ct_handles, user_address, public_key, extra_data,
                signature, handle_owner_addresses, handle_contract_addresses, allowed_contracts,
                start_timestamp, duration_seconds,
                source, tx_hash, already_sent, error_counter, created_at, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_user_decryption_row)
        .collect()
    }

    async fn pick_prep_keygen_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE prep_keygen_requests
                SET status = 'under_process'
                FROM (
                    SELECT prep_keygen_id
                    FROM prep_keygen_requests
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE prep_keygen_requests.prep_keygen_id = req.prep_keygen_id
                RETURNING req.prep_keygen_id, params_type, existing_key_id, extra_data, tx_hash,
                already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_prep_keygen_row)
        .collect()
    }

    async fn pick_keygen_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE keygen_requests
                SET status = 'under_process'
                FROM (
                    SELECT key_id
                    FROM keygen_requests
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE keygen_requests.key_id = req.key_id
                RETURNING prep_keygen_id, req.key_id, existing_key_id, extra_data, tx_hash,
                already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_keygen_row)
        .collect()
    }

    async fn pick_new_kms_context_events(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE new_kms_context
                SET status = 'under_process'
                FROM (
                    SELECT context_id
                    FROM new_kms_context
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE new_kms_context.context_id = req.context_id
                RETURNING req.context_id, previous_context_id, kms_node_params, thresholds,
                software_version, pcr_values, tx_hash, already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_new_kms_context_row)
        .collect()
    }

    async fn pick_new_kms_epoch_events(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE new_kms_epoch
                SET status = 'under_process'
                FROM (
                    SELECT epoch_id
                    FROM new_kms_epoch
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE new_kms_epoch.epoch_id = req.epoch_id
                RETURNING context_id, previous_context_id, new_kms_epoch.epoch_id,
                previous_epoch_id, material_block_number, tx_hash, already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_new_kms_epoch_row)
        .collect()
    }

    async fn pick_kms_context_destroyed_events(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE kms_context_destroyed
                SET status = 'under_process'
                FROM (
                    SELECT context_id
                    FROM kms_context_destroyed
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE kms_context_destroyed.context_id = req.context_id
                RETURNING req.context_id, tx_hash, already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_kms_context_destroyed_row)
        .collect()
    }

    async fn pick_kms_epoch_destroyed_events(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE kms_epoch_destroyed
                SET status = 'under_process'
                FROM (
                    SELECT epoch_id
                    FROM kms_epoch_destroyed
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE kms_epoch_destroyed.epoch_id = req.epoch_id
                RETURNING req.epoch_id, tx_hash, already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_kms_epoch_destroyed_row)
        .collect()
    }

    async fn pick_crsgen_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE crsgen_requests
                SET status = 'under_process'
                FROM (
                    SELECT crs_id
                    FROM crsgen_requests
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE crsgen_requests.crs_id = req.crs_id
                RETURNING req.crs_id, max_bit_length, params_type, extra_data, tx_hash,
                already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_crsgen_row)
        .collect()
    }

    async fn pick_abort_keygen_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE abort_keygen_requests
                SET status = 'under_process'
                FROM (
                    SELECT prep_keygen_id
                    FROM abort_keygen_requests
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE abort_keygen_requests.prep_keygen_id = req.prep_keygen_id
                RETURNING req.prep_keygen_id, tx_hash, already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_abort_keygen_row)
        .collect()
    }

    async fn pick_abort_crsgen_requests(&self) -> anyhow::Result<Vec<ProtocolEvent>> {
        sqlx::query(
            "
                UPDATE abort_crsgen_requests
                SET status = 'under_process'
                FROM (
                    SELECT crs_id
                    FROM abort_crsgen_requests
                    WHERE status = 'pending'
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE abort_crsgen_requests.crs_id = req.crs_id
                RETURNING req.crs_id, tx_hash, already_sent, created_at, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(event::from_abort_crsgen_row)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_notifications_are_coalesced_per_event_type() {
        // The pool is never used, as no event is picked from the database
        let db_pool = PgPoolOptions::new()
            .connect_lazy("postgres://localhost")
            .unwrap();
        let (notif_sender, notif_receiver) = mpsc::channel(1001);
        let mut event_picker = DbEventPicker::new(db_pool, notif_receiver, 50);

        // A burst of public decryption notifications, followed by one user decryption
        for _ in 0..1000 {
            notif_sender
                .send(EventType::PublicDecryptionRequest)
                .await
                .unwrap();
        }
        notif_sender
            .send(EventType::UserDecryptionRequest)
            .await
            .unwrap();

        // The user decryption must not be queued behind the 1000 public decryption notifications
        event_picker.drain_notifications();
        assert_eq!(
            event_picker.pending_event_types,
            [
                EventType::PublicDecryptionRequest,
                EventType::UserDecryptionRequest
            ]
        );
    }
}
