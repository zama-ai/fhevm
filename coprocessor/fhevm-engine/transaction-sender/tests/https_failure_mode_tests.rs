//! Failure modes of the v0.13.4 submission design over HTTP.
//!
//! The design allocates a nonce under a brief lock, releases it, then submits
//! concurrently. Submissions therefore reach the node out of order, and a
//! failure leaves a gap that later nonces sit behind. These tests drive the
//! real sender through a fault-injecting proxy in front of Anvil and check what
//! happens to the work, not just to the transaction.

mod common;
mod support;

use std::time::Duration;

use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::sol;
use rand::random;
use rstest::rstest;
use serial_test::serial;
use tokio::time::sleep;
use transaction_sender::{
    gateway_http_client, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

use common::{SignerType, TestEnvironment};
use support::{Fault, FaultProxy};
use test_harness::db_utils::{insert_ciphertext_digest, insert_random_keys_and_host_chain};

sol!(
    #[sol(rpc)]
    CiphertextCommits,
    "artifacts/CiphertextCommits.sol/CiphertextCommits.json"
);

/// Builds the sender's provider pointed at the proxy rather than at Anvil.
fn provider_via(
    env: &TestEnvironment,
    proxy: &FaultProxy,
) -> anyhow::Result<NonceManagedProvider<alloy::providers::DynProvider>> {
    use alloy::network::TxSigner as _;
    let url = proxy.url();
    let inner = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(env.wallet.clone())
        .connect_reqwest(gateway_http_client(&url)?, url)
        .erased();
    Ok(NonceManagedProvider::new(inner, Some(env.signer.address())))
}

/// A submission whose response is lost is an *ambiguous acceptance*: the
/// transaction is on chain, the caller saw an error. The design resets its
/// cached nonce on any send error, so the next allocation re-reads the pending
/// count. This checks that the sequence recovers and no digest is stranded.
#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn lost_submission_response_does_not_strand_work(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_batch_limit: 10,
        add_ciphertexts_max_retries: i32::MAX,
        graceful_shutdown_timeout: Duration::from_secs(2),
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(signer_type, conf, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = env.http_provider()?;
    let provider = provider_via(&env, &proxy)?;
    let ciphertext_commits = CiphertextCommits::deploy(&provider_deploy, false).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        alloy::signers::local::PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });
    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Lose exactly one submission response. The transaction still executes.
    proxy.set_fault(
        "eth_sendRawTransactionSync",
        Fault::SuppressResponse,
        Some(1),
    );

    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = random::<[u8; 32]>();
        insert_ciphertext_digest(
            &env.db_pool,
            host_chain_id,
            key_id,
            &handle,
            &random::<[u8; 32]>(),
            &random::<[u8; 32]>(),
            1,
        )
        .await?;
        handles.push(handle);
    }
    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.add_ciphertexts_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Every digest must end up sent, including the one whose response was lost.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(90);
    loop {
        // Runtime-checked: this shape is not in the offline query cache.
        let pending: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM ciphertext_digest WHERE txn_is_sent = false")
                .fetch_one(&env.db_pool)
                .await?;
        if pending == 0 {
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "{pending} digest(s) still unsent after 90 s"
        );
        sleep(Duration::from_millis(250)).await;
    }

    let signer_addr = {
        use alloy::network::TxSigner as _;
        env.signer.address()
    };
    let latest = provider_deploy.get_transaction_count(signer_addr).await?;
    let pending = provider_deploy
        .get_transaction_count(signer_addr)
        .pending()
        .await?;
    assert_eq!(
        latest, pending,
        "the nonce sequence must be gap-free at rest ({pending} pending vs {latest} mined)"
    );

    env.cancel_token.cancel();
    let _ = tokio::time::timeout(Duration::from_secs(10), run_handle).await;
    Ok(())
}

