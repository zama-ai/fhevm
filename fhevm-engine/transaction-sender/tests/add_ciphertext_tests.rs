use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use common::{CiphertextManager, TestEnvironment};

use rand::{random, Rng};
use serial_test::serial;
use sqlx::PgPool;
use std::time::Duration;
use test_harness::db_utils::insert_random_tenant;
use tokio::time::sleep;
use transaction_sender::{ConfigSettings, ProviderFillers, TransactionSender};

mod common;

#[tokio::test]
#[serial(db)]
async fn test_add_ciphertext_digests() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::default()
        .wallet(env.wallet)
        .filler(ProviderFillers::default())
        .on_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;

    let ciphertext_manager = CiphertextManager::deploy(&provider).await?;
    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        *ciphertext_manager.address(),
        PrivateKeySigner::random().address(),
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
    let initial_tx_count = provider.get_transaction_count(env.signer.address()).await?;

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
    let mut digests_sent = false;
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
            digests_sent = true;
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }
    sqlx::query!(
        "
        delete from tenants where tenant_id = $1",
        tenant_id
    )
    .execute(&env.db_pool)
    .await?;

    assert!(
        digests_sent,
        "Expected the digests to be tagged as sent after sending a notification"
    );

    // Verify that a transaction has been sent.
    let tx_count = provider.get_transaction_count(env.signer.address()).await?;
    assert_eq!(
        tx_count,
        initial_tx_count + 1,
        "Expected a new transaction to be sent"
    );

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_retry_mechanism() -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_max_retries: 3,
        ..Default::default()
    };

    let env = TestEnvironment::new_with_config(conf).await?;

    // Create a provider without a wallet.
    let provider = ProviderBuilder::default()
        .filler(ProviderFillers::default())
        .on_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let txn_sender_task = tokio::spawn(async move { txn_sender.run().await });

    let tenant_id = insert_random_tenant(&env.db_pool).await?;

    let mut rng = rand::rng();
    let handle = rng.random::<[u8; 32]>().to_vec();

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

    let mut valid_retries_count = false;
    // Make sure the digest was not tagged as sent.
    for _retries in 0..10 {
        let rows = sqlx::query!(
            "SELECT txn_is_sent, txn_retry_count
             FROM ciphertext_digest
             WHERE handle = $1",
            handle,
        )
        .fetch_one(&env.db_pool)
        .await?;

        match rows.txn_is_sent {
            Some(true) => panic!("Expected txn_is_sent to be false"),
            Some(false) => {
                print!(
                    "txn_retry_count: {:?}",
                    rows.txn_retry_count.unwrap_or_default()
                );
                if rows.txn_retry_count.unwrap_or_default()
                    == env.conf.add_ciphertexts_max_retries as i32 - 1
                {
                    valid_retries_count = true;
                    break;
                }
            }
            None => {}
        }

        sleep(Duration::from_millis(500)).await;
    }

    sqlx::query!(
        "
        delete from tenants where tenant_id = $1",
        tenant_id
    )
    .execute(&env.db_pool)
    .await?;

    assert!(
        valid_retries_count,
        "Expected the retry count to be greater than 0"
    );

    env.cancel_token.cancel();
    txn_sender_task.await??;
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
