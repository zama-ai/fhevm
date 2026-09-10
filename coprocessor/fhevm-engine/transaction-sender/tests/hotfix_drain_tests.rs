//! Regression case 2 of the minimal v0.13 hotfix plan: the batch drain must
//! also apply to *preparation* errors.
//!
//! > Fail preparation after an earlier row was accepted and delay that earlier
//! > response. Assert the batch does not return until the started task finishes
//! > its handling. Include a later `BackendGone` after an earlier nonfatal error
//! > to verify fatal-error precedence.
//!
//! `ops` is private, so `execute()` cannot be called directly from an
//! integration test. Instead these tests assert the observable consequence the
//! amendment exists to prevent, which is also the signature seen in the field:
//! a transaction mined on-chain while its row never reached a DB transition, so
//! the row is sent a second time. The proxy holds the send response until after
//! the transaction is mined, which makes that outcome deterministic instead of a
//! race.

mod common;
mod support;

use std::time::Duration;

use alloy::network::TxSigner;
use alloy::providers::ext::AnvilApi;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use rand::random;
use serial_test::serial;
use support::{Fault, FaultProxy};
use test_harness::db_utils::insert_random_keys_and_host_chain;
use tokio::time::sleep;
use transaction_sender::{
    ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

const USER: &str = "0x1234567890abcdef1234567890abcdef12345678";
const CONTRACT: &str = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd";

async fn wait_until<F, Fut>(what: &str, limit: Duration, mut probe: F) -> anyhow::Result<()>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<bool>>,
{
    let deadline = tokio::time::Instant::now() + limit;
    loop {
        if probe().await? {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            anyhow::bail!("timed out after {limit:?} waiting for: {what}");
        }
        sleep(Duration::from_millis(200)).await;
    }
}

/// A malformed later row must not cancel an earlier accepted send: the earlier
/// row reaches its DB transition, so it is never sent twice.
#[tokio::test]
#[serial(db)]
async fn add_ciphertext_preparation_error_drains_started_sends() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let ciphertext_commits = CiphertextCommits::deploy(&provider_deploy, false).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_http(proxy.url()),
        Some(env.signer_address()),
    );

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Good row first by created_at, malformed row second: a 31-byte handle
    // fails `try_into_array::<32>` during preparation.
    let good_handle = random::<[u8; 32]>().to_vec();
    let bad_handle = random::<[u8; 31]>().to_vec();
    for (handle, age_secs) in [(good_handle.clone(), 10), (bad_handle.clone(), 0)] {
        sqlx::query(
            "INSERT INTO ciphertext_digest
                (host_chain_id, key_id_gw, handle, ciphertext, ciphertext128, created_at)
             VALUES ($1, $2, $3, $4, $5, now() - ($6 || ' seconds')::interval)",
        )
        .bind(host_chain_id)
        .bind(key_id.to_vec())
        .bind(handle)
        .bind(random::<[u8; 32]>().to_vec())
        .bind(random::<[u8; 32]>().to_vec())
        .bind(age_secs.to_string())
        .execute(&env.db_pool)
        .await?;
    }

    // Hold the send response until well after the transaction is mined, so the
    // pre-hotfix cancellation window is wide and deterministic.
    proxy.set_fault(
        "eth_sendRawTransaction",
        Fault::DelayResponse(Duration::from_secs(3)),
        Some(1),
    );

    let start_nonce = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    let conf = ConfigSettings {
        // Long enough that the held response still arrives inside the deadline.
        send_txn_sync_timeout_secs: 30,
        ..env.conf.clone()
    };
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        conf,
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let pool = env.db_pool.clone();
    let h = good_handle.clone();
    wait_until(
        "the good row to be marked sent",
        Duration::from_secs(90),
        || {
            let pool = pool.clone();
            let h = h.clone();
            async move {
                let sent: Option<bool> = sqlx::query_scalar(
                    "SELECT txn_is_sent FROM ciphertext_digest WHERE handle = $1",
                )
                .bind(h)
                .fetch_optional(&pool)
                .await?;
                Ok(sent.unwrap_or(false))
            }
        },
    )
    .await?;

    // The decisive assertion: exactly one transaction was submitted for that
    // row. Pre-hotfix the accepted send was cancelled by the sibling
    // preparation error, leaving on-chain work with no DB record, and the row
    // was then sent again.
    let nonce = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;
    assert_eq!(
        nonce - start_nonce,
        1,
        "expected exactly one on-chain transaction for the accepted row, \
         saw {} (a second send means the first was cancelled before its DB transition)",
        nonce - start_nonce
    );
    assert_eq!(
        proxy.calls("eth_sendRawTransaction"),
        1,
        "the accepted send must not be reissued"
    );

    // The malformed row is still unsent and still blocks nothing else.
    let bad_sent: Option<bool> =
        sqlx::query_scalar("SELECT txn_is_sent FROM ciphertext_digest WHERE handle = $1")
            .bind(bad_handle)
            .fetch_optional(&env.db_pool)
            .await?;
    assert_eq!(bad_sent, Some(false), "malformed row must remain unsent");

    env.cancel_token.cancel();
    let _ = run_handle.await;
    Ok(())
}

