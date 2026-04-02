use crate::{
    config::{PollerConfig, ResolvedPollerConfig},
    database::{ComputationRow, Database, DelegationRow},
    events::decode_host_event_logs,
    health_check::run_health_server,
    rpc::{ConfirmedTransaction, SolanaRpcClient},
};
use anyhow::{bail, Result};
use fhevm_engine_common::types::{AllowEvents, SupportedFheOperations};
use sha3::{Digest, Keccak256};
use solana_host_contracts_core::{FheType, HostEvent, Operator};
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use tokio::task::JoinHandle;
use tracing::{info, warn};

const MAX_LIVE_EMPTY_BLOCK_RANGE: u64 = 8;

#[derive(Debug)]
pub struct SolanaHostPoller {
    config: ResolvedPollerConfig,
    rpc: SolanaRpcClient,
    db: Database,
    caught_up_once: AtomicBool,
}

#[derive(Debug, Default)]
struct EventInsertResult {
    inserted: usize,
    dependency_candidates: Vec<Vec<u8>>,
    created_computation: bool,
}

impl SolanaHostPoller {
    pub async fn new(config: ResolvedPollerConfig) -> Result<Self> {
        let rpc = SolanaRpcClient::new(config.rpc_url.clone());
        let db = Database::new(&config.database_url, config.host_chain_id).await?;
        Ok(Self {
            config,
            rpc,
            db,
            caught_up_once: AtomicBool::new(false),
        })
    }

