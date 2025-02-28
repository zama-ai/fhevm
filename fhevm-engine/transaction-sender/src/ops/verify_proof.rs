use super::TransactionOperation;
use crate::VERIFY_PROOFS_TARGET;
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::SignerSync;
use alloy::sol;
use alloy::{network::Ethereum, primitives::FixedBytes, sol_types::SolStruct};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::convert::TryInto;
use tokio::task::JoinSet;
use tracing::{debug, error, info};
use ZKPoKManager::ZKPoKManagerErrors;

sol! {
    struct EIP712ZKPoK {
        bytes32[] handles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
    }
}

sol!(
    #[sol(rpc)]
    ZKPoKManager,
    "artifacts/ZKPoKManager.sol/ZKPoKManager.json"
);

#[derive(Clone)]
pub(crate) struct VerifyProofOperation<P: Provider<Ethereum> + Clone + 'static> {
    zkpok_manager_address: Address,
    provider: P,
    signer: PrivateKeySigner,
    conf: crate::ConfigSettings,
    gas: Option<u64>,
    gw_chain_id: u64,
}

impl<P: alloy::providers::Provider<Ethereum> + Clone + 'static> VerifyProofOperation<P> {
    pub(crate) async fn new(
        zkpok_manager_address: Address,
        provider: P,
        signer: PrivateKeySigner,
        conf: crate::ConfigSettings,
        gas: Option<u64>,
    ) -> anyhow::Result<Self> {
        let gw_chain_id = provider.get_chain_id().await?;
        Ok(Self {
            zkpok_manager_address,
            provider,
            signer,
            conf,
            gas,
            gw_chain_id,
        })
    }

    async fn remove_proof_by_id(
        &self,
        db_pool: &Pool<Postgres>,
        zk_proof_id: i64,
    ) -> anyhow::Result<()> {
        debug!(target: VERIFY_PROOFS_TARGET, "Removing proof with id {}", zk_proof_id);
        sqlx::query!(
            "DELETE FROM verify_proofs WHERE zk_proof_id = $1",
            zk_proof_id
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    async fn update_retry_count_by_proof_id(
        &self,
        db_pool: &Pool<Postgres>,
        zk_proof_id: i64,
        error: &str,
    ) -> anyhow::Result<()> {
        debug!(target: VERIFY_PROOFS_TARGET, "Updating retry count of proof with id {}", zk_proof_id);
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
        .execute(db_pool)
        .await?;
        Ok(())
    }

    async fn remove_proofs_by_retry_count(
        &self,
        db_pool: &Pool<Postgres>,
        max_retries: u32,
    ) -> anyhow::Result<()> {
        debug!(target: VERIFY_PROOFS_TARGET, "Removing proof with retry count >= {}", max_retries);
        sqlx::query!(
            "DELETE FROM verify_proofs WHERE retry_count >= $1",
            max_retries as i64
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    async fn process_proof(
        &self,
        db_pool: Pool<Postgres>,
        provider: P,
        txn_request: (i64, impl Into<TransactionRequest>),
    ) -> anyhow::Result<()> {
        info!(target: VERIFY_PROOFS_TARGET, "Processing proof with proof id {}", txn_request.0);
        let txn_req = txn_request.1.into();
        let transaction = match provider.send_transaction(txn_req.clone()).await {
            Ok(txn) => txn,
            Err(e) => {
                error!(target: VERIFY_PROOFS_TARGET, "Transaction {:?} sending failed with error: {}", txn_req, e);
                if let Some(ZKPoKManagerErrors::CoprocessorHasAlreadySigned(_)) = e
                    .as_error_resp()
                    .and_then(|payload| payload.as_decoded_error::<ZKPoKManagerErrors>(true))
                {
                    info!(target: VERIFY_PROOFS_TARGET, "Coprocessor has already signed, removing proof");
                    self.remove_proof_by_id(&db_pool, txn_request.0).await?;
                    return Ok(());
                } else {
                    self.update_retry_count_by_proof_id(&db_pool, txn_request.0, &e.to_string())
                        .await?;
                    return Err(anyhow::Error::new(e));
                }
            }
        };

        // Here, we assume we are sending the transaction to a rollup, hence the confirmations of 1.
        let receipt = match transaction
            .with_required_confirmations(1)
            .get_receipt()
            .await
        {
            Ok(receipt) => receipt,
            Err(e) => {
                error!(target: VERIFY_PROOFS_TARGET, "Getting receipt failed with error: {}", e);
                self.update_retry_count_by_proof_id(&db_pool, txn_request.0, &e.to_string())
                    .await?;
                return Err(anyhow::Error::new(e));
            }
        };

        if receipt.status() {
            info!(target: VERIFY_PROOFS_TARGET, "Transaction {} succeeded", receipt.transaction_hash);
            self.remove_proof_by_id(&db_pool, txn_request.0).await?;
        } else {
            error!(target: VERIFY_PROOFS_TARGET, "Transaction {} failed with status {}", 
                receipt.transaction_hash, receipt.status());
            self.update_retry_count_by_proof_id(&db_pool, txn_request.0, "receipt status = false")
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

    async fn execute(&self, db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
        let zkpok_manager = ZKPoKManager::new(self.zkpok_manager_address, &self.provider);
        if self.conf.verify_proof_remove_after_max_retries {
            self.remove_proofs_by_retry_count(db_pool, self.conf.verify_proof_resp_max_retries)
                .await?;
        }
        let rows = sqlx::query!(
            "SELECT zk_proof_id, chain_id, contract_address, user_address, handles
             FROM verify_proofs
             WHERE verified = true AND retry_count < $1
             ORDER BY zk_proof_id
             LIMIT $2",
            self.conf.verify_proof_resp_max_retries as i64,
            self.conf.verify_proof_resp_batch_limit as i64
        )
        .fetch_all(db_pool)
        .await?;
        info!(target: VERIFY_PROOFS_TARGET, "Selected {} rows to process", rows.len());
        let maybe_has_more_work = rows.len() == self.conf.verify_proof_resp_batch_limit as usize;
        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            let handles = row
                .handles
                .ok_or(anyhow::anyhow!("handles field is None"))?;
            if handles.is_empty() || handles.len() % 32 != 0 {
                error!(target: VERIFY_PROOFS_TARGET, "Bad handles field, len {} is 0 or not divisible by 32", handles.len());
                self.remove_proof_by_id(db_pool, row.zk_proof_id).await?;
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
                name: "ZKPoKManager",
                version: "1",
                chain_id: self.gw_chain_id,
                verifying_contract: self.zkpok_manager_address,
            };
            let signing_hash = EIP712ZKPoK {
                handles: handles.clone(),
                userAddress: row.user_address.parse().expect("invalid user address"),
                contractAddress: row
                    .contract_address
                    .parse()
                    .expect("invalid contract address"),
                contractChainId: U256::from(row.chain_id),
            }
            .eip712_signing_hash(&domain);
            let signature = self
                .signer
                .sign_hash_sync(&signing_hash)
                .expect("signing failed");

            let txn_request = if let Some(gas) = self.gas {
                (
                    row.zk_proof_id,
                    zkpok_manager
                        .verifyProofResponse(
                            U256::from(row.zk_proof_id),
                            handles,
                            signature.as_bytes().into(),
                        )
                        .into_transaction_request()
                        .with_gas_limit(gas),
                )
            } else {
                (
                    row.zk_proof_id,
                    zkpok_manager
                        .verifyProofResponse(
                            U256::from(row.zk_proof_id),
                            handles,
                            signature.as_bytes().into(),
                        )
                        .into_transaction_request(),
                )
            };

            let db_pool = db_pool.clone();
            let provider = self.provider.clone();
            let self_clone = self.clone();
            join_set.spawn(async move {
                self_clone
                    .process_proof(db_pool, provider, txn_request)
                    .await
            });
        }
        while let Some(res) = join_set.join_next().await {
            res??;
        }
        Ok(maybe_has_more_work)
    }
}
