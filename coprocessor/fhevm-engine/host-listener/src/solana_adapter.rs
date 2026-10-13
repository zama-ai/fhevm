//! Transport-agnostic core of the Solana ingestion path: maps reconstructed
//! Solana compute and material requests into the coprocessor database.
//!
//! Design rationale lives in `solana/docs/DESIGN_DECISIONS.md` (event transport:
//! DD-003; eager ciphertext-material preparation: DD-024).

use std::collections::{HashMap, HashSet};

use crate::database::computation::Computation;
use alloy_primitives::FixedBytes;
use sqlx::Error as SqlxError;

use crate::cmd::block_history::BlockSummary;
use crate::database::dependence_chains::dependence_chains;
use crate::database::ingest::{
    classify_slow_chains, populate_operand_boundary_masks,
};
use crate::database::tfhe_event_propagate::{
    Database, Handle, LogTfhe, Transaction, TransactionId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaMaterialRequest {
    pub handle: Handle,
}

#[derive(Clone, Debug)]
pub enum SolanaHostRecord {
    Computation(Computation),
    MaterialRequest(SolanaMaterialRequest),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaBlockMeta {
    /// The block height, not the slot.
    pub block_number: u64,
    pub block_timestamp: time::PrimitiveDateTime,
    pub block_hash: [u8; 32],
    pub parent_hash: [u8; 32],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SolanaIngestStats {
    pub tfhe_events: usize,
    pub material_requests: usize,
    /// Computations and material requests that inserted at least one new row.
    pub inserted_records: usize,
}

pub(crate) fn material_request(handle: [u8; 32]) -> SolanaMaterialRequest {
    SolanaMaterialRequest {
        handle: Handle::from(handle),
    }
}

fn dedup_material_requests(requests: &mut Vec<SolanaMaterialRequest>) {
    let mut seen = HashSet::new();
    requests.retain(|request| seen.insert(request.handle));
}

/// Maps one transaction's records to computation logs and material requests. A step output is
/// allowed, and so becomes a block producer, only when the transaction writes it into an encrypted
/// store, which is also when it requests the output's material. Every other output dies with the
/// transaction's TransientStore: the tfhe-worker computes it only for an allowed consumer in the
/// same transaction, as EVM computes a value that no `Allowed` event persists. The KMS validates the
/// live EncryptedStore and any MMR proof before releasing plaintext, so a scheduled computation
/// authorizes no decryption.
pub fn normalize_solana_records_for_db(
    records: impl IntoIterator<Item = SolanaHostRecord>,
    transaction_id: TransactionId,
    block: SolanaBlockMeta,
) -> (Vec<LogTfhe>, Vec<SolanaMaterialRequest>) {
    let records: Vec<SolanaHostRecord> = records.into_iter().collect();
    let stored: HashSet<Handle> = records
        .iter()
        .filter_map(|record| match record {
            SolanaHostRecord::MaterialRequest(request) => Some(request.handle),
            _ => None,
        })
        .collect();
    let mut tfhe_logs = Vec::new();
    let mut material_requests = Vec::new();
    for record in records {
        match record {
            SolanaHostRecord::Computation(computation) => tfhe_logs
                .push(to_log_tfhe(computation, transaction_id, block, &stored)),
            SolanaHostRecord::MaterialRequest(request) => {
                material_requests.push(request)
            }
        }
    }

    dedup_material_requests(&mut material_requests);
    (tfhe_logs, material_requests)
}

/// Ingests ordered transaction groups from one sealed block. Dependency grouping sees the whole
/// block; operand origins and material requests retain their transaction identity.
pub async fn insert_solana_block_records(
    db: &Database,
    tx: &mut Transaction<'_>,
    transactions: impl IntoIterator<Item = (TransactionId, Vec<SolanaHostRecord>)>,
    block: SolanaBlockMeta,
    dependent_ops_max_per_chain: u32,
) -> Result<SolanaIngestStats, SqlxError> {
    let mut tfhe_logs = Vec::new();
    let mut material_requests = Vec::new();
    for (transaction_id, records) in transactions {
        let (logs, requests) =
            normalize_solana_records_for_db(records, transaction_id, block);
        tfhe_logs.extend(logs);
        material_requests.extend(
            requests
                .into_iter()
                .map(|request| (transaction_id, request)),
        );
    }
    for (index, log) in tfhe_logs.iter_mut().enumerate() {
        log.log_index = Some(index as u64);
    }
    populate_operand_boundary_masks(&mut tfhe_logs)?;
    let chains = dependence_chains(
        &mut tfhe_logs,
        &db.dependence_chain,
        &db.consumed_boundaries,
        &db.sealed_chains,
        false,
        true,
    )
    .await;

    let mut inserted_records = 0;
    let mut dependent_ops_by_chain = HashMap::new();
    for log in &tfhe_logs {
        let inserted = db.insert_tfhe_event(tx, log).await?;
        inserted_records += usize::from(inserted);
        if dependent_ops_max_per_chain > 0 && inserted {
            let count = dependent_ops_by_chain
                .entry(log.dependence_chain)
                .or_insert(0_u64);
            *count = count.saturating_add(1);
        }
    }
    let computation_inserted = inserted_records > 0;
    for (transaction_id, request) in &material_requests {
        if db
            .insert_pbs_computations(
                tx,
                &[request.handle.to_vec()],
                Some(transaction_id.to_vec()),
                block.block_number,
            )
            .await?
        {
            inserted_records += 1;
        }
    }
    // A replay that inserts nothing must not rearm a processed chain.
    if computation_inserted {
        let slow_chains = classify_slow_chains(
            db,
            tx,
            &chains,
            &dependent_ops_by_chain,
            dependent_ops_max_per_chain,
        )
        .await?;
        db.update_dependence_chain(
            tx,
            chains,
            block.block_timestamp,
            &solana_block_summary(block),
            &slow_chains,
        )
        .await?;
    }
    Ok(SolanaIngestStats {
        tfhe_events: tfhe_logs.len(),
        material_requests: material_requests.len(),
        inserted_records,
    })
}

/// A computation row the listener inserted but must not let the worker compute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeldBackComputation {
    pub transaction_id: TransactionId,
    pub output_handle: [u8; 32],
    pub reason: String,
}

/// Marks rows inserted in `tx` as terminal errors and returns how many it found. Without a
/// retry marker in the message the tfhe-worker treats them as dead boundaries and drains their
/// dependents, so nothing derived from a held-back handle is computed.
pub async fn hold_back_computations(
    tx: &mut Transaction<'_>,
    held_back: &[HeldBackComputation],
) -> Result<u64, SqlxError> {
    let mut held_rows = 0;
    for held in held_back {
        let updated = sqlx::query!(
            r#"
            UPDATE computations
            SET is_error = true, is_completed = false, error_message = $3
            WHERE output_handle = $1 AND transaction_id = $2
            "#,
            held.output_handle.as_slice(),
            held.transaction_id.as_slice(),
            held.reason,
        )
        .execute(tx.as_mut())
        .await?;
        held_rows += updated.rows_affected();
    }
    Ok(held_rows)
}

fn solana_block_summary(block: SolanaBlockMeta) -> BlockSummary {
    BlockSummary {
        number: block.block_number,
        hash: FixedBytes::<32>::from(block.block_hash),
        parent_hash: FixedBytes::<32>::from(block.parent_hash),
        timestamp: 0,
    }
}

fn to_log_tfhe(
    computation: Computation,
    transaction_id: TransactionId,
    block: SolanaBlockMeta,
    stored: &HashSet<Handle>,
) -> LogTfhe {
    LogTfhe {
        allowed_outputs: computation
            .outputs()
            .iter()
            .copied()
            .filter(|output| stored.contains(output))
            .collect(),
        computation,
        transaction_hash: Some(transaction_id),
        block_number: block.block_number,
        block_hash: FixedBytes::<32>::from(block.block_hash),
        block_timestamp: block.block_timestamp,
        // Placeholders: overwritten by the shared `dependence_chains()` union-find in
        // `insert_solana_block_records` before insertion, exactly like the EVM ingest path's
        // `Default::default()` placeholders.
        tx_depth_size: 0,
        dependence_chain: transaction_id,
        // Assigned across the complete sealed block before origin reconstruction.
        log_index: None,
        // Every reconstructed host op ran on-chain in this signature. Operand
        // origin bits are derived afterwards by `populate_operand_boundary_masks`,
        // the same walk the EVM ingest path uses.
        operand_boundary_mask: None,
        is_executor_minted: true,
        is_fallback_grant: false,
        is_synthetic: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::computation::Operand;
    use fhevm_engine_common::types::SupportedFheOperations as O;
    use time::{Date, Month, PrimitiveDateTime, Time};
    use Operand::{Clear as P, Encrypted as H};

    fn handle(byte: u8) -> Handle {
        Handle::from([byte; 32])
    }

    #[test]
    fn preserves_native_solana_signature() {
        let signature = [7_u8; 64];
        let id = TransactionId::from(signature);
        assert_eq!(id.as_slice(), signature);
        assert_eq!(id.to_string(), bs58::encode(signature).into_string());
    }

    #[test]
    fn dependence_chain_uses_validator_block_hashes() {
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let summary = solana_block_summary(SolanaBlockMeta {
            block_number: 42,
            block_timestamp,
            block_hash: [7; 32],
            parent_hash: [6; 32],
        });

        assert_eq!(summary.number, 42);
        assert_eq!(summary.hash, FixedBytes::<32>::from([7; 32]));
        assert_eq!(summary.parent_hash, FixedBytes::<32>::from([6; 32]));
    }

    #[test]
    fn builds_existing_db_log_shape() {
        let tx_id = TransactionId::from([1_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );
        let event = Computation::single(
            O::FheSub,
            vec![H(handle(1)), Operand::binary_rhs(handle(2), true)],
            handle(3),
        )
        .unwrap();

        let log = to_log_tfhe(
            event,
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
            &HashSet::from([handle(3)]),
        );

        assert_eq!(log.transaction_hash, Some(tx_id));
        assert_eq!(log.block_number, 42);
        assert_eq!(log.block_timestamp, block_timestamp);
        assert_eq!(log.allowed_outputs, HashSet::from([handle(3)]));
        assert_eq!(log.log_index, None);
        assert!(log.is_executor_minted);
        assert!(log.operand_boundary_mask.is_none());
    }

    fn block() -> SolanaBlockMeta {
        SolanaBlockMeta {
            block_number: 42,
            block_timestamp: PrimitiveDateTime::new(
                Date::from_calendar_date(2026, Month::May, 9).unwrap(),
                Time::MIDNIGHT,
            ),
            block_hash: [1; 32],
            parent_hash: [0; 32],
        }
    }

    fn trivial(result: u8) -> SolanaHostRecord {
        SolanaHostRecord::Computation(Computation::trivial(
            [result; 32],
            5,
            handle(result),
        ))
    }

    fn stored(handle: u8) -> SolanaHostRecord {
        SolanaHostRecord::MaterialRequest(material_request([handle; 32]))
    }

    #[test]
    fn only_an_output_its_transaction_stores_is_allowed() {
        let (tfhe_logs, material_requests) = normalize_solana_records_for_db(
            [trivial(3), trivial(4), stored(4)],
            TransactionId::from([7_u8; 64]),
            block(),
        );

        assert!(
            tfhe_logs[0].allowed_outputs.is_empty(),
            "an output no store write names dies with the transaction"
        );
        assert_eq!(tfhe_logs[1].allowed_outputs, HashSet::from([handle(4)]));
        assert_eq!(material_requests, [material_request([4; 32])]);
    }

    /// Trivial-encrypt handles depend on the plaintext alone, so two executions of one transaction
    /// can produce the same handle. The listener inserts one row per handle and transaction, the
    /// first, so it must be allowed when any execution stores the handle.
    #[test]
    fn a_handle_produced_twice_is_allowed_when_a_later_execution_stores_it() {
        let (tfhe_logs, _) = normalize_solana_records_for_db(
            [trivial(3), trivial(3), stored(3)],
            TransactionId::from([7_u8; 64]),
            block(),
        );

        for log in &tfhe_logs {
            assert_eq!(log.allowed_outputs, HashSet::from([handle(3)]));
        }
    }

    /// `make_store_handle_public` writes a handle an earlier transaction produced, which allows
    /// none of this transaction's outputs.
    #[test]
    fn storing_an_older_handle_allows_no_output() {
        let (tfhe_logs, material_requests) = normalize_solana_records_for_db(
            [trivial(3), stored(9)],
            TransactionId::from([8_u8; 64]),
            block(),
        );

        assert!(tfhe_logs[0].allowed_outputs.is_empty());
        assert_eq!(material_requests, [material_request([9; 32])]);
    }

    #[test]
    fn material_requests_keep_distinct_handles_in_one_batch() {
        let tx_id = TransactionId::from([9_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (_, material_requests) = normalize_solana_records_for_db(
            [
                SolanaHostRecord::MaterialRequest(material_request([1; 32])),
                SolanaHostRecord::MaterialRequest(material_request([2; 32])),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        );

        assert_eq!(material_requests.len(), 2);
        assert!(material_requests
            .iter()
            .any(|request| request.handle == handle(1)));
        assert!(material_requests
            .iter()
            .any(|request| request.handle == handle(2)));
    }

    #[test]
    fn normalizes_interleaved_batch_events_for_worker_replay() {
        let tx_id = TransactionId::from([5_u8; 64]);
        let block_timestamp = PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::May, 9).unwrap(),
            Time::MIDNIGHT,
        );

        let (tfhe_logs, material_requests) = normalize_solana_records_for_db(
            [
                SolanaHostRecord::Computation(Computation::trivial(
                    {
                        let mut plaintext = [0_u8; 32];
                        plaintext[31] = 1;
                        plaintext
                    },
                    0,
                    handle(1),
                )),
                SolanaHostRecord::Computation(
                    Computation::single(
                        O::FheRand,
                        vec![P(vec![2; 16]), P(vec![5])],
                        handle(2),
                    )
                    .unwrap(),
                ),
                SolanaHostRecord::Computation(
                    Computation::single(
                        O::FheIfThenElse,
                        vec![H(handle(1)), H(handle(2)), H(handle(1))],
                        handle(3),
                    )
                    .unwrap(),
                ),
            ],
            tx_id,
            SolanaBlockMeta {
                block_number: 42,
                block_timestamp,
                block_hash: [1; 32],
                parent_hash: [0; 32],
            },
        );

        assert!(material_requests.is_empty());
        assert_eq!(tfhe_logs.len(), 3);
        assert_eq!(
            tfhe_logs
                .iter()
                .map(|log| log.log_index)
                .collect::<Vec<_>>(),
            vec![None; 3],
            "block ingestion assigns indexes across transaction groups"
        );
        assert!(
            tfhe_logs.iter().all(|log| log.allowed_outputs.is_empty()),
            "a transaction that stores nothing allows nothing"
        );
        assert_eq!(tfhe_logs[0].computation.operation(), O::FheTrivialEncrypt);
        assert_eq!(tfhe_logs[1].computation.operation(), O::FheRand);
        assert_eq!(tfhe_logs[2].computation.operation(), O::FheIfThenElse);
        assert!(tfhe_logs[0].computation.inputs().is_empty());
        assert!(tfhe_logs[1].computation.inputs().is_empty());
        assert_eq!(
            tfhe_logs[2].computation.inputs(),
            vec![handle(1), handle(2), handle(1)]
        );
    }
}
