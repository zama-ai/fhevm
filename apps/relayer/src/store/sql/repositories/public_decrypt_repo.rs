use anyhow::Result;
use uuid::Uuid;

use crate::store::sql::client::PgClient;

pub struct PublicDecryptRepository {
    pool: PgClient,
}

impl PublicDecryptRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with err_reason = 'response timed out' (ACL propagation error).
    // OR IN THE TIMEOUT REPO.

    // INITIAL POST REQUEST:

    // Check if there is already existing internal_indexer_id and return ext_reference_id if there is one
    /// Check if there is already an existing internal_indexer_id.
    /// Returns the ext_reference_id (UUID) if found.
    pub async fn find_ext_ref_by_int_indexer_id(
        &self,
        int_indexer_id_bytes: &[u8],
    ) -> Result<Option<Uuid>> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT ext_reference_id
            FROM public_decrypt_req
            WHERE int_indexer_id = $1
            LIMIT 1
            "#,
            int_indexer_id_bytes
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }
}
