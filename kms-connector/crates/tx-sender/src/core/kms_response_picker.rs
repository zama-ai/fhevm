use anyhow::anyhow;
use connector_utils::types::KmsResponse;
use sqlx::{Pool, Postgres, postgres::PgListener};
use std::future::Future;
use tracing::info;

/// Interface used to pick KMS Core's responses from some storage.
pub trait KmsResponsePicker {
    fn pick_response(&mut self) -> impl Future<Output = anyhow::Result<KmsResponse>>;
}

// Postgres notifications for KMS Core's responses
const PUBLIC_DECRYPT_NOTIFICATION: &str = "public_decryption_response_available";
const USER_DECRYPT_NOTIFICATION: &str = "user_decryption_response_available";

/// Struct that collects KMS Core's responses from a `Postgres` database.
pub struct DbKmsResponsePicker {
    /// The DB connection pool used to query responses when notified.
    db_pool: Pool<Postgres>,

    /// The DB listener to watch for notifications.
    db_listener: PgListener,
}

impl DbKmsResponsePicker {
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

        let mut response_picker = DbKmsResponsePicker::new(db_pool, db_listener);
        response_picker
            .listen()
            .await
            .map_err(|e| anyhow!("Failed to listen to responses: {e}"))?;

        Ok(response_picker)
    }

    async fn listen(&mut self) -> sqlx::Result<()> {
        self.db_listener.listen(PUBLIC_DECRYPT_NOTIFICATION).await?;
        self.db_listener.listen(USER_DECRYPT_NOTIFICATION).await
    }
}

impl KmsResponsePicker for DbKmsResponsePicker {
    async fn pick_response(&mut self) -> anyhow::Result<KmsResponse> {
        // Wait for notification
        let notification = self.db_listener.recv().await?;
        info!("Received Postgres notification: {}", notification.channel());

        let response = match notification.channel() {
            PUBLIC_DECRYPT_NOTIFICATION => pick_public_decryption_response(&self.db_pool).await?,
            USER_DECRYPT_NOTIFICATION => pick_user_decryption_response(&self.db_pool).await?,
            channel => return Err(anyhow!("Unexpected notification: {channel}")),
        };

        Ok(response)
    }
}

async fn pick_public_decryption_response(db_pool: &Pool<Postgres>) -> sqlx::Result<KmsResponse> {
    let row = sqlx::query(
        "
            SELECT decryption_id, decrypted_result, signature
            FROM public_decryption_responses
            LIMIT 1
        ",
    )
    .fetch_one(db_pool)
    .await?;

    KmsResponse::from_public_decryption_row(&row)
}

async fn pick_user_decryption_response(db_pool: &Pool<Postgres>) -> sqlx::Result<KmsResponse> {
    let row = sqlx::query(
        "
            SELECT decryption_id, user_decrypted_shares, signature
            FROM user_decryption_responses
            LIMIT 1
        ",
    )
    .fetch_one(db_pool)
    .await?;

    KmsResponse::from_user_decryption_row(&row)
}
