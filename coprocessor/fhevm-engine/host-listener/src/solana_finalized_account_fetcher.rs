use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use fhevm_engine_common::types::AllowEvents;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use tracing::{info, warn};

use crate::database::tfhe_event_propagate::Database;
use crate::solana_adapter::{
    claim_pending_finalized_account_fetches, complete_finalized_account_fetch,
    fail_finalized_account_fetch, retry_finalized_account_fetch,
    store_finalized_account_witness, SolanaFinalizedAccountFetchJob,
    SolanaFinalizedAccountFetchKind, SolanaFinalizedAccountWitness,
};

/// A queued account can be `processed`/`confirmed` on-chain but not yet
/// `finalized` when we first poll (Solana finalization lags the tip by ~32
/// slots), so "not found at finalized commitment" is treated as transient and
/// retried until this many claim attempts have elapsed. Only then is it a hard
/// failure — guarding against an account that genuinely never finalizes (e.g. a
/// dropped/reorged transaction) so the queue cannot hot-loop forever.
const MAX_FINALIZATION_ATTEMPTS: i32 = 60;

#[derive(Clone, Debug)]
pub struct SolanaFinalizedAccountFetcherConfig {
    pub rpc_url: String,
    pub batch_size: i64,
    pub poll_interval: Duration,
    /// base58 zama_host program id used as the trust anchor: a finalized
    /// EncryptedValue account must be owned by this program before its handle is
    /// released for decryption. `None` disables the owner check (e.g. local fixtures).
    pub host_program_id: Option<[u8; 32]>,
    pub retry_interval: Duration,
}

