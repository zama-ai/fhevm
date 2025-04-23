use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::{primitives::Address, providers::WsConnect};
use common::{MultichainAcl, TestEnvironment};

use fhevm_engine_common::types::AllowEvents;
use rand::random;
use serial_test::serial;
use sqlx::PgPool;
use std::time::Duration;
use test_harness::db_utils::insert_random_tenant;
use tokio::time::sleep;
use transaction_sender::{FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender};

mod common;

#[tokio::test]
#[serial(db)]
async fn test_allow_account() -> anyhow::Result<()> {
    test_allow_call(AllowEvents::AllowedAccount).await
    // TODO: Emit AllowAccount event in the mocked contract and assert AllowAccount is called.
}

#[tokio::test]
#[serial(db)]
async fn test_allow_for_decrypt() -> anyhow::Result<()> {
    test_allow_call(AllowEvents::AllowedForDecryption).await
    // TODO: Emit AllowedForDecryption event in the mocked contract and assert AllowedForDecryption is called.
}

async fn test_allow_call(event_type: AllowEvents) -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider_deploy = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .on_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(env.wallet.clone())
            .on_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );
    let multichain_acl = MultichainAcl::deploy(&provider_deploy).await?;

    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *multichain_acl.address(),
        env.signer.clone(),
        provider.clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let tenant_id = insert_random_tenant(&env.db_pool).await?;

    let handle = random::<[u8; 32]>().to_vec();
    insert_allowed_handle(
        &env.db_pool,
        tenant_id,
        handle.clone(),
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
    let mut allow_handle_is_sent = false;
    for _retries in 0..10 {
        let rows = sqlx::query!(
            "SELECT txn_is_sent
             FROM allowed_handles
             WHERE handle = $1",
            handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if rows.txn_is_sent.unwrap_or_default() {
            allow_handle_is_sent = true;
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
        allow_handle_is_sent,
        "Expected the allowed handle to be tagged as sent"
    );

    env.cancel_token.cancel();
    run_handle.await??;

    Ok(())
}

async fn insert_allowed_handle(
    pool: &PgPool,
    tenant_id: i32,
    handle: Vec<u8>,
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
