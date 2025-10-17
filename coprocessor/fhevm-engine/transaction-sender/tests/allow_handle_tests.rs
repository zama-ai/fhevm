use alloy::network::TxSigner;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::{primitives::Address, providers::WsConnect};
use common::{MultichainACL, SignerType, TestEnvironment};

use fhevm_engine_common::types::AllowEvents;
use rand::random;
use rstest::*;
use serial_test::serial;
use sqlx::PgPool;
use std::time::Duration;
use test_harness::db_utils::insert_random_tenant;
use tokio::time::sleep;
use transaction_sender::is_backend_gone;
use transaction_sender::{
    ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

mod common;

async fn insert_allowed_handle(
    pool: &PgPool,
    tenant_id: i32,
    handle: &[u8; 32],
    account_address: Address,
    event_type: AllowEvents,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO allowed_handles (tenant_id, handle, account_address, event_type)
        VALUES ($1, $2, $3, $4)
        "#,
        tenant_id,
        handle,
        account_address.to_string(),
        event_type as i16,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn allow_account(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let already_allowed_revert = false;
    allow_call(
        signer_type,
        AllowEvents::AllowedAccount,
        already_allowed_revert,
    )
    .await
    // TODO: Emit AllowAccount event in the mocked contract and assert AllowAccount is called.
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn allow_for_decrypt(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let already_allowed_revert = false;
    allow_call(
        signer_type,
        AllowEvents::AllowedForDecryption,
        already_allowed_revert,
    )
    .await
    // TODO: Emit AllowedForDecryption event in the mocked contract and assert AllowedForDecryption is called.
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn allow_account_already_allowed(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let already_allowed_revert = true;
    allow_call(
        signer_type,
        AllowEvents::AllowedAccount,
        already_allowed_revert,
    )
    .await
}

async fn allow_call(
    signer_type: SignerType,
    event_type: AllowEvents,
    already_allowed_revert: bool,
) -> anyhow::Result<()> {
    let env = TestEnvironment::new(signer_type).await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );
    let multichain_acl = MultichainACL::deploy(&provider_deploy, already_allowed_revert).await?;

    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *multichain_acl.address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(), // shared blockchain
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let tenant_id = insert_random_tenant(&env.db_pool).await?;

    // Record initial transaction count.
    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    let handle = random::<[u8; 32]>();
    insert_allowed_handle(
        &env.db_pool,
        tenant_id,
        &handle,
        PrivateKeySigner::random().address(),
        event_type,
    )
    .await?;

    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.allow_handle_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the allowed handle was tagged as sent.
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent
             FROM allowed_handles
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

    // Verify that a transaction has been sent if not reverted during gas estimation.
    if !already_allowed_revert {
        let tx_count = provider.get_transaction_count(env.signer.address()).await?;
        assert_eq!(
            tx_count,
            initial_tx_count + 1,
            "Expected a new transaction to be sent"
        );
    }

    env.cancel_token.cancel();
    run_handle.await??;

    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn stop_on_backend_gone(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        allow_handle_max_retries: 2,
        graceful_shutdown_timeout: Duration::from_secs(2),
        ..Default::default()
    };

    let force_per_test_localstack: bool = false;
    let mut env =
        TestEnvironment::new_with_config(signer_type, conf.clone(), force_per_test_localstack)
            .await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(
            // Reduce the retries count and the interval for alloy's internal retry to make this test faster.
            WsConnect::new(env.ws_endpoint_url())
                .with_max_retries(1)
                .with_retry_interval(Duration::from_millis(200)),
        )
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(
                // Reduce the retries count and the interval for alloy's internal retry to make this test faster.
                WsConnect::new(env.ws_endpoint_url())
                    .with_max_retries(1)
                    .with_retry_interval(Duration::from_millis(200)),
            )
            .await?,
        Some(env.wallet.default_signer().address()),
    );
    let already_allowed_revert = false;
    let multichain_acl = MultichainACL::deploy(&provider_deploy, already_allowed_revert).await?;

    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *multichain_acl.address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(), // shared blockchain
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let tenant_id = insert_random_tenant(&env.db_pool).await?;

    // Simulate a transport error by stopping the anvil instance.
    env.drop_anvil();

    let handle = random::<[u8; 32]>();
    insert_allowed_handle(
        &env.db_pool,
        tenant_id,
        &handle,
        PrivateKeySigner::random().address(),
        AllowEvents::AllowedAccount,
    )
    .await?;

    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.allow_handle_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the digest is not sent, the retry count is 0 and the unlimited retry count is 1.
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent, txn_limited_retries_count, txn_unlimited_retries_count
             FROM allowed_handles
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if !rows.txn_is_sent
            && rows.txn_limited_retries_count == 0
            && rows.txn_unlimited_retries_count == 1
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

    // Expect that the sender will stop on its own due to BackendGone.
    let err = run_handle.await?.err().unwrap();
    assert!(is_backend_gone(&err));
    Ok(())
}

#[rstest]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn retry_on_aws_kms_error(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        allow_handle_max_retries: 2,
        ..Default::default()
    };

    let force_per_test_localstack = true;
    let mut env =
        TestEnvironment::new_with_config(signer_type, conf.clone(), force_per_test_localstack)
            .await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );
    let already_allowed_revert = false;
    let multichain_acl = MultichainACL::deploy(&provider_deploy, already_allowed_revert).await?;

    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *multichain_acl.address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let tenant_id = insert_random_tenant(&env.db_pool).await?;

    // Simulate an AWS KMS error by stopping the localstack instance.
    env.stop_localstack().await;

    let handle = random::<[u8; 32]>();
    insert_allowed_handle(
        &env.db_pool,
        tenant_id,
        &handle,
        PrivateKeySigner::random().address(),
        AllowEvents::AllowedAccount,
    )
    .await?;

    sqlx::query!(
        "
        SELECT pg_notify($1, '')",
        env.conf.allow_handle_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the digest is not sent, the retry count is 0 and the unlimited retry count is greater than the txn max retry count.
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent, txn_limited_retries_count, txn_unlimited_retries_count
             FROM allowed_handles
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if !rows.txn_is_sent
            && rows.txn_limited_retries_count == 0
            && rows.txn_unlimited_retries_count > conf.allow_handle_max_retries as i32
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