/// Same shape on the verify-proof path. The malformed row here is an
/// unparsable `user_address`, which the amendment converted from an `expect`
/// (a panic that took the whole operation down) into a handled preparation
/// error that still drains started sends.
#[tokio::test]
#[serial(db)]
async fn verify_proof_preparation_error_drains_started_sends() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let input_verification =
        InputVerification::deploy(&provider_deploy, false, false, false, false).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_http(proxy.url()),
        Some(env.signer_address()),
    );

    // Rows are selected `ORDER BY zk_proof_id`, so the good row is prepared and
    // spawned first and the malformed one fails preparation afterwards.
    let good_id: i64 = 1;
    let bad_id: i64 = 2;
    sqlx::query(
        "INSERT INTO verify_proofs
            (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
         VALUES ($1, 1, $2, $3, $4, true), ($5, 1, $2, 'not-an-address', $4, true)",
    )
    .bind(good_id)
    .bind(CONTRACT)
    .bind(USER)
    .bind(random::<[u8; 32]>().to_vec())
    .bind(bad_id)
    .execute(&env.db_pool)
    .await?;

    proxy.set_fault(
        "eth_sendRawTransaction",
        Fault::DelayResponse(Duration::from_secs(3)),
        Some(1),
    );

    let start_nonce = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    let conf = ConfigSettings {
        send_txn_sync_timeout_secs: 30,
        ..env.conf.clone()
    };
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        *input_verification.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        conf,
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let pool = env.db_pool.clone();
    wait_until(
        "the good proof to be retired",
        Duration::from_secs(90),
        || {
            let pool = pool.clone();
            async move {
                let n: i64 =
                    sqlx::query_scalar("SELECT count(*) FROM verify_proofs WHERE zk_proof_id = $1")
                        .bind(good_id)
                        .fetch_one(&pool)
                        .await?;
                Ok(n == 0)
            }
        },
    )
    .await?;

    let nonce = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;
    assert_eq!(
        nonce - start_nonce,
        1,
        "expected exactly one on-chain verify transaction, saw {}",
        nonce - start_nonce
    );

    // The operation loop survived the malformed row instead of panicking.
    assert!(
        !run_handle.is_finished(),
        "the verify-proof loop must not die on an unparsable address"
    );

    env.cancel_token.cancel();
    let _ = run_handle.await;
    Ok(())
}

