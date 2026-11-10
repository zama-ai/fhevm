use std::collections::HashSet;
use std::future::Future;

use fhevm_engine_common::chain_recovery::{
    host_role_tx, lock_history_tx, record_halt, HistoryConflict, ListenerSource,
};

use super::tfhe_event_propagate::{Database, Transaction};
use crate::cmd::block_history::BlockHash;

type Anchor = (i64, Vec<u8>);

/// Upgrade-time inspection is deliberately bounded. Older disconnected history
/// is trusted until replay touches it; this is not a whole-database audit.
const INITIAL_AUDIT_BLOCKS: i64 = 1_000;

// Checkpoint a bounded amount of RPC work. A late RPC failure or concurrent
// admission must not discard an entire cold scan of genuinely sparse history.
const VERIFICATION_BATCH_SIZE: usize = 64;

/// Remember the bounded startup audit and partial progress across its retries.
/// The current tip is always checked again. After startup, finalized admissions
/// preserve continuity instead of relying on repeated segment discovery.
#[derive(Default)]
pub(crate) struct VerifiedHistory {
    role: Option<bool>,
    tip: Option<Anchor>,
    older_segments: HashSet<Anchor>,
    audited: bool,
    relation: Option<i64>,
}

pub(crate) async fn finalized_tip(
    tx: &mut Transaction<'_>,
    chain_id: i64,
) -> Result<Option<Anchor>, sqlx::Error> {
    sqlx::query_as(
        "SELECT block_number, block_hash FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_status = 'finalized'
         ORDER BY block_number DESC LIMIT 1",
    )
    .bind(chain_id)
    .fetch_optional(&mut **tx)
    .await
}

/// Called under the admission lock, after checking adjacent identities for
/// conflicts. Return the next block to recover, never an entire unbounded gap.
/// Existing finalized rows are the accepted baseline; replay may repair their
/// metadata without traversing history outside the startup audit.
pub(crate) async fn missing_finalized_link(
    tx: &mut Transaction<'_>,
    chain_id: i64,
    height: i64,
    hash: &[u8],
    parent: Option<&[u8]>,
) -> Result<Option<u64>, sqlx::Error> {
    let nearby: Vec<(i64, Vec<u8>, Option<Vec<u8>>)> = sqlx::query_as(
        "SELECT block_number, block_hash, parent_hash
         FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_number BETWEEN $2 - 1 AND $2 + 1
           AND block_status = 'finalized'",
    )
    .bind(chain_id)
    .bind(height)
    .fetch_all(&mut **tx)
    .await?;
    if nearby.iter().any(|(n, h, _)| *n == height && h == hash) {
        return Ok(None);
    }
    // A pending legacy row needs a real header before becoming an ancestry
    // anchor. The deferred finalizer otherwise only knows its hash.
    let Some(parent) = parent.filter(|p| !p.is_empty()) else {
        return Ok(Some(height as u64));
    };
    if nearby
        .iter()
        .any(|(n, h, _)| *n == height - 1 && h == parent)
        || nearby
            .iter()
            .any(|(n, _, p)| *n == height + 1 && p.as_deref() == Some(hash))
    {
        return Ok(None);
    }
    let before: Option<i64> = sqlx::query_scalar(
        "SELECT block_number FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_number < $2 AND block_status = 'finalized'
         ORDER BY block_number DESC LIMIT 1",
    )
    .bind(chain_id)
    .bind(height)
    .fetch_optional(&mut **tx)
    .await?;
    // Older replay attaches backwards. An adjacent successor whose parent is
    // unknown must first be replayed itself to recover the missing linkage.
    let after: Option<i64> = sqlx::query_scalar(
        "SELECT block_number FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_number > $2 AND block_status = 'finalized'
         ORDER BY block_number ASC LIMIT 1",
    )
    .bind(chain_id)
    .bind(height)
    .fetch_optional(&mut **tx)
    .await?;
    // Prefer the closer anchor for a replay inside an old gap. Otherwise a
    // block just below the tip could trigger recovery of years of pruned
    // history merely because an ancient referenced header remains below it.
    if let Some(before) = before {
        if after.is_none_or(|after| height - before <= after - height) {
            return Ok(Some((before + 1) as u64));
        }
    }
    // Only an empty chain permits a new independent anchor.
    Ok(after.map(|after| {
        if after == height + 1 {
            after as u64
        } else {
            (after - 1) as u64
        }
    }))
}

