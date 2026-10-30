use super::common::{
    is_retryable_nonce_error, is_transient_gateway_error, try_extract_non_retryable_config_error,
};
use super::state::{Attempt, Status, Task};
use super::{begin_live_write, TransactionOperation};
use crate::metrics::{VERIFY_PROOF_FAIL_COUNTER, VERIFY_PROOF_SUCCESS_COUNTER};
use crate::nonce_managed_provider::NonceManagedProvider;
use crate::AbstractSigner;
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::sol;
use alloy::{network::Ethereum, primitives::FixedBytes, sol_types::SolStruct};
use async_trait::async_trait;
use fhevm_engine_common::synthetic_input::SYNTHETIC_ZK_PROOF_ID_BASE;
use fhevm_engine_common::telemetry;
use sqlx::{Pool, Postgres};
use std::convert::TryInto;
use std::sync::Arc;
use tokio::{sync::Semaphore, task::JoinSet};
use tracing::{debug, error, info, warn, Instrument};

use fhevm_gateway_bindings::input_verification::InputVerification;
use fhevm_gateway_bindings::input_verification::InputVerification::InputVerificationErrors;

sol! {
    struct CiphertextVerification {
        bytes32[] ctHandles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
        bytes extraData;
    }
}

struct ProofTask {
    zk_proof_id: i64,
    chain_id: i64,
    contract_address: String,
    user_address: String,
    handles: Option<Vec<u8>>,
    verified: Option<bool>,
    retry_count: i32,
    extra_data: Vec<u8>,
    transaction_id: Option<Vec<u8>>,
    sender_attempt_no: i64,
}

#[derive(Clone)]
pub(crate) struct VerifyProofOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    input_verification_address: Address,
    provider: NonceManagedProvider<P>,
    signer: AbstractSigner,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    gw_chain_id: u64,
    db_pool: Pool<Postgres>,
    submission_slots: Arc<Semaphore>,
}

