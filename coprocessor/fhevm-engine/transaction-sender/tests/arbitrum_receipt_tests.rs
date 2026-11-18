//! Tests for Gateway receipts after the Arbitrum transaction submission change
//! (fhevm-internal#2384). `eth_sendRawTransactionSync` can return a receipt for
//! an executed transaction before its block is sealed (`blockHash = 0x00..0`).
//! The block can also fail. Work must complete only on a sealed receipt. If the
//! outcome of a submission is unknown, the sender must check its hash before it
//! sends again. This rule also applies after a restart.
//!
//! Each case runs for proof responses and for ciphertext commitments.
mod common;
mod support;

use alloy::network::TxSigner;
use alloy::primitives::{Address, B256};
use alloy::providers::{Provider, ProviderBuilder};
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use rand::random;
use rstest::rstest;
use serial_test::serial;
use std::time::Duration;
use support::{Fault, FaultProxy};
use test_harness::db_utils::{insert_ciphertext_digest, insert_random_keys_and_host_chain};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use transaction_sender::{
    gateway_http_client, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

const SEND: &str = "eth_sendRawTransactionSync";
const RECEIPT: &str = "eth_getTransactionReceipt";
const WAIT: Duration = Duration::from_secs(60);

#[derive(Clone, Copy, Debug)]
enum Kind {
    Proof,
    Ciphertext,
}

/// How the mock Gateway contracts answer a submission.
#[derive(Clone, Copy)]
enum Gateway {
    Accepts,
    /// Reverts with `CoprocessorAlreadyVerified` or `CoprocessorAlreadyAdded`.
    AlreadyDone,
    /// Reverts on chain. The sealed receipt has a failed status.
    Reverts,
}

struct Job {
    kind: Kind,
    proof_id: i64,
    handle: [u8; 32],
}

#[derive(Debug)]
struct JobState {
    done: bool,
    /// The number of limited retries used (`retry_count` or `txn_limited_retries_count`).
    spent: i32,
    pending: Option<Vec<u8>>,
}

struct Harness {
    env: TestEnvironment,
    proxy: FaultProxy,
    input_verification: Address,
    ciphertext_commits: Address,
    gas: Option<u64>,
}

impl Harness {
    async fn new(gateway: Gateway) -> anyhow::Result<Self> {
        let conf = ConfigSettings {
            send_txn_sync_timeout_secs: 1,
            db_polling_interval_secs: 1,
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 1,
            graceful_shutdown_timeout: Duration::from_secs(2),
            ..Default::default()
        };
        let env = TestEnvironment::new_with_config(SignerType::PrivateKey, conf, false).await?;
        let deploy = env.http_provider()?;
        let (already_verified, other_revert, already_added) = match gateway {
            Gateway::Accepts => (false, false, false),
            Gateway::AlreadyDone => (true, false, true),
            Gateway::Reverts => (false, true, true),
        };
        let input =
            InputVerification::deploy(&deploy, already_verified, false, false, other_revert)
                .await?;
        let ciphertext = CiphertextCommits::deploy(&deploy, already_added).await?;
        let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
        Ok(Self {
            input_verification: *input.address(),
            ciphertext_commits: *ciphertext.address(),
            // With a fixed gas limit, the sender does not estimate gas. Anvil then
            // mines the reverting call, and the receipt has a failed status.
            gas: matches!(gateway, Gateway::Reverts).then_some(1_000_000),
            env,
            proxy,
        })
    }

    /// Starts a sender that connects to the Gateway through the fault proxy.
    async fn start_sender(&self) -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
        let url = self.proxy.url();
        let inner = ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(self.env.wallet.clone())
            .connect_reqwest(gateway_http_client(&url)?, url);
        let provider =
            NonceManagedProvider::new(inner, Some(self.env.wallet.default_signer().address()));
        let sender = TransactionSender::new(
            self.env.db_pool.clone(),
            self.input_verification,
            self.ciphertext_commits,
            self.env.signer.clone(),
            provider,
            CancellationToken::new(),
            self.env.conf.clone(),
            self.gas,
        )
        .await?;
        Ok(tokio::spawn(async move { sender.run().await }))
    }

    async fn nonce(&self) -> anyhow::Result<u64> {
        Ok(self
            .env
            .http_provider()?
            .get_transaction_count(self.env.signer.address())
            .await?)
    }

    async fn insert(&self, kind: Kind, pending: Option<B256>) -> anyhow::Result<Job> {
        let job = Job {
            kind,
            proof_id: i64::from(random::<u32>()),
            handle: random(),
        };
        let pool = &self.env.db_pool;
        let pending = pending.map(|hash| hash.to_vec());
        match kind {
            Kind::Proof => {
                sqlx::query(
                    "INSERT INTO verify_proofs
                     (zk_proof_id, chain_id, contract_address, user_address, handles, verified, pending_txn_hash)
                     VALUES ($1, 42, $2, $3, $4, TRUE, $5)",
                )
                .bind(job.proof_id)
                .bind(self.env.contract_address.to_string())
                .bind(self.env.user_address.to_string())
                .bind(vec![1u8; 64])
                .bind(pending)
                .execute(pool)
                .await?;
            }
            Kind::Ciphertext => {
                let (host_chain_id, key_id) = insert_random_keys_and_host_chain(pool).await?;
                insert_ciphertext_digest(
                    pool,
                    host_chain_id,
                    key_id,
                    &job.handle,
                    &random::<[u8; 32]>(),
                    &random::<[u8; 32]>(),
                    0,
                )
                .await?;
                sqlx::query("UPDATE ciphertext_digest SET pending_txn_hash = $1 WHERE handle = $2")
                    .bind(pending)
                    .bind(job.handle.as_slice())
                    .execute(pool)
                    .await?;
            }
        }
        Ok(job)
    }

    async fn state(&self, job: &Job) -> anyhow::Result<JobState> {
        let pool = &self.env.db_pool;
        Ok(match job.kind {
            Kind::Proof => {
                let row: Option<(i32, Option<Vec<u8>>)> = sqlx::query_as(
                    "SELECT retry_count, pending_txn_hash FROM verify_proofs WHERE zk_proof_id = $1",
                )
                .bind(job.proof_id)
                .fetch_optional(pool)
                .await?;
                match row {
                    // When a verified proof response completes, the sender deletes its row.
                    None => JobState {
                        done: true,
                        spent: 0,
                        pending: None,
                    },
                    Some((spent, pending)) => JobState {
                        done: false,
                        spent,
                        pending,
                    },
                }
            }
            Kind::Ciphertext => {
                let (done, spent, pending): (bool, i32, Option<Vec<u8>>) = sqlx::query_as(
                    "SELECT txn_is_sent, txn_limited_retries_count, pending_txn_hash
                     FROM ciphertext_digest WHERE handle = $1",
                )
                .bind(job.handle.as_slice())
                .fetch_one(pool)
                .await?;
                JobState {
                    done,
                    spent,
                    pending,
                }
            }
        })
    }

    async fn wait_for(
        &self,
        job: &Job,
        what: &str,
        until: impl Fn(&JobState) -> bool,
    ) -> anyhow::Result<JobState> {
        let deadline = tokio::time::Instant::now() + WAIT;
        loop {
            let state = self.state(job).await?;
            if until(&state) {
                return Ok(state);
            }
            assert!(
                tokio::time::Instant::now() < deadline,
                "timed out waiting for {what}: {state:?}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn unsealed_receipt_completes_once_sealed(#[case] kind: Kind) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Accepts).await?;
    h.proxy.set_fault(SEND, Fault::UnsealedReceipt, None);
    let sender = h.start_sender().await?;

    let job = h.insert(kind, None).await?;
    let state = h.wait_for(&job, "completion", |s| s.done).await?;

    assert_eq!(state.spent, 0);
    assert_eq!(h.proxy.calls(SEND), 1, "must not resubmit");
    assert!(h.proxy.calls(RECEIPT) >= 1, "must fetch the sealed receipt");
    sender.abort();
    Ok(())
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn unsealed_failed_receipt_counts_as_failure(#[case] kind: Kind) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Reverts).await?;
    h.proxy.set_fault(SEND, Fault::UnsealedReceipt, None);
    let sender = h.start_sender().await?;

    let job = h.insert(kind, None).await?;
    let state = h.wait_for(&job, "a failure", |s| s.spent >= 1).await?;

    assert!(!state.done);
    assert_eq!(state.pending, None, "a sealed failure is settled");
    sender.abort();
    Ok(())
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn retracted_receipt_is_resubmitted_with_recovered_nonce(
    #[case] kind: Kind,
) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Accepts).await?;
    // The first submission executes, but its block is never sealed.
    h.proxy.set_fault(SEND, Fault::RetractedReceipt, Some(1));
    let nonce_before = h.nonce().await?;
    let sender = h.start_sender().await?;

    let job = h.insert(kind, None).await?;
    let state = h
        .wait_for(&job, "the unconfirmed hash", |s| {
            s.pending.is_some() || s.done
        })
        .await?;
    assert!(!state.done, "a retracted receipt must not complete work");
    assert_eq!(state.spent, 0, "an unknown outcome must not spend retries");

    h.wait_for(&job, "completion", |s| s.done).await?;
    assert_eq!(h.proxy.calls(SEND), 2);
    // The sender used the nonce of the retracted transaction again. One
    // transaction landed, and no nonce gap stops later submissions.
    assert_eq!(h.nonce().await?, nonce_before + 1);
    sender.abort();
    Ok(())
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn timed_out_submission_later_sealed_is_not_resubmitted(
    #[case] kind: Kind,
) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Accepts).await?;
    // Anvil mines the transaction, but no sealed receipt is visible before the deadline.
    h.proxy.set_fault(SEND, Fault::UnsealedReceipt, None);
    h.proxy.set_fault(RECEIPT, Fault::Blackhole, None);
    let sender = h.start_sender().await?;

    let job = h.insert(kind, None).await?;
    let state = h
        .wait_for(&job, "the unconfirmed hash", |s| s.pending.is_some())
        .await?;
    assert_eq!(state.spent, 0, "an unknown outcome must not spend retries");
    let pending = B256::try_from(state.pending.unwrap().as_slice())?;
    assert!(
        h.env
            .http_provider()?
            .get_transaction_receipt(pending)
            .await?
            .is_some(),
        "the recorded hash is the transaction that was mined"
    );

    h.proxy.clear_fault(RECEIPT);
    let state = h.wait_for(&job, "completion", |s| s.done).await?;
    assert_eq!(state.spent, 0);
    assert_eq!(h.proxy.calls(SEND), 1, "must settle by hash, not resubmit");
    sender.abort();
    Ok(())
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn unconfirmed_submission_is_settled_after_restart(#[case] kind: Kind) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Accepts).await?;
    h.proxy.set_fault(SEND, Fault::UnsealedReceipt, None);
    h.proxy.set_fault(RECEIPT, Fault::Blackhole, None);
    let sender = h.start_sender().await?;

    let job = h.insert(kind, None).await?;
    h.wait_for(&job, "the unconfirmed hash", |s| s.pending.is_some())
        .await?;
    // Stop the process. The new process must read the hash from the database.
    sender.abort();
    let _ = sender.await;

    h.proxy.clear_fault(RECEIPT);
    let restarted = h.start_sender().await?;
    let state = h.wait_for(&job, "completion", |s| s.done).await?;
    assert_eq!(state.spent, 0);
    assert_eq!(h.proxy.calls(SEND), 1, "must settle by hash, not resubmit");
    restarted.abort();
    Ok(())
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn resubmission_after_unknown_outcome_is_idempotent(
    #[case] kind: Kind,
) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::AlreadyDone).await?;
    let sender = h.start_sender().await?;

    // The recorded transaction was never sealed. But the Gateway already has the
    // submission of this coprocessor, for example from a later retry.
    let job = h
        .insert(kind, Some(B256::from(random::<[u8; 32]>())))
        .await?;
    let state = h.wait_for(&job, "completion", |s| s.done).await?;

    assert_eq!(state.pending, None);
    assert!(h.proxy.calls(RECEIPT) >= 1, "must check the recorded hash");
    assert_eq!(
        h.proxy.calls(SEND),
        0,
        "the duplicate is rejected at estimation"
    );
    sender.abort();
    Ok(())
}

/// A failed submission must not cancel the other submissions in the same
/// ciphertext batch before they record their unconfirmed hashes.
#[tokio::test]
#[serial(db)]
async fn unconfirmed_ciphertext_batch_keeps_every_hash() -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Accepts).await?;
    h.proxy.set_fault(SEND, Fault::UnsealedReceipt, None);
    h.proxy.set_fault(RECEIPT, Fault::Blackhole, None);
    let mut jobs = Vec::new();
    for _ in 0..3 {
        jobs.push(h.insert(Kind::Ciphertext, None).await?);
    }
    // Start the sender after the inserts, so that all three go in one batch.
    let sender = h.start_sender().await?;

    for job in &jobs {
        h.wait_for(job, "the unconfirmed hash", |s| s.pending.is_some())
            .await?;
    }
    h.proxy.clear_fault(RECEIPT);
    for job in &jobs {
        h.wait_for(job, "completion", |s| s.done).await?;
    }
    assert_eq!(h.proxy.calls(SEND), 3, "every submission settles by hash");
    sender.abort();
    Ok(())
}

