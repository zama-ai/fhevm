use crate::{
    core::{
        Config,
        event_picker::notifier::{DbEventNotifier, EventNotification},
    },
    monitoring::metrics::{EVENT_RECEIVED_COUNTER, EVENT_RECEIVED_ERRORS},
};
use anyhow::anyhow;
use connector_utils::types::{GatewayEvent, gw_event};
use sqlx::{Pool, Postgres};
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
    notif_receiver: Receiver<EventNotification>,

    /// The limit number of events to fetch from the database.
    events_batch_size: u8,
}

impl DbEventPicker {
    pub fn new(
        db_pool: Pool<Postgres>,
        notif_receiver: Receiver<EventNotification>,
        events_batch_size: u8,
    ) -> Self {
        Self {
            db_pool,
            notif_receiver,
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
    type Event = GatewayEvent;

    /// Picks events from the database.
    ///
    /// Should only return an error if the notification channel is closed, so the `kms_worker` can
    /// shutdown gracefully.
    /// If another error is encountered, it will just be logged with a warning and will wait for
    /// next events.
    async fn pick_events(&mut self) -> anyhow::Result<Vec<Self::Event>> {
        loop {
            let Some(notification) = self.notif_receiver.recv().await else {
                return Err(anyhow!("notification channel was closed!"));
            };

            match self.pick_notified_events(&notification).await {
                Err(e) => {
                    warn!("Error while picking events: {e}");
                    EVENT_RECEIVED_ERRORS.inc();
                    continue;
                }
                Ok(events) if events.is_empty() => {
                    debug!("Events have already been picked");
                    continue;
                }
                Ok(events) => {
                    info!(
                        "Picked {} {} successfully",
                        events.len(),
                        notification.event_str()
                    );
                    EVENT_RECEIVED_COUNTER.inc_by(events.len() as u64);
                    return Ok(events);
                }
            }
        }
    }
}

impl DbEventPicker {
    async fn pick_notified_events(
        &self,
        notification: &EventNotification,
    ) -> anyhow::Result<Vec<GatewayEvent>> {
        match notification {
            EventNotification::PublicDecryption => self.pick_public_decryption_requests().await,
            EventNotification::UserDecryption => self.pick_user_decryption_requests().await,
            EventNotification::PrepKeygen => self.pick_prep_keygen_requests().await,
            EventNotification::Keygen => self.pick_keygen_requests().await,
            EventNotification::Crsgen => self.pick_crsgen_requests().await,
            EventNotification::PrssInit => self.pick_prss_init().await,
            EventNotification::KeyReshareSameSet => self.pick_key_reshare_same_set().await,
        }
    }

    async fn pick_public_decryption_requests(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE public_decryption_requests
                SET under_process = TRUE
                FROM (
                    SELECT decryption_id
                    FROM public_decryption_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE public_decryption_requests.decryption_id = req.decryption_id
                RETURNING req.decryption_id, sns_ct_materials, extra_data, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_public_decryption_row)
        .collect()
    }

    async fn pick_user_decryption_requests(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE user_decryption_requests
                SET under_process = TRUE
                FROM (
                    SELECT decryption_id
                    FROM user_decryption_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE user_decryption_requests.decryption_id = req.decryption_id
                RETURNING req.decryption_id, sns_ct_materials, user_address, public_key, extra_data, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_user_decryption_row)
        .collect()
    }

    async fn pick_prep_keygen_requests(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE prep_keygen_requests
                SET under_process = TRUE
                FROM (
                    SELECT prep_keygen_id
                    FROM prep_keygen_requests
                    WHERE under_process = FALSE
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE prep_keygen_requests.prep_keygen_id = req.prep_keygen_id
                RETURNING req.prep_keygen_id, epoch_id, params_type, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_prep_keygen_row)
        .collect()
    }

    async fn pick_keygen_requests(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE keygen_requests
                SET under_process = TRUE
                FROM (
                    SELECT key_id
                    FROM keygen_requests
                    WHERE under_process = FALSE
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE keygen_requests.key_id = req.key_id
                RETURNING prep_keygen_id, req.key_id, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_keygen_row)
        .collect()
    }

    async fn pick_crsgen_requests(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE crsgen_requests
                SET under_process = TRUE
                FROM (
                    SELECT crs_id
                    FROM crsgen_requests
                    WHERE under_process = FALSE
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE crsgen_requests.crs_id = req.crs_id
                RETURNING req.crs_id, max_bit_length, params_type, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_crsgen_row)
        .collect()
    }

    async fn pick_prss_init(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE prss_init
                SET under_process = TRUE
                FROM (
                    SELECT id
                    FROM prss_init
                    WHERE under_process = FALSE
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE prss_init.id = req.id
                RETURNING req.id, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_prss_init_row)
        .collect()
    }

    async fn pick_key_reshare_same_set(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE key_reshare_same_set
                SET under_process = TRUE
                FROM (
                    SELECT key_id
                    FROM key_reshare_same_set
                    WHERE under_process = FALSE
                    LIMIT 1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE key_reshare_same_set.key_id = req.key_id
                RETURNING prep_keygen_id, req.key_id, key_reshare_id, params_type, otlp_context
            ",
        )
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(gw_event::from_key_reshare_same_set_row)
        .collect()
    }
}
