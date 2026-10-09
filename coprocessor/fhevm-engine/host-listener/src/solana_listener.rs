//! The coprocessor's sink for the Solana host follower: reconstructs each sealed block's
//! coprocessor work and ingests it into the coprocessor database.
//!
//! - **Version pairing.** Handle re-derivation uses the program crate's `computed_*` functions
//!   (INVARIANTS #28) and hashes the followed `--program-id`, not the crate's compiled
//!   `declare_id!`. Instruction layout still has no runtime handshake: deploy the listener from
//!   the same rev as the program (INVARIANTS #33).
//!
//! Each sealed block is applied in one database transaction: its compute rows, its
//! `host_chain_blocks_valid` row and the resume checkpoint. Rows are numbered by block height,
//! which steps by one along a fork, as the manifest ranges of the consensus detector require;
//! the slot stays in the checkpoint and the logs. Compute rows carry the result handles each
//! `FheExecutedEvent` emitted, so they name the chain's handles even where re-derivation
//! disagrees.
//! A step whose handle this listener does not re-derive is held back: its row is
//! inserted as a terminal error, and the tfhe-worker drains everything that depends
//! on it. The rest of the block is ingested normally.

use std::sync::LazyLock;

use anchor_lang::prelude::Pubkey;
use anyhow::{anyhow, Result};
use prometheus::{register_int_counter_vec, IntCounterVec};
use solana_host_follower::host::{
    host_operations, DecodedInstruction, HostOperation,
};
use solana_host_follower::{
    BlockSink, IngestFailure, PreparedBlock, SealedBlock,
};
use time::{OffsetDateTime, PrimitiveDateTime};
use tracing::{error, info};

use crate::cmd::block_history::BlockSummary;
use crate::database::solana_checkpoint::store_checkpoint;
use crate::database::tfhe_event_propagate::{
    Database, Transaction, TransactionId,
};
use crate::solana_adapter::{
    hold_back_computations, insert_solana_block_records, material_request,
    HeldBackComputation, SolanaBlockMeta, SolanaHostRecord, SolanaIngestStats,
};
use crate::solana_reconstruct::{reconstruct_fhe_execute, HandleMismatch};

static HANDLE_CHECK_FAILURES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_solana_host_listener_handle_check_failures_total",
        "fhe_execute steps whose emitted result handle the listener did not re-derive; each is held back as an errored computation",
        &["host_chain_id"]
    )
    .unwrap()
});

/// Old finalized block rows are pruned once every this many heights, after the block commits.
/// Each pass deletes at most `BLOCKS_VALID_PRUNE_BATCH` rows.
const PRUNE_EVERY_HEIGHTS: u64 = 100;

#[derive(Clone, Debug)]
pub struct SolanaListenerConfig {
    /// zama-host program id hashed into reconstructed handles (not the id this crate was
    /// compiled with).
    pub program_id: Pubkey,
    /// On-chain HostConfig chain_id used in handle derivation (distinct from the
    /// coprocessor host-chain id).
    pub chain_id: u64,
    /// Shared scheduler cap; zero disables the slow lane.
    pub dependent_ops_max_per_chain: u32,
}

pub struct SolanaListenerSink<'a> {
    db: &'a Database,
    config: SolanaListenerConfig,
}

impl<'a> SolanaListenerSink<'a> {
    /// Exports the handle-check counter at zero, so `increase()` counts the first failure.
    pub fn new(db: &'a Database, config: SolanaListenerConfig) -> Self {
        HANDLE_CHECK_FAILURES
            .with_label_values(&[&config.chain_id.to_string()]);
        Self { db, config }
    }
}

impl BlockSink for SolanaListenerSink<'_> {
    async fn apply(
        &self,
        block: &PreparedBlock,
    ) -> std::result::Result<(), IngestFailure> {
        apply_block(self.db, &self.config, block).await
    }
}