impl<P> VerifyProofOperation<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    pub(crate) async fn new(
        input_verification_address: Address,
        provider: NonceManagedProvider<P>,
        signer: AbstractSigner,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
        submission_slots: Arc<Semaphore>,
    ) -> anyhow::Result<Self> {
        let gw_chain_id = provider.get_chain_id().await?;
        Ok(Self {
            input_verification_address,
            provider,
            signer,
            conf,
            gas,
            gw_chain_id,
            db_pool,
            submission_slots,
        })
    }

    async fn finish_proof(
        &self,
        attempt: &Attempt,
        status: Status,
        reason: Option<&str>,
    ) -> anyhow::Result<()> {
        let Some(mut tx) = attempt.transition(&self.db_pool, status, reason, 0).await? else {
            return Ok(());
        };
        if matches!(status, Status::Completed) {
            let Task::Proof(id) = attempt.task else {
                unreachable!()
            };
            // Keep rejected proofs for the existing Gateway-finalization/replay lifecycle.
            sqlx::query!("UPDATE verify_proofs SET handles = CASE WHEN verified = FALSE THEN NULL ELSE handles END
                          WHERE zk_proof_id = $1", id).execute(tx.as_mut()).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn update_retry_count_by_proof_id(
        &self,
        attempt: &Attempt,
        current_retry_count: i32,
        error: &str,
    ) -> anyhow::Result<()> {
        let Task::Proof(zk_proof_id) = attempt.task else {
            unreachable!()
        };
        let status = if current_retry_count + 1 >= self.conf.verify_proof_resp_max_retries as i32 {
            Status::Stopped
        } else {
            Status::RetryWait
        };
        let Some(mut tx) = attempt
            .transition(
                &self.db_pool,
                status,
                Some(error),
                self.conf.error_sleep_max_secs,
            )
            .await?
        else {
            return Ok(());
        };
        if current_retry_count == (self.conf.verify_proof_resp_max_retries as i32) - 1 {
            error!(zk_proof_id = zk_proof_id, "Max retries reached for proof");
        }
        debug!(zk_proof_id = zk_proof_id, "Updating retry count of proof");
        sqlx::query!(
            "UPDATE verify_proofs
            SET
                retry_count = retry_count + 1,
                last_error = $2,
                last_retry_at = NOW()
            WHERE zk_proof_id = $1",
            zk_proof_id,
            error
        )
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Keep exhausted jobs inspectable instead of deleting their failure evidence.
    ///
    /// Skips synthetic blue-green rows (`zk_proof_id >= SYNTHETIC_ZK_PROOF_ID_BASE`): they are
    /// never sent, so their `retry_count` never advances, but the guard also keeps this path
    /// from racing the zkproof-worker for a row cutover cleanup owns.
    async fn stop_exhausted_and_clean_completed_proofs(&self) -> anyhow::Result<()> {
        let Some(mut tx) = begin_live_write(&self.db_pool).await? else {
            return Ok(());
        };
        debug!(
            max_retries = self.conf.verify_proof_resp_max_retries,
            "Stopping proofs with retry count >= max_retries"
        );
        sqlx::query!(
            "UPDATE verify_proofs SET sender_status = 'STOPPED', sender_status_updated_at = NOW(),
                 sender_next_action_at = NULL, sender_last_error = COALESCE(last_error, 'retry budget exhausted')
             WHERE retry_count >= $1 AND sender_status IN ('QUEUED', 'RETRY_WAIT')
               AND (verified = TRUE OR (verified = FALSE AND handles IS NOT NULL))
               AND sender_raw_tx IS NULL AND zk_proof_id < $2",
            self.conf.verify_proof_resp_max_retries as i64,
            SYNTHETIC_ZK_PROOF_ID_BASE
        )
        .execute(tx.as_mut())
        .await?;
        sqlx::query!(
            "DELETE FROM verify_proofs
            WHERE sender_status = 'COMPLETED' AND verified = TRUE AND sender_raw_tx IS NULL
              AND sender_status_updated_at < NOW() - INTERVAL '1 day'"
        )
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn prepare_response(
        &self,
        row: &ProofTask,
        attempt: &Attempt,
    ) -> anyhow::Result<Option<(TransactionRequest, bool)>> {
        let input_verification =
            InputVerification::new(self.input_verification_address, self.provider.inner());
        let transaction_id = row.transaction_id.clone();
        let span = tracing::info_span!("prepare_verify_proof_resp", txn_id = tracing::field::Empty);
        telemetry::record_short_hex_if_some(&span, "txn_id", transaction_id.as_deref());

        let locally_verified = if let Some(raw) = &attempt.raw {
            use alloy::consensus::{Transaction, TxEnvelope};
            use alloy::eips::eip2718::Decodable2718;
            use alloy::sol_types::SolCall;
            let envelope = TxEnvelope::decode_2718(&mut raw.as_slice())?;
            envelope
                .input()
                .starts_with(&InputVerification::verifyProofResponseCall::SELECTOR)
        } else {
            row.verified.expect("selected proof has a local result")
        };
        let txn_request = if attempt.raw.is_some() {
            (row.zk_proof_id, TransactionRequest::default())
        } else {
            match row.verified {
                Some(true) => {
                    info!(parent: &span, zk_proof_id = row.zk_proof_id, "Processing verified proof");
                    let Some(handles) = row.handles.as_deref() else {
                        self.finish_proof(attempt, Status::Stopped, Some("missing proof handles"))
                            .await?;
                        return Ok(None);
                    };
                    if handles.len() % 32 != 0 {
                        error!(parent: &span,
                            handles_len = handles.len(),
                            "Bad handles field, len is not divisible by 32"
                        );
                        self.finish_proof(attempt, Status::Stopped, Some("invalid proof handles"))
                            .instrument(span.clone())
                            .await?;
                        return Ok(None);
                    }
                    let handles: Vec<FixedBytes<32>> = handles
                        .chunks(32)
                        .map(|chunk| {
                            let array: [u8; 32] = chunk.try_into().expect("chunk size must be 32");
                            FixedBytes(array)
                        })
                        .collect();
                    let domain = alloy::sol_types::eip712_domain! {
                        name: "InputVerification",
                        version: "1",
                        chain_id: self.gw_chain_id,
                        verifying_contract: self.input_verification_address,
                    };
                    let (Ok(user_address), Ok(contract_address)) = (
                        row.user_address.parse::<Address>(),
                        row.contract_address.parse::<Address>(),
                    ) else {
                        self.finish_proof(
                            attempt,
                            Status::Stopped,
                            Some("invalid proof addresses"),
                        )
                        .await?;
                        return Ok(None);
                    };
                    let signing_hash = CiphertextVerification {
                        ctHandles: handles.clone(),
                        userAddress: user_address,
                        contractAddress: contract_address,
                        contractChainId: U256::from(row.chain_id),
                        extraData: row.extra_data.clone().into(),
                    }
                    .eip712_signing_hash(&domain);
                    let signature = self
                        .signer
                        .sign_hash(&signing_hash)
                        .instrument(span.clone())
                        .await?;

                    if let Some(gas) = self.gas {
                        (
                            row.zk_proof_id,
                            input_verification
                                .verifyProofResponse(
                                    U256::from(row.zk_proof_id),
                                    handles,
                                    signature.as_bytes().into(),
                                    row.extra_data.clone().into(),
                                )
                                .into_transaction_request()
                                .with_gas_limit(gas),
                        )
                    } else {
                        (
                            row.zk_proof_id,
                            input_verification
                                .verifyProofResponse(
                                    U256::from(row.zk_proof_id),
                                    handles,
                                    signature.as_bytes().into(),
                                    row.extra_data.clone().into(),
                                )
                                .into_transaction_request(),
                        )
                    }
                }
                Some(false) => {
                    info!(parent: &span, zk_proof_id = row.zk_proof_id, "Processing rejected proof");
                    if let Some(gas) = self.gas {
                        (
                            row.zk_proof_id,
                            input_verification
                                .rejectProofResponse(
                                    U256::from(row.zk_proof_id),
                                    row.extra_data.clone().into(),
                                )
                                .into_transaction_request()
                                .with_gas_limit(gas),
                        )
                    } else {
                        (
                            row.zk_proof_id,
                            input_verification
                                .rejectProofResponse(
                                    U256::from(row.zk_proof_id),
                                    row.extra_data.clone().into(),
                                )
                                .into_transaction_request(),
                        )
                    }
                }
                None => {
                    error!(parent: &span,
                        zk_proof_id = row.zk_proof_id,
                        "verified field is unexpectedly None for proof"
                    );
                    return Ok(None);
                }
            }
        };
        Ok(Some((txn_request.1, locally_verified)))
    }

    #[tracing::instrument(name = "call_verify_proof_resp", skip_all, fields(txn_id = tracing::field::Empty))]
    async fn process_proof(
        &self,
        mut attempt: Attempt,
        txn_request: impl Into<TransactionRequest>,
        locally_verified: bool,
        current_retry_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let Task::Proof(zk_proof_id) = attempt.task else {
            unreachable!()
        };
        telemetry::record_short_hex_if_some(
            &tracing::Span::current(),
            "txn_id",
            src_transaction_id.as_deref(),
        );
        info!(zk_proof_id = zk_proof_id, "Processing transaction");

        // Wait outside the pool, retaining admission through bookkeeping so a
        // completed send can record its result before more work is admitted.
        let _permit = self.submission_slots.acquire().await?;
        let Some(mut submission_fence) = begin_live_write(&self.db_pool).await? else {
            return Ok(());
        };
        let submission = attempt
            .submit(
                &self.db_pool,
                &mut submission_fence,
                &self.provider,
                self.signer.clone(),
                txn_request.into(),
                &self.conf,
            )
            .await;
        submission_fence.rollback().await?;
        let receipt = match submission? {
            Ok(receipt) => receipt,
            Err(e) => {
                if let Some(InputVerificationErrors::CoprocessorAlreadyVerified(_)) =
                    e.as_error_resp().and_then(|payload| {
                        payload.as_decoded_interface_error::<InputVerificationErrors>()
                    })
                {
                    if locally_verified {
                        warn!(
                            zk_proof_id = zk_proof_id,
                            "Coprocessor has already verified the proof; marking publication complete"
                        );
                        self.finish_proof(&attempt, Status::Completed, None).await?;
                    } else {
                        warn!(
                            zk_proof_id = zk_proof_id,
                            "Coprocessor previously verified a locally unavailable proof; retaining it until Gateway finalization"
                        );
                        self.finish_proof(&attempt, Status::Completed, None).await?;
                    }
                    return Ok(());
                } else if let Some(InputVerificationErrors::CoprocessorAlreadyRejected(_)) =
                    e.as_error_resp().and_then(|payload| {
                        payload.as_decoded_interface_error::<InputVerificationErrors>()
                    })
                {
                    if locally_verified {
                        warn!(
                            zk_proof_id = zk_proof_id,
                            "Coprocessor previously rejected a proof now marked verified; recording conflict"
                        );
                        self.finish_proof(
                            &attempt,
                            Status::Stopped,
                            Some("CoprocessorAlreadyRejected conflicts with local verification"),
                        )
                        .await?;
                    } else {
                        warn!(
                            zk_proof_id = zk_proof_id,
                            "Coprocessor has already rejected the proof; retaining it until Gateway finalization"
                        );
                        self.finish_proof(&attempt, Status::Completed, None).await?;
                    }
                    return Ok(());
                } else if let Some(InputVerificationErrors::VerifyProofNotRequested(_)) =
                    e.as_error_resp().and_then(|payload| {
                        payload.as_decoded_interface_error::<InputVerificationErrors>()
                    })
                {
                    warn!(
                        zk_proof_id = zk_proof_id,
                        "Proof request is obsolete; stopping publication"
                    );
                    self.finish_proof(&attempt, Status::Stopped, Some("VerifyProofNotRequested"))
                        .await?;
                    return Ok(());
                } else if let Some(non_retryable_config_error) =
                    try_extract_non_retryable_config_error(&e)
                {
                    VERIFY_PROOF_FAIL_COUNTER.inc();
                    warn!(
                        zk_proof_id = zk_proof_id,
                        error = %non_retryable_config_error,
                        "Non-retryable gateway coprocessor config error while sending verify_proof transaction"
                    );
                    self.stop_retrying_verify_proof_on_config_error(
                        &attempt,
                        &non_retryable_config_error.to_string(),
                    )
                    .await?;
                    return Ok(());
                } else if is_transient_gateway_error(&e)
                    || is_retryable_nonce_error(&e)
                    || attempt.raw.is_some()
                {
                    VERIFY_PROOF_FAIL_COUNTER.inc();
                    warn!(
                        zk_proof_id = zk_proof_id,
                        error = %crate::diagnostics::safe_rpc_error(&e),
                        "Recoverable Gateway error; preserving proof retry budget"
                    );
                    // Keep the finite budget and last error intact, but persist
                    // the attempt so a failing batch cannot monopolize selection.
                    if let Some(mut tx) = attempt
                        .transition(
                            &self.db_pool,
                            attempt.retry_status(),
                            Some(&crate::diagnostics::safe_rpc_error(&e)),
                            self.conf.error_sleep_max_secs,
                        )
                        .await?
                    {
                        sqlx::query!(
                            "UPDATE verify_proofs SET last_retry_at = NOW() WHERE zk_proof_id = $1",
                            zk_proof_id
                        )
                        .execute(tx.as_mut())
                        .await?;
                        tx.commit().await?;
                    }
                    // Retain operation backoff and BackendGone shutdown handling.
                    return Err(anyhow::Error::new(e));
                } else {
                    VERIFY_PROOF_FAIL_COUNTER.inc();
                    error!(
                        zk_proof_id = zk_proof_id,
                        error = %crate::diagnostics::safe_rpc_error(&e),
                        "Transaction sending failed"
                    );
                    self.update_retry_count_by_proof_id(
                        &attempt,
                        current_retry_count,
                        &crate::diagnostics::safe_rpc_error(&e),
                    )
                    .await?;
                    return Err(anyhow::Error::new(e));
                }
            }
        };

        if receipt.status() {
            info!(
                zk_proof_id = zk_proof_id,
                transaction_hash = %receipt.transaction_hash,
                "Transaction succeeded"
            );
            self.finish_proof(&attempt, Status::Completed, None).await?;
            VERIFY_PROOF_SUCCESS_COUNTER.inc();

            telemetry::try_end_zkproof_transaction(
                &self.db_pool,
                &src_transaction_id.unwrap_or_default(),
            )
            .await?;
        } else {
            VERIFY_PROOF_FAIL_COUNTER.inc();
            error!(
                zk_proof_id = zk_proof_id,
                transaction_hash = %receipt.transaction_hash,
                status = receipt.status(),
                "Transaction failed"
            );
            self.update_retry_count_by_proof_id(
                &attempt,
                current_retry_count,
                "receipt status = false",
            )
            .await?;
            return Err(anyhow::anyhow!(
                "Transaction {} for zk_proof_id {} failed with status {}",
                receipt.transaction_hash,
                zk_proof_id,
                receipt.status(),
            ));
        }
        Ok(())
    }

    async fn stop_retrying_verify_proof_on_config_error(
        &self,
        attempt: &Attempt,
        error: &str,
    ) -> anyhow::Result<()> {
        let Task::Proof(zk_proof_id) = attempt.task else {
            unreachable!()
        };
        let Some(mut tx) = attempt
            .transition(&self.db_pool, Status::Stopped, Some(error), 0)
            .await?
        else {
            return Ok(());
        };
        // Preserve the existing counter convention while retaining the terminal job.
        sqlx::query!(
            "UPDATE verify_proofs
            SET
                retry_count = $2,
                last_error = $3,
                last_retry_at = NOW()
            WHERE zk_proof_id = $1",
            zk_proof_id,
            self.conf.verify_proof_resp_max_retries as i32,
            error,
        )
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl<P> TransactionOperation<P> for VerifyProofOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        &self.conf.verify_proof_resp_db_channel
    }

    async fn execute(&self) -> anyhow::Result<bool> {
        self.stop_exhausted_and_clean_completed_proofs().await?;
        // `zk_proof_id < $3` excludes synthetic blue-green dry-run inputs. Those are proved
        // and verified locally by every operator so the Gateway consensus track can anchor
        // while no user traffic exists, but no contract ever requested them — publishing a
        // response would hit `VerifyProofNotRequested` on chain.
        let rows = sqlx::query_as!(ProofTask,
            "SELECT zk_proof_id, chain_id, contract_address, user_address, handles, verified, retry_count, extra_data, transaction_id, sender_attempt_no
             FROM verify_proofs
             WHERE (verified = TRUE OR (verified = FALSE AND handles IS NOT NULL) OR sender_raw_tx IS NOT NULL)
               AND sender_status IN ('QUEUED', 'RETRY_WAIT', 'AWAITING_OUTCOME')
               AND (sender_next_action_at IS NULL OR sender_next_action_at <= NOW())
               AND (retry_count < $1 OR sender_raw_tx IS NOT NULL)
               AND zk_proof_id < $3
               AND (last_retry_at IS NULL OR last_retry_at <= NOW() - make_interval(secs => $4))
             ORDER BY COALESCE(last_retry_at, created_at), zk_proof_id
             LIMIT $2",
            self.conf.verify_proof_resp_max_retries as i64,
            self.conf.verify_proof_resp_batch_limit as i64,
            SYNTHETIC_ZK_PROOF_ID_BASE,
            f64::from(self.conf.error_sleep_max_secs.max(1))
        )
        .fetch_all(&self.db_pool)
        .await?;
        info!(rows_count = rows.len(), "Selected rows to process");
        let maybe_has_more_work = rows.len() == self.conf.verify_proof_resp_batch_limit as usize;
        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let operation = self.clone();
            join_set.spawn(async move {
                let Some(attempt) = Attempt::claim(
                    &operation.db_pool,
                    Task::Proof(row.zk_proof_id),
                    row.sender_attempt_no,
                )
                .await?
                else {
                    return Ok(());
                };
                let result: anyhow::Result<()> = async {
                    if let Some((request, verified)) =
                        operation.prepare_response(&row, &attempt).await?
                    {
                        operation
                            .process_proof(
                                attempt.clone(),
                                request,
                                verified,
                                row.retry_count,
                                row.transaction_id,
                            )
                            .await?;
                    }
                    Ok(())
                }
                .await;
                if let Err(error) = &result {
                    // Also cover signing, malformed payloads, and persistence failures.
                    // A durable transaction must survive even if the last DB commit was uncertain.
                    if let Some(tx) = attempt
                        .transition(
                            &operation.db_pool,
                            Status::AwaitingOutcome,
                            Some(&crate::diagnostics::safe_error(error.as_ref())),
                            operation.conf.error_sleep_max_secs,
                        )
                        .await?
                    {
                        tx.commit().await?;
                    }
                }
                result
            });
        }
        // Drain mixed batches before returning an error. Dropping the JoinSet
        // would cancel healthy submissions, possibly after nonce allocation.
        let mut first_error = None;
        while let Some(res) = join_set.join_next().await {
            if let Err(error) = res.map_err(anyhow::Error::from).and_then(|result| result) {
                // Keep fatal transport shutdown even if another task failed first.
                if first_error.is_none() || crate::is_backend_gone(&error) {
                    first_error = Some(error);
                }
            }
        }
        if let Some(error) = first_error {
            return Err(error);
        }
        Ok(maybe_has_more_work)
    }
}
