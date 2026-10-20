use std::collections::HashSet;
use std::future::Future;

use fhevm_engine_common::chain_recovery::{
    host_role_tx, lock_history_tx, record_halt, HistoryConflict, ListenerSource,
};

use super::tfhe_event_propagate::{Database, Transaction};
use crate::cmd::block_history::BlockHash;

type Anchor = (i64, Vec<u8>);

/// Earlier disconnected segments only need a new RPC check when first seen.
/// The newest segment is NEVER cached: if it becomes an older segment across
/// a gap, it must be checked again before trusting the new tip. Contiguous
/// extensions instead carry its identity in their parent links.
#[derive(Default)]
pub(crate) struct VerifiedHistory {
    role: Option<bool>,
    tip: Option<Anchor>,
    older_segments: HashSet<Anchor>,
}

async fn segment_tips(
    tx: &mut Transaction<'_>,
    chain_id: i64,
) -> Result<Vec<Anchor>, sqlx::Error> {
    sqlx::query_as(
        "SELECT b.block_number, b.block_hash
         FROM host_chain_blocks_valid b
         WHERE b.chain_id = $1 AND b.block_status = 'finalized'
           AND NOT EXISTS (
             SELECT 1 FROM host_chain_blocks_valid child
             WHERE child.chain_id = b.chain_id
               AND child.block_number = b.block_number + 1
               AND child.block_status = 'finalized'
               AND child.parent_hash = b.block_hash)
         ORDER BY b.block_number DESC, b.block_hash",
    )
    .bind(chain_id)
    .fetch_all(&mut **tx)
    .await
}

/// Check every disconnected committed segment, including empty heights that
/// range catchup never stored. MAX(block_number) alone can hide an older fork.
/// RPC runs outside database transactions; recheck the snapshot and stack role
/// under the history lock before recording a halt or caching verification.
/// Return this pass's RPC observations for reuse by deferred finalization.
pub async fn verify_finalized_history<GetHash, HashFuture>(
    db: &Database,
    mut get_hash: GetHash,
) -> anyhow::Result<Vec<(u64, BlockHash)>>
where
    GetHash: FnMut(u64) -> HashFuture,
    HashFuture: Future<Output = anyhow::Result<BlockHash>>,
{
    let mut verified = db.verified_history.lock().await;
    let Some(mut tx) = db.new_transaction().await? else {
        return Ok(vec![]);
    };
    let role = host_role_tx(&mut tx).await?;
    let chain_id = db.chain_id.as_i64();
    let tips = segment_tips(&mut tx, chain_id).await?;
    let previous_tip_retained = if let Some((height, hash)) = &verified.tip {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM host_chain_blocks_valid
             WHERE chain_id = $1 AND block_number = $2 AND block_hash = $3
               AND block_status = 'finalized')",
        )
        .bind(chain_id)
        .bind(height)
        .bind(hash)
        .fetch_one(&mut *tx)
        .await?
    } else {
        false
    };
    tx.rollback().await?;
    if verified.role != Some(role) || !previous_tip_retained {
        *verified = VerifiedHistory::default();
    }

    let newest_height = tips.first().map(|(height, _)| *height);
    let mut conflict = None;
    let mut observations = vec![];
    for (height, expected) in &tips {
        if Some(*height) != newest_height
            && verified
                .older_segments
                .contains(&(*height, expected.clone()))
        {
            continue;
        }
        let observed = get_hash(u64::try_from(*height)?).await?;
        observations.push((*height as u64, observed));
        if observed.as_slice() != expected {
            conflict = Some(HistoryConflict {
                block_number: *height,
                expected_hash: expected.clone(),
                observed_hash: observed.to_vec(),
                reason:
                    "host block does not extend committed canonical history",
            });
            break;
        }
    }

    let Some(mut tx) = db.new_transaction().await? else {
        return Ok(vec![]);
    };
    let current_role = host_role_tx(&mut tx).await?;
    lock_history_tx(&mut tx, ListenerSource::Host, current_role, chain_id)
        .await?;
    anyhow::ensure!(
        current_role == role && segment_tips(&mut tx, chain_id).await? == tips,
        "committed history changed during verification; retry before advancing"
    );
    tx.rollback().await?;
    if let Some(conflict) = conflict {
        record_halt(
            &db.pool().await,
            conflict.observation(ListenerSource::Host, role, chain_id),
        )
        .await?;
        return Err(sqlx::Error::Protocol(conflict.reason.into()).into());
    }
    verified.role = Some(role);
    verified.tip = tips.first().cloned();
    verified.older_segments = tips
        .into_iter()
        .filter(|(height, _)| Some(*height) != newest_height)
        .collect();
    Ok(observations)
}
