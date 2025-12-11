use crate::store::sql::client::PgClient;
use anyhow::Result;

pub struct ExpiryRepository {
    pool: PgClient,
}

impl ExpiryRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // TODO: make the values configurables, and pass 7 days user-decryption and input proof to 1 day.
    pub async fn purge_stale_data(&self) -> Result<u64> {
        let public_decrypt = sqlx::query!(
            r#"
            DELETE FROM public_decrypt_req 
            WHERE updated_at < NOW() - INTERVAL '365 days'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        let user_decrypt_shares = sqlx::query!(
            r#"
            DELETE FROM user_decrypt_share 
            WHERE updated_at < NOW() - INTERVAL '7 days'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        let user_decrypt = sqlx::query!(
            r#"
            DELETE FROM user_decrypt_req 
            WHERE updated_at < NOW() - INTERVAL '7 days'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        let input_proof = sqlx::query!(
            r#"
            DELETE FROM input_proof_req 
            WHERE updated_at < NOW() - INTERVAL '7 days'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        Ok(public_decrypt + user_decrypt_shares + user_decrypt + input_proof)
    }
}
