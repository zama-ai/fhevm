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
use std::{collections::VecDeque, future::Future};
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

    /// The response types that need to be picked, without duplicates.
    pending: VecDeque<KmsResponseNotification>,

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
            pending: VecDeque::new(),
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
            if self.pending.is_empty() {
                let Some(notification) = self.notif_receiver.recv().await else {
                    return Err(anyhow!("notification channel was closed!"));
                };
                self.mark_pending(notification);
            }
            self.drain_notifications();

            let Some(notification) = self.pending.pop_front() else {
                continue;
            };

            match self.pick_notified_responses(&notification).await {
                Err(e) => {
                    warn!("Error while picking responses: {e}");
                    RESPONSE_RECEIVED_ERRORS
                        .with_label_values(&[notification.response_str()])
                        .inc();
                    continue;
                }
                Ok(responses) if responses.is_empty() => {
                    debug!("Responses have already been picked");
                    continue;
                }
                Ok(responses) => {
                    // A full batch means more responses may still be pending: re-queue the
                    // response type.
                    if responses.len() >= self.responses_batch_size as usize {
                        self.mark_pending(notification);
                    }
                    info!("Picked {} {} successfully", responses.len(), notification);
                    RESPONSE_RECEIVED_COUNTER
                        .with_label_values(&[notification.response_str()])
                        .inc_by(responses.len() as u64);
                    return Ok(responses);
                }
            }
        }
    }
}

impl DbKmsResponsePicker {
    /// Moves all the notifications already received into the `pending` queue, without blocking.
    ///
    /// A closed channel is ignored here, it will be reported by `recv` once `pending` is empty.
    fn drain_notifications(&mut self) {
        while let Ok(notification) = self.notif_receiver.try_recv() {
            self.mark_pending(notification);
        }
    }

    fn mark_pending(&mut self, notification: KmsResponseNotification) {
        if !self.pending.contains(&notification) {
            self.pending.push_back(notification);
        }
    }

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
            KmsResponseNotification::NewKmsContext => self.pick_new_kms_context_responses().await,
            KmsResponseNotification::EpochResult => self.pick_epoch_result_responses().await,
        }
    }

    async fn pick_public_decryption_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE public_decryption_responses
                SET status = 'under_process'
                FROM (
                    SELECT decryption_id
                    FROM public_decryption_responses
                    WHERE status = 'pending' AND source = 'onchain'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE public_decryption_responses.decryption_id = resp.decryption_id
                RETURNING resp.decryption_id, decrypted_result, signature, extra_data, created_at,
                otlp_context, source
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
                SET status = 'under_process'
                FROM (
                    SELECT decryption_id
                    FROM user_decryption_responses
                    WHERE status = 'pending' AND source = 'onchain'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE user_decryption_responses.decryption_id = resp.decryption_id
                RETURNING resp.decryption_id, user_decrypted_shares, signature, extra_data,
                created_at, otlp_context, source
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
                SET status = 'under_process'
                FROM (
                    SELECT prep_keygen_id
                    FROM prep_keygen_responses
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE prep_keygen_responses.prep_keygen_id = resp.prep_keygen_id
                RETURNING resp.prep_keygen_id, signature, created_at, otlp_context
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
                SET status = 'under_process'
                FROM (
                    SELECT key_id
                    FROM keygen_responses
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE keygen_responses.key_id = resp.key_id
                RETURNING resp.key_id, key_digests, signature, created_at, otlp_context
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
                SET status = 'under_process'
                FROM (
                    SELECT crs_id
                    FROM crsgen_responses
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE crsgen_responses.crs_id = resp.crs_id
                RETURNING resp.crs_id, crs_digest, signature, created_at, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_crsgen_row)
        .collect()
    }

    async fn pick_new_kms_context_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE new_kms_context_responses
                SET status = 'under_process'
                FROM (
                    SELECT context_id
                    FROM new_kms_context_responses
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE new_kms_context_responses.context_id = resp.context_id
                RETURNING resp.context_id, created_at, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_new_kms_context_response_row)
        .collect()
    }

    async fn pick_epoch_result_responses(&self) -> anyhow::Result<Vec<KmsResponse>> {
        sqlx::query(
            "
                UPDATE epoch_result_responses
                SET status = 'under_process'
                FROM (
                    SELECT context_id, epoch_id
                    FROM epoch_result_responses
                    WHERE status = 'pending'
                    LIMIT $1 FOR UPDATE SKIP LOCKED
                ) AS resp
                WHERE epoch_result_responses.epoch_id = resp.epoch_id
                RETURNING resp.context_id, resp.epoch_id, keys, crs_list, created_at, otlp_context
            ",
        )
        .bind(self.responses_batch_size as i16)
        .fetch_all(&self.db_pool)
        .await?
        .iter()
        .map(kms_response::from_epoch_result_row)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_notifications_are_coalesced_per_response_type() {
        // The pool is never used, as no response is picked from the database
        let db_pool = PgPoolOptions::new()
            .connect_lazy("postgres://localhost")
            .unwrap();
        let (notif_sender, notif_receiver) = mpsc::channel(1001);
        let mut response_picker = DbKmsResponsePicker::new(db_pool, notif_receiver, 50);

        // A burst of public decryption notifications, followed by a user decryption one
        for _ in 0..1000 {
            notif_sender
                .send(KmsResponseNotification::PublicDecryption)
                .await
                .unwrap();
        }
        notif_sender
            .send(KmsResponseNotification::UserDecryption)
            .await
            .unwrap();

        // The user decryption must not be queued behind the 1000 public decryption notifications
        response_picker.drain_notifications();
        assert_eq!(
            response_picker.pending,
            [
                KmsResponseNotification::PublicDecryption,
                KmsResponseNotification::UserDecryption
            ]
        );
    }
}
