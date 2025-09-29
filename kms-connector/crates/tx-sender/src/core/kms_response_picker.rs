use crate::{
    core::Config,
    monitoring::metrics::{RESPONSE_RECEIVED_COUNTER, RESPONSE_RECEIVED_ERRORS},
};
use anyhow::anyhow;
use connector_utils::types::KmsResponse;
use sqlx::{
    Pool, Postgres,
    postgres::{PgListener, PgNotification},
};
use std::{future::Future, time::Duration};
use tokio::select;
use tracing::{debug, info};

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

    /// The timeout for polling the database for responses.
    polling_timeout: Duration,
}

impl DbKmsResponsePicker {
    pub fn new(
        db_pool: Pool<Postgres>,
        db_listener: PgListener,
        response_batch_size: u8,
        polling_timeout: Duration,
    ) -> Self {
        Self {
            db_pool,
            db_listener,
            responses_batch_size: response_batch_size,
            polling_timeout,
        }
    }

    pub async fn connect(db_pool: Pool<Postgres>, config: &Config) -> anyhow::Result<Self> {
        let db_listener = PgListener::connect_with(&db_pool)
            .await
            .map_err(|e| anyhow!("Failed to init Postgres Listener: {e}"))?;

        let mut response_picker = DbKmsResponsePicker::new(
            db_pool,
            db_listener,
            config.responses_batch_size,
            config.database_polling_timeout,
        );
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
            let responses = select! {
                notification = self.db_listener.recv() => {
                    let notification = notification?;
                    info!("Received Postgres notification: {}", notification.channel());
                    self.pick_notified_responses(notification)
                        .await
                        .inspect_err(|_| RESPONSE_RECEIVED_ERRORS.inc())?
                },
                _ = tokio::time::sleep(self.polling_timeout) => {
                    debug!("Polling timeout, rechecking for responses");
                    self.pick_any_responses().await.inspect_err(|_| RESPONSE_RECEIVED_ERRORS.inc())?
                },
            };

            if responses.is_empty() {
                debug!("Responses have already been picked");
                continue;
            } else {
                info!("Picked {} responses successfully", responses.len());
                RESPONSE_RECEIVED_COUNTER.inc_by(responses.len() as u64);
                return Ok(responses);
            }
        }
    }
}

impl DbKmsResponsePicker {
    async fn pick_notified_responses(
        &self,
        notification: PgNotification,
    ) -> anyhow::Result<Vec<KmsResponse>> {
        match notification.channel() {
            PUBLIC_DECRYPT_NOTIFICATION => self.pick_public_decryption_responses().await,
            USER_DECRYPT_NOTIFICATION => self.pick_user_decryption_responses().await,
            channel => return Err(anyhow!("Unexpected notification: {channel}")),
        }
        .map_err(anyhow::Error::from)
    }

    async fn pick_any_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        Ok([
            self.pick_public_decryption_responses().await?,
            self.pick_user_decryption_responses().await?,
        ]
        .concat())
    }

    async fn pick_public_decryption_responses(&self) -> sqlx::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE public_decryption_responses
                SET under_process = TRUE
                FROM (
                    SELECT decryption_id
                    FROM public_decryption_responses
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE public_decryption_responses.decryption_id = resp.decryption_id
                RETURNING resp.decryption_id, decrypted_result, signature, extra_data
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
                UPDATE user_decryption_responses
                SET under_process = TRUE
                FROM (
                    SELECT decryption_id
                    FROM user_decryption_responses
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE user_decryption_responses.decryption_id = resp.decryption_id
                RETURNING resp.decryption_id, user_decrypted_shares, signature, extra_data
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
