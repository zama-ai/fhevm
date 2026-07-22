//! Atomic PostgreSQL proof store.
//!
//! One SQL transaction per [`CompletedBlock`]: lock progress, exact-replay
//! no-op or conflict halt, parent checks, deterministic reduce, stage, write,
//! single commit. No leases, dual backends, or second pool.

use std::collections::BTreeMap;

use solana_proof_source::{BlockCheckpoint, CompletedBlock};
use sqlx::{PgPool, Postgres, Transaction};
use zama_solana_acl::mmr::mmr_peaks_from_leaves;

use crate::decode::decode_program_instructions;
use crate::reduce::{reduce_completed_block, PriorLineageState, StagedBlockReduction};
use crate::replay::LineageReplayState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityStatus {
    pub history_complete: bool,
    pub history_start: Option<BlockCheckpoint>,
    pub checkpoint: Option<BlockCheckpoint>,
    pub integrity_halted: bool,
    pub integrity_halt_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofSnapshot {
    pub lineage: [u8; 32],
    pub current_handle: Option<[u8; 32]>,
    pub subjects: Vec<[u8; 32]>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
    pub leaves: Vec<[u8; 32]>,
    pub last_slot: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyOutcome {
    Applied,
    AlreadyApplied,
    /// Contiguous parent link is missing (typical after a program-filtered
    /// Yellowstone gap). No domain writes; bounded RPC recovery must fill
    /// intermediate blocks before ingest can continue.
    ///
    /// `gap_end_slot` is the observed block slot that could not be applied
    /// (`UntilExclusive` fill target).
    RecoveryRequired {
        reason: String,
        gap_end_slot: u64,
    },
    IntegrityHalted {
        reason: String,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum StoreError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migrate error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    /// Persisted lineage metadata disagrees with leaf rows (torn / corrupt snapshot).
    #[error(
        "snapshot inconsistent for lineage (leaf_count {leaf_count} vs {leaf_rows} leaf rows)"
    )]
    SnapshotInconsistent { leaf_count: u64, leaf_rows: u64 },
}

#[derive(Clone)]
pub struct SqlProofStore {
    pool: PgPool,
    program_id: [u8; 32],
}

fn as_i64(value: u64) -> Result<i64, StoreError> {
    i64::try_from(value).map_err(|_| {
        StoreError::Database(sqlx::Error::Protocol(format!(
            "u64 value {value} does not fit in BIGINT"
        )))
    })
}

fn bytes32(value: &[u8]) -> Option<[u8; 32]> {
    value.try_into().ok()
}

fn bytes64(value: &[u8]) -> Option<[u8; 64]> {
    value.try_into().ok()
}

fn peaks_to_sql(peaks: &[[u8; 32]]) -> Vec<Vec<u8>> {
    peaks.iter().map(|peak| peak.to_vec()).collect()
}

fn peaks_from_sql(peaks: Vec<Vec<u8>>) -> Result<Vec<[u8; 32]>, StoreError> {
    peaks
        .into_iter()
        .map(|peak| {
            bytes32(&peak).ok_or_else(|| {
                StoreError::Database(sqlx::Error::Protocol(
                    "invalid peak length in solana_proof_lineages".into(),
                ))
            })
        })
        .collect()
}

fn subjects_to_sql(subjects: &[[u8; 32]]) -> Vec<Vec<u8>> {
    subjects.iter().map(|subject| subject.to_vec()).collect()
}

fn subjects_from_sql(subjects: Vec<Vec<u8>>) -> Result<Vec<[u8; 32]>, StoreError> {
    subjects
        .into_iter()
        .map(|subject| {
            bytes32(&subject).ok_or_else(|| {
                StoreError::Database(sqlx::Error::Protocol(
                    "invalid subject length in solana_proof_lineages".into(),
                ))
            })
        })
        .collect()
}

/// Postgres unique_violation — treated as a deterministic integrity signal
/// (e.g. signature reuse across slots), not a bare retryable database error.
fn is_unique_violation(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db) => db.code().as_deref() == Some("23505"),
        _ => false,
    }
}

