//! Exercise both real sender operations against disposable PostgreSQL and Anvil.
mod common;
mod support;

use alloy::providers::{Provider, ProviderBuilder};
use common::{CiphertextCommits, InputVerification, SignerType, TestEnvironment};
use fhevm_engine_common::chain_recovery::{
    begin_replay, complete_replay, record_halt, HistoryMismatch, ListenerSource,
};
use serial_test::serial;
use std::time::Duration;
use support::FaultProxy;
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

#[tokio::test]
#[serial(db)]
async fn replay_allows_dispatch_while_confirmed_halt_waits_for_recovery() -> anyhow::Result<()> {
    let conf = ConfigSettings {
        db_polling_interval_secs: 1,
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

    // Disposable-fixture equivalent of an operator accepting a verified baseline.
    let mut recovery = env.db_pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(fhevm_engine_common::versioning::CUTOVER_LOCK_ID)
        .execute(&mut *recovery)
        .await?;
    sqlx::query("UPDATE public.chain_recovery_halts SET resolved_at=NOW(), recovery_note='test baseline verified' WHERE resolved_at IS NULL")
        .execute(&mut *recovery).await?;
    recovery.commit().await?;
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