/// Downstream decrypt work to enqueue once a finalized EncryptedValue account
/// is confirmed on-chain. `AllowedForDecryption` releases a handle for public decrypt;
/// `AllowedAccount` records a durable per-subject allow. Both also enqueue the
/// SNS digest (`pbs_computations`) so the handle's ciphertext gets a ct128.
#[derive(Clone, Copy, Debug)]
pub struct DecryptEnqueue {
    pub handle: crate::database::tfhe_event_propagate::Handle,
    pub allow_event: AllowEvents,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FinalizedEncryptedValueAccount {
    current_handle: [u8; 32],
    leaf_count: u64,
}

/// Decide, for a finalized-account fetch whose account was FOUND on-chain,
/// whether to release the carried handle for decryption — and as which allow
/// kind. Returns `None` when nothing should be enqueued.
///
/// Trust guard (WHY): the whole finalized-fetch model exists so we re-read
/// authoritative on-chain state at `finalized` commitment before releasing
/// decrypt work, instead of trusting an unfinalized event log. We require a
/// recognized allow reason, a carried handle, host ownership when configured,
/// and an `EncryptedValue` account state that supports the claim.
pub fn decrypt_enqueue_for_fetch(
    job: &SolanaFinalizedAccountFetchJob,
    witness: &SolanaFinalizedAccountWitness,
    host_program_id: Option<[u8; 32]>,
) -> Option<DecryptEnqueue> {
    if job.kind != SolanaFinalizedAccountFetchKind::EncryptedValueAccount {
        return None;
    }
    let allow_event = match job.reason.as_str() {
        "public_decrypt_allowed" | "handle_made_public" => {
            AllowEvents::AllowedForDecryption
        }
        "acl_subject_allowed"
        | "acl_record_bound"
        | "encrypted_value_created"
        | "handle_superseded"
        | "subject_allowed" => AllowEvents::AllowedAccount,
        // handle_material_sealed also emits an EncryptedValue-account fetch, but
        // the allow it confirms already arrived via its own subject/public-decrypt
        // event; the material-sealed fetch is only a witness for the on-chain
        // consume path.
        _ => return None,
    };
    let handle = match job.handle {
        Some(handle) => handle,
        // Queued without a handle on a decode-time tracker miss (lineage
        // created before the listener started): for current-handle reasons the
        // finalized account itself is the authority for which handle the allow
        // refers to. Resolved below after the account is decoded.
        None if matches!(
            job.reason.as_str(),
            "subject_allowed" | "handle_made_public"
        ) =>
        {
            match finalized_current_handle(&witness.data) {
                Some(handle) => handle,
                None => {
                    warn!(
                        reason = %job.reason,
                        "handle-less finalized fetch on an invalid EncryptedValue account; refusing to release"
                    );
                    return None;
                }
            }
        }
        None => return None,
    };
    if let Some(program_id) = host_program_id {
        if witness.owner != program_id {
            warn!(
                handle = %handle,
                reason = %job.reason,
                "finalized EncryptedValue account not owned by zama_host program; refusing to release handle for decryption"
            );
            return None;
        }
    }
    if !encrypted_value_claim_is_finalized(job, witness, handle) {
        return None;
    }
    Some(DecryptEnqueue {
        handle,
        allow_event,
    })
}

/// The finalized account's `current_handle`, or `None` when the data does not
/// decode as an `EncryptedValue` account.
fn finalized_current_handle(
    data: &[u8],
) -> Option<crate::database::tfhe_event_propagate::Handle> {
    decode_finalized_encrypted_value_account(data)
        .ok()
        .map(|account| account.current_handle.into())
}

fn encrypted_value_claim_is_finalized(
    job: &SolanaFinalizedAccountFetchJob,
    witness: &SolanaFinalizedAccountWitness,
    handle: crate::database::tfhe_event_propagate::Handle,
) -> bool {
    let account = match decode_finalized_encrypted_value_account(&witness.data)
    {
        Ok(account) => account,
        Err(err) => {
            warn!(
                reason = %job.reason,
                error = %err,
                "finalized account is not a valid EncryptedValue account; refusing to release handle"
            );
            return false;
        }
    };
    let handle = handle_to_bytes(handle);
    match job.reason.as_str() {
        "public_decrypt_allowed"
        | "acl_subject_allowed"
        | "acl_record_bound"
        | "encrypted_value_created"
        | "handle_made_public"
        | "subject_allowed" => {
            if account.current_handle == handle {
                true
            } else {
                warn!(
                    reason = %job.reason,
                    "finalized EncryptedValue current_handle does not match queued handle; refusing to release handle"
                );
                false
            }
        }
        "handle_superseded" => {
            if account.current_handle == handle {
                return true;
            }
            // DD-034 reorg safety net: superseded handles are only represented
            // inside the MMR, so proving membership needs the historical proof
            // service. The fetcher can cheaply reject non-advanced lineages.
            if account.leaf_count > 0 {
                true
            } else {
                warn!(
                    "finalized EncryptedValue has no historical leaves for superseded handle; refusing to release handle"
                );
                false
            }
        }
        _ => false,
    }
}

fn decode_finalized_encrypted_value_account(
    data: &[u8],
) -> Result<FinalizedEncryptedValueAccount> {
    let mut reader = AccountDataReader::new(data);
    if reader.read_array::<8>()? != encrypted_value_discriminator() {
        bail!("EncryptedValue account discriminator mismatch");
    }
    reader.skip(32)?; // acl_domain_key
    reader.skip(32)?; // app_account
    reader.skip(32)?; // encrypted_value_label
    let current_handle = reader.read_array::<32>()?;
    let subjects_len = reader.read_u32()? as usize;
    reader.skip(checked_vec_bytes(subjects_len, 32)?)?;
    let leaf_count = reader.read_u64()?;
    let peaks_len = reader.read_u32()? as usize;
    reader.skip(checked_vec_bytes(peaks_len, 32)?)?;
    reader.read_array::<1>()?; // bump
    Ok(FinalizedEncryptedValueAccount {
        current_handle,
        leaf_count,
    })
}

fn checked_vec_bytes(len: usize, item_size: usize) -> Result<usize> {
    len.checked_mul(item_size)
        .ok_or_else(|| anyhow!("EncryptedValue vector length overflow"))
}

fn encrypted_value_discriminator() -> [u8; 8] {
    let digest = Sha256::digest(b"account:EncryptedValue");
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&digest[..8]);
    discriminator
}

