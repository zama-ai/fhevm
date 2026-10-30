//! These regressions run in the normal transaction-sender CI test job.
mod common;
mod support;

use alloy::{
    primitives::Address,
    providers::{DynProvider, Provider, ProviderBuilder},
};
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use rstest::rstest;
use serial_test::serial;
use std::time::Duration;
use support::{Fault, FaultProxy};
use tokio_util::sync::CancellationToken;
use transaction_sender::{
    gateway_http_client, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

async fn sender(
    env: &TestEnvironment,
    proxy: &FaultProxy,
    input: Address,
    ciphertext: Address,
    cancel: CancellationToken,
) -> anyhow::Result<TransactionSender<DynProvider>> {
    let url = proxy.url();
    let inner = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(env.wallet.clone())
        .connect_reqwest(gateway_http_client(&url)?, url)
        .erased();
    TransactionSender::new(
        env.db_pool.clone(),
        input,
        ciphertext,
        env.signer.clone(),
        NonceManagedProvider::new(inner, Some(env.wallet.default_signer().address())),
        cancel,
        env.conf.clone(),
        None,
    )
    .await
}

async fn seed(env: &TestEnvironment, first: i64, count: i64) -> anyhow::Result<()> {
    for id in first..first + count {
        sqlx::query("INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, 42, $2, $3, $4, TRUE)")
            .bind(id).bind(env.contract_address.to_string()).bind(env.user_address.to_string())
            .bind(vec![id as u8; 32]).execute(&env.db_pool).await?;
        test_harness::db_utils::insert_ciphertext_digest(
            &env.db_pool,
            42,
            [0; 32],
            &[id as u8; 32],
            &[1; 32],
            &[2; 32],
            0,
        )
        .await?;
    }
    Ok(())
}

async fn wait_completed(env: &TestEnvironment, expected: i64) -> anyhow::Result<()> {
    tokio::time::timeout(Duration::from_secs(45), async {
        loop {
            let count: i64 = sqlx::query_scalar("SELECT
                (SELECT count(*) FROM verify_proofs WHERE sender_status = 'COMPLETED') +
                (SELECT count(*) FROM ciphertext_digest WHERE sender_status = 'COMPLETED' AND txn_is_sent)")
                .fetch_one(&env.db_pool).await?;
            if count == expected { return anyhow::Ok(()) }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }).await??;
    Ok(())
}

#[rstest]
#[case::interrupted_broadcast(false)]
#[case::lost_response(true)]
#[tokio::test]
#[serial(db)]
async fn restart_reconciles_original_transactions_and_drains_both_queues(
    #[case] accepted: bool,
) -> anyhow::Result<()> {
    let env = TestEnvironment::new_with_config(
        SignerType::PrivateKey,
        ConfigSettings {
            send_txn_sync_timeout_secs: 30,
            db_polling_interval_secs: 1,
            error_sleep_max_secs: 1,
            ..Default::default()
        },
        false,
    )
    .await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let initial_nonce = deploy
        .get_transaction_count(env.wallet.default_signer().address())
        .await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    proxy.set_fault(
        "eth_sendRawTransactionSync",
        if accepted {
            Fault::SuppressResponse
        } else {
            Fault::DiscardAfter(Duration::from_secs(60))
        },
        None,
    );
    seed(&env, 1, 2).await?;
    let first = sender(
        &env,
        &proxy,
        *input.address(),
        *ciphertext.address(),
        CancellationToken::new(),
    )
    .await?;
    let run = tokio::spawn(async move { first.run().await });
    let hashes = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            let hashes: Vec<Vec<u8>> = sqlx::query_scalar("SELECT sender_tx_hash FROM verify_proofs WHERE sender_status = 'SENDING' AND sender_tx_hash IS NOT NULL
                UNION ALL SELECT sender_tx_hash FROM ciphertext_digest WHERE sender_status = 'SENDING' AND sender_tx_hash IS NOT NULL")
                .fetch_all(&env.db_pool).await?;
            if hashes.len() == 4 && proxy.calls("eth_sendRawTransactionSync") >= 4 { break anyhow::Ok(hashes) }
            assert!(!run.is_finished());
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }).await??;
    if accepted {
        tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                let mut all_mined = true;
                for hash in &hashes {
                    all_mined &= deploy
                        .get_transaction_receipt(alloy::primitives::B256::from_slice(hash))
                        .await?
                        .is_some();
                }
                if all_mined {
                    break anyhow::Ok(());
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
        .await??;
    }
    // A second live sender must not reclaim or reset these attempts.
    let competing = sender(
        &env,
        &proxy,
        *input.address(),
        *ciphertext.address(),
        CancellationToken::new(),
    )
    .await?;
    assert!(competing
        .run()
        .await
        .unwrap_err()
        .to_string()
        .contains("another transaction sender"));
    run.abort();
    let _ = run.await;
    // Wait for the aborted run's ownership connection to close; avoid timing assumptions.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let mut connection = env.db_pool.acquire().await?.detach();
            let free: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock(7346182640291)")
                .fetch_one(&mut connection)
                .await?;
            if free {
                break anyhow::Ok(());
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await??;
    proxy.clear_all_faults();
    // Fresh jobs must share the account with restored reservations without consuming their nonces.
    seed(&env, 3, 2).await?;
    let cancel = CancellationToken::new();
    let second = sender(
        &env,
        &proxy,
        *input.address(),
        *ciphertext.address(),
        cancel.clone(),
    )
    .await?;
    let restarted = tokio::spawn(async move { second.run().await });
    wait_completed(&env, 8).await?;
    cancel.cancel();
    restarted.await??;
    for hash in hashes {
        assert!(deploy
            .get_transaction_receipt(alloy::primitives::B256::from_slice(&hash))
            .await?
            .unwrap()
            .status());
    }
    assert_eq!(
        input
            .VerifyProofResponse_filter()
            .from_block(0)
            .query()
            .await?
            .len(),
        4
    );
    assert_eq!(
        deploy
            .get_transaction_count(env.wallet.default_signer().address())
            .await?,
        initial_nonce + 8,
        "recovery must not allocate replacement nonces or duplicate successful work"
    );
    let retained: i64 = sqlx::query_scalar(
        "SELECT
        (SELECT count(*) FROM verify_proofs WHERE sender_raw_tx IS NOT NULL) +
        (SELECT count(*) FROM ciphertext_digest WHERE sender_raw_tx IS NOT NULL)",
    )
    .fetch_one(&env.db_pool)
    .await?;
    assert_eq!(retained, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn immediate_receipt_check_recovers_lost_success_without_resubmitting() -> anyhow::Result<()>
{
    let env = TestEnvironment::new_with_config(
        SignerType::PrivateKey,
        ConfigSettings {
            send_txn_sync_timeout_secs: 2,
            db_polling_interval_secs: 1,
            ..Default::default()
        },
        false,
    )
    .await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    proxy.set_fault("eth_sendRawTransactionSync", Fault::SuppressResponse, None);
    seed(&env, 1, 1).await?;
    let cancel = CancellationToken::new();
    let sender = sender(
        &env,
        &proxy,
        *input.address(),
        *ciphertext.address(),
        cancel.clone(),
    )
    .await?;
    let run = tokio::spawn(async move { sender.run().await });
    wait_completed(&env, 2).await?;
    cancel.cancel();
    run.await??;
    assert_eq!(proxy.calls("eth_sendRawTransactionSync"), 2);
    assert_eq!(
        input
            .VerifyProofResponse_filter()
            .from_block(0)
            .query()
            .await?
            .len(),
        1
    );
    let retries: i32 =
        sqlx::query_scalar("SELECT retry_count FROM verify_proofs WHERE zk_proof_id = 1")
            .fetch_one(&env.db_pool)
            .await?;
    assert_eq!(retries, 0);
    Ok(())
}

#[rstest]
#[case::wrong_initial_nonce("eth_getTransactionCount", Fault::RespondWith(serde_json::json!("0x3e8")), 1)]
#[case::false_nonce_too_low("eth_sendRawTransactionSync", Fault::RpcError { code: -32000, message: "nonce too low".into() }, 4)]
#[case::false_already_known("eth_sendRawTransactionSync", Fault::RpcError { code: -32000, message: "already known".into() }, 4)]
#[case::partial_response("eth_sendRawTransactionSync", Fault::RawResponse { status: 200, body: "{\"jsonrpc\":\"2.0\",\"result\":".into() }, 4)]
#[tokio::test]
#[serial(db)]
async fn rpc_faults_preserve_budget_and_drain_without_nonce_gaps(
    #[case] method: &str,
    #[case] fault: Fault,
    #[case] failures: usize,
) -> anyhow::Result<()> {
    let env = TestEnvironment::new_with_config(
        SignerType::PrivateKey,
        ConfigSettings {
            verify_proof_resp_max_retries: 1,
            add_ciphertexts_max_retries: 1,
            send_txn_sync_timeout_secs: 1,
            db_polling_interval_secs: 1,
            error_sleep_max_secs: 1,
            ..Default::default()
        },
        false,
    )
    .await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let initial_nonce = deploy
        .get_transaction_count(env.wallet.default_signer().address())
        .await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    proxy.set_fault(method, fault, Some(failures));
    seed(&env, 1, 2).await?;
    let cancel = CancellationToken::new();
    let sender = sender(
        &env,
        &proxy,
        *input.address(),
        *ciphertext.address(),
        cancel.clone(),
    )
    .await?;
    let run = tokio::spawn(async move { sender.run().await });
    wait_completed(&env, 4).await?;
    cancel.cancel();
    run.await??;
    assert!(proxy.calls(method) >= failures);
    let failures: i64 = sqlx::query_scalar(
        "SELECT
        (SELECT COALESCE(sum(retry_count), 0) FROM verify_proofs) +
        (SELECT COALESCE(sum(txn_limited_retries_count), 0) FROM ciphertext_digest)",
    )
    .fetch_one(&env.db_pool)
    .await?;
    assert_eq!(failures, 0);
    assert_eq!(
        deploy
            .get_transaction_count(env.wallet.default_signer().address())
            .await?,
        initial_nonce + 4
    );
    assert_eq!(
        input
            .VerifyProofResponse_filter()
            .from_block(0)
            .query()
            .await?
            .len(),
        2
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn legacy_exhausted_rows_become_stopped_while_healthy_work_completes() -> anyhow::Result<()> {
    let env = TestEnvironment::new_with_config(
        SignerType::PrivateKey,
        ConfigSettings {
            verify_proof_resp_max_retries: 1,
            add_ciphertexts_max_retries: 1,
            db_polling_interval_secs: 1,
            ..Default::default()
        },
        false,
    )
    .await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    seed(&env, 1, 2).await?;
    sqlx::query("UPDATE verify_proofs SET retry_count = 1 WHERE zk_proof_id = 1")
        .execute(&env.db_pool)
        .await?;
    sqlx::query("UPDATE ciphertext_digest SET txn_limited_retries_count = 1 WHERE handle = $1")
        .bind(vec![1u8; 32])
        .execute(&env.db_pool)
        .await?;
    let cancel = CancellationToken::new();
    let sender = sender(
        &env,
        &proxy,
        *input.address(),
        *ciphertext.address(),
        cancel.clone(),
    )
    .await?;
    let run = tokio::spawn(async move { sender.run().await });
    wait_completed(&env, 2).await?;
    cancel.cancel();
    run.await??;
    let stopped: i64 = sqlx::query_scalar("SELECT
        (SELECT count(*) FROM verify_proofs WHERE sender_status = 'STOPPED' AND sender_last_error IS NOT NULL) +
        (SELECT count(*) FROM ciphertext_digest WHERE sender_status = 'STOPPED' AND sender_last_error IS NOT NULL)")
        .fetch_one(&env.db_pool).await?;
    assert_eq!(stopped, 2);
    assert_eq!(proxy.calls("eth_sendRawTransactionSync"), 2);
    Ok(())
}
