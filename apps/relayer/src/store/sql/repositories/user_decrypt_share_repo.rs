use anyhow::Result;

use crate::store::sql::{client::PgClient, models::user_decrypt_share_model::UserDecryptShare};

pub struct UserDecryptShareRepository {
    pool: PgClient,
}

impl UserDecryptShareRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// (High frequency): Insert a new share for a given gateway decryption ID.
    pub async fn insert_share(
        &self,
        gw_decryption_id: i32,
        share_index: i32,
        share: &str,
        kms_signature: &str,
        extra_data: Option<&str>,
    ) -> Result<UserDecryptShare> {
        let share = sqlx::query_as!(
            UserDecryptShare,
            r#"
            INSERT INTO user_decrypt_share (gw_decryption_id, share_index, share, kms_signature, extra_data)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            gw_decryption_id,
            share_index,
            share,
            kms_signature,
            extra_data
        )
        .fetch_one(&self.pool.get_pool())
        .await?;
        Ok(share)
    }

    /// (APPLICATION LOGIC helper): Count existing shares for a gateway ID.
    pub async fn count_shares_by_gw_id(&self, gw_decryption_id: i32) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            SELECT count(*) as "count!"
            FROM user_decrypt_share
            WHERE gw_decryption_id = $1
            "#,
            gw_decryption_id
        )
        .fetch_one(&self.pool.get_pool())
        .await?;
        Ok(result.count)
    }

    /// (Medium frequency): Select all shares by gateway Id.
    pub async fn find_all_by_gw_id(&self, gw_decryption_id: i32) -> Result<Vec<UserDecryptShare>> {
        let shares = sqlx::query_as!(
            UserDecryptShare,
            r#"
            SELECT *
            FROM user_decrypt_share
            WHERE gw_decryption_id = $1
            ORDER BY share_index ASC
            "#,
            gw_decryption_id,
        )
        .fetch_all(&self.pool.get_pool())
        .await?;
        Ok(shares)
    }

    /// (Periodic query): LATER: Remove shares by gateway Id.
    pub async fn delete_by_gw_id(&self, gw_decryption_id: i32) -> Result<u64> {
        let rows_affected = sqlx::query!(
            "DELETE FROM user_decrypt_share WHERE gw_decryption_id = $1",
            gw_decryption_id
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }
}
