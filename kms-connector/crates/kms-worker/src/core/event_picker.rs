use crate::{
    core::Config,
    monitoring::metrics::{EVENT_RECEIVED_COUNTER, EVENT_RECEIVED_ERRORS},
};
use anyhow::anyhow;
use connector_utils::types::{GatewayEvent, gw_event};
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification},
};
use std::time::Duration;
use tokio::select;
use tracing::{debug, info, warn};

/// Interface used to pick Gateway's events from some storage.
pub trait EventPicker {
    type Event;

    fn pick_events(&mut self) -> impl Future<Output = anyhow::Result<Vec<Self::Event>>>;
}

// Postgres notifications
const PUBLIC_DECRYPT_NOTIFICATION: &str = "public_decryption_request_available";
const USER_DECRYPT_NOTIFICATION: &str = "user_decryption_request_available";
const PREP_KEYGEN_NOTIFICATION: &str = "prep_keygen_request_available";
const KEYGEN_NOTIFICATION: &str = "keygen_request_available";
const CRSGEN_NOTIFICATION: &str = "crsgen_request_available";
const PRSS_INIT_NOTIFICATION: &str = "prss_init_available";
const KEY_RESHARE_SAME_SET_NOTIFICATION: &str = "key_reshare_same_set_available";

/// Struct that collects Gateway's events from a `Postgres` database.
pub struct DbEventPicker {
    /// The DB connection pool used to query events when notified.
    db_pool: Pool<Postgres>,

    /// The DB listener to watch for notifications.
    db_listener: PgListener,

    /// The limit number of events to fetch from the database.
    events_batch_size: u8,

    /// The timeout for polling the database for events.
    polling_timeout: Duration,
}

impl DbEventPicker {
    pub fn new(
        db_pool: Pool<Postgres>,
        db_listener: PgListener,
        events_batch_size: u8,
        polling_timeout: Duration,
    ) -> Self {
        Self {
            db_pool,
            db_listener,
            events_batch_size,
            polling_timeout,
        }
    }

    pub async fn connect(db_pool: Pool<Postgres>, config: &Config) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;

        let mut event_picker = DbEventPicker::new(
            db_pool,
            db_listener,
            config.events_batch_size,
            config.database_polling_timeout,
        );
        event_picker
            .listen()
            .await
            .map_err(|e| anyhow!("Failed to listen to events: {e}"))?;

        Ok(event_picker)
    }

    async fn listen(&mut self) -> sqlx::Result<()> {
        self.db_listener.listen(PUBLIC_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(USER_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(PREP_KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(CRSGEN_NOTIFICATION).await?;
        self.db_listener.listen(PRSS_INIT_NOTIFICATION).await?;
        self.db_listener
            .listen(KEY_RESHARE_SAME_SET_NOTIFICATION)
            .await
    }
}

impl EventPicker for DbEventPicker {
    type Event = GatewayEvent;

    async fn pick_events(&mut self) -> anyhow::Result<Vec<Self::Event>> {
        loop {
            let events = select! {
                notification = self.db_listener.recv() => {
                    let notification = notification?;
                    info!("Received Postgres notification: {}", notification.channel());
                    self.pick_notified_events(notification)
                        .await
                        .inspect_err(|_| EVENT_RECEIVED_ERRORS.inc())?
                },
                _ = tokio::time::sleep(self.polling_timeout) => {
                    debug!("Polling timeout, rechecking for events");
                    self.pick_any_events().await
                },
            };

            if events.is_empty() {
                debug!("Events have already been picked");
                continue;
            } else {
                info!("Picked {} events successfully", events.len());
                EVENT_RECEIVED_COUNTER.inc_by(events.len() as u64);
                return Ok(events);
            }
        }
    }
}

impl DbEventPicker {
    async fn pick_notified_events(
        &self,
        notification: PgNotification,
    ) -> anyhow::Result<Vec<GatewayEvent>> {
        match notification.channel() {
            PUBLIC_DECRYPT_NOTIFICATION => self.pick_public_decryption_requests().await,
            USER_DECRYPT_NOTIFICATION => self.pick_user_decryption_requests().await,
            PREP_KEYGEN_NOTIFICATION => self.pick_prep_keygen_requests().await,
            KEYGEN_NOTIFICATION => self.pick_keygen_requests().await,
            CRSGEN_NOTIFICATION => self.pick_crsgen_requests().await,
            PRSS_INIT_NOTIFICATION => self.pick_prss_init().await,
            KEY_RESHARE_SAME_SET_NOTIFICATION => self.pick_key_reshare_same_set().await,
            channel => Err(anyhow!("Unexpected notification: {channel}")),
        }
    }

    async fn pick_any_events(&self) -> Vec<GatewayEvent> {
        let mut all_events = vec![];
        [
            self.pick_public_decryption_requests().await,
            self.pick_user_decryption_requests().await,
            self.pick_prep_keygen_requests().await,
            self.pick_keygen_requests().await,
            self.pick_crsgen_requests().await,
            self.pick_prss_init().await,
            self.pick_key_reshare_same_set().await,
        ]
        .into_iter()
        .for_each(|res| match res {
            Ok(events) => all_events.extend(events),
            Err(e) => {
                warn!("Failed to fetch events from one of the DB tables: {e}");
                EVENT_RECEIVED_ERRORS.inc();
            }
        });

        all_events
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
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE prep_keygen_requests.prep_keygen_id = req.prep_keygen_id
                RETURNING req.prep_keygen_id, epoch_id, params_type, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
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
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE keygen_requests.key_id = req.key_id
                RETURNING prep_keygen_id, req.key_id, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
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
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE crsgen_requests.crs_id = req.crs_id
                RETURNING req.crs_id, max_bit_length, params_type, otlp_context
            ",
        )
        .bind(self.events_batch_size as i16)
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
