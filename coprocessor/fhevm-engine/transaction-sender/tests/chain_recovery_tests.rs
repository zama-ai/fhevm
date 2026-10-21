//! Exercise both real sender operations against disposable PostgreSQL and Anvil.
mod common;
mod support;

use alloy::providers::{Provider, ProviderBuilder};
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use fhevm_engine_common::chain_recovery::{
    begin_replay, complete_replay, record_halt, HistoryMismatch, ListenerSource,
};
use fhevm_engine_common::versioning::CUTOVER_LOCK_ID;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use support::{Fault, FaultProxy};
use test_harness::db_utils::{insert_ciphertext_digest, insert_random_keys_and_host_chain};
use transaction_sender::{
    gateway_http_client, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

async fn queue_work(
    env: &TestEnvironment,
    chain: i64,
    key: [u8; 32],
    id: u8,
) -> anyhow::Result<()> {
    insert_ciphertext_digest(
        &env.db_pool,
        chain,
        key,
        &[id; 32],
        &[0xBB; 32],
        &[0xCC; 32],
        1,
    )
    .await?;
    sqlx::query("INSERT INTO verify_proofs (zk_proof_id,chain_id,contract_address,user_address,handles,verified)
        VALUES ($1,$2,$3,$4,$5,true)")
        .bind(i64::from(id)).bind(chain).bind(env.contract_address.to_string())
        .bind(env.user_address.to_string()).bind(vec![id;64]).execute(&env.db_pool).await?;
    Ok(())
}

async fn pending(env: &TestEnvironment, id: u8) -> anyhow::Result<bool> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM verify_proofs WHERE zk_proof_id=$1)
             OR EXISTS(SELECT 1 FROM ciphertext_digest WHERE handle=$2 AND NOT txn_is_sent)",
    )
    .bind(i64::from(id))
    .bind(vec![id; 32])
    .fetch_one(&env.db_pool)
    .await?)
}

