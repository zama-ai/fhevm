use alloy::primitives::U256;
use connector_utils::types::KmsResponse;
use sqlx::{
    Pool, Postgres,
    postgres::{PgArguments, PgQueryResult},
    query::Query,
};
use tracing::{info, warn};

/// Interface used to remove KMS Core's responses from some storage.
pub trait KmsResponseRemover: Send {
    fn remove_response(
        &self,
        response: &KmsResponse,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn mark_response_as_pending(&self, response: KmsResponse) -> impl Future<Output = ()> + Send;
}

/// Struct that removes KMS Core's responses from a `Postgres` database.
#[derive(Clone)]
pub struct DbKmsResponseRemover {
    db_pool: Pool<Postgres>,
}

impl KmsResponseRemover for DbKmsResponseRemover {
    #[tracing::instrument(skip_all)]
    async fn remove_response(&self, response: &KmsResponse) -> anyhow::Result<()> {
        info!("Removing response from DB...");

        let query_result = match response {
            KmsResponse::PublicDecryption(r) => {
                self.remove_public_decryption(r.decryption_id).await?
            }
            KmsResponse::UserDecryption(r) => self.remove_user_decryption(r.decryption_id).await?,
        };

        if query_result.rows_affected() == 1 {
            info!("Successfully removed response from DB!");
        } else {
            warn!("Unexpected query result while removing response: {query_result:?}");
        }
        Ok(())
    }

    /// Sets the `under_process` field of the response as `FALSE` in the database.
    #[tracing::instrument(skip_all)]
    async fn mark_response_as_pending(&self, response: KmsResponse) {
        match response {
            KmsResponse::PublicDecryption(r) => {
                self.mark_public_decryption_as_pending(r.decryption_id)
                    .await
            }
            KmsResponse::UserDecryption(r) => {
                self.mark_user_decryption_as_pending(r.decryption_id).await
            }
        };
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

    /// Sets the `under_process` field of the `PublicDecryptionResponse` as `FALSE` in the database.
    pub async fn mark_public_decryption_as_pending(&self, id: U256) {
        let query = sqlx::query!(
            "UPDATE public_decryption_responses SET under_process = FALSE WHERE decryption_id = $1",
            id.as_le_slice()
        );
        self.execute_free_response_query(query).await;
    }

    /// Sets the `under_process` field of the `UserDecryptionResponse` as `FALSE` in the database.
    pub async fn mark_user_decryption_as_pending(&self, id: U256) {
        let query = sqlx::query!(
            "UPDATE user_decryption_responses SET under_process = FALSE WHERE decryption_id = $1",
            id.as_le_slice()
        );
        self.execute_free_response_query(query).await;
    }

    /// Executes the query to mark the restore the response's `under_process` field to `FALSE`.
    async fn execute_free_response_query(&self, query: Query<'_, Postgres, PgArguments>) {
        warn!("Failed to process response. Restoring `under_process` field to `FALSE` in DB...");
        let query_result = match query.execute(&self.db_pool).await {
            Ok(result) => result,
            Err(e) => return warn!("Failed to restore `under_process` field to `FALSE`: {e}"),
        };

        if query_result.rows_affected() == 1 {
            info!("Successfully restore `under_process` field to `FALSE` in DB!");
        } else {
            warn!(
                "Unexpected query result while restoring `under_process` field to `FALSE`: {:?}",
                query_result
            )
        }
    }
}
