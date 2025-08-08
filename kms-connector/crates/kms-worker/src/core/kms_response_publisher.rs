use connector_utils::types::{KmsResponse, PublicDecryptionResponse, UserDecryptionResponse};
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tracing::{info, warn};

/// Interface used to publish KMS Core's responses in some storage.
pub trait KmsResponsePublisher {
    fn publish(&self, response: KmsResponse) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Struct that stores KMS Core's responses in a `Postgres` database.
#[derive(Clone)]
pub struct DbKmsResponsePublisher {
    db_pool: Pool<Postgres>,
}

impl DbKmsResponsePublisher {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

impl KmsResponsePublisher for DbKmsResponsePublisher {
    #[tracing::instrument(skip_all)]
    async fn publish(&self, response: KmsResponse) -> anyhow::Result<()> {
        info!("Storing response in DB...");

        // Execute sqlx query
        let sqlx_result = match response.clone() {
            KmsResponse::PublicDecryption(response) => {
                self.publish_public_decryption(response).await
            }
            KmsResponse::UserDecryption(response) => self.publish_user_decryption(response).await,
        };

        // Mark event associated to the current response as free on error
        let query_result = match sqlx_result {
            Ok(result) => result,
            Err(e) => {
                response
                    .mark_associated_event_as_pending(&self.db_pool)
                    .await;
                return Err(e.into());
            }
        };

        // Check query result is what we expect
        if query_result.rows_affected() == 1 {
            info!("Successfully stored response in DB!");
        } else {
            warn!("Unexpected query result while publishing response: {query_result:?}");
        }
        Ok(())
    }
}

impl DbKmsResponsePublisher {
    async fn publish_public_decryption(
        &self,
        response: PublicDecryptionResponse,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO public_decryption_responses VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            response.decryption_id.as_le_slice(),
            response.decrypted_result,
            response.signature,
            response.extra_data,
        )
        .execute(&self.db_pool)
        .await
    }

    async fn publish_user_decryption(
        &self,
        response: UserDecryptionResponse,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "INSERT INTO user_decryption_responses VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            response.decryption_id.as_le_slice(),
            response.user_decrypted_shares,
            response.signature,
            response.extra_data,
        )
        .execute(&self.db_pool)
        .await
    }
}
