use crate::core::solana_acl::{
    SolanaNativeAcceptedRequestV0, SolanaNativeReplayAction, SolanaNativeReplayKeyV0,
    SolanaNativeRequestError, check_solana_native_replay,
};
use sqlx::{Pool, Postgres, Row};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct DbSolanaNativeReplayStore {
    db_pool: Pool<Postgres>,
}

pub trait SolanaNativeReplayStore {
    fn reserve_accepted_request(
        &self,
        accepted: &SolanaNativeAcceptedRequestV0,
    ) -> impl std::future::Future<
        Output = Result<Option<SolanaNativeReplayAction>, SolanaNativeReplayStoreError>,
    > + Send;
}

#[derive(Debug, Error)]
pub enum SolanaNativeReplayStoreError {
    #[error("native Solana replay key was already used for a different request")]
    ReplayDetected,
    #[error("stored native Solana replay request hash is malformed")]
    MalformedStoredRequestHash,
    #[error("unexpected native Solana replay check error: {0}")]
    UnexpectedReplayCheckError(String),
    #[error("database error while reserving native Solana replay key: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SolanaNativeReplayKeyColumns {
    host_chain_id: [u8; 8],
    solana_cluster_id: [u8; 32],
    kms_context_id: [u8; 32],
    request_signer_pubkey: [u8; 32],
    nonce: [u8; 32],
}

impl DbSolanaNativeReplayStore {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    pub async fn reserve_accepted_request(
        &self,
        accepted: &SolanaNativeAcceptedRequestV0,
    ) -> Result<Option<SolanaNativeReplayAction>, SolanaNativeReplayStoreError> {
        let Some(replay_key) = accepted.replay_key.as_ref() else {
            return Ok(None);
        };
        self.reserve(replay_key, accepted.request_hash)
            .await
            .map(Some)
    }

    pub async fn reserve(
        &self,
        replay_key: &SolanaNativeReplayKeyV0,
        request_hash: [u8; 32],
    ) -> Result<SolanaNativeReplayAction, SolanaNativeReplayStoreError> {
        let key = SolanaNativeReplayKeyColumns::from(replay_key);
        let inserted = sqlx::query(
            "\
            INSERT INTO solana_native_decryption_replay_v0 (
                host_chain_id,
                solana_cluster_id,
                kms_context_id,
                request_signer_pubkey,
                nonce,
                request_hash,
                created_at,
                last_seen_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            ON CONFLICT DO NOTHING\
            ",
        )
        .bind(key.host_chain_id.as_slice())
        .bind(key.solana_cluster_id.as_slice())
        .bind(key.kms_context_id.as_slice())
        .bind(key.request_signer_pubkey.as_slice())
        .bind(key.nonce.as_slice())
        .bind(request_hash.as_slice())
        .execute(&self.db_pool)
        .await?;

        if inserted.rows_affected() == 1 {
            return Ok(SolanaNativeReplayAction::Reserve);
        }

        let row = sqlx::query(
            "\
            SELECT request_hash
            FROM solana_native_decryption_replay_v0
            WHERE host_chain_id = $1
              AND solana_cluster_id = $2
              AND kms_context_id = $3
              AND request_signer_pubkey = $4
              AND nonce = $5\
            ",
        )
        .bind(key.host_chain_id.as_slice())
        .bind(key.solana_cluster_id.as_slice())
        .bind(key.kms_context_id.as_slice())
        .bind(key.request_signer_pubkey.as_slice())
        .bind(key.nonce.as_slice())
        .fetch_one(&self.db_pool)
        .await?;
        let existing = bytes32(row.try_get::<Vec<u8>, _>("request_hash")?)?;
        let action = check_solana_native_replay(Some(existing), request_hash)?;

        if action == SolanaNativeReplayAction::ReuseExisting {
            sqlx::query(
                "\
                UPDATE solana_native_decryption_replay_v0
                SET last_seen_at = NOW()
                WHERE host_chain_id = $1
                  AND solana_cluster_id = $2
                  AND kms_context_id = $3
                  AND request_signer_pubkey = $4
                  AND nonce = $5
                  AND request_hash = $6\
                ",
            )
            .bind(key.host_chain_id.as_slice())
            .bind(key.solana_cluster_id.as_slice())
            .bind(key.kms_context_id.as_slice())
            .bind(key.request_signer_pubkey.as_slice())
            .bind(key.nonce.as_slice())
            .bind(request_hash.as_slice())
            .execute(&self.db_pool)
            .await?;
        }

        Ok(action)
    }
}

impl SolanaNativeReplayStore for DbSolanaNativeReplayStore {
    #[allow(clippy::manual_async_fn)]
    fn reserve_accepted_request(
        &self,
        accepted: &SolanaNativeAcceptedRequestV0,
    ) -> impl std::future::Future<
        Output = Result<Option<SolanaNativeReplayAction>, SolanaNativeReplayStoreError>,
    > + Send {
        async move { DbSolanaNativeReplayStore::reserve_accepted_request(self, accepted).await }
    }
}

impl From<&SolanaNativeReplayKeyV0> for SolanaNativeReplayKeyColumns {
    fn from(value: &SolanaNativeReplayKeyV0) -> Self {
        Self {
            host_chain_id: value.host_chain_id.to_le_bytes(),
            solana_cluster_id: value.solana_cluster_id,
            kms_context_id: value.kms_context_id,
            request_signer_pubkey: value.request_signer_pubkey,
            nonce: value.nonce,
        }
    }
}

impl From<SolanaNativeRequestError> for SolanaNativeReplayStoreError {
    fn from(value: SolanaNativeRequestError) -> Self {
        match value {
            SolanaNativeRequestError::ReplayDetected => Self::ReplayDetected,
            _ => Self::UnexpectedReplayCheckError(value.to_string()),
        }
    }
}

fn bytes32(value: Vec<u8>) -> Result<[u8; 32], SolanaNativeReplayStoreError> {
    value
        .try_into()
        .map_err(|_| SolanaNativeReplayStoreError::MalformedStoredRequestHash)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn replay_key() -> SolanaNativeReplayKeyV0 {
        SolanaNativeReplayKeyV0 {
            host_chain_id: 0x0102_0304_0506_0708,
            solana_cluster_id: [1; 32],
            kms_context_id: [2; 32],
            request_signer_pubkey: [3; 32],
            nonce: [4; 32],
        }
    }

    #[test]
    fn replay_key_columns_use_fixed_width_binary_fields() {
        let columns = SolanaNativeReplayKeyColumns::from(&replay_key());

        assert_eq!(
            columns.host_chain_id,
            0x0102_0304_0506_0708_u64.to_le_bytes()
        );
        assert_eq!(columns.solana_cluster_id, [1; 32]);
        assert_eq!(columns.kms_context_id, [2; 32]);
        assert_eq!(columns.request_signer_pubkey, [3; 32]);
        assert_eq!(columns.nonce, [4; 32]);
    }

    #[test]
    fn bytes32_rejects_malformed_database_values() {
        assert_eq!(bytes32(vec![7; 32]).unwrap(), [7; 32]);
        assert!(matches!(
            bytes32(vec![7; 31]),
            Err(SolanaNativeReplayStoreError::MalformedStoredRequestHash)
        ));
    }
}
