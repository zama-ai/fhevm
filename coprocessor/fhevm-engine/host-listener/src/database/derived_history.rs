//! Replay may repair work until its inventory has entered a commitment.
//!
//! Hash creation and manifest sealing take the same history admission lock as
//! ingestion. Keep it from capture through the final check/commit. We deliberately
//! freeze at creation/sealing, not the uploaded flag: S3 may have accepted a PUT
//! whose database transaction subsequently failed. Peers also cache upgrade
//! commitments, and manifest descendants commit to previously sealed history.
//! Replacing that evidence requires operator recovery, not an in-place rewrite.

use std::collections::BTreeMap;

use sqlx::{Postgres, Transaction};

pub(super) const COMMITTED_INVENTORY_CHANGED: &str =
    "replay changes inventory already committed by a state hash or manifest seal; operator recovery required";

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
struct Inventory {
    block_number: i64,
    // None identifies an upgrade state hash; Some identifies a manifest seal.
    block_hash: Option<Vec<u8>>,
    computations: i64,
    allowed: i64,
    producers: i64,
    nonempty: bool,
}

pub(super) struct CommittedInventory {
    chain_id: i64,
    before: BTreeMap<(i64, Option<Vec<u8>>), Inventory>,
}

impl CommittedInventory {
    /// Include stored producers: an ACL-only delivery can repair an older block.
    pub async fn capture(
        tx: &mut Transaction<'_, Postgres>,
        chain_id: i64,
        block_numbers: &[i64],
        handles: &[Vec<u8>],
    ) -> Result<Self, sqlx::Error> {
        let candidates: Vec<i64> = sqlx::query_scalar(
            "SELECT UNNEST($1::bigint[])
             UNION SELECT block_number FROM computations
             WHERE host_chain_id = $2 AND output_handle = ANY($3::bytea[])
               AND block_number IS NOT NULL",
        )
        .bind(block_numbers)
        .bind(chain_id)
        .bind(handles)
        .fetch_all(tx.as_mut())
        .await?;
        let before = load(tx, chain_id, &candidates)
            .await?
            .into_iter()
            .map(|row| ((row.block_number, row.block_hash.clone()), row))
            .collect();
        Ok(Self { chain_id, before })
    }

    pub async fn changed_block(
        self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<i64>, sqlx::Error> {
        if self.before.is_empty() {
            return Ok(None);
        }
        let numbers: Vec<_> =
            self.before.keys().map(|(number, _)| *number).collect();
        for after in load(tx, self.chain_id, &numbers).await? {
            if self
                .before
                .get(&(after.block_number, after.block_hash.clone()))
                .is_some_and(|before| *before != after)
            {
                return Ok(Some(after.block_number));
            }
        }
        Ok(None)
    }
}

async fn load(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    block_numbers: &[i64],
) -> Result<Vec<Inventory>, sqlx::Error> {
    // Ingestion only adds computation/producer rows and upgrades eligibility.
    // These counts therefore detect every inventory change it can make. Do not
    // include completion timestamps or ciphertexts: workers can legitimately
    // finish already-admitted work concurrently. Manifest inventory is fork-
    // specific. Finalized admission may orphan a sealed sibling and retract its
    // fallback observations. That is legitimate fork retirement, not a repair
    // of committed canonical inventory: omit orphaned roots from both snapshots.
    // Unqualified names intentionally follow the listener's BCS/GCS routing.
    // A recorded fallback grant already admits its producer, even while its
    // computation is deferred until finality. Count that eligibility from the
    // observation, not the eventual computation row: synthesis is completion
    // of known work, including after an uncomputed manifest seal. A new grant
    // or an observation on a different fork must still change this inventory.
    //
    // A non-empty state hash already proves FHE activity: finalized headers can
    // be pruned, then restored by a full or ACL-only replay. Losing/restoring
    // that metadata must not change the captured inventory. The empty sentinel
    // is SHA-256 of zero bytes (consensus-detector's EMPTY_BLOCK_STATE_HASH).
    // Still inspect headers so discovering FHE events in a block committed as
    // empty is detected even when its computations were already present.
    sqlx::query_as(
        "WITH RECURSIVE manifest_coverage AS (
             SELECT b.block_number AS root_number, b.block_hash AS root_hash,
                    b.consensus_epoch, b.block_number, b.block_hash,
                    (b.block_handle_count IS NOT NULL OR b.block_content_digest IS NOT NULL
                     OR b.manifest_published) AS committed
             FROM block_manifest_state b
             WHERE b.host_chain_id = $1 AND b.block_number = ANY($2)
               AND NOT EXISTS (SELECT 1 FROM host_chain_blocks_valid h
                   WHERE h.chain_id = b.host_chain_id AND h.block_hash = b.block_hash
                     AND h.block_status = 'orphaned')
               AND (b.block_handle_count IS NOT NULL OR b.block_content_digest IS NOT NULL
                    OR EXISTS(SELECT 1 FROM block_manifest_state published
                        WHERE published.host_chain_id = $1
                          AND published.consensus_epoch = b.consensus_epoch
                          AND published.block_number >= b.block_number AND published.manifest_published))
             UNION ALL
             SELECT parent.root_number, parent.root_hash, child.consensus_epoch,
                    child.block_number, child.block_hash, child.manifest_published
             FROM manifest_coverage parent JOIN block_manifest_state child
               ON child.host_chain_id = $1 AND child.consensus_epoch = parent.consensus_epoch
              AND child.parent_block_hash = parent.block_hash AND child.block_number > parent.block_number
             WHERE NOT parent.committed
         )
         SELECT sh.block_number, NULL::bytea AS block_hash,
                (SELECT COUNT(*) FROM computations c WHERE c.host_chain_id = $1
                    AND c.block_number = sh.block_number) AS computations,
                (SELECT COUNT(*) FROM computations c WHERE c.host_chain_id = $1
                    AND c.block_number = sh.block_number AND c.is_allowed) AS allowed,
                0::bigint AS producers,
                (sh.state_hash <> encode(sha256(''::bytea), 'hex')
                 OR EXISTS(SELECT 1 FROM host_chain_blocks_valid b WHERE b.chain_id = $1
                    AND b.block_number = sh.block_number AND b.block_status = 'finalized'
                    AND b.fhe_event_count > 0)) AS nonempty
         FROM state_hash sh WHERE sh.chain_id = $1 AND sh.block_number = ANY($2)
         UNION
         SELECT b.block_number, b.block_hash, 0::bigint,
                (SELECT COUNT(*) FROM handle_producer_block p
                 WHERE p.host_chain_id = $1 AND p.producer_block_hash = b.block_hash
                   AND (EXISTS (SELECT 1 FROM computations c
                            WHERE c.output_handle = p.handle AND c.host_chain_id = p.host_chain_id
                              AND c.block_number = p.producer_block_number AND c.is_allowed)
                        OR EXISTS (SELECT 1 FROM fallback_granted_events f
                            WHERE f.dst_chain_id = p.host_chain_id AND f.dst_handle = p.handle
                              AND f.block_number = p.producer_block_number
                              AND f.block_hash = p.producer_block_hash))),
                (SELECT COUNT(*) FROM handle_producer_block p
                 WHERE p.host_chain_id = $1 AND p.producer_block_hash = b.block_hash),
                FALSE
         FROM (SELECT DISTINCT root_number AS block_number, root_hash AS block_hash
               FROM manifest_coverage WHERE committed) b",
    )
    .bind(chain_id)
    .bind(block_numbers)
    .fetch_all(tx.as_mut())
    .await
}
