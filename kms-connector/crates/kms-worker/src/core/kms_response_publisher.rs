use alloy::primitives::U256;
use connector_utils::types::KmsResponse;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

/// Struct that stores KMS Core's responses in a `Postgres` database.
pub struct KmsResponsePublisher {
    /// The database used to store `KmsResponse`.
    db_pool: Pool<Postgres>,

    /// The `Receiver` channel used to collect `KmsResponse` to publish.
    receiver: Receiver<KmsResponse>,
}

impl KmsResponsePublisher {
    pub fn new(db_pool: Pool<Postgres>, receiver: Receiver<KmsResponse>) -> Self {
        Self { db_pool, receiver }
    }

    /// Starts the `ResponsePublisher`.
    pub async fn start(self, cancel_token: CancellationToken) {
        debug!("Starting ResponsePublisher");
        tokio::select! {
            _ = cancel_token.cancelled() => debug!("Stopping ResponsePublisher"),
            _ = self.run() => (),
        }
    }

    /// Runs the event handling loop of the `ResponsePublisher`.
    async fn run(mut self) {
        loop {
            match self.receiver.recv().await {
                Some(response) => {
                    if let Err(e) = self.publish(response.clone()).await {
                        error!("Failed to publish response {response}: {e}");
                        response
                            .mark_associated_event_as_pending(&self.db_pool)
                            .await;
                    }
                }
                None => break warn!("Channel closed"),
            };
        }
    }

    /// Publishes the `response` into the database.
    pub async fn publish(&self, response: KmsResponse) -> anyhow::Result<()> {
        let response_str = response.to_string();
        info!("Storing {response_str} in DB...");

        let query_result = match response.clone() {
            KmsResponse::PublicDecryption {
                decryption_id: id,
                decrypted_result: result,
                signature,
            } => self.publish_public_decryption(id, result, signature).await,
            KmsResponse::UserDecryption {
                decryption_id: id,
                user_decrypted_shares: shares,
                signature,
            } => self.publish_user_decryption(id, shares, signature).await,
        }?;

        // Check query result is what we expect
        if query_result.rows_affected() == 1 {
            info!("Successfully stored {response_str} in DB!");
        } else {
            warn!(
                "Unexpected query result while publishing {}: {:?}",
                response_str, query_result
            )
        }
        Ok(())
    }

    async fn publish_public_decryption(
        &self,
        decryption_id: U256,
        decrypted_result: Vec<u8>,
        signature: Vec<u8>,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO public_decryption_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            decryption_id.as_le_slice(),
            decrypted_result,
            signature,
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_user_decryption(
        &self,
        decryption_id: U256,
        user_decrypted_shares: Vec<u8>,
        signature: Vec<u8>,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO user_decryption_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            decryption_id.as_le_slice(),
            user_decrypted_shares,
            signature,
        )
        .execute(&self.db_pool)
        .await
    }
}
