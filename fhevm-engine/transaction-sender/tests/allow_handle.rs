use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use common::{ACLManager, TestEnvironment};

use rand::random;
use serial_test::serial;
use sqlx::PgPool;
use std::time::Duration;
use test_harness::db_utils::insert_random_tenant;
use tokio::time::sleep;
use transaction_sender::{ProviderFillers, TransactionSender};

mod common;

#[tokio::test]
#[serial(db)]
async fn test_allow_handle() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::default()
        .filler(ProviderFillers::default())
        .on_anvil_with_wallet();
    let acl_manager = ACLManager::deploy(&provider).await?;

    let txn_sender = TransactionSender::new(
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *acl_manager.address(),
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
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO allowed_handles (tenant_id, handle, account_address)
        VALUES ($1, $2, $3)
        "#,
        tenant_id,
        handle,
        account_address.to_string(),
    )
    .execute(pool)
    .await?;

    Ok(())
}
