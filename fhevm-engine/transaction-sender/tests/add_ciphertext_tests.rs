use alloy::providers::{Provider, ProviderBuilder, WalletProvider};
use common::{CiphertextManager, TestEnvironment, ZKPoKManager};

use rand::random;
use serial_test::serial;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use test_harness::db_utils::insert_random_tenant;
use tokio::time::sleep;
use transaction_sender::TransactionSender;

mod common;

#[tokio::test]
#[serial(db)]
async fn add_ciphertext_digests() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = Arc::new(ProviderBuilder::new().on_anvil_with_wallet());
    let zkpok_manager = ZKPoKManager::deploy(&provider, false, false).await?;
    let ciphertext_manager = CiphertextManager::deploy(&provider).await?;
    let txn_sender = TransactionSender::new(
        *zkpok_manager.address(),
        *ciphertext_manager.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let tenant_id = insert_random_tenant(&env.db_pool).await?;

    //  Add a ciphertext digest to database
    let handle = random::<[u8; 32]>().to_vec();
    // Record initial transaction count.
    let initial_tx_count = provider
        .get_transaction_count(provider.default_signer_address())
        .await?;

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        tenant_id,
        handle.clone(),
        random::<[u8; 32]>().to_vec(),
        random::<[u8; 32]>().to_vec(),
        1,
    )
    .await?;

    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.add_ciphertexts_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the digest was tagged as sent.
    for _retries in 0..10 {
        let rows = sqlx::query!(
            "SELECT txn_is_sent
             FROM ciphertext_digest
             WHERE handle = $1",
            handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if rows.txn_is_sent.unwrap_or_default() {
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }

    // Verify that a transaction has been sent.
    let tx_count = provider
        .get_transaction_count(provider.default_signer_address())
        .await?;
    assert_eq!(
        tx_count,
        initial_tx_count + 1,
        "Expected a new transaction to be sent"
    );

    sqlx::query!(
        "
        delete from tenants where tenant_id = $1",
        tenant_id
    )
    .execute(&env.db_pool)
    .await?;

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

async fn insert_ciphertext_digest(
    pool: &PgPool,
    tenant_id: i32,
    handle: Vec<u8>,
    ciphertext: Vec<u8>,
    ciphertext128: Vec<u8>,
    txn_retry_count: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO ciphertext_digest (tenant_id, handle, ciphertext, ciphertext128, txn_retry_count)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        tenant_id,
        handle,
        ciphertext,
        ciphertext128,
        txn_retry_count,
    )
    .execute(pool)
    .await?;

    Ok(())
}
