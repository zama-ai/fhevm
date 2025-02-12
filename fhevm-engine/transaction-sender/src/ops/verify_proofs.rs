use super::TransactionOperation;
use crate::VERIFY_PROOFS_TARGET;
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
use tracing::{error, info};
use ZKPoKManager::ZKPoKManagerErrors;

sol! {
    struct VerifyProofSignatureData {
        bytes32[] handles;
        address userAddress;
        address contractAddress;
        uint256 chainId;
    }
}

sol!(
    #[sol(rpc)]
    ZKPoKManager,
    "artifacts/ZKPoKManager.sol/ZKPoKManager.json"
);

#[derive(Clone)]
pub struct VerifyProofsOperation<P: Provider<Ethereum> + Clone + 'static> {
    zkpok_manager_address: Address,
    provider: std::sync::Arc<P>,
    signer: PrivateKeySigner,
    database_conf: crate::ConfigSettings,
}

impl<P: alloy::providers::Provider<Ethereum> + Clone + 'static> VerifyProofsOperation<P> {
    pub fn new(
        zkpok_manager_address: Address,
        provider: std::sync::Arc<P>,
        signer: PrivateKeySigner,
        database_conf: crate::ConfigSettings,
    ) -> Self {
        Self {
            zkpok_manager_address,
            provider,
            signer,
            database_conf,
        }
    }

    async fn remove_proof_by_id(
        &self,
        db_pool: &Pool<Postgres>,
        zk_proof_id: i64,
    ) -> anyhow::Result<()> {
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
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE verify_proofs SET retry_count = retry_count + 1 WHERE zk_proof_id = $1",
            zk_proof_id
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
        sqlx::query!(
            "DELETE FROM verify_proofs WHERE retry_count >= $1",
            max_retries as i64
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    async fn process_transaction(
        &self,
        db_pool: Pool<Postgres>,
        provider: std::sync::Arc<P>,
        txn_request: (i64, impl Into<TransactionRequest>),
    ) -> anyhow::Result<()> {
        let txn_req = txn_request.1.into();
        let transaction = match provider.send_transaction(txn_req).await {
            Ok(txn) => txn,
            Err(e) => {
                error!(target: VERIFY_PROOFS_TARGET, "Transaction sending failed with error {}", e);
                if let Some(ZKPoKManagerErrors::CoprocessorHasAlreadySigned(_)) = e
                    .as_error_resp()
                    .and_then(|payload| payload.as_decoded_error::<ZKPoKManagerErrors>(true))
                {
                    info!(target: VERIFY_PROOFS_TARGET, "Coprocessor has already signed, removing proof");
                    self.remove_proof_by_id(&db_pool, txn_request.0).await?;
                    return Ok(());
                } else {
                    self.update_retry_count_by_proof_id(&db_pool, txn_request.0)
                        .await?;
                    return Err(anyhow::Error::new(e));
                }
            }
        };

        let receipt = match transaction.get_receipt().await {
            Ok(receipt) => receipt,
            Err(e) => {
                error!(target: VERIFY_PROOFS_TARGET, "Transaction failed with error {}", e);
                self.update_retry_count_by_proof_id(&db_pool, txn_request.0)
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
            self.update_retry_count_by_proof_id(&db_pool, txn_request.0)
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
impl<P> TransactionOperation<P> for VerifyProofsOperation<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    fn channel(&self) -> &str {
        "verify_proofs"
    }

    async fn execute(&self, db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
        let zkpok_manager = ZKPoKManager::new(self.zkpok_manager_address, &self.provider);
        self.remove_proofs_by_retry_count(
            db_pool,
            self.database_conf.verify_proof_resp_max_retries,
        )
        .await?;
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE retry_count < $1
             ORDER BY zk_proof_id
             LIMIT $2",
            self.database_conf.verify_proof_resp_max_retries as i64,
            self.database_conf.verify_proof_resp_batch_limit as i64
        )
        .fetch_all(db_pool)
        .await?;
        let maybe_has_more_work =
            rows.len() == self.database_conf.verify_proof_resp_batch_limit as usize;
        let mut join_set = JoinSet::new();
        for row in rows.into_iter() {
            if row.handles.is_empty() || row.handles.len() % 32 != 0 {
                error!(target: VERIFY_PROOFS_TARGET, "Bad handles field, len {} is 0 or not divisible by 32", row.handles.len());
                self.remove_proof_by_id(db_pool, row.zk_proof_id).await?;
                continue;
            }
            let handles: Vec<FixedBytes<32>> = row
                .handles
                .chunks(32)
                .map(|chunk| {
                    let array: [u8; 32] = chunk.try_into().expect("chunk size must be 32");
                    FixedBytes(array)
                })
                .collect();
            let domain = alloy::sol_types::eip712_domain! {
                name: "InputVerifier",
                version: "1",
                chain_id: row.chain_id as u64,
                verifying_contract: self.zkpok_manager_address,
            };
            let signing_hash = VerifyProofSignatureData {
                handles: handles.clone(),
                userAddress: row.user_address.parse().expect("invalid user address"),
                contractAddress: row
                    .contract_address
                    .parse()
                    .expect("invalid contract address"),
                chainId: U256::from(row.chain_id),
            }
            .eip712_signing_hash(&domain);
            let signature = self
                .signer
                .sign_hash_sync(&signing_hash)
                .expect("signing failed");
            let txn_request = (
                row.zk_proof_id,
                zkpok_manager
                    .verifyProofResponse(
                        U256::from(row.zk_proof_id),
                        handles,
                        signature.as_bytes().into(),
                    )
                    .into_transaction_request(),
            );
            let db_pool = db_pool.clone();
            let provider = self.provider.clone();
            let self_clone = self.clone();
            join_set.spawn(async move {
                self_clone
                    .process_transaction(db_pool, provider, txn_request)
                    .await
            });
        }
        while let Some(res) = join_set.join_next().await {
            res??;
        }
        Ok(maybe_has_more_work)
    }
}