/// A submission that is simply slow, past the deadline. The design's timeout
/// abandons the wait while the transaction may still be mined, so the nonce is
/// consumed. This measures whether the batch loop recovers or stalls behind the
/// abandoned nonce.
#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn submission_past_the_deadline_does_not_stall_the_batch_loop(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_batch_limit: 10,
        add_ciphertexts_max_retries: i32::MAX,
        send_txn_sync_timeout_secs: 2,
        graceful_shutdown_timeout: Duration::from_secs(2),
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(signer_type, conf, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = env.http_provider()?;
    let provider = provider_via(&env, &proxy)?;
    let ciphertext_commits = CiphertextCommits::deploy(&provider_deploy, false).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        alloy::signers::local::PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });
    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Hold two responses past the 2 s deadline; the transactions still land.
    proxy.set_fault(
        "eth_sendRawTransactionSync",
        Fault::DelayResponse(Duration::from_secs(4)),
        Some(2),
    );

    for _ in 0..10 {
        let handle = random::<[u8; 32]>();
        insert_ciphertext_digest(
            &env.db_pool,
            host_chain_id,
            key_id,
            &handle,
            &random::<[u8; 32]>(),
            &random::<[u8; 32]>(),
            1,
        )
        .await?;
    }
    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.add_ciphertexts_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(120);
    loop {
        // Runtime-checked: this shape is not in the offline query cache.
        let pending: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM ciphertext_digest WHERE txn_is_sent = false")
                .fetch_one(&env.db_pool)
                .await?;
        if pending == 0 {
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "{pending} digest(s) still unsent after 120 s; the loop did not recover"
        );
        sleep(Duration::from_millis(250)).await;
    }

    println!(
        "sendRawTransactionSync calls seen by the proxy: {}",
        proxy.calls("eth_sendRawTransactionSync")
    );

    env.cancel_token.cancel();
    let _ = tokio::time::timeout(Duration::from_secs(10), run_handle).await;
    Ok(())
}

/// Sanity check on the environment itself: the ciphertext balance is untouched,
/// so a failing assertion above is about the sender, not about funding.
#[tokio::test]
async fn proxy_forwards_normally_when_no_fault_is_set() -> anyhow::Result<()> {
    let anvil = alloy::node_bindings::Anvil::new().try_spawn()?;
    let proxy = FaultProxy::start(anvil.endpoint_url()).await?;
    let url = proxy.url();
    let provider = ProviderBuilder::new().connect_reqwest(gateway_http_client(&url)?, url);
    assert_eq!(provider.get_chain_id().await?, anvil.chain_id());
    assert!(provider.get_balance(anvil.addresses()[0]).await? > U256::ZERO);
    assert_eq!(proxy.calls("eth_chainId"), 1);
    Ok(())
}

/// A Gateway that is slow rather than absent.
///
/// The `Queue` fault serves matching requests through one channel, so latency
/// grows with concurrency — the same shape that made the WebSocket transport
/// fail, but applied to the endpoint itself. At a 200 ms service time and ten
/// concurrent submissions the last one waits 2 s, so a 4 s deadline holds and a
/// 1 s deadline does not. This checks which way the work goes when it does not.
#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn a_congested_gateway_retries_rather_than_loses_ciphertext_work(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_batch_limit: 10,
        add_ciphertexts_max_retries: i32::MAX,
        // Deliberately too short for the congestion injected below.
        send_txn_sync_timeout_secs: 1,
        graceful_shutdown_timeout: Duration::from_secs(2),
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(signer_type, conf, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;

    let provider_deploy = env.http_provider()?;
    let provider = provider_via(&env, &proxy)?;
    let ciphertext_commits = CiphertextCommits::deploy(&provider_deploy, false).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        alloy::signers::local::PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let run_handle = tokio::spawn(async move { txn_sender.run().await });
    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Congest submission: one channel, 300 ms each. Ten concurrent submissions
    // therefore finish between 0.3 s and 3 s, straddling the 1 s deadline.
    proxy.set_fault(
        "eth_sendRawTransactionSync",
        Fault::Queue {
            service: Duration::from_millis(300),
        },
        None,
    );

    for _ in 0..10 {
        let handle = random::<[u8; 32]>();
        insert_ciphertext_digest(
            &env.db_pool,
            host_chain_id,
            key_id,
            &handle,
            &random::<[u8; 32]>(),
            &random::<[u8; 32]>(),
            1,
        )
        .await?;
    }
    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.add_ciphertexts_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Let the congested phase run, then relieve it.
    sleep(Duration::from_secs(12)).await;
    let during = proxy.calls("eth_sendRawTransactionSync");
    proxy.clear_all_faults();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(90);
    loop {
        let pending: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM ciphertext_digest WHERE txn_is_sent = false")
                .fetch_one(&env.db_pool)
                .await?;
        if pending == 0 {
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "{pending} digest(s) still unsent 90 s after congestion cleared"
        );
        sleep(Duration::from_millis(250)).await;
    }

    let unlimited: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(txn_unlimited_retries_count), 0) FROM ciphertext_digest",
    )
    .fetch_one(&env.db_pool)
    .await?;
    let limited: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(txn_limited_retries_count), 0) FROM ciphertext_digest",
    )
    .fetch_one(&env.db_pool)
    .await?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ciphertext_digest")
        .fetch_one(&env.db_pool)
        .await?;

    println!(
        "congested Gateway: {during} submissions attempted while congested; \
all {total} digests sent; max limited retries {limited}, max unlimited retries {unlimited}"
    );
    assert_eq!(total, 10, "no digest may be dropped by congestion");

    env.cancel_token.cancel();
    let _ = tokio::time::timeout(Duration::from_secs(10), run_handle).await;
    Ok(())
}