/// Fatal-error precedence: a batch that has already collected a nonfatal
/// preparation error must still surface a `BackendGone` raised by a drained
/// task, so the existing stop-and-restart behaviour keeps working.
///
/// Deterministic by construction: mining is paused, so the good row's send is
/// still awaiting its receipt when the backend is dropped, while the malformed
/// row behind it has already produced the nonfatal preparation error.
#[tokio::test]
#[serial(db)]
async fn backend_gone_takes_precedence_over_nonfatal_batch_errors() -> anyhow::Result<()> {
    let mut env = TestEnvironment::new_with_config(
        SignerType::PrivateKey,
        ConfigSettings {
            // The send must still be in flight when anvil goes away.
            send_txn_sync_timeout_secs: 120,
            ..ConfigSettings::default()
        },
        false,
    )
    .await?;

    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    // Deploy while mining still works, then pause it so the send below parks.
    let ciphertext_commits = CiphertextCommits::deploy(&provider_deploy, false).await?;
    provider_deploy.anvil_set_interval_mining(0).await?;
    provider_deploy.anvil_set_auto_mine(false).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.signer_address()),
    );

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;
    // Good row first, malformed row second: the malformed one yields the
    // nonfatal preparation error while the good one's send is parked.
    for (handle, age_secs) in [
        (random::<[u8; 32]>().to_vec(), 10),
        (random::<[u8; 31]>().to_vec(), 0),
    ] {
        sqlx::query(
            "INSERT INTO ciphertext_digest
                (host_chain_id, key_id_gw, handle, ciphertext, ciphertext128, created_at)
             VALUES ($1, $2, $3, $4, $5, now() - ($6 || ' seconds')::interval)",
        )
        .bind(host_chain_id)
        .bind(key_id.to_vec())
        .bind(handle)
        .bind(random::<[u8; 32]>().to_vec())
        .bind(random::<[u8; 32]>().to_vec())
        .bind(age_secs.to_string())
        .execute(&env.db_pool)
        .await?;
    }

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // The batch is now open: one send parked on an unmined transaction, one
    // preparation error already recorded.
    sleep(Duration::from_secs(3)).await;
    assert!(
        !run_handle.is_finished(),
        "the batch should still be draining a parked send"
    );
    env.drop_anvil();

    let outcome = tokio::time::timeout(Duration::from_secs(60), run_handle).await;
    let err = match outcome {
        Ok(joined) => joined?.expect_err("sender must stop on BackendGone"),
        Err(_) => anyhow::bail!("sender did not stop after the backend went away"),
    };
    assert!(
        transaction_sender::is_backend_gone(&err),
        "the drained BackendGone must win over the earlier nonfatal \
         preparation error, got: {err}"
    );

    Ok(())
}

/// Documents a limitation the hotfix does *not* address, present identically
/// before and after: a row whose preparation fails without any DB transition is
/// re-selected forever at the same position, so every row ordered after it is
/// starved. `retry_count` is never incremented for preparation failures, so
/// nothing ages the bad row out.
#[tokio::test]
#[serial(db)]
async fn malformed_row_starves_later_rows_in_the_same_batch() -> anyhow::Result<()> {
    let env = TestEnvironment::new(SignerType::PrivateKey).await?;

    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let input_verification =
        InputVerification::deploy(&provider_deploy, false, false, false, false).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.signer_address()),
    );

    // Malformed row *first* in `zk_proof_id` order; a healthy row behind it.
    sqlx::query(
        "INSERT INTO verify_proofs
            (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
         VALUES (1, 1, $1, 'not-an-address', $2, true), (2, 1, $1, $3, $2, true)",
    )
    .bind(CONTRACT)
    .bind(random::<[u8; 32]>().to_vec())
    .bind(USER)
    .execute(&env.db_pool)
    .await?;

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        *input_verification.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    sleep(Duration::from_secs(20)).await;

    let healthy_still_there: i64 =
        sqlx::query_scalar("SELECT count(*) FROM verify_proofs WHERE zk_proof_id = 2")
            .fetch_one(&env.db_pool)
            .await?;
    let bad_retry_count: i32 =
        sqlx::query_scalar("SELECT retry_count FROM verify_proofs WHERE zk_proof_id = 1")
            .fetch_one(&env.db_pool)
            .await?;

    assert_eq!(
        healthy_still_there, 1,
        "KNOWN LIMITATION no longer reproduces: the healthy row behind a \
         malformed one was processed. Update this test and the release notes."
    );
    assert_eq!(
        bad_retry_count, 0,
        "preparation failures do not increment retry_count, so the malformed \
         row is never aged out (retry_count = {bad_retry_count})"
    );

    env.cancel_token.cancel();
    let _ = run_handle.await;
    Ok(())
}