impl SqlProofStore {
    pub fn new(pool: PgPool, program_id: [u8; 32]) -> Self {
        Self { pool, program_id }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn connect(database_url: &str, program_id: [u8; 32]) -> Result<Self, StoreError> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self::new(pool, program_id))
    }

    pub async fn migrate(&self) -> Result<(), StoreError> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn checkpoint(&self) -> Result<Option<BlockCheckpoint>, StoreError> {
        let row = sqlx::query!(
            r#"
            SELECT checkpoint_slot, checkpoint_block_hash
            FROM solana_proof_progress
            WHERE singleton = 1
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        match (row.checkpoint_slot, row.checkpoint_block_hash) {
            (Some(slot), Some(hash)) => {
                let block_hash = bytes32(&hash).ok_or_else(|| {
                    StoreError::Database(sqlx::Error::Protocol(
                        "invalid checkpoint_block_hash length".into(),
                    ))
                })?;
                Ok(Some(BlockCheckpoint {
                    slot: slot as u64,
                    block_hash,
                }))
            }
            (None, None) => Ok(None),
            _ => Err(StoreError::Database(sqlx::Error::Protocol(
                "partial checkpoint in solana_proof_progress".into(),
            ))),
        }
    }

    pub async fn integrity_status(&self) -> Result<IntegrityStatus, StoreError> {
        let row = sqlx::query!(
            r#"
            SELECT
                history_complete,
                history_start_slot,
                history_start_block_hash,
                checkpoint_slot,
                checkpoint_block_hash,
                integrity_halted,
                integrity_halt_reason
            FROM solana_proof_progress
            WHERE singleton = 1
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        let history_start = match (row.history_start_slot, row.history_start_block_hash) {
            (Some(slot), Some(hash)) => Some(BlockCheckpoint {
                slot: slot as u64,
                block_hash: bytes32(&hash).ok_or_else(|| {
                    StoreError::Database(sqlx::Error::Protocol(
                        "invalid history_start_block_hash length".into(),
                    ))
                })?,
            }),
            (None, None) => None,
            _ => {
                return Err(StoreError::Database(sqlx::Error::Protocol(
                    "partial history_start in solana_proof_progress".into(),
                )))
            }
        };
        let checkpoint = match (row.checkpoint_slot, row.checkpoint_block_hash) {
            (Some(slot), Some(hash)) => Some(BlockCheckpoint {
                slot: slot as u64,
                block_hash: bytes32(&hash).ok_or_else(|| {
                    StoreError::Database(sqlx::Error::Protocol(
                        "invalid checkpoint_block_hash length".into(),
                    ))
                })?,
            }),
            (None, None) => None,
            _ => {
                return Err(StoreError::Database(sqlx::Error::Protocol(
                    "partial checkpoint in solana_proof_progress".into(),
                )))
            }
        };

        Ok(IntegrityStatus {
            history_complete: row.history_complete,
            history_start,
            checkpoint,
            integrity_halted: row.integrity_halted,
            integrity_halt_reason: row.integrity_halt_reason,
        })
    }

    /// Bootstrap A / recovery seam: completeness becomes true only after an
    /// explicit bounded recovery pass proves continuity from the configured
    /// start (`bootstrap_slot`). Called by the sequential runner when justified.
    pub async fn set_history_complete_after_recovery(
        &self,
        complete: bool,
    ) -> Result<(), StoreError> {
        sqlx::query!(
            r#"
            UPDATE solana_proof_progress
            SET history_complete = $1, updated_at = NOW()
            WHERE singleton = 1
              AND integrity_halted = FALSE
            "#,
            complete
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn proof_snapshot(
        &self,
        lineage: [u8; 32],
    ) -> Result<Option<ProofSnapshot>, StoreError> {
        // REPEATABLE READ so lineage metadata and leaf rows are one snapshot;
        // under READ COMMITTED a concurrent apply can tear leaf_count vs leaves.
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
            .execute(&mut *tx)
            .await?;
        let row = sqlx::query!(
            r#"
            SELECT
                current_handle,
                subjects,
                leaf_count,
                peaks,
                last_slot
            FROM solana_proof_lineages
            WHERE lineage = $1
            "#,
            lineage.as_slice()
        )
        .fetch_optional(&mut *tx)
        .await?;

        let Some(row) = row else {
            tx.commit().await?;
            return Ok(None);
        };

        let leaves = sqlx::query!(
            r#"
            SELECT commitment
            FROM solana_proof_leaves
            WHERE lineage = $1
            ORDER BY leaf_index ASC
            "#,
            lineage.as_slice()
        )
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;

        let leaf_commitments = leaves
            .into_iter()
            .map(|leaf| {
                bytes32(&leaf.commitment).ok_or_else(|| {
                    StoreError::Database(sqlx::Error::Protocol(
                        "invalid leaf commitment length".into(),
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let peaks = peaks_from_sql(row.peaks)?;
        let subjects = subjects_from_sql(row.subjects)?;
        let current_handle = match row.current_handle {
            Some(handle) => Some(bytes32(&handle).ok_or_else(|| {
                StoreError::Database(sqlx::Error::Protocol(
                    "invalid current_handle length".into(),
                ))
            })?),
            None => None,
        };

        let leaf_count = row.leaf_count as u64;
        let leaf_rows = leaf_commitments.len() as u64;
        if leaf_count != leaf_rows {
            return Err(StoreError::SnapshotInconsistent {
                leaf_count,
                leaf_rows,
            });
        }

        Ok(Some(ProofSnapshot {
            lineage,
            current_handle,
            subjects,
            leaf_count,
            peaks,
            leaves: leaf_commitments,
            last_slot: row.last_slot as u64,
        }))
    }

    pub async fn apply_completed_block(
        &self,
        block: &CompletedBlock,
    ) -> Result<ApplyOutcome, StoreError> {
        let mut tx = self.pool.begin().await?;

        let progress = sqlx::query!(
            r#"
            SELECT
                checkpoint_slot,
                checkpoint_block_hash,
                integrity_halted,
                integrity_halt_reason
            FROM solana_proof_progress
            WHERE singleton = 1
            FOR UPDATE
            "#
        )
        .fetch_one(&mut *tx)
        .await?;

        if progress.integrity_halted {
            let reason = progress
                .integrity_halt_reason
                .unwrap_or_else(|| "integrity halted".to_owned());
            tx.commit().await?;
            return Ok(ApplyOutcome::IntegrityHalted { reason });
        }

        match self.exact_or_conflicting_replay(&mut tx, block).await? {
            Some(ApplyOutcome::AlreadyApplied) => {
                tx.commit().await?;
                return Ok(ApplyOutcome::AlreadyApplied);
            }
            Some(ApplyOutcome::IntegrityHalted { reason }) => {
                self.persist_halt(&mut tx, &reason).await?;
                tx.commit().await?;
                return Ok(ApplyOutcome::IntegrityHalted { reason });
            }
            Some(ApplyOutcome::RecoveryRequired { .. }) => {
                unreachable!("replay probe never returns RecoveryRequired")
            }
            Some(ApplyOutcome::Applied) => unreachable!("replay probe never returns Applied"),
            None => {}
        }

        if let (Some(checkpoint_slot), Some(checkpoint_hash)) = (
            progress.checkpoint_slot,
            progress.checkpoint_block_hash.as_ref(),
        ) {
            let checkpoint_hash = bytes32(checkpoint_hash).ok_or_else(|| {
                StoreError::Database(sqlx::Error::Protocol(
                    "invalid checkpoint_block_hash length".into(),
                ))
            })?;
            let checkpoint_slot = checkpoint_slot as u64;
            if block.slot <= checkpoint_slot {
                // Exact inclusive replay is handled above. Any other slot at or
                // behind the checkpoint is an integrity failure.
                let reason = format!(
                    "unstored or conflicting slot {} at or behind checkpoint {}",
                    block.slot, checkpoint_slot
                );
                self.persist_halt(&mut tx, &reason).await?;
                tx.commit().await?;
                return Ok(ApplyOutcome::IntegrityHalted { reason });
            }
            if block.parent_slot != checkpoint_slot {
                // Program-filtered Yellowstone streams omit empty blocks, so
                // consecutive program-touching blocks may skip slots. Contiguous
                // parent links are still required; a gap is not a silent skip —
                // bounded RPC recovery must supply the missing blocks.
                let reason = format!(
                    "contiguous ingest gap at slot {}: parent slot {} does not extend checkpoint {}; recovery required",
                    block.slot, block.parent_slot, checkpoint_slot
                );
                tx.commit().await?;
                return Ok(ApplyOutcome::RecoveryRequired {
                    reason,
                    gap_end_slot: block.slot,
                });
            }
            if block.parent_hash != checkpoint_hash {
                let reason = format!(
                    "block {} ancestry does not extend checkpoint {}: parent hash {:02x?} != checkpoint {:02x?}",
                    block.slot,
                    checkpoint_slot,
                    &block.parent_hash[..4],
                    &checkpoint_hash[..4]
                );
                self.persist_halt(&mut tx, &reason).await?;
                tx.commit().await?;
                return Ok(ApplyOutcome::IntegrityHalted { reason });
            }
        }

        // Stage reduction before any domain writes.
        let existing = self.load_prior_lineages(&mut tx, block).await?;

        let staged = match reduce_completed_block(self.program_id, block, &existing) {
            Ok(staged) => staged,
            Err(err) => {
                let reason = err.to_string();
                self.persist_halt(&mut tx, &reason).await?;
                tx.commit().await?;
                return Ok(ApplyOutcome::IntegrityHalted { reason });
            }
        };

        if let Err(reason) = self.validate_staged_mmr(&staged) {
            self.persist_halt(&mut tx, &reason).await?;
            tx.commit().await?;
            return Ok(ApplyOutcome::IntegrityHalted { reason });
        }

        if let Err(err) = self.insert_block(&mut tx, block).await {
            return self.finish_unique_violation(tx, block.slot, err).await;
        }
        if let Err(err) = self.insert_transactions(&mut tx, block).await {
            return self.finish_unique_violation(tx, block.slot, err).await;
        }
        if let Err(err) = self.apply_staged(&mut tx, block.slot, &staged).await {
            return self.finish_unique_violation(tx, block.slot, err).await;
        }

        let is_bootstrap = progress.checkpoint_slot.is_none();
        sqlx::query!(
            r#"
            UPDATE solana_proof_progress
            SET
                checkpoint_slot = $1,
                checkpoint_block_hash = $2,
                history_start_slot = CASE
                    WHEN $3 THEN $1
                    ELSE history_start_slot
                END,
                history_start_block_hash = CASE
                    WHEN $3 THEN $2
                    ELSE history_start_block_hash
                END,
                history_complete = CASE
                    WHEN $3 THEN FALSE
                    ELSE history_complete
                END,
                updated_at = NOW()
            WHERE singleton = 1
            "#,
            as_i64(block.slot)?,
            block.block_hash.as_slice(),
            is_bootstrap
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(ApplyOutcome::Applied)
    }

    async fn finish_unique_violation(
        &self,
        tx: Transaction<'_, Postgres>,
        slot: u64,
        err: StoreError,
    ) -> Result<ApplyOutcome, StoreError> {
        match err {
            StoreError::Database(db_err) if is_unique_violation(&db_err) => {
                let reason = format!(
                    "deterministic constraint conflict while applying slot {slot}: {db_err}"
                );
                // The failed statement aborted this transaction; roll it back
                // and persist the halt in a fresh transaction.
                tx.rollback().await?;
                let mut halt_tx = self.pool.begin().await?;
                self.persist_halt(&mut halt_tx, &reason).await?;
                halt_tx.commit().await?;
                Ok(ApplyOutcome::IntegrityHalted { reason })
            }
            other => {
                // Drop the open transaction; connection returns to the pool on drop.
                drop(tx);
                Err(other)
            }
        }
    }

    async fn persist_halt(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        reason: &str,
    ) -> Result<(), StoreError> {
        sqlx::query!(
            r#"
            UPDATE solana_proof_progress
            SET
                integrity_halted = TRUE,
                integrity_halt_reason = $1,
                updated_at = NOW()
            WHERE singleton = 1
            "#,
            reason
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// Returns `Some(AlreadyApplied)` / `Some(Halted)` when the slot is known,
    /// or `None` when the slot is new.
    async fn exact_or_conflicting_replay(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block: &CompletedBlock,
    ) -> Result<Option<ApplyOutcome>, StoreError> {
        let stored = sqlx::query!(
            r#"
            SELECT
                slot,
                block_hash,
                parent_slot,
                parent_hash,
                block_time,
                block_height,
                executed_transaction_count
            FROM solana_proof_blocks
            WHERE slot = $1
            "#,
            as_i64(block.slot)?
        )
        .fetch_optional(&mut **tx)
        .await?;

        let Some(stored) = stored else {
            return Ok(None);
        };

        let stored_hash = bytes32(&stored.block_hash).ok_or_else(|| {
            StoreError::Database(sqlx::Error::Protocol(
                "invalid stored block_hash length".into(),
            ))
        })?;
        let stored_parent_hash = bytes32(&stored.parent_hash).ok_or_else(|| {
            StoreError::Database(sqlx::Error::Protocol(
                "invalid stored parent_hash length".into(),
            ))
        })?;

        let header_matches = stored_hash == block.block_hash
            && stored.parent_slot as u64 == block.parent_slot
            && stored_parent_hash == block.parent_hash
            && stored.block_time == block.block_time
            && stored.block_height.map(|height| height as u64) == block.block_height
            && stored.executed_transaction_count as u64 == block.executed_transaction_count;

        let txs = sqlx::query!(
            r#"
            SELECT transaction_index, signature, succeeded, is_vote
            FROM solana_proof_transactions
            WHERE block_slot = $1
            ORDER BY transaction_index ASC
            "#,
            as_i64(block.slot)?
        )
        .fetch_all(&mut **tx)
        .await?;

        let tx_matches = txs.len() == block.transactions.len()
            && txs
                .iter()
                .zip(block.transactions.iter())
                .all(|(stored_tx, incoming)| {
                    stored_tx.transaction_index as u64 == incoming.index
                        && bytes64(&stored_tx.signature) == Some(incoming.signature)
                        && stored_tx.succeeded == incoming.succeeded
                        && stored_tx.is_vote == incoming.is_vote
                });

        if header_matches && tx_matches {
            return Ok(Some(ApplyOutcome::AlreadyApplied));
        }

        Ok(Some(ApplyOutcome::IntegrityHalted {
            reason: format!("conflicting normalized block replay at slot {}", block.slot),
        }))
    }

    async fn load_prior_lineages(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block: &CompletedBlock,
    ) -> Result<BTreeMap<[u8; 32], PriorLineageState>, StoreError> {
        let mut lineages = BTreeMap::new();
        for tx_info in &block.transactions {
            if tx_info.instructions.is_empty() {
                continue;
            }
            // Decode only to discover lineage keys; reduce re-decodes.
            let decoded = match decode_program_instructions(self.program_id, &tx_info.instructions)
            {
                Ok(decoded) => decoded,
                Err(_) => continue, // reduce will halt on decode failure
            };
            for instruction in decoded {
                lineages.insert(instruction.encrypted_value(), ());
            }
        }

        let mut existing = BTreeMap::new();
        for lineage in lineages.keys() {
            let row = sqlx::query!(
                r#"
                SELECT current_handle, subjects, leaf_count, peaks
                FROM solana_proof_lineages
                WHERE lineage = $1
                "#,
                lineage.as_slice()
            )
            .fetch_optional(&mut **tx)
            .await?;
            let Some(row) = row else {
                continue;
            };
            let current_handle = match row.current_handle {
                Some(handle) => Some(bytes32(&handle).ok_or_else(|| {
                    StoreError::Database(sqlx::Error::Protocol(
                        "invalid current_handle length".into(),
                    ))
                })?),
                None => None,
            };
            existing.insert(
                *lineage,
                PriorLineageState {
                    replay: LineageReplayState {
                        current_handle,
                        subjects: subjects_from_sql(row.subjects)?,
                    },
                    leaf_count: row.leaf_count as u64,
                    peaks: peaks_from_sql(row.peaks)?,
                },
            );
        }
        Ok(existing)
    }

    fn validate_staged_mmr(&self, staged: &StagedBlockReduction) -> Result<(), String> {
        for lineage in &staged.lineages {
            if lineage.peaks.len() != lineage.leaf_count.count_ones() as usize {
                return Err(format!(
                    "inconsistent MMR peaks for lineage {:02x?}",
                    &lineage.lineage[..4]
                ));
            }
            let new_leaves: Vec<[u8; 32]> = staged
                .leaves
                .iter()
                .filter(|leaf| leaf.lineage == lineage.lineage)
                .map(|leaf| leaf.commitment)
                .collect();
            // When this block holds the lineage's full leaf list, peaks must
            // match an independent fold over those leaves.
            if lineage.leaf_count == new_leaves.len() as u64 {
                let recomputed = mmr_peaks_from_leaves(&new_leaves);
                if recomputed != lineage.peaks {
                    return Err(format!(
                        "persisted peaks diverge from independent MMR reconstruction for {:02x?}",
                        &lineage.lineage[..4]
                    ));
                }
            }
        }
        Ok(())
    }

    async fn insert_block(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block: &CompletedBlock,
    ) -> Result<(), StoreError> {
        sqlx::query!(
            r#"
            INSERT INTO solana_proof_blocks (
                slot,
                block_hash,
                parent_slot,
                parent_hash,
                block_time,
                block_height,
                executed_transaction_count
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            as_i64(block.slot)?,
            block.block_hash.as_slice(),
            as_i64(block.parent_slot)?,
            block.parent_hash.as_slice(),
            block.block_time,
            block.block_height.map(as_i64).transpose()?,
            as_i64(block.executed_transaction_count)?
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn insert_transactions(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block: &CompletedBlock,
    ) -> Result<(), StoreError> {
        for incoming in &block.transactions {
            sqlx::query!(
                r#"
                INSERT INTO solana_proof_transactions (
                    block_slot,
                    transaction_index,
                    signature,
                    succeeded,
                    is_vote
                ) VALUES ($1, $2, $3, $4, $5)
                "#,
                as_i64(block.slot)?,
                as_i64(incoming.index)?,
                incoming.signature.as_slice(),
                incoming.succeeded,
                incoming.is_vote
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    async fn apply_staged(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        slot: u64,
        staged: &StagedBlockReduction,
    ) -> Result<(), StoreError> {
        for lineage in &staged.lineages {
            let current_handle = lineage
                .current_handle
                .as_ref()
                .map(|handle| handle.as_slice());
            sqlx::query!(
                r#"
                INSERT INTO solana_proof_lineages (
                    lineage,
                    current_handle,
                    subjects,
                    leaf_count,
                    peaks,
                    last_slot
                ) VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (lineage) DO UPDATE SET
                    current_handle = EXCLUDED.current_handle,
                    subjects = EXCLUDED.subjects,
                    leaf_count = EXCLUDED.leaf_count,
                    peaks = EXCLUDED.peaks,
                    last_slot = EXCLUDED.last_slot
                "#,
                lineage.lineage.as_slice(),
                current_handle,
                &subjects_to_sql(&lineage.subjects) as &[Vec<u8>],
                as_i64(lineage.leaf_count)?,
                &peaks_to_sql(&lineage.peaks) as &[Vec<u8>],
                as_i64(slot)?
            )
            .execute(&mut **tx)
            .await?;
        }

        for leaf in &staged.leaves {
            sqlx::query!(
                r#"
                INSERT INTO solana_proof_leaves (
                    lineage,
                    leaf_index,
                    commitment,
                    block_slot,
                    transaction_index
                ) VALUES ($1, $2, $3, $4, $5)
                "#,
                leaf.lineage.as_slice(),
                as_i64(leaf.leaf_index)?,
                leaf.commitment.as_slice(),
                as_i64(slot)?,
                as_i64(leaf.transaction_index)?
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }
}