/// Applies one sealed block in one database transaction: every covered
/// transaction's compute rows, the block's `host_chain_blocks_valid` row and
/// the checkpoint.
async fn apply_block(
    db: &Database,
    config: &SolanaListenerConfig,
    prepared: &PreparedBlock,
) -> std::result::Result<(), IngestFailure> {
    let sealed_block = &prepared.block;
    let block_height = sealed_block.block_height.ok_or_else(|| {
        IngestFailure::fatal(anyhow!(
            "slot {} has no block height",
            sealed_block.slot
        ))
    })?;
    let block_timestamp =
        sealed_block_timestamp(sealed_block).ok_or_else(|| {
            IngestFailure::fatal(anyhow!(
                "missing or invalid block time for slot {}",
                sealed_block.slot
            ))
        })?;
    let mut reconstructed = Vec::new();
    for transaction in &prepared.transactions {
        let outcome = reconstruct_records_for_insert(
            config,
            &transaction.instructions,
            sealed_block.slot,
        )
        .map_err(|err| {
            IngestFailure::fatal(err).context("reconstruct Solana host records")
        })?;
        if let ReconstructionOutcome::Complete(records) = outcome {
            reconstructed.push((transaction, records));
        }
    }

    // Same cutover/schema-reset write boundary as the EVM ingest path: a retired stack takes no
    // more rows, so drop the block instead of failing the subscription.
    let Some(mut db_tx) = db
        .new_transaction()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("open db tx"))?
    else {
        info!(
            slot = sealed_block.slot,
            "Cutover completed - skipping Solana block on retired stack"
        );
        return Ok(());
    };
    let summary = BlockSummary {
        number: block_height,
        hash: sealed_block.block_hash.into(),
        parent_hash: sealed_block.parent_block_hash.into(),
        timestamp: block_timestamp.assume_utc().unix_timestamp() as u64,
    };
    require_parent_height(&mut db_tx, db.chain_id.as_i64(), &summary).await?;

    let mut records_by_transaction = Vec::new();
    let mut held_back = Vec::new();
    let mut check_failures = Vec::new();
    for (transaction, records) in reconstructed {
        let transaction_id = TransactionId::from(transaction.signature);
        records_by_transaction.push((transaction_id, records.records));
        for failure in records.check_failures {
            held_back.push(HeldBackComputation {
                transaction_id,
                output_handle: failure.mismatch.emitted,
                reason: failure.describe(sealed_block.slot),
            });
            check_failures.push((transaction.signature, failure));
        }
    }
    let stats = if records_by_transaction.is_empty() {
        SolanaIngestStats::default()
    } else {
        let block = SolanaBlockMeta {
            block_number: block_height,
            block_timestamp,
            block_hash: sealed_block.block_hash,
            parent_hash: sealed_block.parent_block_hash,
        };
        insert_solana_block_records(
            db,
            &mut db_tx,
            records_by_transaction,
            block,
            config.dependent_ops_max_per_chain,
        )
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("insert_solana_block_records")
        })?
    };
    let held_rows = hold_back_computations(&mut db_tx, &held_back)
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("hold back computations")
        })?;
    if held_rows != held_back.len() as u64 {
        return Err(IngestFailure::fatal(anyhow!(
            "slot {}: {} held-back steps but {held_rows} computation rows",
            sealed_block.slot,
            held_back.len()
        )));
    }

    // The listener reads at finalized (DD-070), so every block is final when it is recorded.
    db.mark_block_as_valid(
        &mut db_tx,
        &summary,
        true,
        stats.tfhe_events as i32,
        stats.material_requests as i32,
    )
    .await
    .map_err(|err| {
        IngestFailure::retryable(err).context("record the host block")
    })?;
    store_checkpoint(&mut db_tx, &sealed_block.checkpoint())
        .await
        .map_err(|err| {
            IngestFailure::retryable(err).context("store checkpoint")
        })?;
    db_tx
        .commit()
        .await
        .map_err(|err| IngestFailure::retryable(err).context("commit db tx"))?;

    for (signature, failure) in &check_failures {
        error!(
            signature = %signature,
            "{}; the step is held back",
            failure.describe(sealed_block.slot)
        );
    }
    if !check_failures.is_empty() {
        HANDLE_CHECK_FAILURES
            .with_label_values(&[&config.chain_id.to_string()])
            .inc_by(check_failures.len() as u64);
    }

    if stats.inserted_records > 0 {
        info!(
            slot = sealed_block.slot,
            block_height,
            tfhe_events = stats.tfhe_events,
            material_requests = stats.material_requests,
            inserted_records = stats.inserted_records,
            "ingested Solana host records (gRPC)"
        );
    }
    // Best effort, as on EVM: a failure only delays pruning.
    if block_height % PRUNE_EVERY_HEIGHTS == 0 {
        match db.prune_finalized_block_history(block_height as i64).await {
            Ok(0) => {}
            Ok(pruned) => info!(pruned, "pruned finalized block history"),
            Err(err) => {
                error!(?err, "failed to prune finalized block history")
            }
        }
    }
    Ok(())
}

/// The block's parent row must be at the height below it. The consensus detector numbers manifest
/// ranges by height, and `mark_block_as_valid` records an ingested block as finalized without
/// refusing a parent that disagrees. Only the chain's first row has no parent row. A block that is
/// already recorded was checked when it was first applied, by this replica or another, and its
/// replay writes nothing new.
async fn require_parent_height(
    db_tx: &mut Transaction<'_>,
    chain_id: i64,
    block: &BlockSummary,
) -> std::result::Result<(), IngestFailure> {
    let parent = sqlx::query!(
        r#"
        SELECT
            (SELECT block_number FROM host_chain_blocks_valid
              WHERE chain_id = $1 AND block_hash = $2) AS parent_height,
            EXISTS (SELECT 1 FROM host_chain_blocks_valid WHERE chain_id = $1)
                AS "chain_has_rows!",
            EXISTS (SELECT 1 FROM host_chain_blocks_valid
                     WHERE chain_id = $1 AND block_hash = $3) AS "recorded!"
        "#,
        chain_id,
        block.parent_hash.as_slice(),
        block.hash.as_slice(),
    )
    .fetch_one(db_tx.as_mut())
    .await
    .map_err(|err| {
        IngestFailure::retryable(err).context("read the parent block row")
    })?;
    if parent.recorded {
        return Ok(());
    }
    match parent.parent_height {
        Some(height) if height as u64 + 1 == block.number => Ok(()),
        Some(height) => Err(IngestFailure::fatal(anyhow!(
            "block {} at height {} has its parent at height {height}",
            block.hash,
            block.number
        ))),
        None if parent.chain_has_rows => Err(IngestFailure::fatal(anyhow!(
            "block {} at height {} has no recorded parent {}",
            block.hash,
            block.number,
            block.parent_hash
        ))),
        None => Ok(()),
    }
}

fn unix_to_pdt(ts: i64) -> Option<PrimitiveDateTime> {
    let dt = OffsetDateTime::from_unix_timestamp(ts).ok()?;
    Some(PrimitiveDateTime::new(dt.date(), dt.time()))
}

