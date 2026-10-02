//! The coprocessor's sink for the Solana host follower: reconstructs each sealed block's
//! coprocessor work and ingests it into the coprocessor database.
//!
//! - **Version pairing.** Handle re-derivation uses the program crate's `computed_*` functions
//!   (INVARIANTS #28) and hashes the followed `--program-id`, not the crate's compiled
//!   `declare_id!`. Instruction layout still has no runtime handshake: deploy the listener from
//!   the same rev as the program (INVARIANTS #33).
//!
//! Each sealed block is applied in one database transaction: its compute rows and the
//! resume checkpoint. Compute rows carry the result handles each `FheExecutedEvent`
//! emitted, so they name the chain's handles even where re-derivation disagrees.
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

use crate::database::solana_checkpoint::store_checkpoint;
use crate::database::tfhe_event_propagate::{Database, TransactionId};
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
/// transaction's compute rows and the checkpoint.
async fn apply_block(
    db: &Database,
    config: &SolanaListenerConfig,
    prepared: &PreparedBlock,
) -> std::result::Result<(), IngestFailure> {
    let sealed_block = &prepared.block;
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
        let block_timestamp =
            sealed_block_timestamp(sealed_block).ok_or_else(|| {
                IngestFailure::fatal(anyhow!(
                    "missing or invalid block time for slot {}",
                    sealed_block.slot
                ))
            })?;
        let block = SolanaBlockMeta {
            block_number: sealed_block.slot,
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
            tfhe_events = stats.tfhe_events,
            material_requests = stats.material_requests,
            inserted_records = stats.inserted_records,
            "ingested Solana host records (gRPC)"
        );
    }
    Ok(())
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
        decode_fhe_execute_args, decode_fhe_executed_event, DecodedInstruction,
    };
    use solana_host_follower::{
        PreparedBlock, PreparedTransaction, SealedBlock,
    };
    use solana_sdk::signature::Signature;
    use sqlx::Row;
    use test_harness::instance::{setup_test_db, ImportMode};
    use zama_host::state::{
        FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteStep,
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

    /// Stores step `step_index`'s result into [`STATE`], allowing the dictionary key at
    /// `key_index`: one historical-access leaf.
    fn storing(
        mut instruction: DecodedInstruction,
        step_index: u8,
        key_index: u8,
        previous_leaf_count: u64,
    ) -> DecodedInstruction {
        let mut args = decode_fhe_execute_args(&instruction.data).unwrap();
        args.account_count = 1;
        args.effects = vec![zama_host::FheExecuteEffect {
            result: zama_host::ExecutionResultRef {
                step_index,
                output_index: 0,
            },
            store_index: 0,
            previous_leaf_count,
            slot: None,
            allow_indexes: vec![key_index],
            make_public: false,
            grants: vec![],
        }];
        instruction.data = encoded_execution(args);
        instruction.accounts =
            vec![[0; 32]; zama_host::FHE_EXECUTE_FIXED_ACCOUNTS];
        instruction.accounts.push(STATE);
        instruction
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

    /// The repair: rewind the checkpoint and revert the rows past a slot with the operator
    /// scripts, then replay. The consumer the worker drained comes back as a fresh row and
    /// the checkpoint returns to the tip.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial(handle_check_failures)]
    async fn a_reverted_slot_replays_with_fresh_rows() {
        let key = [0x33; 32];
        let slot_41 = PreparedBlock {
            block: sealed(41, [0x41; 32], [0x40; 32]),
            transactions: vec![PreparedTransaction {
                signature: Signature::from([1; 64]),
                index: 0,
                instructions: with_events([storing(
                    execution(
                        vec![FheExecuteStep::TrivialEncrypt {
                            plaintext: [1; 32],
                            fhe_type: 5,
                        }],
                        vec![key],
                    ),
                    0,
                    0,
                    0,
                )]),
            }],
        };
        let mut tampered = with_events([storing(
            two_steps([2; 32], vec![SCALAR, key]),
            1,
            1,
            1,
        )]);
        let (consumer, _) = tamper(&mut tampered);
        let slot_42 = PreparedBlock {
            block: sealed(42, [0x42; 32], [0x41; 32]),
            transactions: vec![PreparedTransaction {
                signature: Signature::from([2; 64]),
                index: 0,
                instructions: tampered,
            }],
        };

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let chain_id = ChainId::from_canonical_u64(config().chain_id);
        let db = Database::new(&instance.db_url, chain_id, 100)
            .await
            .unwrap();
        let pool = db.pool().await;
        for block in [&slot_41, &slot_42] {
            apply_block(&db, &config(), block).await.unwrap();
        }
        // The worker drained the consumer of the held step.
        sqlx::query("UPDATE computations SET is_error = true, error_message = 'drained' WHERE output_handle = $1")
            .bind(consumer.to_vec())
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'solana', '') ON CONFLICT DO NOTHING")
            .bind(chain_id.as_i64())
            .execute(&pool)
            .await
            .unwrap();
        let revert = test_harness::db_utils::revert_coprocessor_db_state_sql(
            chain_id.as_i64(),
            41,
        );
        // A refused script leaves its session in the aborted transaction, as psql would
        // before exiting, so it gets a connection of its own.
        let mut session: sqlx::PgConnection =
            sqlx::Connection::connect(instance.db_url()).await.unwrap();
        let refused = sqlx::raw_sql(&revert)
            .execute(&mut session)
            .await
            .unwrap_err();
        drop(session);
        assert!(
            refused
                .to_string()
                .contains("rewind_solana_listener_checkpoint.sql"),
            "{refused}"
        );
        let rewind = include_str!(
            "../../db-migration/db-scripts/rewind_solana_listener_checkpoint.sql"
        )
        .lines()
        .filter(|line| !line.starts_with("\\set "))
        .collect::<Vec<_>>()
        .join("\n")
        .replace(":'slot'", "41")
        .replace(":'block_hash'", &format!("'{}'", hex::encode([0x41; 32])));
        sqlx::raw_sql(&rewind).execute(&pool).await.unwrap();
        sqlx::raw_sql(&revert).execute(&pool).await.unwrap();
        let checkpoint = load_checkpoint(&pool).await.unwrap().unwrap();
        assert_eq!((checkpoint.slot, checkpoint.block_hash), (41, [0x41; 32]));
        let reverted: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM computations WHERE block_number = 42",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(reverted, 0);

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
        assert_eq!(load_checkpoint(&pool).await.unwrap().unwrap().slot, 42);
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
        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let db = Database::new(
            &instance.db_url,
            ChainId::from_canonical_u64(config().chain_id),
            100,
        )
        .await
        .unwrap();
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
}