async fn segment_tips(
    tx: &mut Transaction<'_>,
    chain_id: i64,
    tip: i64,
) -> Result<Vec<Anchor>, sqlx::Error> {
    // v0.13 did not persist parents; the later nullable migration had no
    // backfill. As in admission, an absent parent is unknown linkage, not a
    // contradiction. Do not split every adjacent legacy row into a segment.
    // Missing heights and recorded parent mismatches still form boundaries.
    sqlx::query_as(
        "SELECT b.block_number, b.block_hash
         FROM host_chain_blocks_valid b
         WHERE b.chain_id = $1 AND b.block_status = 'finalized'
           AND b.block_number BETWEEN $2 AND $3
           AND NOT EXISTS (
             SELECT 1 FROM host_chain_blocks_valid child
             WHERE child.chain_id = b.chain_id
               AND child.block_number BETWEEN $2 + 1 AND $3
               AND child.block_number = b.block_number + 1
               AND child.block_status = 'finalized'
               AND (child.parent_hash = b.block_hash
                    OR child.parent_hash IS NULL
                    OR octet_length(child.parent_hash) = 0))
         ORDER BY b.block_number DESC, b.block_hash",
    )
    .bind(chain_id)
    .bind(tip.saturating_sub(INITIAL_AUDIT_BLOCKS - 1))
    .bind(tip)
    .fetch_all(&mut **tx)
    .await
}

async fn anchor_is_finalized(
    tx: &mut Transaction<'_>,
    chain_id: i64,
    height: i64,
    hash: &[u8],
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_number = $2 AND block_hash = $3
           AND block_status = 'finalized')",
    )
    .bind(chain_id)
    .bind(height)
    .bind(hash)
    .fetch_one(&mut **tx)
    .await
}

/// Retain successful work even if a later RPC fails or another replica extends
/// the tip. Check only the bounded batch's identities under the admission lock;
/// rescanning the entire history after each batch would make cold scans quadratic.
/// The snapshot's newest tip is deliberately excluded from the reusable cache.
async fn cache_verified_batch(
    db: &Database,
    verified: &mut VerifiedHistory,
    role: bool,
    tip: &Anchor,
    checked: &[Anchor],
) -> anyhow::Result<bool> {
    let Some(mut tx) = db.new_transaction().await? else {
        return Ok(false);
    };
    let current_role = host_role_tx(&mut tx).await?;
    lock_history_tx(
        &mut tx,
        ListenerSource::Host,
        current_role,
        db.chain_id.as_i64(),
    )
    .await?;
    let hashes: Vec<_> = checked
        .iter()
        .chain(std::iter::once(tip))
        .map(|(_, hash)| hash.clone())
        .collect();
    let retained: HashSet<Anchor> = sqlx::query_as(
        "SELECT block_number, block_hash FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_hash = ANY($2)
           AND block_status = 'finalized'",
    )
    .bind(db.chain_id.as_i64())
    .bind(hashes)
    .fetch_all(&mut *tx)
    .await?
    .into_iter()
    .collect();
    tx.rollback().await?;
    if current_role != role || !retained.contains(tip) {
        *verified = VerifiedHistory::default();
        anyhow::bail!("committed history changed during verification; retry before advancing");
    }
    verified.role = Some(role);
    verified.tip = Some(tip.clone());
    verified.older_segments.extend(
        checked
            .iter()
            .filter(|anchor| anchor.0 != tip.0 && retained.contains(*anchor))
            .cloned(),
    );
    Ok(true)
}

