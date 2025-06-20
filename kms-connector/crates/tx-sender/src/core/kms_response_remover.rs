use alloy::primitives::U256;
use connector_utils::types::KmsResponse;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use tracing::{info, warn};

/// Interface used to remove KMS Core's responses from some storage.
pub trait KmsResponseRemover: Send {
    fn remove_response(
        &self,
        response: &KmsResponse,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Struct that removes KMS Core's responses from a `Postgres` database.
#[derive(Clone)]
pub struct DbKmsResponseRemover {
    db_pool: Pool<Postgres>,
}

impl KmsResponseRemover for DbKmsResponseRemover {
    async fn remove_response(&self, response: &KmsResponse) -> anyhow::Result<()> {
        info!("Removing {response} from DB...");
        let query_result = match response {
            KmsResponse::PublicDecryption { decryption_id, .. } => {
                self.remove_public_decryption(*decryption_id).await?
            }
            KmsResponse::UserDecryption { decryption_id, .. } => {
                self.remove_user_decryption(*decryption_id).await?
            }
        };
        if query_result.rows_affected() == 1 {
            info!("Successfully removed {response} from DB!");
        } else {
            warn!(
                "Unexpected query result while removing {}: {:?}",
                response, query_result
            )
        }
        Ok(())
    }
}

impl DbKmsResponseRemover {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    async fn remove_public_decryption(&self, decryption_id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM public_decryption_responses WHERE decryption_id = $1",
            decryption_id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }

    async fn remove_user_decryption(&self, decryption_id: U256) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "DELETE FROM user_decryption_responses WHERE decryption_id = $1",
            decryption_id.as_le_slice()
        )
        .execute(&self.db_pool)
        .await
    }
}
