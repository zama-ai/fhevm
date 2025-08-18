use std::time::Duration;

use crate::{
    core::Config,
    monitoring::metrics::{EVENT_RECEIVED_COUNTER, EVENT_RECEIVED_ERRORS},
};
use anyhow::anyhow;
use connector_utils::types::GatewayEvent;
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification},
};
use tokio::select;
use tracing::{debug, info};

/// Interface used to pick Gateway's events from some storage.
pub trait EventPicker {
    type Event;

    fn pick_events(&mut self) -> impl Future<Output = anyhow::Result<Vec<Self::Event>>>;
}

// Postgres notifications
const PUBLIC_DECRYPT_NOTIFICATION: &str = "public_decryption_request_available";
const USER_DECRYPT_NOTIFICATION: &str = "user_decryption_request_available";
const PRE_KEYGEN_NOTIFICATION: &str = "preprocess_keygen_request_available";
const PRE_KSKGEN_NOTIFICATION: &str = "preprocess_kskgen_request_available";
const KEYGEN_NOTIFICATION: &str = "keygen_request_available";
const KSKGEN_NOTIFICATION: &str = "kskgen_request_available";
const CRSGEN_NOTIFICATION: &str = "crs_request_available";

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
        self.db_listener.listen(PRE_KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(PRE_KSKGEN_NOTIFICATION).await?;
        self.db_listener.listen(KEYGEN_NOTIFICATION).await?;
        self.db_listener.listen(KSKGEN_NOTIFICATION).await?;
        self.db_listener.listen(CRSGEN_NOTIFICATION).await
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
                    self.pick_any_events().await.inspect_err(|_| EVENT_RECEIVED_ERRORS.inc())?
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
            PRE_KEYGEN_NOTIFICATION => self.pick_pre_keygen_requests().await,
            PRE_KSKGEN_NOTIFICATION => self.pick_pre_kskgen_requests().await,
            KEYGEN_NOTIFICATION => self.pick_keygen_requests().await,
            KSKGEN_NOTIFICATION => self.pick_kskgen_requests().await,
            CRSGEN_NOTIFICATION => self.pick_crsgen_requests().await,
            channel => return Err(anyhow!("Unexpected notification: {channel}")),
        }
        .map_err(anyhow::Error::from)
    }

    async fn pick_any_events(&self) -> anyhow::Result<Vec<GatewayEvent>> {
        Ok([
            self.pick_public_decryption_requests().await?,
            self.pick_user_decryption_requests().await?,
            self.pick_pre_keygen_requests().await?,
            self.pick_pre_kskgen_requests().await?,
            self.pick_keygen_requests().await?,
            self.pick_kskgen_requests().await?,
            self.pick_crsgen_requests().await?,
        ]
        .concat())
    }

    async fn pick_public_decryption_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
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
                RETURNING req.decryption_id, sns_ct_materials, extra_data
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_public_decryption_row)
        .collect()
    }

    async fn pick_user_decryption_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
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
                RETURNING req.decryption_id, sns_ct_materials, user_address, public_key, extra_data
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_user_decryption_row)
        .collect()
    }

    async fn pick_pre_keygen_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE preprocess_keygen_requests
                SET under_process = TRUE
                FROM (
                    SELECT pre_keygen_request_id
                    FROM preprocess_keygen_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE preprocess_keygen_requests.pre_keygen_request_id = req.pre_keygen_request_id
                RETURNING req.pre_keygen_request_id, fhe_params_digest
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_pre_keygen_row)
        .collect()
    }

    async fn pick_pre_kskgen_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE preprocess_kskgen_requests
                SET under_process = TRUE
                FROM (
                    SELECT pre_kskgen_request_id
                    FROM preprocess_kskgen_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE preprocess_kskgen_requests.pre_kskgen_request_id = req.pre_kskgen_request_id
                RETURNING req.pre_kskgen_request_id, fhe_params_digest
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_pre_kskgen_row)
        .collect()
    }

    async fn pick_keygen_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE keygen_requests
                SET under_process = TRUE
                FROM (
                    SELECT pre_key_id
                    FROM keygen_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE keygen_requests.pre_key_id = req.pre_key_id
                RETURNING req.pre_key_id, fhe_params_digest
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_keygen_row)
        .collect()
    }

    async fn pick_kskgen_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE kskgen_requests
                SET under_process = TRUE
                FROM (
                    SELECT pre_ksk_id
                    FROM kskgen_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE kskgen_requests.pre_ksk_id = req.pre_ksk_id
                RETURNING req.pre_ksk_id, source_key_id, dest_key_id, fhe_params_digest
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_kskgen_row)
        .collect()
    }

    async fn pick_crsgen_requests(&self) -> sqlx::Result<Vec<GatewayEvent>> {
        sqlx::query(
            "
                UPDATE crsgen_requests
                SET under_process = TRUE
                FROM (
                    SELECT crsgen_request_id
                    FROM crsgen_requests
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS req
                WHERE crsgen_requests.crsgen_request_id = req.crsgen_request_id
                RETURNING req.crsgen_request_id, fhe_params_digest
            ",
        )
        .bind(self.events_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(GatewayEvent::from_crsgen_row)
        .collect()
    }
}