/// Geyser's block time is the bank's `Clock::unix_timestamp`, the value every handle in the
/// block was derived with.
fn sealed_block_timestamp(block: &SealedBlock) -> Option<PrimitiveDateTime> {
    block.block_time.and_then(unix_to_pdt)
}

/// One covered transaction, rebuilt off-chain: the compute rows to insert, in on-chain
/// order, and the steps whose emitted handle re-derivation did not reproduce.
#[derive(Debug, Default)]
struct ReconstructedTransaction {
    records: Vec<SolanaHostRecord>,
    check_failures: Vec<HandleCheckFailure>,
}

/// A [`HandleMismatch`] located in its transaction: `execution_index` counts the
/// transaction's `fhe_execute` invocations from zero.
#[derive(Debug)]
struct HandleCheckFailure {
    execution_index: usize,
    mismatch: HandleMismatch,
}

impl HandleCheckFailure {
    /// The held-back row's `error_message` and the alert's log line. The row's `transaction_id`
    /// already names the transaction; leaving the base58 signature out keeps the message from
    /// ever spelling the tfhe-worker's retry marker, which would make the hold-back retryable.
    fn describe(&self, slot: u64) -> String {
        format!(
            "solana handle check failed: slot {slot}, execution {}, step {}: emitted 0x{}, re-derived 0x{}",
            self.execution_index,
            self.mismatch.step_index,
            hex::encode(self.mismatch.emitted),
            hex::encode(self.mismatch.derived),
        )
    }
}

#[derive(Debug)]
enum ReconstructionOutcome {
    Complete(ReconstructedTransaction),
    NotCovered,
}

/// Rebuilds the ingestable record set off-chain from a transaction's instructions.
/// Covers `fhe_execute` (one op record per step, plus a material request for each
/// store write) and `make_store_handle_public`, decoded from the same ordered
/// instruction list together with each execution's `FheExecutedEvent`.
fn reconstruct_records_for_insert(
    config: &SolanaListenerConfig,
    instructions: &[DecodedInstruction],
    slot: u64,
) -> Result<ReconstructionOutcome> {
    let operations = host_operations(instructions, slot)?;
    if operations.is_empty() {
        return Ok(ReconstructionOutcome::NotCovered);
    }

    let mut reconstructed = ReconstructedTransaction::default();
    let mut produced_in_tx = std::collections::HashSet::new();
    let mut execution_index = 0;
    for operation in &operations {
        if let HostOperation::FheExecute { args, event, .. } = operation {
            let Some(steps) = reconstruct_fhe_execute(
                args,
                event,
                config.program_id,
                config.chain_id,
                &mut produced_in_tx,
            ) else {
                anyhow::bail!(
                    "reconstruct: fhe_execute in slot {slot} has an operand its event \
                     or dictionary does not resolve"
                );
            };
            reconstructed.check_failures.extend(
                steps.mismatches.into_iter().map(|mismatch| {
                    HandleCheckFailure {
                        execution_index,
                        mismatch,
                    }
                }),
            );
            execution_index += 1;
            reconstructed.records.extend(steps.records);
        }
        for write in operation.store_writes() {
            reconstructed
                .records
                .push(SolanaHostRecord::MaterialRequest(material_request(
                    write.handle,
                )));
        }
    }
    Ok(ReconstructionOutcome::Complete(reconstructed))
}

#[cfg(test)]
mod test_support {
    use super::{
        reconstruct_records_for_insert, ReconstructedTransaction,
        ReconstructionOutcome, SolanaListenerConfig,
    };
    use anchor_lang::{AnchorSerialize, Discriminator};
    use std::collections::HashSet;
    use zama_host::state::{FheExecuteArgs, FheExecuteStep};
    use zama_host::{
        FheExecuteRandomSeed, FheExecutedEvent, HandleDerivationContext,
    };

    use crate::solana_reconstruct::event_with_derived_results;
    use solana_host_follower::host::{
        decode_fhe_execute_args, DecodedInstruction,
    };

    /// `event`'s bytes as the host's event CPI carries them.
    pub(super) fn event_cpi_data(event: &FheExecutedEvent) -> Vec<u8> {
        anchor_lang::event::EVENT_IX_TAG_LE
            .iter()
            .copied()
            .chain(anchor_lang::Event::data(event))
            .collect()
    }

    // A valid pubkey that is not the compiled-in `zama_host::ID`: derivation must follow the
    // configured deployment.
    pub(super) const ZAMA_HOST: &str =
        "7DYCAhqwQSKqqL1h8V1XmY1BTcMWxrASQYKNMy87jeg3";
    pub(super) const STATE: [u8; 32] = [0x22; 32];

    pub(super) fn config() -> SolanaListenerConfig {
        SolanaListenerConfig {
            program_id: ZAMA_HOST.parse().unwrap(),
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
            dependent_ops_max_per_chain: 0,
        }
    }

    pub(super) fn encoded_execution(args: FheExecuteArgs) -> Vec<u8> {
        let mut data =
            zama_host::instruction::FheExecute::DISCRIMINATOR.to_vec();
        args.serialize(&mut data).unwrap();
        data
    }

    pub(super) fn context() -> HandleDerivationContext {
        HandleDerivationContext {
            program_id: ZAMA_HOST.parse().unwrap(),
            chain_id: config().chain_id,
            previous_bank_hash: [0x44; 32],
            unix_timestamp: 1_700_000_000,
        }
    }

