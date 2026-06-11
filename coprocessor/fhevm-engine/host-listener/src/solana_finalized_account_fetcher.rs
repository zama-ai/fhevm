use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde_json::json;
use tracing::{info, warn};

use crate::database::tfhe_event_propagate::Database;
use crate::solana_adapter::{
    claim_pending_finalized_account_fetches, complete_finalized_account_fetch,
    fail_finalized_account_fetch, retry_finalized_account_fetch,
    store_finalized_account_witness, SolanaFinalizedAccountFetchJob,
    SolanaFinalizedAccountWitness,
};

#[derive(Clone, Debug)]
pub struct SolanaFinalizedAccountFetcherConfig {
    pub rpc_url: String,
    pub batch_size: i64,
    pub poll_interval: Duration,
    pub retry_interval: Duration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaFinalizedAccount {
    pub owner: [u8; 32],
    pub lamports: u64,
    pub executable: bool,
    pub data: Vec<u8>,
    pub observed_slot: u64,
}

#[async_trait]
pub trait SolanaFinalizedAccountClient: Send + Sync {
    async fn get_finalized_accounts(
        &self,
        account_keys: &[[u8; 32]],
    ) -> Result<Vec<Option<SolanaFinalizedAccount>>>;
}

#[derive(Clone)]
pub struct JsonRpcSolanaFinalizedAccountClient {
    rpc_url: String,
    client: reqwest::Client,
}

impl JsonRpcSolanaFinalizedAccountClient {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl SolanaFinalizedAccountClient for JsonRpcSolanaFinalizedAccountClient {
    async fn get_finalized_accounts(
        &self,
        account_keys: &[[u8; 32]],
    ) -> Result<Vec<Option<SolanaFinalizedAccount>>> {
        if account_keys.is_empty() {
            return Ok(Vec::new());
        }

        let encoded_keys = account_keys
            .iter()
            .map(|key| bs58::encode(key).into_string())
            .collect::<Vec<_>>();
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getMultipleAccounts",
            "params": [
                encoded_keys,
                {
                    "encoding": "base64",
                    "commitment": "finalized"
                }
            ]
        });

        let response = self
            .client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await
            .context("Solana finalized-account RPC request failed")?
            .error_for_status()
            .context("Solana finalized-account RPC returned HTTP error")?
            .json::<RpcEnvelope>()
            .await
            .context(
                "Solana finalized-account RPC response was not valid JSON",
            )?;

        if let Some(error) = response.error {
            bail!(
                "Solana finalized-account RPC error {}: {}",
                error.code,
                error.message
            );
        }
        let result = response.result.ok_or_else(|| {
            anyhow!("Solana finalized-account RPC missing result")
        })?;
        if result.value.len() != account_keys.len() {
            bail!(
                "Solana finalized-account RPC returned {} accounts for {} keys",
                result.value.len(),
                account_keys.len()
            );
        }

        result
            .value
            .into_iter()
            .map(|account| {
                account
                    .map(|account| account.into_account(result.context.slot))
                    .transpose()
            })
            .collect()
    }
}

pub async fn run_solana_finalized_account_fetcher(
    mut db: Database,
    config: SolanaFinalizedAccountFetcherConfig,
) -> Result<()> {
    let client = JsonRpcSolanaFinalizedAccountClient::new(config.rpc_url);
    loop {
        match process_finalized_account_fetch_batch(
            &db,
            &client,
            config.batch_size,
        )
        .await
        {
            Ok(0) => {
                db.tick.update();
                tokio::time::sleep(config.poll_interval).await;
            }
            Ok(processed) => {
                db.tick.update();
                info!(
                    processed,
                    "Processed Solana finalized-account fetch batch"
                );
            }
            Err(err) => {
                warn!(
                    error = %err,
                    "Solana finalized-account fetch batch failed"
                );
                if err.downcast_ref::<sqlx::Error>().is_some() {
                    db.reconnect().await;
                }
                tokio::time::sleep(config.retry_interval).await;
            }
        }
    }
}

pub async fn process_finalized_account_fetch_batch<C>(
    db: &Database,
    client: &C,
    limit: i64,
) -> Result<usize>
where
    C: SolanaFinalizedAccountClient,
{
    let mut claim_tx = db.new_transaction().await?;
    let jobs =
        claim_pending_finalized_account_fetches(&mut claim_tx, limit).await?;
    claim_tx.commit().await?;

    if jobs.is_empty() {
        return Ok(0);
    }

    let account_keys =
        jobs.iter().map(|job| job.account_key).collect::<Vec<_>>();
    let accounts = match client.get_finalized_accounts(&account_keys).await {
        Ok(accounts) => accounts,
        Err(err) => {
            retry_jobs(db, &jobs, &err.to_string()).await?;
            return Err(err);
        }
    };
    if accounts.len() != jobs.len() {
        let error = format!(
            "Solana finalized-account client returned {} accounts for {} jobs",
            accounts.len(),
            jobs.len()
        );
        retry_jobs(db, &jobs, &error).await?;
        bail!(error);
    }

    let mut tx = db.new_transaction().await?;
    for (job, account) in jobs.iter().zip(accounts) {
        match account {
            Some(account) => {
                let witness = SolanaFinalizedAccountWitness {
                    account_key: job.account_key,
                    owner: account.owner,
                    lamports: account.lamports,
                    executable: account.executable,
                    data: account.data,
                    observed_slot: account.observed_slot,
                };
                store_finalized_account_witness(&mut tx, &witness).await?;
                complete_finalized_account_fetch(&mut tx, job).await?;
            }
            None => {
                fail_finalized_account_fetch(
                    &mut tx,
                    job,
                    "account not found at finalized commitment",
                )
                .await?;
            }
        }
    }
    tx.commit().await?;

    Ok(jobs.len())
}