    pub async fn run(self) -> Result<()> {
        info!(
            rpc_url = %self.config.rpc_url,
            program_id = %self.config.program_id,
            host_chain_id = self.config.host_chain_id,
            batch_size_slots = self.config.batch_size_slots,
            "solana host listener started"
        );

        if self.config.once {
            let inserted = self.poll_once().await?;
            if inserted > 0 {
                info!(inserted, "solana host listener ingested new rows");
            }
            return Ok(());
        }

        let _health_handle = spawn_health_server(self.config.health_port);
        let mut last_observed_slot = None;

        loop {
            let poll_result = async {
                let current_slot = self.rpc.get_slot(&self.config.commitment).await?;
                if last_observed_slot != Some(current_slot) {
                    info!(
                        current_slot,
                        commitment = %self.config.commitment,
                        "solana host listener observed new slot"
                    );
                    last_observed_slot = Some(current_slot);
                }
                self.poll_once().await
            }
            .await;

            match poll_result {
                Ok(inserted) => {
                    if inserted > 0 {
                        info!(inserted, "solana host listener ingested new rows");
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        self.config.poll_interval_ms,
                    ))
                    .await;
                }
                Err(err) => {
                    warn!(error = %err, "solana host listener poll failed");
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        self.config.retry_interval_ms,
                    ))
                    .await;
                }
            }
        }
    }

    pub async fn poll_once(&self) -> Result<usize> {
        let last_caught_up_block = self.db.get_last_caught_up_block().await?.unwrap_or(0);
        let current_slot = self.rpc.get_slot(&self.config.commitment).await?;
        let from_slot = (last_caught_up_block + 1).max(0) as u64;
        let batch_upper_bound = from_slot
            .saturating_add(self.config.batch_size_slots.saturating_sub(1));
        let to_slot = current_slot.min(batch_upper_bound);
        let reaches_tip = from_slot <= to_slot && to_slot == current_slot;
        let slot_gap = to_slot.saturating_sub(from_slot);
        let finalized_slot = if self.config.commitment != "finalized" {
            Some(self.rpc.get_slot("finalized").await?)
        } else {
            None
        };

        if from_slot <= to_slot {
            let live_blocks_finalized = self.config.commitment == "finalized";
            // Mirror the EVM listener semantics:
            // - historical catch-up remains sparse
            // - empty blocks are recorded only once the listener is already at tip
            // A small near-tip gap is treated as live mode; larger gaps remain catch-up even
            // if the process itself has already caught up before.
            if self.caught_up_once.load(Ordering::Relaxed)
                && reaches_tip
                && slot_gap <= MAX_LIVE_EMPTY_BLOCK_RANGE
            {
                self.record_block_range(
                    from_slot,
                    to_slot,
                    &self.config.commitment,
                    live_blocks_finalized,
                )
                .await?;
            }

            if let Some(finalized_slot) = finalized_slot {
                let finalize_to = finalized_slot.min(to_slot);
                if from_slot <= finalize_to {
                    let block_numbers = self
                        .db
                        .get_pending_blocks_to_finalize(finalize_to)
                        .await?;
                    for block_number in block_numbers {
                        let Some(block_info) = self
                            .rpc
                            .get_block(block_number as u64, "finalized")
                            .await?
                        else {
                            continue;
                        };
                        let mut tx = self.db.begin().await?;
                        self.db
                            .mark_block_as_valid(
                                &mut tx,
                                block_info.slot,
                                &block_info.blockhash,
                                true,
                            )
                            .await?;
                        tx.commit().await?;
                    }
                }
            }
        }

        let mut signatures = self
            .rpc
            .get_signatures_for_address(
                &self.config.program_id,
                self.config.batch_size_slots as usize,
                &self.config.commitment,
            )
            .await?;
        signatures.retain(|entry| {
            entry.slot as i64 > last_caught_up_block && entry.slot <= to_slot
        });
        signatures.sort_by_key(|entry| (entry.slot, entry.signature.clone()));

        if signatures.is_empty() {
            if from_slot <= to_slot {
                self.db.set_last_caught_up_block(to_slot).await?;
            }
            return Ok(0);
        }

        let max_slot = signatures.iter().map(|entry| entry.slot).max();
        let signature_strings = signatures
            .into_iter()
            .map(|entry| entry.signature)
            .collect::<Vec<_>>();
        if let Some(max_slot) = max_slot {
            info!(
                from_slot = last_caught_up_block + 1,
                to_slot = max_slot as i64,
                signatures = signature_strings.len(),
                "solana host listener observed new host-program activity"
            );
        }
        let inserted = self
            .ingest_signatures(&signature_strings, finalized_slot)
            .await?;
        if from_slot <= to_slot {
            self.db.set_last_caught_up_block(to_slot).await?;
        } else if let Some(max_slot) = max_slot {
            self.db.set_last_caught_up_block(max_slot).await?;
        }
        if reaches_tip {
            self.caught_up_once.store(true, Ordering::Relaxed);
        }
        Ok(inserted)
    }

    pub async fn ingest_signatures(
        &self,
        signatures: &[String],
        finalized_slot: Option<u64>,
    ) -> Result<usize> {
        let mut inserted = 0usize;
        let mut max_slot = None;

        for (tx_index, signature) in signatures.iter().enumerate() {
            let Some(transaction) = self
                .rpc
                .get_transaction(signature, &self.config.commitment)
                .await?
            else {
                continue;
            };
            max_slot = Some(max_slot.unwrap_or(transaction.slot).max(transaction.slot));
            let finalized = finalized_slot.is_none_or(|slot| transaction.slot <= slot);
            inserted += self
                .ingest_transaction(transaction, tx_index, finalized)
                .await?;
        }

        if let Some(slot) = max_slot {
            self.db.set_last_caught_up_block(slot).await?;
        }

        Ok(inserted)
    }

    async fn ingest_transaction(
        &self,
        transaction: ConfirmedTransaction,
        tx_index: usize,
        finalized: bool,
    ) -> Result<usize> {
        let decoded = decode_host_event_logs(transaction.log_messages.iter().map(String::as_str))?;
        if decoded.is_empty() {
            return Ok(0);
        }

        info!(
            signature = %transaction.signature,
            slot = transaction.slot,
            event_count = decoded.len(),
            "solana host listener decoding host-program transaction"
        );

        let transaction_id = bs58::decode(&transaction.signature).into_vec()?;
        let dependence_chain_id = keccak_bytes(&transaction_id);
        let mut tx = self.db.begin().await?;
        self.db
            .mark_block_as_valid(&mut tx, transaction.slot, &transaction.blockhash, finalized)
            .await?;

        let mut inserted = 0usize;
        let mut dependency_candidates = BTreeSet::new();
        let mut created_computation = false;
        for decoded_event in decoded {
            let schedule_order =
                schedule_order_for(transaction.slot, tx_index, decoded_event.log_index);
            let event_result = self
                .ingest_event(
                    &mut tx,
                    decoded_event.event,
                    &transaction_id,
                    &dependence_chain_id,
                    &transaction.blockhash,
                    transaction.slot,
                    schedule_order,
                )
                .await?;
            inserted += event_result.inserted;
            created_computation |= event_result.created_computation;
            for dependency in event_result.dependency_candidates {
                dependency_candidates.insert(dependency);
            }
        }

        if created_computation {
            let dependency_candidates = dependency_candidates.into_iter().collect::<Vec<_>>();
            let dependency_chain_ids = self
                .db
                .lookup_dependency_chain_ids(&mut tx, &dependency_candidates, &dependence_chain_id)
                .await?;
            let dependence_chain_schedule = schedule_order_for(transaction.slot, tx_index, 0);
            self.db
                .upsert_dependence_chain(
                    &mut tx,
                    &dependence_chain_id,
                    dependence_chain_schedule,
                    dependency_chain_ids.len(),
                    &transaction.blockhash,
                    transaction.slot,
                )
                .await?;
            self.db
                .append_dependents(&mut tx, &dependency_chain_ids, &dependence_chain_id)
                .await?;
        }

        tx.commit().await?;
        info!(
            signature = %transaction.signature,
            slot = transaction.slot,
            inserted_rows = inserted,
            "solana host listener committed host-program transaction"
        );
        Ok(inserted)
    }

    async fn record_block_range(
        &self,
        from_slot: u64,
        to_slot: u64,
        commitment: &str,
        finalized: bool,
    ) -> Result<()> {
        let block_slots = self.rpc.get_blocks(from_slot, to_slot, commitment).await?;
        if block_slots.is_empty() {
            return Ok(());
        }

        let mut tx = self.db.begin().await?;
        for slot in block_slots {
            let Some(block) = self.rpc.get_block(slot, commitment).await? else {
                continue;
            };
            self.db
                .mark_block_as_valid(&mut tx, block.slot, &block.blockhash, finalized)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn ingest_event(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        event: HostEvent,
        transaction_id: &[u8],
        dependence_chain_id: &[u8],
        block_hash: &[u8],
        block_number: u64,
        schedule_order: PrimitiveDateTime,
    ) -> Result<EventInsertResult> {
        match event {
            HostEvent::Operation {
                op,
                operands,
                scalar_flag,
                result_type,
                result,
                ..
            } => {
                let Some(fhe_operation) = map_operation(op) else {
                    warn!(
                        ?op,
                        "skipping unsupported Solana operation for current coprocessor milestone"
                    );
                    return Ok(EventInsertResult::default());
                };
                let (dependencies, is_scalar) =
                    map_dependencies(op, operands, scalar_flag, result_type)?;
                let inserted = self
                    .db
                    .insert_computation(
                        tx,
                        &ComputationRow {
                            output_handle: result.as_bytes().to_vec(),
                            dependencies: dependencies.clone(),
                            fhe_operation,
                            is_scalar,
                            transaction_id: transaction_id.to_vec(),
                            dependence_chain_id: dependence_chain_id.to_vec(),
                            is_allowed: false,
                            schedule_order,
                            block_number,
                        },
                    )
                    .await? as usize;
                Ok(EventInsertResult {
                    inserted,
                    dependency_candidates: dependencies,
                    created_computation: true,
                })
            }
            HostEvent::Allowed {
                account, handle, ..
            } => {
                let account_address = bs58::encode(account.as_bytes()).into_string();
                let mut inserted = 0usize;
                inserted += self
                    .db
                    .insert_allowed_handle(
                        tx,
                        handle.as_bytes(),
                        &account_address,
                        AllowEvents::AllowedAccount,
                        transaction_id,
                        block_number,
                    )
                    .await? as usize;
                inserted += self
                    .db
                    .insert_pbs_computation(tx, handle.as_bytes(), transaction_id, block_number)
                    .await? as usize;
                Ok(EventInsertResult {
                    inserted,
                    ..EventInsertResult::default()
                })
            }
            HostEvent::AllowedMany {
                account, handles, ..
            } => {
                let account_address = bs58::encode(account.as_bytes()).into_string();
                let mut inserted = 0usize;
                for handle in handles {
                    inserted += self
                        .db
                        .insert_allowed_handle(
                            tx,
                            handle.as_bytes(),
                            &account_address,
                            AllowEvents::AllowedAccount,
                            transaction_id,
                            block_number,
                        )
                        .await? as usize;
                    inserted += self
                        .db
                        .insert_pbs_computation(tx, handle.as_bytes(), transaction_id, block_number)
                        .await? as usize;
                }
                Ok(EventInsertResult {
                    inserted,
                    ..EventInsertResult::default()
                })
            }
            HostEvent::AllowedForDecryption { handles, .. } => {
                let mut inserted = 0usize;
                for handle in handles {
                    inserted += self
                        .db
                        .insert_allowed_handle(
                            tx,
                            handle.as_bytes(),
                            "",
                            AllowEvents::AllowedForDecryption,
                            transaction_id,
                            block_number,
                        )
                        .await? as usize;
                    inserted += self
                        .db
                        .insert_pbs_computation(tx, handle.as_bytes(), transaction_id, block_number)
                        .await? as usize;
                }
                Ok(EventInsertResult {
                    inserted,
                    ..EventInsertResult::default()
                })
            }
            HostEvent::DelegatedForUserDecryption {
                delegator,
                delegate,
                contract_address,
                delegation_counter,
                old_expiration_date,
                new_expiration_date,
            } => {
                let inserted = self
                    .db
                    .insert_delegation(
                        tx,
                        &DelegationRow {
                            delegator: delegator.as_bytes().to_vec(),
                            delegate: delegate.as_bytes().to_vec(),
                            contract_address: contract_address.as_bytes().to_vec(),
                            delegation_counter,
                            old_expiration_date,
                            new_expiration_date,
                            block_number,
                            block_hash: block_hash.to_vec(),
                            transaction_id: transaction_id.to_vec(),
                        },
                    )
                    .await? as usize;
                Ok(inserted.into())
            }
            HostEvent::RevokedDelegationForUserDecryption {
                delegator,
                delegate,
                contract_address,
                delegation_counter,
                old_expiration_date,
            } => {
                let inserted = self
                    .db
                    .insert_delegation(
                        tx,
                        &DelegationRow {
                            delegator: delegator.as_bytes().to_vec(),
                            delegate: delegate.as_bytes().to_vec(),
                            contract_address: contract_address.as_bytes().to_vec(),
                            delegation_counter,
                            old_expiration_date,
                            new_expiration_date: 0,
                            block_number,
                            block_hash: block_hash.to_vec(),
                            transaction_id: transaction_id.to_vec(),
                        },
                    )
                    .await? as usize;
                Ok(inserted.into())
            }
            HostEvent::VerifyInput { .. } => {
                info!(
                    "ignoring Solana host VerifyInput event because input-proof materialization is sourced from the gateway InputVerification flow"
                );
                Ok(EventInsertResult::default())
            }
            HostEvent::BlockedAccount { .. }
            | HostEvent::UnblockedAccount { .. }
            | HostEvent::InputVerifierContextUpdated { .. }
            | HostEvent::KmsContextUpdated { .. }
            | HostEvent::KmsContextDestroyed { .. }
            | HostEvent::HcuPerBlockSet { .. }
            | HostEvent::MaxHcuDepthPerTxSet { .. }
            | HostEvent::MaxHcuPerTxSet { .. }
            | HostEvent::BlockHcuWhitelistAdded { .. }
            | HostEvent::BlockHcuWhitelistRemoved { .. } => Ok(EventInsertResult::default()),
        }
    }
}

impl From<usize> for EventInsertResult {
    fn from(inserted: usize) -> Self {
        Self {
            inserted,
            ..Self::default()
        }
    }
}

pub async fn run_from_cli(config: PollerConfig) -> Result<()> {
    SolanaHostPoller::new(config.resolve()?).await?.run().await
}

fn map_operation(op: Operator) -> Option<i16> {
    let mapped = match op {
        Operator::FheAdd => SupportedFheOperations::FheAdd,
        Operator::FheSub => SupportedFheOperations::FheSub,
        Operator::FheMul => SupportedFheOperations::FheMul,
        Operator::FheDiv => SupportedFheOperations::FheDiv,
        Operator::FheRem => SupportedFheOperations::FheRem,
        Operator::FheBitAnd => SupportedFheOperations::FheBitAnd,
        Operator::FheBitOr => SupportedFheOperations::FheBitOr,
        Operator::FheBitXor => SupportedFheOperations::FheBitXor,
        Operator::FheShl => SupportedFheOperations::FheShl,
        Operator::FheShr => SupportedFheOperations::FheShr,
        Operator::FheRotl => SupportedFheOperations::FheRotl,
        Operator::FheRotr => SupportedFheOperations::FheRotr,
        Operator::FheEq => SupportedFheOperations::FheEq,
        Operator::FheNe => SupportedFheOperations::FheNe,
        Operator::FheGe => SupportedFheOperations::FheGe,
        Operator::FheGt => SupportedFheOperations::FheGt,
        Operator::FheLe => SupportedFheOperations::FheLe,
        Operator::FheLt => SupportedFheOperations::FheLt,
        Operator::FheMin => SupportedFheOperations::FheMin,
        Operator::FheMax => SupportedFheOperations::FheMax,
        Operator::FheNeg => SupportedFheOperations::FheNeg,
        Operator::FheNot => SupportedFheOperations::FheNot,
        Operator::Cast => SupportedFheOperations::FheCast,
        Operator::TrivialEncrypt => SupportedFheOperations::FheTrivialEncrypt,
        Operator::FheIfThenElse => SupportedFheOperations::FheIfThenElse,
        Operator::FheRand => SupportedFheOperations::FheRand,
        Operator::FheRandBounded => SupportedFheOperations::FheRandBounded,
        Operator::VerifyInput => return None,
    };
    Some(mapped as i16)
}

fn map_dependencies(
    op: Operator,
    operands: Vec<[u8; 32]>,
    scalar_flag: Option<u8>,
    result_type: FheType,
) -> Result<(Vec<Vec<u8>>, bool)> {
    let result = match op {
        Operator::Cast => {
            let input = operands
                .first()
                .copied()
                .ok_or_else(|| anyhow::anyhow!("cast missing operand"))?;
            (vec![input.to_vec(), vec![result_type as u8]], true)
        }
        Operator::TrivialEncrypt => {
            let plaintext = operands
                .first()
                .copied()
                .ok_or_else(|| anyhow::anyhow!("trivial encrypt missing plaintext"))?;
            (vec![plaintext.to_vec(), vec![result_type as u8]], true)
        }
        Operator::FheRand => {
            let seed = operands
                .first()
                .copied()
                .ok_or_else(|| anyhow::anyhow!("rand missing seed"))?;
            (vec![seed.to_vec(), vec![result_type as u8]], true)
        }
        Operator::FheRandBounded => {
            if operands.len() < 2 {
                bail!("rand bounded missing operands");
            }
            let upper_bound = operands[0];
            let seed = operands[1];
            (
                vec![seed.to_vec(), upper_bound.to_vec(), vec![result_type as u8]],
                true,
            )
        }
        _ => {
            let dependencies = operands
                .into_iter()
                .map(|operand| operand.to_vec())
                .collect::<Vec<_>>();
            let is_scalar = scalar_flag == Some(1);
            (dependencies, is_scalar)
        }
    };
    Ok(result)
}

fn schedule_order_for(slot: u64, transaction_index: usize, log_index: usize) -> PrimitiveDateTime {
    let base =
        OffsetDateTime::from_unix_timestamp(slot as i64).unwrap_or(OffsetDateTime::UNIX_EPOCH);
    let with_offsets = base
        + Duration::milliseconds(transaction_index as i64)
        + Duration::microseconds(log_index as i64);
    PrimitiveDateTime::new(with_offsets.date(), with_offsets.time())
}

fn keccak_bytes(input: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

fn spawn_health_server(port: u16) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(err) = run_health_server(port).await {
            tracing::warn!(port, error = %err, "health server stopped");
        }
    })
}