/// Audit the most recent 1,000 committed heights at startup, then check only
/// the current tip. Losing the baseline repeats this bounded audit.
/// Every finalized admission must enforce
/// continuity under the history lock, including replay and deferred finalization.
/// That invariant, rather than repeated discovery scans, protects later history.
/// RPC runs outside database transactions; recheck the stack role and either
/// the conflicting anchor or the successful snapshot under the history lock.
/// Successful batches survive retries in memory; process restart rechecks them.
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
    let relation: i64 = sqlx::query_scalar(
        "SELECT 'host_chain_blocks_valid'::regclass::oid::bigint",
    )
    .fetch_one(&mut *tx)
    .await?;
    let newest = finalized_tip(&mut tx, chain_id).await?;
    let previous_tip_retained = if let Some((height, hash)) = &verified.tip {
        anchor_is_finalized(&mut tx, chain_id, *height, hash).await?
    } else {
        verified.audited
    };
    if verified.role != Some(role)
        || verified.relation != Some(relation)
        || !previous_tip_retained
    {
        *verified = VerifiedHistory::default();
    }
    verified.relation = Some(relation);
    let auditing = !verified.audited;
    let tips = match &newest {
        Some((height, _)) if auditing => {
            segment_tips(&mut tx, chain_id, *height).await?
        }
        _ => newest.iter().cloned().collect(),
    };
    tx.rollback().await?;
    let tip_set: HashSet<_> = tips.iter().collect();
    verified
        .older_segments
        .retain(|anchor| tip_set.contains(anchor));

    let newest_height = tips.first().map(|(height, _)| *height);
    let mut conflict = None;
    let mut rpc_error = None;
    let mut observations = vec![];
    let mut checked = Vec::with_capacity(VERIFICATION_BATCH_SIZE);
    for (height, expected) in &tips {
        if Some(*height) != newest_height
            && verified
                .older_segments
                .contains(&(*height, expected.clone()))
        {
            continue;
        }
        let observed = match get_hash(u64::try_from(*height)?).await {
            Ok(hash) => hash,
            Err(error) => {
                rpc_error = Some(error);
                break;
            }
        };
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
        checked.push((*height, expected.clone()));
        if auditing && checked.len() == VERIFICATION_BATCH_SIZE {
            if !cache_verified_batch(
                db,
                &mut verified,
                role,
                &tips[0],
                &checked,
            )
            .await?
            {
                return Ok(vec![]);
            }
            checked.clear();
        }
    }
    // A negative observation is validated independently below. Checkpointing
    // the positive prefix here could discard that observation merely because
    // the snapshot's newest tip was removed while the RPC was in flight.
    if auditing
        && conflict.is_none()
        && !checked.is_empty()
        && !cache_verified_batch(db, &mut verified, role, &tips[0], &checked)
            .await?
    {
        return Ok(vec![]);
    }
    if let Some(error) = rpc_error {
        return Err(error);
    }

    let Some(mut tx) = db.new_transaction().await? else {
        return Ok(vec![]);
    };
    let current_role = host_role_tx(&mut tx).await?;
    lock_history_tx(&mut tx, ListenerSource::Host, current_role, chain_id)
        .await?;
    anyhow::ensure!(
        current_role == role
            && sqlx::query_scalar::<_, i64>(
                "SELECT 'host_chain_blocks_valid'::regclass::oid::bigint"
            )
            .fetch_one(&mut *tx)
            .await?
                == relation,
        "committed history changed during verification; retry before advancing"
    );
    if let Some(conflict) = &conflict {
        // A concurrent extension can change the segment tips, even absorb the
        // conflicting tip into a longer segment. It does not invalidate a
        // mismatch against the same retained finalized block. Only removal of
        // that anchor (or a role change above) makes the observation obsolete.
        anyhow::ensure!(
            anchor_is_finalized(
                &mut tx,
                chain_id,
                conflict.block_number,
                &conflict.expected_hash,
            )
            .await?,
            "committed history changed during verification; retry before advancing"
        );
    } else {
        anyhow::ensure!(
            finalized_tip(&mut tx, chain_id).await? == newest,
            "committed history changed during verification; retry before advancing"
        );
        if auditing {
            if let Some((height, _)) = &newest {
                anyhow::ensure!(
                    segment_tips(&mut tx, chain_id, *height).await? == tips,
                    "committed history changed during verification; retry before advancing"
                );
            }
        }
    }
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
    verified.audited = true;
    verified.tip = tips.first().cloned();
    verified.older_segments = tips
        .into_iter()
        .filter(|(height, _)| Some(*height) != newest_height)
        .collect();
    Ok(observations)
}
