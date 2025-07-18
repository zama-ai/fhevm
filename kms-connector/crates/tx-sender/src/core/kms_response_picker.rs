use crate::metrics::{RESPONSE_RECEIVED_COUNTER, RESPONSE_RECEIVED_ERRORS};
use anyhow::anyhow;
use connector_utils::types::KmsResponse;
use sqlx::{Pool, Postgres, postgres::PgListener};
use std::future::Future;
use tracing::{debug, info, warn};

/// Interface used to pick KMS Core's responses from some storage.
pub trait KmsResponsePicker {
    fn pick_responses(&mut self) -> impl Future<Output = anyhow::Result<Vec<KmsResponse>>>;
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

    /// The maximum number of responses to fetch at once.
    responses_batch_size: u8,
}

impl DbKmsResponsePicker {
    pub fn new(db_pool: Pool<Postgres>, db_listener: PgListener, response_batch_size: u8) -> Self {
        Self {
            db_pool,
            db_listener,
            responses_batch_size: response_batch_size,
        }
    }

    pub async fn connect(db_pool: Pool<Postgres>, response_batch_size: u8) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;

        let mut response_picker =
            DbKmsResponsePicker::new(db_pool, db_listener, response_batch_size);
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
    async fn pick_responses(&mut self) -> anyhow::Result<Vec<KmsResponse>> {
        loop {
            // Wait for notification
            let notification = self.db_listener.recv().await?;
            info!("Received Postgres notification: {}", notification.channel());

            let query_result = match notification.channel() {
                PUBLIC_DECRYPT_NOTIFICATION => self.pick_public_decryption_responses().await,
                USER_DECRYPT_NOTIFICATION => self.pick_user_decryption_responses().await,
                channel => {
                    warn!("Unexpected notification: {channel}");
                    continue;
                }
            };

            match query_result {
                Ok(responses) => {
                    if responses.is_empty() {
                        debug!("Responses have already been picked");
                        continue;
                    }
                    info!("Picked {} responses successfully", responses.len());
                    RESPONSE_RECEIVED_COUNTER.inc_by(responses.len() as u64);
                    return Ok(responses);
                }
                Err(err) => {
                    RESPONSE_RECEIVED_ERRORS.inc();
                    return Err(err.into());
                }
            }
        }
    }
}

impl DbKmsResponsePicker {
    async fn pick_public_decryption_responses(&self) -> sqlx::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                SELECT decryption_id, decrypted_result, signature
                FROM public_decryption_responses
                LIMIT $1
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(KmsResponse::from_public_decryption_row)
        .collect()
    }

    async fn pick_user_decryption_responses(&self) -> sqlx::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                SELECT decryption_id, user_decrypted_shares, signature
                FROM user_decryption_responses
                LIMIT $1
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(KmsResponse::from_user_decryption_row)
        .collect()
    }
}
