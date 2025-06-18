use anyhow::anyhow;
use connector_utils::types::{GatewayEvent, db::GatewayEventTransaction};
use sqlx::{Pool, Postgres, postgres::PgListener};

pub trait EventPicker {
    type Event;

    fn pick_event(&mut self) -> impl Future<Output = anyhow::Result<Self::Event>>;
}

const PUBLIC_DECRYPT_NOTIFICATION: &str = "public_decryption_request_available";
const USER_DECRYPT_NOTIFICATION: &str = "user_decryption_request_available";
const PRE_KEYGEN_NOTIFICATION: &str = "preprocess_keygen_request_available";
const PRE_KSKGEN_NOTIFICATION: &str = "preprocess_kskgen_request_available";
const KEYGEN_NOTIFICATION: &str = "keygen_request_available";
const KSKGEN_NOTIFICATION: &str = "kskgen_request_available";
const CRSGEN_NOTIFICATION: &str = "crs_request_available";

pub struct DbEventPicker {
    db_pool: Pool<Postgres>,
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
        let notification = self.db_listener.recv().await?;
        let tx = self.db_pool.begin().await?;
        let event = match notification.channel() {
            PUBLIC_DECRYPT_NOTIFICATION => self.pick_public_decryption_request().await?,
            USER_DECRYPT_NOTIFICATION => self.pick_user_decryption_request().await?,
            PRE_KEYGEN_NOTIFICATION => self.pick_pre_keygen_request().await?,
            PRE_KSKGEN_NOTIFICATION => self.pick_pre_kskgen_request().await?,
            KEYGEN_NOTIFICATION => self.pick_keygen_request().await?,
            KSKGEN_NOTIFICATION => self.pick_kskgen_request().await?,
            CRSGEN_NOTIFICATION => self.pick_crsgen_request().await?,
            channel => return Err(anyhow!("Unexpected notification: {channel}")),
        };
        Ok(GatewayEventTransaction::new(tx, event))
    }
}

impl DbEventPicker {
    async fn pick_public_decryption_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_public_decryption_row(&row)?;
        Ok(event)
    }

    async fn pick_user_decryption_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_user_decryption_row(&row)?;
        Ok(event)
    }

    async fn pick_pre_keygen_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_pre_keygen_row(&row)?;
        Ok(event)
    }

    async fn pick_pre_kskgen_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_pre_kskgen_row(&row)?;
        Ok(event)
    }

    async fn pick_keygen_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_keygen_row(&row)?;
        Ok(event)
    }

    async fn pick_kskgen_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_kskgen_row(&row)?;
        Ok(event)
    }

    async fn pick_crsgen_request(&self) -> anyhow::Result<GatewayEvent> {
        let row = sqlx::query("TODO").fetch_one(&self.db_pool).await?;
        let event = GatewayEvent::from_crsgen_row(&row)?;
        Ok(event)
    }
}
