use crate::{
    core::{
        Config,
        kms_response_picker::notifier::{DbKmsResponseNotifier, KmsResponseNotification},
    },
    monitoring::metrics::{RESPONSE_RECEIVED_COUNTER, RESPONSE_RECEIVED_ERRORS},
};
use anyhow::anyhow;
use connector_utils::types::{KmsResponse, kms_response};
use sqlx::{Pool, Postgres};
use std::future::Future;
use tokio::sync::mpsc::{self, Receiver};
use tracing::{debug, info, warn};

/// Interface used to pick KMS Core's responses from some storage.
pub trait KmsResponsePicker {
    fn pick_responses(&mut self) -> impl Future<Output = anyhow::Result<Vec<KmsResponse>>>;
}

/// Struct that collects KMS Core's responses from a `Postgres` database.
pub struct DbKmsResponsePicker {
    /// The DB connection pool used to query responses when notified.
    db_pool: Pool<Postgres>,

    /// The receiver channel used to receive KMS response notification.
    notif_receiver: Receiver<KmsResponseNotification>,

    /// The maximum number of responses to fetch at once.
    responses_batch_size: u8,
}

impl DbKmsResponsePicker {
    pub fn new(
        db_pool: Pool<Postgres>,
        notif_receiver: Receiver<KmsResponseNotification>,
        responses_batch_size: u8,
    ) -> Self {
        Self {
            db_pool,
            notif_receiver,
            responses_batch_size,
        }
    }

    pub async fn connect(db_pool: Pool<Postgres>, config: &Config) -> anyhow::Result<Self> {
        let (notif_sender, notif_receiver) = mpsc::channel(config.task_limit);
        let response_notifier =
            DbKmsResponseNotifier::connect(db_pool.clone(), notif_sender, config).await?;
        tokio::spawn(response_notifier.start());

        let response_picker =
            DbKmsResponsePicker::new(db_pool, notif_receiver, config.responses_batch_size);
        Ok(response_picker)
    }
}

impl KmsResponsePicker for DbKmsResponsePicker {
    /// Picks KMS responses from the database.
    ///
    /// Should only return an error if the notification channel is closed, so the `tx_sender` can
    /// shutdown gracefully.
    /// If another error is encountered, it will just be logged with a warning and will wait for
    /// next responses.
    async fn pick_responses(&mut self) -> anyhow::Result<Vec<KmsResponse>> {
        loop {
            let Some(notification) = self.notif_receiver.recv().await else {
                return Err(anyhow!("notification channel was closed!"));
            };

            match self.pick_notified_responses(&notification).await {
                Err(e) => {
                    warn!("Error while picking responses: {e}");
                    RESPONSE_RECEIVED_ERRORS.inc();
                    continue;
                }
                Ok(responses) if responses.is_empty() => {
                    debug!("Responses have already been picked");
                    continue;
                }
                Ok(responses) => {
                    info!(
                        "Picked {} {} successfully",
                        responses.len(),
                        notification.response_str()
                    );
                    RESPONSE_RECEIVED_COUNTER.inc_by(responses.len() as u64);
                    return Ok(responses);
                }
            }
        }
    }
}

impl DbKmsResponsePicker {
    async fn pick_notified_responses(
        &self,
        notification: &KmsResponseNotification,
    ) -> anyhow::Result<Vec<KmsResponse>> {
        match notification {
            KmsResponseNotification::PublicDecryption => {
                self.pick_public_decryption_responses().await
            }
            KmsResponseNotification::UserDecryption => self.pick_user_decryption_responses().await,
            KmsResponseNotification::PrepKeygen => self.pick_prep_keygen_responses().await,
            KmsResponseNotification::Keygen => self.pick_keygen_responses().await,
            KmsResponseNotification::Crsgen => self.pick_crsgen_responses().await,
        }
    }

    async fn pick_public_decryption_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
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
                RETURNING resp.decryption_id, decrypted_result, signature, extra_data, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_public_decryption_row)
        .collect()
    }

    async fn pick_user_decryption_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
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
                RETURNING resp.decryption_id, user_decrypted_shares, signature, extra_data, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_user_decryption_row)
        .collect()
    }

    async fn pick_prep_keygen_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE prep_keygen_responses
                SET under_process = TRUE
                FROM (
                    SELECT prep_keygen_id
                    FROM prep_keygen_responses
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE prep_keygen_responses.prep_keygen_id = resp.prep_keygen_id
                RETURNING resp.prep_keygen_id, signature, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_prep_keygen_row)
        .collect()
    }

    async fn pick_keygen_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE keygen_responses
                SET under_process = TRUE
                FROM (
                    SELECT key_id
                    FROM keygen_responses
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE keygen_responses.key_id = resp.key_id
                RETURNING resp.key_id, key_digests, signature, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_keygen_row)
        .collect()
    }

    async fn pick_crsgen_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE crsgen_responses
                SET under_process = TRUE
                FROM (
                    SELECT crs_id
                    FROM crsgen_responses
                    WHERE under_process = FALSE
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE crsgen_responses.crs_id = resp.crs_id
                RETURNING resp.crs_id, crs_digest, signature, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_crsgen_row)
        .collect()
    }
}