    pub(super) fn event_instruction(
        event: &FheExecutedEvent,
    ) -> DecodedInstruction {
        DecodedInstruction {
            data: event_cpi_data(event),
            accounts: vec![],
        }
    }

    /// Follows every host `fhe_execute` with the event a host would emit for it.
    pub(super) fn with_events(
        instructions: impl IntoIterator<Item = DecodedInstruction>,
    ) -> Vec<DecodedInstruction> {
        let mut produced = HashSet::new();
        let mut transaction = Vec::new();
        for instruction in instructions {
            let execution = decode_fhe_execute_args(&instruction.data);
            transaction.push(instruction.clone());
            if let Some(execution) = execution {
                let seeds = execution
                    .steps
                    .iter()
                    .enumerate()
                    .filter(|(_, step)| {
                        matches!(
                            step,
                            FheExecuteStep::Rand { .. }
                                | FheExecuteStep::RandBounded { .. }
                        )
                    })
                    .map(|(index, _)| FheExecuteRandomSeed {
                        step_index: index as u16,
                        seed: [7; 16],
                    })
                    .collect();
                let event = event_with_derived_results(
                    &execution,
                    &context(),
                    seeds,
                    &produced,
                );
                produced.extend(event.results.iter().copied());
                transaction.push(event_instruction(&event));
            }
        }
        transaction
    }

    pub(super) fn reconstruct(
        instructions: &[DecodedInstruction],
    ) -> anyhow::Result<ReconstructedTransaction> {
        match reconstruct_records_for_insert(&config(), instructions, 42)? {
            ReconstructionOutcome::Complete(reconstructed) => Ok(reconstructed),
            other => panic!("expected a covered transaction, got {other:?}"),
        }
    }
}

#[cfg(test)]
mod fhe_execute_acl_tests {
    use super::test_support::{
        context, encoded_execution, event_instruction, reconstruct,
        with_events, STATE,
    };
    use super::HandleCheckFailure;
    use solana_host_follower::host::{
        decode_fhe_executed_event, DecodedInstruction,
    };
    use zama_host::state::{FheExecuteArgs, FheExecuteStep};

    use crate::solana_adapter::SolanaHostRecord;
    use crate::solana_reconstruct::HandleMismatch;

    #[test]
    fn check_failures_name_the_execution_and_keep_the_emitted_handle() {
        let execute = |plaintext| DecodedInstruction {
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                effects: vec![],
                returned_results: Vec::new(),
                account_count: 0,
                dictionary: vec![],
                steps: vec![FheExecuteStep::TrivialEncrypt {
                    plaintext,
                    fhe_type: 5,
                }],
            }),
            accounts: vec![],
        };
        let mut instructions =
            with_events([execute([1; 32]), execute([2; 32])]);
        let mut event =
            decode_fhe_executed_event(&instructions[3].data).unwrap();
        let derived = event.results[0];
        event.results[0] = [0xEE; 32];
        instructions[3] = event_instruction(&event);

        let reconstructed = reconstruct(&instructions).unwrap();
        let [HandleCheckFailure {
            execution_index: 1,
            mismatch,
        }] = &reconstructed.check_failures[..]
        else {
            panic!("{:?}", reconstructed.check_failures)
        };
        assert_eq!(
            mismatch,
            &HandleMismatch {
                step_index: 0,
                emitted: [0xEE; 32],
                derived,
            }
        );
        assert!(reconstructed.records.iter().any(|record| matches!(
            record,
            crate::solana_adapter::SolanaHostRecord::TrivialEncrypt(op)
                if op.result == [0xEE; 32]
        )));
    }

    #[test]
    fn store_slot_preimage_depends_on_prior_calls_in_the_reconstruction() {
        use crate::solana_adapter::SolanaHostRecord;
        use zama_host::{
            ExecutionResultRef, FheBinaryOpCode, FheExecuteEffect,
            FheExecuteOperand, SlotWrite,
        };
        let context = context();
        let handle =
            zama_host::computed_eval_trivial_handle([7; 32], 5, &context);
        let mut accounts = vec![[0; 32]; zama_host::FHE_EXECUTE_FIXED_ACCOUNTS];
        accounts.push(STATE);
        let first = DecodedInstruction {
            accounts,
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: vec![[9; 32]],
                steps: vec![FheExecuteStep::TrivialEncrypt {
                    plaintext: [7; 32],
                    fhe_type: 5,
                }],
                effects: vec![FheExecuteEffect {
                    result: ExecutionResultRef {
                        step_index: 0,
                        output_index: 0,
                    },
                    store_index: 0,
                    previous_leaf_count: 0,
                    slot: Some(SlotWrite {
                        key_index: 0,
                        previous_handle_index: None,
                    }),
                    allow_indexes: vec![],
                    make_public: false,
                    grants: vec![],
                }],
                returned_results: vec![],
            }),
        };
        let second = DecodedInstruction {
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: vec![handle, [9; 32], [0; 32]],
                steps: vec![FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::StoreSlot {
                        handle_index: 0,
                        key_index: 1,
                        store_index: 0,
                    },
                    rhs: FheExecuteOperand::Scalar { value_index: 2 },
                    output_fhe_type: 5,
                }],
                effects: vec![],
                returned_results: vec![],
            }),
            ..first.clone()
        };
        for (instructions, boundary) in
            [(vec![first, second.clone()], 0), (vec![second], 1)]
        {
            let rebuilt = reconstruct(&with_events(instructions)).unwrap();
            let result = rebuilt
                .records
                .iter()
                .find_map(|record| match record {
                    SolanaHostRecord::FheBinaryOp(op) => Some(op.result),
                    _ => None,
                })
                .unwrap();
            let mut mask = [0; 32];
            mask[31] = boundary;
            assert_eq!(
                result,
                zama_host::computed_eval_handle(
                    FheBinaryOpCode::Add,
                    handle,
                    [0; 32],
                    true,
                    5,
                    mask,
                    &context
                )
            );
        }
    }

    #[test]
    fn a_store_output_requests_its_material() {
        let args = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 4,
                slot: None,
                allow_indexes: vec![0],
                make_public: true,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![[0x33; 32]],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
            }],
        };
        let mut accounts: Vec<[u8; 32]> = (0..12).map(|n| [n; 32]).collect();
        accounts[zama_host::FHE_EXECUTE_FIXED_ACCOUNTS] = STATE;
        let reconstructed = reconstruct(&with_events([DecodedInstruction {
            data: encoded_execution(args),
            accounts,
        }]))
        .unwrap();
        let output = reconstructed
            .records
            .iter()
            .find_map(|record| match record {
                SolanaHostRecord::TrivialEncrypt(op) => Some(op.result),
                _ => None,
            })
            .unwrap();
        assert!(reconstructed.records.iter().any(|record| matches!(
            record,
            SolanaHostRecord::MaterialRequest(request)
                if request.handle == output
        )));
    }
}