/// DB/chain consistency under a preparation error, the plan's gate 8: *every*
/// mined transaction must have a DB transition.
///
/// Runs on a multi-threaded runtime, as production does, so the spawned sends
/// really are in flight when a later row fails preparation. Preparation of the
/// verify path awaits a signature per row, which yields to the runtime and lets
/// the earlier sends reach the mempool — the exact ordering that produced
/// on-chain transactions with no DB record in the field.
///
/// The malformed row here is a `verified` proof with NULL `handles`, which is a
/// preparation error rather than a panic, so it exercises the drain itself.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[serial(db)]
async fn preparation_error_leaves_no_orphaned_verify_transactions() -> anyhow::Result<()> {
    const GOOD_PROOFS: i64 = 20;

    let env = TestEnvironment::new(SignerType::PrivateKey).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let input_verification =
        InputVerification::deploy(&provider_deploy, false, false, false, false).await?;

    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_http(proxy.url()),
        Some(env.signer_address()),
    );

    for id in 1..=GOOD_PROOFS {
        sqlx::query(
            "INSERT INTO verify_proofs
                (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
             VALUES ($1, 1, $2, $3, $4, true)",
        )
        .bind(id)
        .bind(CONTRACT)
        .bind(USER)
        .bind(random::<[u8; 32]>().to_vec())
        .execute(&env.db_pool)
        .await?;
    }
    // Prepared last, and fails: `handles field is None`.
    sqlx::query(
        "INSERT INTO verify_proofs
            (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
         VALUES ($1, 1, $2, $3, NULL, true)",
    )
    .bind(GOOD_PROOFS + 1)
    .bind(CONTRACT)
    .bind(USER)
    .execute(&env.db_pool)
    .await?;

    let start_nonce = provider.get_transaction_count(env.signer_address()).await?;

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        *input_verification.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // All healthy proofs must eventually be retired.
    let pool = env.db_pool.clone();
    wait_until(
        "every healthy proof to be retired",
        Duration::from_secs(180),
        || {
            let pool = pool.clone();
            async move {
                let left: i64 = sqlx::query_scalar(
                    "SELECT count(*) FROM verify_proofs WHERE zk_proof_id <= $1",
                )
                .bind(GOOD_PROOFS)
                .fetch_one(&pool)
                .await?;
                Ok(left == 0)
            }
        },
    )
    .await?;

    let nonce = provider.get_transaction_count(env.signer_address()).await?;
    let submitted = nonce - start_nonce;

    // One transaction per retired proof: no orphans, no resends.
    assert_eq!(
        submitted, GOOD_PROOFS as u64,
        "expected exactly one mined transaction per retired proof ({GOOD_PROOFS}), \
         saw {submitted}: a surplus means a send was cancelled after reaching the \
         mempool and then reissued"
    );

    // The malformed row is still there and the loop is still alive.
    let bad_left: i64 =
        sqlx::query_scalar("SELECT count(*) FROM verify_proofs WHERE zk_proof_id = $1")
            .bind(GOOD_PROOFS + 1)
            .fetch_one(&env.db_pool)
            .await?;
    assert_eq!(bad_left, 1, "the malformed row should remain");
    assert!(
        !run_handle.is_finished(),
        "the verify-proof loop must survive the preparation error"
    );

    env.cancel_token.cancel();
    let _ = run_handle.await;
    Ok(())
}