async fn wait_for_submission(env: &TestEnvironment, id: u8) -> anyhow::Result<()> {
    tokio::time::timeout(Duration::from_secs(20), async {
        while pending(env, id).await? {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    Ok(())
}

async fn resolve_halt(env: &TestEnvironment) -> anyhow::Result<()> {
    // Disposable-fixture equivalent of an operator accepting a verified baseline.
    let mut recovery = env.db_pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *recovery)
        .await?;
    sqlx::query("UPDATE public.chain_recovery_halts SET resolved_at=NOW(), recovery_note='test baseline verified' WHERE resolved_at IS NULL")
        .execute(&mut *recovery).await?;
    recovery.commit().await?;
    Ok(())
}

#[rstest::rstest]
#[case(5, "eth_estimateGas")]
#[case(10, "eth_sendRawTransactionSync")]
#[tokio::test]
#[serial(db)]
async fn slow_mixed_submissions_reserve_pool_capacity_and_obey_halt(
    #[case] pool_size: u32,
    #[case] stalled_method: &str,
) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        db_polling_interval_secs: 1,
        send_txn_sync_timeout_secs: 60,
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(SignerType::PrivateKey, conf, false).await?;
    let _cancel_on_exit = env.cancel_token.clone().drop_guard();
    // Exercise both the minimum supported pool and the default. Monitoring and
    // fixture writes use a separate pool, as they would from another process.
    let pool = PgPoolOptions::new()
        .max_connections(pool_size)
        .acquire_timeout(Duration::from_secs(1))
        .connect_with((*env.db_pool.connect_options()).clone())
        .await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let url = proxy.url();
    let inner = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(env.wallet.clone())
        .connect_reqwest(gateway_http_client(&url)?, url);
    let provider = NonceManagedProvider::new(inner, Some(env.wallet.default_signer().address()));
    let sender = TransactionSender::new(
        pool.clone(),
        *input.address(),
        *ciphertext.address(),
        env.signer.clone(),
        provider,
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let (chain, key) = insert_random_keys_and_host_chain(&env.db_pool).await?;
    // More work in each queue than either pool can admit; batch sizes retain
    // their production defaults (128 proofs and 10 ciphertexts).
    for id in 1..=12 {
        queue_work(&env, chain, key, id).await?;
    }
    proxy.set_fault(stalled_method, Fault::Blackhole, None);
    let running_sender = sender.clone();
    let run = tokio::spawn(async move { running_sender.run().await });
    let limit = (pool_size - 4) as usize;
    tokio::time::timeout(Duration::from_secs(10), async {
        while proxy.calls(stalled_method) < limit {
            assert!(!run.is_finished());
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await?;
    // Hold all admission slots longer than the pool-acquisition timeout. A
    // waiter for submission admission must not take the connection needed by
    // the real health check, nor open an extra fence.
    let until = tokio::time::Instant::now() + Duration::from_secs(2);
    while tokio::time::Instant::now() < until {
        let health = tokio::time::timeout(Duration::from_secs(3), sender.health_check()).await?;
        assert!(health.healthy, "{health:?}");
        assert_eq!(proxy.calls(stalled_method), limit);
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let halt_pool = env.db_pool.clone();
    let halt = tokio::spawn(async move {
        record_halt(
            &halt_pool,
            HistoryMismatch {
                source: ListenerSource::Host,
                gcs_mode: false,
                chain_id: chain,
                block_number: 1,
                expected_hash: None,
                observed_hash: None,
                reason: "halt while submissions wait for admission",
            },
        )
        .await
    });
    // Observe the exclusive fence actually waiting behind the active sends
    // before releasing the Gateway; admission must not weaken that fence.
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM pg_locks
                 WHERE locktype = 'advisory' AND NOT granted AND mode = 'ExclusiveLock'
                   AND database = (SELECT oid FROM pg_database WHERE datname = current_database())
                   AND classid = $1::bigint::oid AND objid = $2::bigint::oid)",
            )
            .bind(CUTOVER_LOCK_ID >> 32)
            .bind(CUTOVER_LOCK_ID & 0xffff_ffff)
            .fetch_one(&env.db_pool)
            .await?;
            if waiting {
                break;
            }
            assert!(
                !halt.is_finished(),
                "halt did not wait for in-flight submissions"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    proxy.clear_fault(stalled_method);
    tokio::time::timeout(Duration::from_secs(20), halt).await???;
    let sends = proxy.calls("eth_sendRawTransactionSync");
    assert_eq!(sends, limit);
    tokio::time::sleep(Duration::from_millis(1200)).await;
    assert_eq!(proxy.calls("eth_sendRawTransactionSync"), sends);
    assert!(
        pending(&env, 12).await?,
        "queued work was lost during the halt"
    );
    assert!(!run.is_finished());

    resolve_halt(&env).await?;
    // Also exercise sustained slow responses and bookkeeping across multiple
    // waves after recovery, rather than just the initially stalled requests.
    proxy.set_fault(
        "eth_sendRawTransactionSync",
        Fault::DelayResponse(Duration::from_millis(100)),
        None,
    );
    for id in 1..=12 {
        wait_for_submission(&env, id).await?;
    }
    assert!(proxy.max_concurrency("eth_estimateGas") <= limit);
    assert!(proxy.max_concurrency("eth_sendRawTransactionSync") <= limit);
    let health = sender.health_check().await;
    assert!(health.healthy, "{health:?}");
    env.cancel_token.cancel();
    tokio::time::timeout(Duration::from_secs(10), run).await???;
    pool.close().await;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_allows_dispatch_while_confirmed_halt_waits_for_recovery() -> anyhow::Result<()> {
    let conf = ConfigSettings {
        db_polling_interval_secs: 1,
        // Both pending queues fill a batch while contained; that must not
        // prevent either operation from resuming after recovery.
        add_ciphertexts_batch_limit: 1,
        verify_proof_resp_batch_limit: 1,
        ..Default::default()
    };
    let env = TestEnvironment::new_with_config(SignerType::PrivateKey, conf, false).await?;
    let deploy = env.http_provider()?;
    let input = InputVerification::deploy(&deploy, false, false, false, false).await?;
    let ciphertext = CiphertextCommits::deploy(&deploy, false).await?;
    let proxy = FaultProxy::start(env.http_endpoint_url()).await?;
    let url = proxy.url();
    let inner = ProviderBuilder::default()
        .filler(FillersWithoutNonceManagement::default())
        .wallet(env.wallet.clone())
        .connect_reqwest(gateway_http_client(&url)?, url);
    let provider = NonceManagedProvider::new(inner, Some(env.wallet.default_signer().address()));
    let sender = TransactionSender::new(
        env.db_pool.clone(),
        *input.address(),
        *ciphertext.address(),
        env.signer.clone(),
        provider,
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;
    let (chain, key) = insert_random_keys_and_host_chain(&env.db_pool).await?;
    let replay = begin_replay(&env.db_pool, ListenerSource::Host, false, chain, 1, 1).await?;
    queue_work(&env, chain, key, 1).await?;
    let before = deploy
        .get_transaction_count(alloy::signers::Signer::address(env.signer.as_ref()))
        .await?;
    let run = tokio::spawn(async move { sender.run().await });
    // A restart obligation must not stop admitted work, even before any replay
    // progress is made (for example while the host RPC is unavailable).
    wait_for_submission(&env, 1).await?;
    assert!(!run.is_finished());
    assert!(fhevm_engine_common::chain_recovery::readiness_blocked(&env.db_pool).await?);
    complete_replay(
        &env.db_pool,
        ListenerSource::Host,
        false,
        chain,
        replay.from_block,
        replay.to_block,
        replay.generation,
    )
    .await?;
    wait_for_submission(&env, 1).await?;
    assert_eq!(
        deploy
            .get_transaction_count(alloy::signers::Signer::address(env.signer.as_ref()))
            .await?,
        before + 2
    );

    record_halt(
        &env.db_pool,
        HistoryMismatch {
            source: ListenerSource::Host,
            gcs_mode: false,
            chain_id: chain,
            block_number: 1,
            expected_hash: None,
            observed_hash: None,
            reason: "test recovery gate",
        },
    )
    .await?;
    queue_work(&env, chain, key, 2).await?;
    let calls = proxy.calls("eth_sendRawTransactionSync");
    tokio::time::sleep(Duration::from_millis(2200)).await;
    assert!(!run.is_finished());
    assert!(pending(&env, 2).await?);
    assert_eq!(proxy.calls("eth_sendRawTransactionSync"), calls);
    assert_eq!(
        deploy
            .get_transaction_count(alloy::signers::Signer::address(env.signer.as_ref()))
            .await?,
        before + 2
    );

    resolve_halt(&env).await?;
    wait_for_submission(&env, 2).await?;
    assert_eq!(
        deploy
            .get_transaction_count(alloy::signers::Signer::address(env.signer.as_ref()))
            .await?,
        before + 4
    );
    env.cancel_token.cancel();
    tokio::time::timeout(Duration::from_secs(10), run).await???;
    Ok(())
}
