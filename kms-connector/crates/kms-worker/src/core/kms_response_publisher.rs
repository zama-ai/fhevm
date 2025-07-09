use alloy::primitives::U256;
use connector_utils::types::KmsResponse;
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
    async fn publish(&self, response: KmsResponse) -> anyhow::Result<()> {
        let response_str = response.to_string();
        info!("Storing {response_str} in DB...");

        // Execute sqlx query
        let sqlx_result = match response.clone() {
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
        };

        // Mark event associated to the current response as free on error
        let query_result = match sqlx_result {
            Ok(result) => result,
            Err(e) => {
                response.free_associated_event(&self.db_pool).await;
                return Err(e.into());
            }
        };

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
}

impl DbKmsResponsePublisher {
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