async fn retry_jobs(
    db: &Database,
    jobs: &[SolanaFinalizedAccountFetchJob],
    error: &str,
) -> Result<()> {
    let mut tx = db.new_transaction().await?;
    for job in jobs {
        retry_finalized_account_fetch(&mut tx, job, error).await?;
    }
    tx.commit().await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct RpcEnvelope {
    result: Option<RpcResult>,
    error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    code: i64,
    message: String,
}

#[derive(Debug, Deserialize)]
struct RpcResult {
    context: RpcContext,
    value: Vec<Option<RpcAccount>>,
}

#[derive(Debug, Deserialize)]
struct RpcContext {
    slot: u64,
}

#[derive(Debug, Deserialize)]
struct RpcAccount {
    lamports: u64,
    owner: String,
    executable: bool,
    data: Vec<String>,
}

impl RpcAccount {
    fn into_account(
        self,
        observed_slot: u64,
    ) -> Result<SolanaFinalizedAccount> {
        Ok(SolanaFinalizedAccount {
            owner: decode_pubkey(&self.owner)?,
            lamports: self.lamports,
            executable: self.executable,
            data: decode_account_data(self.data)?,
            observed_slot,
        })
    }
}

fn decode_account_data(values: Vec<String>) -> Result<Vec<u8>> {
    if values.len() != 2 {
        bail!(
            "Solana account data must be [payload, encoding], got {} fields",
            values.len()
        );
    }
    if values[1] != "base64" {
        bail!("unsupported Solana account data encoding {}", values[1]);
    }
    BASE64_STANDARD
        .decode(values[0].as_bytes())
        .context("Solana account data is not valid base64")
}

fn decode_pubkey(encoded: &str) -> Result<[u8; 32]> {
    let bytes = bs58::decode(encoded)
        .into_vec()
        .context("Solana account owner is not valid base58")?;
    bytes.try_into().map_err(|bytes: Vec<u8>| {
        anyhow!(
            "Solana account owner decoded to {} bytes, expected 32",
            bytes.len()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_base64_account_response() {
        let owner = [9_u8; 32];
        let account = RpcAccount {
            lamports: 42,
            owner: bs58::encode(owner).into_string(),
            executable: false,
            data: vec![
                BASE64_STANDARD.encode([1_u8, 2, 3]),
                "base64".to_owned(),
            ],
        }
        .into_account(77)
        .expect("account should decode");

        assert_eq!(
            account,
            SolanaFinalizedAccount {
                owner,
                lamports: 42,
                executable: false,
                data: vec![1, 2, 3],
                observed_slot: 77,
            }
        );
    }

    #[test]
    fn rejects_owner_that_is_not_a_pubkey() {
        let err = RpcAccount {
            lamports: 42,
            owner: bs58::encode([1_u8; 31]).into_string(),
            executable: false,
            data: vec![BASE64_STANDARD.encode([]), "base64".to_owned()],
        }
        .into_account(77)
        .expect_err("short owner should fail");

        assert!(
            err.to_string().contains("expected 32"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn rejects_non_base64_account_data() {
        let err = decode_account_data(vec![
            "not-base64".to_owned(),
            "base64".to_owned(),
        ])
        .expect_err("invalid payload should fail");

        assert!(
            err.to_string().contains("not valid base64"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn rejects_account_data_with_wrong_field_count() {
        let err = decode_account_data(vec![BASE64_STANDARD.encode([1_u8])])
            .expect_err("single-field account data should fail");

        assert!(
            err.to_string().contains("[payload, encoding]"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn rejects_unsupported_account_data_encoding() {
        let err = decode_account_data(vec![
            "00".to_owned(),
            "base58".to_owned(),
        ])
        .expect_err("non-base64 encoding should fail");

        assert!(
            err.to_string().contains("unsupported"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn get_finalized_accounts_short_circuits_on_empty_keys() {
        // No key means no RPC call, so an unreachable URL must not be contacted.
        let client = JsonRpcSolanaFinalizedAccountClient::new(
            "http://127.0.0.1:1/unused",
        );
        let accounts = client
            .get_finalized_accounts(&[])
            .await
            .expect("empty key set must short-circuit without a request");
        assert!(accounts.is_empty());
    }
}