#[cfg(test)]
mod apply_block_tests {
    use super::apply_block;
    use super::test_support::{
        config, context, encoded_execution, event_instruction, with_events,
        STATE,
    };
    use crate::database::solana_checkpoint::load_checkpoint;
    use crate::database::tfhe_event_propagate::Database;
    use fhevm_engine_common::chain_id::ChainId;
    use serial_test::serial;
    use solana_host_follower::host::{
        decode_fhe_executed_event, DecodedInstruction,
    };
    use solana_host_follower::{
        PreparedBlock, PreparedTransaction, SealedBlock,
    };
    use solana_sdk::signature::Signature;
    use sqlx::Row;
    use test_harness::instance::{setup_test_db, ImportMode};
    use zama_host::state::{
        ExecutionResultRef, FheBinaryOpCode, FheExecuteArgs, FheExecuteEffect,
        FheExecuteOperand, FheExecuteStep,
    };

    /// The value added to [`two_steps`]'s first result.
    const SCALAR: [u8; 32] = [3; 32];

    /// The handle a tampered event emits for the first step instead of the derived one.
    const WRONG: [u8; 32] = [0xEE; 32];

    fn handle_check_failures() -> f64 {
        let label = config().chain_id.to_string();
        prometheus::gather()
            .iter()
            .find(|family| {
                family.name()
                    == "coprocessor_solana_host_listener_handle_check_failures_total"
            })
            .and_then(|family| {
                family
                    .get_metric()
                    .iter()
                    .find(|metric| {
                        metric.get_label().iter().any(|pair| pair.value() == label)
                    })
                    .map(|metric| metric.get_counter().value())
            })
            .unwrap_or(0.0)
    }

