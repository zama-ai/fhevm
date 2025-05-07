mod common;

use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use common::{CiphertextCommits, TestEnvironment};

use rand::{random, Rng};
use serial_test::serial;
use sqlx::PgPool;
use std::time::Duration;
use test_harness::db_utils::insert_random_tenant;
use tokio::time::sleep;
use transaction_sender::{
    ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

#[tokio::test]
#[serial(db)]
async fn add_ciphertext_digests() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .on_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .on_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
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
    let handle = random::<[u8; 32]>();
    // Record initial transaction count.
    let initial_tx_count = provider.get_transaction_count(env.signer.address()).await?;

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        tenant_id,
        &handle,
        &random::<[u8; 32]>(),
        &random::<[u8; 32]>(),
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
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if rows.txn_is_sent {
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
async fn ciphertext_digest_already_added() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .on_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .on_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let already_added_revert = true;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
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
    let handle = random::<[u8; 32]>();

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        tenant_id,
        &handle,
        &random::<[u8; 32]>(),
        &random::<[u8; 32]>(),
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
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if rows.txn_is_sent {
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

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn recover_from_transport_error() -> anyhow::Result<()> {
    let mut env = TestEnvironment::new().await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .on_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .on_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
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

    // Record a transaction count, to make sure the provider is connected before the transport error.
    let _ = provider.get_transaction_count(env.signer.address()).await?;

    // Simulate a transport error by recreating the anvil instance.
    env.recreate_anvil()?;

    // Insert a ciphertext digest into the database.
    let handle = random::<[u8; 32]>();
    insert_ciphertext_digest(
        &env.db_pool,
        tenant_id,
        &handle,
        &random::<[u8; 32]>(),
        &random::<[u8; 32]>(),
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
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if rows.txn_is_sent {
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

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn retry_on_transport_error() -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_max_retries: 2,
        ..Default::default()
    };

    let mut env = TestEnvironment::new_with_config(conf.clone()).await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .on_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .on_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        *ciphertext_commits.address(),
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

    // Simulate a transport error by stopping the anvil instance.
    env.drop_anvil();

    // Insert a ciphertext digest into the database.
    let handle = random::<[u8; 32]>();
    insert_ciphertext_digest(
        &env.db_pool,
        tenant_id,
        &handle,
        &random::<[u8; 32]>(),
        &random::<[u8; 32]>(),
        0,
    )
    .await?;

    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.add_ciphertexts_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the digest is not sent, the retry count is 0 and the transport retry count is greater than the txn max retry count.
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent, txn_retry_count, txn_transport_retry_count
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if !rows.txn_is_sent
            && rows.txn_retry_count == 0
            && rows.txn_transport_retry_count > conf.add_ciphertexts_max_retries as i32
        {
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

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn retry_mechanism() -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_max_retries: 3,
        ..Default::default()
    };

    let env = TestEnvironment::new_with_config(conf).await?;

    // Create a provider without a wallet.
    let provider = NonceManagedProvider::new(
        ProviderBuilder::new()
            .on_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        None,
    );
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
    let handle = rng.random::<[u8; 32]>();

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        tenant_id,
        &handle,
        &random::<[u8; 32]>(),
        &random::<[u8; 32]>(),
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
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;

        if rows.txn_is_sent {
            panic!("Expected txn_is_sent to be false");
        } else {
            println!("txn_retry_count: {}", rows.txn_retry_count);
            if rows.txn_retry_count == env.conf.add_ciphertexts_max_retries as i32 - 1 {
                valid_retries_count = true;
                break;
            }
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
    handle: &[u8; 32],
    ciphertext: &[u8],
    ciphertext128: &[u8],
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