struct AccountDataReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> AccountDataReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let bytes = self.take(N)?;
        let mut out = [0u8; N];
        out.copy_from_slice(bytes);
        Ok(out)
    }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_array::<4>()?))
    }

    fn read_u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.read_array::<8>()?))
    }

    fn skip(&mut self, len: usize) -> Result<()> {
        self.take(len).map(|_| ())
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or_else(|| anyhow!("EncryptedValue account offset overflow"))?;
        let bytes = self
            .data
            .get(self.offset..end)
            .ok_or_else(|| anyhow!("EncryptedValue account data too short"))?;
        self.offset = end;
        Ok(bytes)
    }
}

fn handle_to_bytes(
    handle: crate::database::tfhe_event_propagate::Handle,
) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(handle.as_slice());
    bytes
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
            config.host_program_id,
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
                // Pace every batch to `poll_interval`, not just empty ones. A
                // not-yet-finalized account is retried back to `pending` and
                // re-counts as "processed", so without this sleep the loop
                // hot-loops and burns the whole `MAX_FINALIZATION_ATTEMPTS`
                // budget in milliseconds — defeating the finality-lag tolerance.
                // Pacing makes the attempt budget track real time (~1 attempt /
                // poll_interval), so finalization (which lags the tip by ~32
                // slots) has time to catch up before a job is failed hard.
                tokio::time::sleep(config.poll_interval).await;
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
    host_program_id: Option<[u8; 32]>,
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
    // Jobs that reached a terminal state (found+completed, or hard-failed) this
    // batch. Retries that are merely awaiting finalization are NOT terminal, so
    // a batch of only those returns 0 and the caller idles instead of hot-looping
    // the RPC against not-yet-finalized accounts.
    let mut terminal = 0usize;
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

                // Finalized state confirmed: release the handle for decryption
                // in the SAME tx that records the witness, so a crash can never
                // leave a completed fetch without its downstream pbs/allow work.
                if let Some(enqueue) =
                    decrypt_enqueue_for_fetch(job, &witness, host_program_id)
                {
                    let handle = enqueue.handle.to_vec();
                    let transaction_id = Some(job.transaction_id.to_vec());
                    db.insert_allowed_handle(
                        &mut tx,
                        handle.clone(),
                        String::new(),
                        enqueue.allow_event,
                        transaction_id.clone(),
                        job.block_number,
                    )
                    .await?;
                    // ON CONFLICT DO NOTHING + the SNS worker only selects pbs
                    // rows whose ciphertext IS NOT NULL, so enqueuing here is
                    // safe whether or not the tfhe-worker has materialized the
                    // ciphertext yet — the digest simply waits for material.
                    db.insert_pbs_computations(
                        &mut tx,
                        &vec![handle],
                        transaction_id,
                        job.block_number,
                    )
                    .await?;
                }
                terminal += 1;
            }
            None => {
                // The allow may simply not have finalized yet: retry until the
                // attempt budget is spent, then fail hard.
                if job.attempts < MAX_FINALIZATION_ATTEMPTS {
                    retry_finalized_account_fetch(
                        &mut tx,
                        job,
                        "account not yet found at finalized commitment; awaiting finalization",
                    )
                    .await?;
                } else {
                    fail_finalized_account_fetch(
                        &mut tx,
                        job,
                        "account not found at finalized commitment after max attempts",
                    )
                    .await?;
                    terminal += 1;
                }
            }
        }
    }
    tx.commit().await?;

    Ok(terminal)
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
    use crate::database::tfhe_event_propagate::Handle;

    const HOST_PROGRAM: [u8; 32] = [7; 32];

    fn acl_record_job(
        reason: &str,
        handle: Option<Handle>,
    ) -> SolanaFinalizedAccountFetchJob {
        SolanaFinalizedAccountFetchJob {
            account_key: [1; 32],
            kind: SolanaFinalizedAccountFetchKind::EncryptedValueAccount,
            reason: reason.to_owned(),
            handle,
            related_account: None,
            subject: None,
            transaction_id: Handle::from([2; 32]),
            block_number: 9,
            attempts: 1,
        }
    }

    fn witness_owned_by(
        owner: [u8; 32],
        current_handle: [u8; 32],
        leaf_count: u64,
    ) -> SolanaFinalizedAccountWitness {
        SolanaFinalizedAccountWitness {
            account_key: [1; 32],
            owner,
            lamports: 1,
            executable: false,
            data: encrypted_value_account_data(current_handle, leaf_count),
            observed_slot: 42,
        }
    }

    fn encrypted_value_account_data(
        current_handle: [u8; 32],
        leaf_count: u64,
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&encrypted_value_discriminator());
        data.extend_from_slice(&[3; 32]); // acl_domain_key
        data.extend_from_slice(&[4; 32]); // app_account
        data.extend_from_slice(&[5; 32]); // encrypted_value_label
        data.extend_from_slice(&current_handle);
        data.extend_from_slice(&1_u32.to_le_bytes());
        data.extend_from_slice(&[6; 32]); // subjects[0]
        data.extend_from_slice(&leaf_count.to_le_bytes());
        data.extend_from_slice(&0_u32.to_le_bytes()); // peaks
        data.push(255); // bump
        data
    }

    #[test]
    fn public_decrypt_allowed_releases_handle_for_public_decryption() {
        let handle = Handle::from([5; 32]);
        let job = acl_record_job("public_decrypt_allowed", Some(handle));
        let enqueue = decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [5; 32], 0),
            Some(HOST_PROGRAM),
        )
        .expect("public decrypt allow must enqueue");
        assert_eq!(enqueue.handle, handle);
        assert_eq!(
            enqueue.allow_event as i16,
            AllowEvents::AllowedForDecryption as i16
        );
    }

    #[test]
    fn subject_and_bound_allows_record_durable_account_allow() {
        for reason in ["acl_subject_allowed", "acl_record_bound"] {
            let handle = Handle::from([6; 32]);
            let job = acl_record_job(reason, Some(handle));
            let enqueue = decrypt_enqueue_for_fetch(
                &job,
                &witness_owned_by(HOST_PROGRAM, [6; 32], 0),
                Some(HOST_PROGRAM),
            )
            .unwrap_or_else(|| panic!("{reason} must enqueue"));
            assert_eq!(enqueue.handle, handle);
            assert_eq!(
                enqueue.allow_event as i16,
                AllowEvents::AllowedAccount as i16
            );
        }
    }

    #[test]
    fn handle_less_current_handle_fetch_resolves_from_finalized_account() {
        // Decode-time tracker miss: allow_subjects/make_handle_public queue
        // without a handle; the finalized account's current_handle is released.
        for (reason, allow_event) in [
            ("subject_allowed", AllowEvents::AllowedAccount),
            ("handle_made_public", AllowEvents::AllowedForDecryption),
        ] {
            let job = acl_record_job(reason, None);
            let enqueue = decrypt_enqueue_for_fetch(
                &job,
                &witness_owned_by(HOST_PROGRAM, [9; 32], 0),
                Some(HOST_PROGRAM),
            )
            .unwrap_or_else(|| panic!("handle-less {reason} must enqueue"));
            assert_eq!(enqueue.handle, Handle::from([9; 32]));
            assert_eq!(enqueue.allow_event as i16, allow_event as i16);
        }
        // Reasons whose handle is always carried in the instruction args stay
        // rejected without one.
        assert!(decrypt_enqueue_for_fetch(
            &acl_record_job("handle_superseded", None),
            &witness_owned_by(HOST_PROGRAM, [9; 32], 1),
            Some(HOST_PROGRAM),
        )
        .is_none());
    }

    #[test]
    fn encrypted_value_instruction_reasons_release_the_expected_allow_kind() {
        // RFC-024 EncryptedValue instruction-decode reasons (create/update/
        // allow_subjects -> durable account allow; make_handle_public -> public
        // decrypt), same trust guard as the legacy finalized-account reasons above.
        for reason in [
            "encrypted_value_created",
            "handle_superseded",
            "subject_allowed",
        ] {
            let handle = Handle::from([6; 32]);
            let job = acl_record_job(reason, Some(handle));
            let enqueue = decrypt_enqueue_for_fetch(
                &job,
                &witness_owned_by(HOST_PROGRAM, [6; 32], 1),
                Some(HOST_PROGRAM),
            )
            .unwrap_or_else(|| panic!("{reason} must enqueue"));
            assert_eq!(enqueue.handle, handle);
            assert_eq!(
                enqueue.allow_event as i16,
                AllowEvents::AllowedAccount as i16
            );
        }

        let handle = Handle::from([7; 32]);
        let job = acl_record_job("handle_made_public", Some(handle));
        let enqueue = decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [7; 32], 0),
            Some(HOST_PROGRAM),
        )
        .expect("handle_made_public must enqueue");
        assert_eq!(enqueue.handle, handle);
        assert_eq!(
            enqueue.allow_event as i16,
            AllowEvents::AllowedForDecryption as i16
        );
    }

    #[test]
    fn current_handle_claim_with_mismatched_account_handle_is_refused() {
        let job = acl_record_job(
            "encrypted_value_created",
            Some(Handle::from([6; 32])),
        );
        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [9; 32], 1),
            Some(HOST_PROGRAM),
        )
        .is_none());
    }

    #[test]
    fn historical_superseded_claim_requires_advanced_leaf_count() {
        let job =
            acl_record_job("handle_superseded", Some(Handle::from([6; 32])));

        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [9; 32], 1),
            Some(HOST_PROGRAM),
        )
        .is_some());
        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [9; 32], 0),
            Some(HOST_PROGRAM),
        )
        .is_none());
    }

    #[test]
    fn material_sealed_encrypted_value_fetch_does_not_release_a_handle() {
        // handle_material_sealed emits an EncryptedValue witness fetch, but the
        // allow it confirms already arrived through its own event; it must not
        // enqueue.
        let job = acl_record_job(
            "handle_material_sealed",
            Some(Handle::from([6; 32])),
        );
        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [6; 32], 0),
            Some(HOST_PROGRAM),
        )
        .is_none());
    }

    #[test]
    fn non_acl_record_kinds_do_not_release_a_handle() {
        let mut job = acl_record_job(
            "public_decrypt_allowed",
            Some(Handle::from([6; 32])),
        );
        for kind in [
            SolanaFinalizedAccountFetchKind::AclPermission,
            SolanaFinalizedAccountFetchKind::HandleMaterialCommitment,
            SolanaFinalizedAccountFetchKind::DisclosureRequest,
            SolanaFinalizedAccountFetchKind::BurnRedemptionRequest,
            SolanaFinalizedAccountFetchKind::TokenMint,
            SolanaFinalizedAccountFetchKind::TokenAccount,
            SolanaFinalizedAccountFetchKind::BurnRedemption,
            SolanaFinalizedAccountFetchKind::TransferCallbackSettlement,
        ] {
            job.kind = kind;
            assert!(
                decrypt_enqueue_for_fetch(
                    &job,
                    &witness_owned_by(HOST_PROGRAM, [6; 32], 0),
                    Some(HOST_PROGRAM),
                )
                .is_none(),
                "{kind:?} must not release a handle"
            );
        }
    }

    #[test]
    fn allow_without_a_handle_does_not_enqueue() {
        let job = acl_record_job("public_decrypt_allowed", None);
        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by(HOST_PROGRAM, [5; 32], 0),
            Some(HOST_PROGRAM),
        )
        .is_none());
    }

    #[test]
    fn wrong_owner_is_refused_when_program_id_is_configured() {
        // A finalized account at the right address but owned by another program
        // (e.g. an attacker-funded look-alike) must not release the handle.
        let job = acl_record_job(
            "public_decrypt_allowed",
            Some(Handle::from([5; 32])),
        );
        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by([9; 32], [5; 32], 0),
            Some(HOST_PROGRAM),
        )
        .is_none());
    }

    #[test]
    fn owner_check_is_skipped_when_no_program_id_is_configured() {
        let job = acl_record_job(
            "public_decrypt_allowed",
            Some(Handle::from([5; 32])),
        );
        assert!(decrypt_enqueue_for_fetch(
            &job,
            &witness_owned_by([9; 32], [5; 32], 0),
            None,
        )
        .is_some());
    }

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
        let err =
            decode_account_data(vec!["00".to_owned(), "base58".to_owned()])
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
