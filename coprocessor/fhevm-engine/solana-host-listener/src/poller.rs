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
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use tokio::task::JoinHandle;
use tracing::{info, warn};

#[derive(Debug)]
pub struct SolanaHostPoller {
    config: ResolvedPollerConfig,
    rpc: SolanaRpcClient,
    db: Database,
}

impl SolanaHostPoller {
    pub async fn new(config: ResolvedPollerConfig) -> Result<Self> {
        let rpc = SolanaRpcClient::new(config.rpc_url.clone());
        let db = Database::new(&config.database_url, config.host_chain_id).await?;
        Ok(Self { config, rpc, db })
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

        loop {
            match self.poll_once().await {
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
        let mut signatures = self
            .rpc
            .get_signatures_for_address(
                &self.config.program_id,
                self.config.batch_size_slots as usize,
                &self.config.commitment,
            )
            .await?;
        signatures.retain(|entry| entry.slot as i64 > last_caught_up_block);
        signatures.sort_by_key(|entry| (entry.slot, entry.signature.clone()));

        if signatures.is_empty() {
            return Ok(0);
        }

        let max_slot = signatures.iter().map(|entry| entry.slot).max();
        let signature_strings = signatures
            .into_iter()
            .map(|entry| entry.signature)
            .collect::<Vec<_>>();
        let inserted = self.ingest_signatures(&signature_strings).await?;
        if let Some(max_slot) = max_slot {
            self.db.set_last_caught_up_block(max_slot).await?;
        }
        Ok(inserted)
    }

    pub async fn ingest_signatures(&self, signatures: &[String]) -> Result<usize> {
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
            inserted += self.ingest_transaction(transaction, tx_index).await?;
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
    ) -> Result<usize> {
        let decoded = decode_host_event_logs(transaction.log_messages.iter().map(String::as_str))?;
        if decoded.is_empty() {
            return Ok(0);
        }

        let transaction_id = bs58::decode(&transaction.signature).into_vec()?;
        let dependence_chain_id = keccak_bytes(&transaction_id);
        let mut tx = self.db.begin().await?;
        self.db
            .mark_block_finalized(&mut tx, transaction.slot, &transaction.recent_blockhash)
            .await?;

        let mut inserted = 0usize;
        for decoded_event in decoded {
            let schedule_order =
                schedule_order_for(transaction.slot, tx_index, decoded_event.log_index);
            inserted += self
                .ingest_event(
                    &mut tx,
                    decoded_event.event,
                    &transaction_id,
                    &dependence_chain_id,
                    &transaction.recent_blockhash,
                    transaction.slot,
                    schedule_order,
                )
                .await?;
        }

        tx.commit().await?;
        Ok(inserted)
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
    ) -> Result<usize> {
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
                    return Ok(0);
                };
                let (dependencies, is_scalar) =
                    map_dependencies(op, operands, scalar_flag, result_type)?;
                Ok(self
                    .db
                    .insert_computation(
                        tx,
                        &ComputationRow {
                            output_handle: result.as_bytes().to_vec(),
                            dependencies,
                            fhe_operation,
                            is_scalar,
                            transaction_id: transaction_id.to_vec(),
                            dependence_chain_id: dependence_chain_id.to_vec(),
                            is_allowed: false,
                            schedule_order,
                            block_number,
                        },
                    )
                    .await? as usize)
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
                Ok(inserted)
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
                Ok(inserted)
            }
            HostEvent::DelegatedForUserDecryption {
                delegator,
                delegate,
                contract_address,
                delegation_counter,
                old_expiration_date,
                new_expiration_date,
            } => Ok(self
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
                .await? as usize),
            HostEvent::RevokedDelegationForUserDecryption {
                delegator,
                delegate,
                contract_address,
                delegation_counter,
                old_expiration_date,
            } => Ok(self
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
                .await? as usize),
            HostEvent::VerifyInput { .. } => {
                warn!("VerifyInput remains unsupported in the Solana coprocessor e2e milestone");
                Ok(0)
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
            | HostEvent::BlockHcuWhitelistRemoved { .. } => Ok(0),
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
