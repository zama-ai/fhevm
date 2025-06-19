use anyhow::anyhow;
use connector_utils::types::{GatewayEvent, db::GatewayEventTransaction};
use sqlx::{Pool, Postgres, Transaction, postgres::PgListener};
use tracing::{debug, warn};

/// Interface used to pick Gateway's events from some storage.
pub trait EventPicker {
    type Event;

    fn pick_event(&mut self) -> impl Future<Output = anyhow::Result<Self::Event>>;
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
}

impl DbEventPicker {
    pub fn new(db_pool: Pool<Postgres>, db_listener: PgListener) -> Self {
        Self {
            db_pool,
            db_listener,
        }
    }

    pub async fn connect(db_pool: Pool<Postgres>) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;

        let mut event_picker = DbEventPicker::new(db_pool, db_listener);
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
    type Event = GatewayEventTransaction;

    async fn pick_event(&mut self) -> anyhow::Result<Self::Event> {
        loop {
            // Wait for notification
            let notification = self.db_listener.recv().await?;

            // Init transaction before retrieving event from the DB
            let mut tx = self.db_pool.begin().await?;

            let query_result = match notification.channel() {
                PUBLIC_DECRYPT_NOTIFICATION => pick_public_decryption_request(&mut tx).await,
                USER_DECRYPT_NOTIFICATION => pick_user_decryption_request(&mut tx).await,
                PRE_KEYGEN_NOTIFICATION => pick_pre_keygen_request(&mut tx).await,
                PRE_KSKGEN_NOTIFICATION => pick_pre_kskgen_request(&mut tx).await,
                KEYGEN_NOTIFICATION => pick_keygen_request(&mut tx).await,
                KSKGEN_NOTIFICATION => pick_kskgen_request(&mut tx).await,
                CRSGEN_NOTIFICATION => pick_crsgen_request(&mut tx).await,
                channel => {
                    warn!("Unexpected notification: {channel}");
                    continue;
                }
            };

            match query_result {
                Ok(event) => return Ok(GatewayEventTransaction::new(tx, event)),
                Err(sqlx::Error::RowNotFound) => {
                    debug!("Event has already been picked");
                }
                Err(err) => return Err(err.into()),
            }
        }
    }
}

async fn pick_public_decryption_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT decryption_id, sns_ct_materials
            FROM public_decryption_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_public_decryption_row(&row)?;
    Ok(event)
}

async fn pick_user_decryption_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT decryption_id, sns_ct_materials, user_address, public_key
            FROM user_decryption_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_user_decryption_row(&row)?;
    Ok(event)
}

async fn pick_pre_keygen_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT pre_keygen_request_id, fhe_params_digest
            FROM preprocess_keygen_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_pre_keygen_row(&row)?;
    Ok(event)
}

async fn pick_pre_kskgen_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT pre_kskgen_request_id, fhe_params_digest
            FROM preprocess_kskgen_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_pre_kskgen_row(&row)?;
    Ok(event)
}

async fn pick_keygen_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT pre_key_id, fhe_params_digest
            FROM keygen_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_keygen_row(&row)?;
    Ok(event)
}

async fn pick_kskgen_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT pre_ksk_id, source_key_id, dest_key_id, fhe_params_digest
            FROM kskgen_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_kskgen_row(&row)?;
    Ok(event)
}

async fn pick_crsgen_request(
    tx: &mut Transaction<'static, Postgres>,
) -> sqlx::Result<GatewayEvent> {
    let row = sqlx::query(
        "
            SELECT crsgen_request_id, fhe_params_digest
            FROM crsgen_requests
            LIMIT 1 FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_one(tx.as_mut())
    .await?;
    let event = GatewayEvent::from_crsgen_row(&row)?;
    Ok(event)
}
