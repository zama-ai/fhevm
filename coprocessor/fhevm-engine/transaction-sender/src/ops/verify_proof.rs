use super::TransactionOperation;
use crate::metrics::{VERIFY_PROOF_FAIL_COUNTER, VERIFY_PROOF_SUCCESS_COUNTER};
use crate::nonce_managed_provider::NonceManagedProvider;
use crate::overprovision_gas_limit::try_overprovision_gas_limit;
use crate::AbstractSigner;
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::sol;
use alloy::{network::Ethereum, primitives::FixedBytes, sol_types::SolStruct};
use async_trait::async_trait;
use fhevm_engine_common::telemetry;
use sqlx::{Pool, Postgres};
use std::convert::TryInto;
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{debug, error, info, warn};

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

#[derive(Clone)]
pub(crate) struct VerifyProofOperation<P: Provider<Ethereum> + Clone + 'static> {
    input_verification_address: Address,
    provider: NonceManagedProvider<P>,
    signer: AbstractSigner,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    gw_chain_id: u64,
    db_pool: Pool<Postgres>,
}

impl<P: alloy::providers::Provider<Ethereum> + Clone + 'static> VerifyProofOperation<P> {
    pub(crate) async fn new(
        input_verification_address: Address,
        provider: NonceManagedProvider<P>,
        signer: AbstractSigner,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
        db_pool: Pool<Postgres>,
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
        })
    }

    async fn remove_proof_by_id(&self, zk_proof_id: i64) -> anyhow::Result<()> {
        debug!(zk_proof_id = zk_proof_id, "Removing proof");
        sqlx::query!(
            "DELETE FROM verify_proofs WHERE zk_proof_id = $1",
            zk_proof_id
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn update_retry_count_by_proof_id(
        &self,
        zk_proof_id: i64,
        current_retry_count: i32,
        error: &str,
    ) -> anyhow::Result<()> {
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
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn remove_proofs_by_retry_count(&self) -> anyhow::Result<()> {
        debug!(
            max_retries = self.conf.verify_proof_resp_max_retries,
            "Removing proofs with retry count >= max_retries"
        );
        sqlx::query!(
            "DELETE FROM verify_proofs WHERE retry_count >= $1",
            self.conf.verify_proof_resp_max_retries as i64
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }

    async fn process_proof(
        &self,
        txn_request: (i64, impl Into<TransactionRequest>),
        current_retry_count: i32,
        src_transaction_id: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        info!(zk_proof_id = txn_request.0, "Processing transaction");
        let _t = telemetry::tracer("call_verify_proof_resp", &src_transaction_id);

        let overprovisioned_txn_req = try_overprovision_gas_limit(
            txn_request.1,
            self.provider.inner(),
            self.conf.gas_limit_overprovision_percent,
        )
        .await;
        let transaction = match self
            .provider
            .send_transaction(overprovisioned_txn_req.clone())
            .await
        {
            Ok(txn) => txn,
            Err(e) => {
                if let Some(InputVerificationErrors::CoprocessorAlreadyVerified(_)) =
                    e.as_error_resp().and_then(|payload| {
                        payload.as_decoded_interface_error::<InputVerificationErrors>()
                    })
                {
                    warn!(
                        zk_proof_id = txn_request.0,
                        "Coprocessor has already verified the proof, removing from DB"
                    );
                    self.remove_proof_by_id(txn_request.0).await?;
                    return Ok(());
                } else if let Some(InputVerificationErrors::CoprocessorAlreadyRejected(_)) =
                    e.as_error_resp().and_then(|payload| {
                        payload.as_decoded_interface_error::<InputVerificationErrors>()
                    })
                {
                    warn!(
                        zk_proof_id = txn_request.0,
                        "Coprocessor has already rejected the proof, removing from DB"
                    );
                    self.remove_proof_by_id(txn_request.0).await?;
                    return Ok(());
                } else {
                    VERIFY_PROOF_FAIL_COUNTER.inc();
                    error!(
                        transaction_request = ?overprovisioned_txn_req,
                        error = %e,
                        "Transaction sending failed"
                    );
                    self.update_retry_count_by_proof_id(
                        txn_request.0,
                        current_retry_count,
                        &e.to_string(),
                    )
                    .await?;
                    return Err(anyhow::Error::new(e));
                }
            }
        };

        let receipt = match transaction
            .with_timeout(Some(Duration::from_secs(
                self.conf.txn_receipt_timeout_secs as u64,
            )))
            .with_required_confirmations(self.conf.required_txn_confirmations as u64)
            .get_receipt()
            .await
        {
            Ok(receipt) => receipt,
            Err(e) => {
                VERIFY_PROOF_FAIL_COUNTER.inc();
                error!(error = %e, "Getting receipt failed");
                self.update_retry_count_by_proof_id(
                    txn_request.0,
                    current_retry_count,
                    &e.to_string(),
                )
                .await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            info!(
                transaction_hash = %receipt.transaction_hash,
                "Transaction succeeded"
            );
            self.remove_proof_by_id(txn_request.0).await?;
            VERIFY_PROOF_SUCCESS_COUNTER.inc();

            telemetry::try_end_zkproof_transaction(
                &self.db_pool,
                &src_transaction_id.unwrap_or_default(),
            )
            .await?;
        } else {
            VERIFY_PROOF_FAIL_COUNTER.inc();
            error!(
                transaction_hash = %receipt.transaction_hash,
                status = receipt.status(),
                "Transaction failed"
            );
            self.update_retry_count_by_proof_id(
                txn_request.0,
                current_retry_count,
                "receipt status = false",
            )
            .await?;
            return Err(anyhow::anyhow!(
                "Transaction {} failed with status {}",
                receipt.transaction_hash,
                receipt.status()
            ));
        }
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
        let input_verification =
            InputVerification::new(self.input_verification_address, self.provider.inner());
        if self.conf.verify_proof_remove_after_max_retries {
            self.remove_proofs_by_retry_count().await?;
        }
        let rows = sqlx::query!(
            "SELECT zk_proof_id, chain_id, contract_address, user_address, handles, verified, retry_count, extra_data, transaction_id
             FROM verify_proofs
             WHERE verified IS NOT NULL AND retry_count < $1
             ORDER BY zk_proof_id
             LIMIT $2",
            self.conf.verify_proof_resp_max_retries as i64,
            self.conf.verify_proof_resp_batch_limit as i64
        )
        .fetch_all(&self.db_pool)
        .await?;
        info!(rows_count = rows.len(), "Selected rows to process");
        let maybe_has_more_work = rows.len() == self.conf.verify_proof_resp_batch_limit as usize;
        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let transaction_id = row.transaction_id.clone();
            let t = telemetry::tracer("prepare_verify_proof_resp", &transaction_id);

            let txn_request = match row.verified {
                Some(true) => {
                    info!(zk_proof_id = row.zk_proof_id, "Processing verified proof");
                    let handles = row
                        .handles
                        .ok_or(anyhow::anyhow!("handles field is None"))?;
                    if handles.len() % 32 != 0 {
                        error!(
                            handles_len = handles.len(),
                            "Bad handles field, len is not divisible by 32"
                        );
                        self.remove_proof_by_id(row.zk_proof_id).await?;
                        continue;
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
                    let signing_hash = CiphertextVerification {
                        ctHandles: handles.clone(),
                        userAddress: row.user_address.parse().expect("invalid user address"),
                        contractAddress: row
                            .contract_address
                            .parse()
                            .expect("invalid contract address"),
                        contractChainId: U256::from(row.chain_id),
                        extraData: row.extra_data.clone().into(),
                    }
                    .eip712_signing_hash(&domain);
                    let signature = self.signer.sign_hash(&signing_hash).await?;

                    if let Some(gas) = self.gas {
                        (
                            row.zk_proof_id,
                            input_verification
                                .verifyProofResponse(
                                    U256::from(row.zk_proof_id),
                                    handles,
                                    signature.as_bytes().into(),
                                    row.extra_data.into(),
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
                                    row.extra_data.into(),
                                )
                                .into_transaction_request(),
                        )
                    }
                }
                Some(false) => {
                    info!(zk_proof_id = row.zk_proof_id, "Processing rejected proof");
                    if let Some(gas) = self.gas {
                        (
                            row.zk_proof_id,
                            input_verification
                                .rejectProofResponse(
                                    U256::from(row.zk_proof_id),
                                    row.extra_data.into(),
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
                                    row.extra_data.into(),
                                )
                                .into_transaction_request(),
                        )
                    }
                }
                None => {
                    error!(
                        zk_proof_id = row.zk_proof_id,
                        "verified field is unexpectedly None for proof"
                    );
                    continue;
                }
            };

            t.end();

            let self_clone = self.clone();
            let src_transaction_id = transaction_id;
            join_set.spawn(async move {
                self_clone
                    .process_proof(txn_request, row.retry_count, src_transaction_id)
                    .await
            });
        }
        while let Some(res) = join_set.join_next().await {
            res??;
        }
        Ok(maybe_has_more_work)
    }
}