    fn execution(
        steps: Vec<FheExecuteStep>,
        dictionary: Vec<[u8; 32]>,
    ) -> DecodedInstruction {
        DecodedInstruction {
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                effects: vec![],
                returned_results: vec![],
                account_count: 0,
                dictionary,
                steps,
            }),
            accounts: vec![],
        }
    }

    /// A trivial encryption of `plaintext` followed by its sum with [`SCALAR`], the first
    /// dictionary entry.
    fn two_steps(
        plaintext: [u8; 32],
        dictionary: Vec<[u8; 32]>,
    ) -> DecodedInstruction {
        assert_eq!(dictionary[0], SCALAR);
        execution(
            vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext,
                    fhe_type: 5,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                },
            ],
            dictionary,
        )
    }

    /// Makes the event of a [`two_steps`] execution emit [`WRONG`] for its first step, and the
    /// honest derivation of the sum from it. Returns the consumer's handle and the handle the
    /// first step really derives to. The consumer's operand was produced in the transaction, so
    /// its mask is zero.
    fn tamper(instructions: &mut [DecodedInstruction]) -> ([u8; 32], [u8; 32]) {
        let consumer = zama_host::computed_eval_handle(
            FheBinaryOpCode::Add,
            WRONG,
            SCALAR,
            true,
            5,
            [0; 32],
            &context(),
        );
        let mut event =
            decode_fhe_executed_event(&instructions[1].data).unwrap();
        let derived = event.results[0];
        event.results = vec![WRONG, consumer];
        instructions[1] = event_instruction(&event);
        (consumer, derived)
    }

    fn sealed(
        slot: u64,
        block_hash: [u8; 32],
        parent_block_hash: [u8; 32],
    ) -> SealedBlock {
        SealedBlock {
            slot,
            block_hash,
            parent_slot: slot - 1,
            parent_block_hash,
            block_time: Some(1_700_000_000),
            block_height: Some(slot - 2),
            executed_transaction_count: 1,
        }
    }

    /// A block without transactions at `slot` and `height`.
    fn empty(
        slot: u64,
        height: u64,
        block_hash: [u8; 32],
        parent_block_hash: [u8; 32],
    ) -> PreparedBlock {
        PreparedBlock {
            block: SealedBlock {
                block_height: Some(height),
                executed_transaction_count: 0,
                ..sealed(slot, block_hash, parent_block_hash)
            },
            transactions: vec![],
        }
    }

    async fn host_rows(
        pool: &sqlx::PgPool,
    ) -> Vec<(i64, Vec<u8>, Vec<u8>, String)> {
        sqlx::query_as(
            "SELECT block_number, block_hash, parent_hash, block_status FROM host_chain_blocks_valid ORDER BY block_number",
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }

    /// The host row a finalized block at `height` writes.
    fn finalized(
        height: i64,
        hash: u8,
        parent: u8,
    ) -> (i64, Vec<u8>, Vec<u8>, String) {
        (
            height,
            vec![hash; 32],
            vec![parent; 32],
            "finalized".to_owned(),
        )
    }

    async fn new_db() -> (test_harness::instance::DBInstance, Database) {
        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let db = Database::new(
            &instance.db_url,
            ChainId::from_canonical_u64(config().chain_id),
            100,
        )
        .await
        .unwrap();
        (instance, db)
    }

    /// A trivial encryption written to the store, which also requests its material.
    fn stored_execution() -> DecodedInstruction {
        let mut accounts = vec![[0; 32]; zama_host::FHE_EXECUTE_FIXED_ACCOUNTS];
        accounts.push(STATE);
        DecodedInstruction {
            accounts,
            data: encoded_execution(FheExecuteArgs {
                execution_store_index: 0,
                account_count: 1,
                dictionary: vec![[0x33; 32]],
                steps: vec![FheExecuteStep::TrivialEncrypt {
                    plaintext: [1; 32],
                    fhe_type: 5,
                }],
                effects: vec![FheExecuteEffect {
                    result: ExecutionResultRef {
                        step_index: 0,
                        output_index: 0,
                    },
                    store_index: 0,
                    previous_leaf_count: 0,
                    slot: None,
                    allow_indexes: vec![0],
                    make_public: false,
                    grants: vec![],
                }],
                returned_results: vec![],
            }),
        }
    }

    /// Slots 40, 41, 43 and 44, with slot 42 skipped, are heights 38 to 41: the rows step by
    /// one and each names its parent, as the manifest ranges require. The output stored at
    /// slot 43 is produced at height 40, the block number the detector joins on.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn rows_are_numbered_by_height_across_a_skipped_slot() {
        let (_instance, db) = new_db().await;
        let mut slot_43 = empty(43, 40, [0x43; 32], [0x41; 32]);
        slot_43.transactions = vec![PreparedTransaction {
            signature: Signature::from([1; 64]),
            index: 0,
            instructions: with_events([stored_execution()]),
        }];
        for block in [
            empty(40, 38, [0x40; 32], [0x39; 32]),
            empty(41, 39, [0x41; 32], [0x40; 32]),
            slot_43,
            empty(44, 41, [0x44; 32], [0x43; 32]),
        ] {
            apply_block(&db, &config(), &block).await.unwrap();
        }

        let pool = db.pool().await;
        assert_eq!(
            host_rows(&pool).await,
            vec![
                finalized(38, 0x40, 0x39),
                finalized(39, 0x41, 0x40),
                finalized(40, 0x43, 0x41),
                finalized(41, 0x44, 0x43),
            ]
        );
        let computed_at: Vec<i64> =
            sqlx::query_scalar("SELECT block_number FROM computations")
                .fetch_all(&pool)
                .await
                .unwrap();
        assert_eq!(computed_at, vec![40]);
        let produced_at: Vec<i64> = sqlx::query_scalar(
            "SELECT producer_block_number FROM handle_producer_block",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(produced_at, vec![40]);
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 44);
    }

    /// A block without a height or a block time, or whose parent row is not at the height below
    /// it, stops the listener and writes nothing. Only the chain's first row may have no parent
    /// row.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn an_incomplete_block_or_a_wrong_parent_writes_nothing() {
        let (_instance, db) = new_db().await;
        let pool = db.pool().await;
        let mut no_height = empty(10, 8, [0x10; 32], [0x09; 32]);
        no_height.block.block_height = None;
        let mut no_time = empty(10, 8, [0x10; 32], [0x09; 32]);
        no_time.block.block_time = None;
        for incomplete in [no_height, no_time] {
            assert!(apply_block(&db, &config(), &incomplete)
                .await
                .unwrap_err()
                .is_fatal());
        }
        assert!(host_rows(&pool).await.is_empty());
        assert!(load_checkpoint(&pool).await.unwrap().is_none());

        apply_block(&db, &config(), &empty(10, 8, [0x10; 32], [0x09; 32]))
            .await
            .unwrap();
        for wrong in [
            empty(11, 10, [0x11; 32], [0x10; 32]),
            empty(11, 9, [0x11; 32], [0xAA; 32]),
        ] {
            assert!(apply_block(&db, &config(), &wrong)
                .await
                .unwrap_err()
                .is_fatal());
        }
        assert_eq!(host_rows(&pool).await.len(), 1);
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 10);
    }

    /// The repair: rewind the checkpoint and revert the rows above a block's height with the
    /// operator scripts, then replay. The revert refuses unless the checkpoint is at its height.
    /// The consumer the worker drained comes back as a fresh row, the block rows return, and
    /// the checkpoint returns to the tip.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(handle_check_failures)]
    async fn a_reverted_slot_replays_with_fresh_rows() {
        let slot_41 = PreparedBlock {
            block: sealed(41, [0x41; 32], [0x40; 32]),
            transactions: vec![PreparedTransaction {
                signature: Signature::from([1; 64]),
                index: 0,
                instructions: with_events([execution(
                    vec![FheExecuteStep::TrivialEncrypt {
                        plaintext: [1; 32],
                        fhe_type: 5,
                    }],
                    vec![],
                )]),
            }],
        };
        let mut tampered = with_events([two_steps([2; 32], vec![SCALAR])]);
        let (consumer, _) = tamper(&mut tampered);
        let slot_42 = PreparedBlock {
            block: sealed(42, [0x42; 32], [0x41; 32]),
            transactions: vec![PreparedTransaction {
                signature: Signature::from([2; 64]),
                index: 0,
                instructions: tampered,
            }],
        };

        let (instance, db) = new_db().await;
        let chain_id = ChainId::from_canonical_u64(config().chain_id);
        let pool = db.pool().await;
        sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'solana', '') ON CONFLICT DO NOTHING")
            .bind(chain_id.as_i64())
            .execute(&pool)
            .await
            .unwrap();

        // A refused script leaves its session in the aborted transaction, as psql would
        // before exiting, so it runs on a connection of its own.
        let db_url = instance.db_url();
        let refused_revert = |height| async move {
            let mut session: sqlx::PgConnection =
                sqlx::Connection::connect(db_url).await.unwrap();
            sqlx::raw_sql(
                &test_harness::db_utils::revert_coprocessor_db_state_sql(
                    chain_id.as_i64(),
                    height,
                ),
            )
            .execute(&mut session)
            .await
            .unwrap_err()
            .to_string()
        };
        let unstarted = refused_revert(39).await;
        assert!(unstarted.contains("has no checkpoint"), "{unstarted}");

        for block in [&slot_41, &slot_42] {
            apply_block(&db, &config(), block).await.unwrap();
        }
        // The worker drained the consumer of the held step.
        sqlx::query("UPDATE computations SET is_error = true, error_message = 'drained' WHERE output_handle = $1")
            .bind(consumer.to_vec())
            .execute(&pool)
            .await
            .unwrap();

        let both_rows =
            vec![finalized(39, 0x41, 0x40), finalized(40, 0x42, 0x41)];
        assert_eq!(host_rows(&pool).await, both_rows);

        // Slot 41 is height 39; the checkpoint is still at slot 42, height 40.
        let ahead = refused_revert(39).await;
        assert!(ahead.contains("at block height 40, not 39"), "{ahead}");
        let rewind =
            test_harness::db_utils::rewind_solana_listener_checkpoint_sql(
                41, [0x41; 32],
            );
        sqlx::raw_sql(&rewind).execute(&pool).await.unwrap();
        let behind = refused_revert(40).await;
        assert!(behind.contains("at block height 39, not 40"), "{behind}");

        sqlx::raw_sql(
            &test_harness::db_utils::revert_coprocessor_db_state_sql(
                chain_id.as_i64(),
                39,
            ),
        )
        .execute(&pool)
        .await
        .unwrap();
        let checkpoint = load_checkpoint(&pool).await.unwrap().unwrap();
        assert_eq!((checkpoint.slot, checkpoint.block_hash), (41, [0x41; 32]));
        let reverted: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM computations WHERE block_number = 40",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(reverted, 0);
        assert_eq!(host_rows(&pool).await, both_rows[..1]);

        apply_block(&db, &config(), &slot_42).await.unwrap();
        let consumer_errored: bool = sqlx::query_scalar(
            "SELECT is_error FROM computations WHERE output_handle = $1",
        )
        .bind(consumer.to_vec())
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(
            !consumer_errored,
            "the replay re-inserts the drained consumer fresh"
        );
        assert_eq!(host_rows(&pool).await, both_rows);
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 42);

        let unrecorded =
            test_harness::db_utils::rewind_solana_listener_checkpoint_sql(
                30, [0x30; 32],
            );
        sqlx::raw_sql(&unrecorded).execute(&pool).await.unwrap();
        let pruned = refused_revert(39).await;
        assert!(pruned.contains("names no recorded block"), "{pruned}");
    }

    /// A block whose second transaction emits a wrong handle for its first step: that step
    /// is held back, its consumer and the other transaction are ingested, the checkpoint
    /// advances and the alert counter moves.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(handle_check_failures)]
    async fn a_wrong_emitted_handle_holds_back_only_its_step() {
        let honest = with_events([execution(
            vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [1; 32],
                fhe_type: 5,
            }],
            vec![],
        )]);
        let mut tampered = with_events([two_steps([2; 32], vec![SCALAR])]);
        let (consumer, derived) = tamper(&mut tampered);

        let block = PreparedBlock {
            block: SealedBlock {
                executed_transaction_count: 2,
                ..sealed(42, [5; 32], [4; 32])
            },
            transactions: vec![
                PreparedTransaction {
                    signature: Signature::from([1; 64]),
                    index: 0,
                    instructions: honest,
                },
                PreparedTransaction {
                    signature: Signature::from([2; 64]),
                    index: 1,
                    instructions: tampered,
                },
            ],
        };
        let (_instance, db) = new_db().await;
        let failures_before = handle_check_failures();

        apply_block(&db, &config(), &block).await.unwrap();

        let pool = db.pool().await;
        let rows = sqlx::query(
            "SELECT output_handle, transaction_id, is_error, error_message FROM computations",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        let mut held = Vec::new();
        let mut ingested = Vec::new();
        for row in &rows {
            let handle = row.get::<Vec<u8>, _>("output_handle");
            if row.get::<bool, _>("is_error") {
                held.push((
                    handle,
                    row.get::<Vec<u8>, _>("transaction_id"),
                    row.get::<Option<String>, _>("error_message").unwrap(),
                ));
            } else {
                ingested.push(handle);
            }
        }
        assert_eq!(
            held,
            vec![(
                WRONG.to_vec(),
                vec![2; 64],
                format!(
                    "solana handle check failed: slot 42, execution 0, step 0: emitted 0x{}, re-derived 0x{}",
                    hex::encode(WRONG),
                    hex::encode(derived)
                )
            )]
        );
        assert_eq!(ingested.len(), 2, "the honest step and the consumer");
        assert!(ingested.contains(&consumer.to_vec()));
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 42);
        assert_eq!(handle_check_failures() - failures_before, 1.0);
    }

    /// Every row a block writes, as sorted JSON, without the columns no result depends on: the
    /// dependence chain a computation joins is scheduling, and each replica assigns it from its
    /// own caches, as the EVM listener does.
    async fn rows_without_chain_topology(pool: &sqlx::PgPool) -> Vec<String> {
        let mut tables = Vec::new();
        for table in [
            "computations",
            "pbs_computations",
            "allowed_handles",
            "handle_producer_block",
            "host_chain_blocks_valid",
            "solana_listener_checkpoint",
        ] {
            tables.push(
                sqlx::query_scalar(&format!(
                    "SELECT coalesce(jsonb_agg(row ORDER BY row::text), '[]')::text FROM \
                     (SELECT to_jsonb(t) - ARRAY['dependence_chain_id', 'created_at', \
                     'updated_at', 'last_updated_at'] AS row FROM {table} t) rows"
                ))
                .fetch_one(pool)
                .await
                .unwrap(),
            );
        }
        tables
    }

    /// The database holds `expected`, and every computation belongs to a chain row.
    async fn assert_single_run_rows(pool: &sqlx::PgPool, expected: &[String]) {
        assert_eq!(rows_without_chain_topology(pool).await, expected);
        let unchained: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM computations c WHERE NOT EXISTS \
             (SELECT 1 FROM dependence_chain d WHERE d.dependence_chain_id = c.dependence_chain_id)",
        )
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(unchained, 0);
    }

    /// Two replicas, each with its own caches, apply every block to one database: one after the
    /// other, overlapping, and concurrently. The rows are those of a single run apart from chain
    /// topology, every computation belongs to a chain, and the checkpoint never moves back.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(handle_check_failures)]
    async fn two_replicas_write_the_single_run_rows() {
        let transaction = |signature: u8, instructions| PreparedTransaction {
            signature: Signature::from([signature; 64]),
            index: 0,
            instructions,
        };
        let mut tampered = with_events([two_steps([2; 32], vec![SCALAR])]);
        tamper(&mut tampered);
        let blocks = [
            PreparedBlock {
                block: sealed(41, [0x41; 32], [0x40; 32]),
                transactions: vec![transaction(
                    1,
                    with_events([two_steps([1; 32], vec![SCALAR])]),
                )],
            },
            PreparedBlock {
                block: sealed(42, [0x42; 32], [0x41; 32]),
                transactions: vec![transaction(2, tampered)],
            },
            PreparedBlock {
                block: sealed(43, [0x43; 32], [0x42; 32]),
                transactions: vec![transaction(
                    3,
                    with_events([stored_execution()]),
                )],
            },
        ];

        let (_reference_instance, reference) = new_db().await;
        for block in &blocks {
            apply_block(&reference, &config(), block).await.unwrap();
        }
        let expected =
            rows_without_chain_topology(&reference.pool().await).await;
        assert_single_run_rows(&reference.pool().await, &expected).await;

        let all = 0..blocks.len();
        let schedules: [Vec<(usize, usize)>; 2] = [
            all.clone()
                .map(|i| (0, i))
                .chain(all.map(|i| (1, i)))
                .collect(),
            [(0, 0), (1, 0), (1, 1), (1, 2), (0, 1), (0, 2)].into(),
        ];
        for schedule in schedules {
            let (instance, first) = new_db().await;
            let second = Database::new(
                &instance.db_url,
                ChainId::from_canonical_u64(config().chain_id),
                100,
            )
            .await
            .unwrap();
            let replicas = [&first, &second];
            let pool = first.pool().await;
            let mut highest = 0;
            for (replica, index) in schedule {
                apply_block(replicas[replica], &config(), &blocks[index])
                    .await
                    .unwrap();
                let slot = load_checkpoint(&pool).await.unwrap().unwrap().slot;
                assert!(
                    slot >= highest,
                    "checkpoint moved back from {highest} to {slot}"
                );
                highest = slot;
            }
            assert_single_run_rows(&pool, &expected).await;
        }

        let (instance, first) = new_db().await;
        let second = Database::new(
            &instance.db_url,
            ChainId::from_canonical_u64(config().chain_id),
            100,
        )
        .await
        .unwrap();
        let config = config();
        for block in &blocks {
            let (a, b) = tokio::join!(
                apply_block(&first, &config, block),
                apply_block(&second, &config, block)
            );
            a.unwrap();
            b.unwrap();
        }
        assert_single_run_rows(&first.pool().await, &expected).await;
    }
}
