use alloy::primitives::{keccak256, Address};
use anyhow::Result;
use fhevm_engine_common::versioning::StaleStackError;

use crate::database::tfhe_event_propagate::Database;

/// Retirement is a permanent pause, not a retryable checkpoint failure.
#[derive(Debug)]
pub(super) struct ListenerRetired;

impl std::fmt::Display for ListenerRetired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("listener stack retired")
    }
}

impl std::error::Error for ListenerRetired {}

/// A finalized, contiguous catchup frontier. Realtime commits never advance it.
/// The filter and ingestion role are part of its identity: a narrower listener
/// must not certify coverage for a listener consuming additional event types.
pub(super) struct RecoveryCheckpoint {
    db: Database,
    scope_hash: Vec<u8>,
}

#[derive(Clone, Copy, sqlx::FromRow)]
pub(super) struct Checkpoint {
    pub next_block: i64,
    pub revision: i64,
}

impl RecoveryCheckpoint {
    async fn new_transaction(
        &self,
    ) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
        match self.db.new_transaction().await {
            Ok(Some(tx)) => Ok(tx),
            // Cutover after BEGIN, while waiting for the shared cutover lock.
            Ok(None) => Err(ListenerRetired.into()),
            // Already retired at BEGIN: the guard fails before taking the lock.
            Err(sqlx::Error::Configuration(err))
                if err.is::<StaleStackError>() =>
            {
                Err(ListenerRetired.into())
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn new(
        db: Database,
        addresses: [Option<Address>; 5],
        protocol_only: bool,
        gcs: bool,
    ) -> Self {
        // ACL, TFHE, KMS generation, protocol config, confidential bridge.
        // Preserve roles and absent slots: swapping roles must not reuse a
        // checkpoint even when the eth_getLogs address filter is unchanged.
        let mut scope = vec![u8::from(protocol_only), u8::from(gcs)];
        for address in addresses {
            scope.push(u8::from(address.is_some()));
            scope.extend_from_slice(address.unwrap_or_default().as_slice());
        }
        Self {
            db,
            scope_hash: keccak256(scope).to_vec(),
        }
    }

    /// Register the range before accepting any new block. On process restart,
    /// replay the overlap before the saved frontier, even with a newer seed.
    pub async fn start(&self, from: u64, overlap: u64) -> Result<Checkpoint> {
        let mut tx = self.new_transaction().await?;
        let state = sqlx::query_as(
            "INSERT INTO public.host_listener_catchup_state
                 (chain_id, scope_hash, next_block) VALUES ($1, $2, $3)
             ON CONFLICT (chain_id, scope_hash) DO UPDATE
             SET next_block = LEAST(GREATEST(host_listener_catchup_state.next_block - $4, 0), EXCLUDED.next_block),
                 revision = host_listener_catchup_state.revision + 1
             RETURNING next_block, revision",
        )
        .bind(self.db.chain_id.as_i64())
        .bind(&self.scope_hash)
        .bind(i64::try_from(from)?)
        .bind(i64::try_from(overlap)?)
        .fetch_one(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(state)
    }

    pub async fn read(&self) -> Result<Checkpoint> {
        Ok(sqlx::query_as(
            "SELECT next_block, revision FROM public.host_listener_catchup_state
             WHERE chain_id = $1 AND scope_hash = $2",
        )
        .bind(self.db.chain_id.as_i64())
        .bind(&self.scope_hash)
        .fetch_one(&self.db.pool().await)
        .await?)
    }

    /// Persist a discovered gap before discarding queued data or attempting its
    /// ingestion. Increment the revision even at the same frontier, invalidating
    /// any page fetched before this recovery request by another listener.
    pub async fn require(&self, from: u64) -> Result<Checkpoint> {
        let mut tx = self.new_transaction().await?;
        sqlx::query(
            "UPDATE public.host_listener_catchup_state
             SET next_block = LEAST(next_block, $3), revision = revision + 1
             WHERE chain_id = $1 AND scope_hash = $2",
        )
        .bind(self.db.chain_id.as_i64())
        .bind(&self.scope_hash)
        .bind(i64::try_from(from)?)
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        self.read().await
    }

    /// Called only after all block transactions in this page committed (or the
    /// page was empty). A crash before this write merely replays committed work.
    pub async fn complete(
        &self,
        from: u64,
        next: u64,
        revision: i64,
    ) -> Result<Checkpoint> {
        let mut tx = self.new_transaction().await?;
        sqlx::query(
            "UPDATE public.host_listener_catchup_state
             SET next_block = $4, revision = revision + 1
             WHERE chain_id = $1 AND scope_hash = $2
               AND next_block = $3 AND revision = $5",
        )
        .bind(self.db.chain_id.as_i64())
        .bind(&self.scope_hash)
        .bind(i64::try_from(from)?)
        .bind(i64::try_from(next)?)
        .bind(revision)
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        self.read().await
    }
}