#[rstest]
#[case::proof(Kind::Proof)]
#[case::ciphertext(Kind::Ciphertext)]
#[tokio::test]
#[serial(db)]
async fn still_unsealed_submission_is_not_resubmitted(#[case] kind: Kind) -> anyhow::Result<()> {
    let h = Harness::new(Gateway::Accepts).await?;
    // Each receipt read shows an executed transaction in a block that is not sealed.
    h.proxy.set_fault(SEND, Fault::UnsealedReceipt, None);
    h.proxy.set_fault(RECEIPT, Fault::UnsealedReceipt, None);
    let sender = h.start_sender().await?;

    let job = h.insert(kind, None).await?;
    let first = h
        .wait_for(&job, "the unconfirmed hash", |s| s.pending.is_some())
        .await?;
    // Wait for some retries to read the unsealed receipt again.
    let reads = h.proxy.calls(RECEIPT);
    tokio::time::sleep(Duration::from_secs(5)).await;
    assert!(
        h.proxy.calls(RECEIPT) > reads,
        "retries must re-check the hash"
    );
    let state = h.state(&job).await?;
    assert_eq!(state.pending, first.pending, "the in-flight hash is kept");
    assert_eq!(state.spent, 0, "an unknown outcome must not spend retries");
    assert_eq!(
        h.proxy.calls(SEND),
        1,
        "an in-flight submission is not resent"
    );

    h.proxy.clear_fault(RECEIPT);
    h.wait_for(&job, "completion", |s| s.done).await?;
    assert_eq!(h.proxy.calls(SEND), 1);
    sender.abort();
    Ok(())
}
