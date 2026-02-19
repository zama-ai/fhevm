use alloy::eips::BlockNumberOrTag;
use alloy::network::TxSigner;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::{primitives::Address, providers::WsConnect};
use common::{is_coprocessor_config_error, MultichainACL, SignerType, TestEnvironment};

use rstest::*;
use serial_test::serial;
use sqlx::PgPool;
use tracing::error;
use transaction_sender::{
    ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender,
};

mod common;

#[allow(clippy::too_many_arguments)]
async fn insert_delegate_user_decrypt(
    pool: &PgPool,
    delegator: Address,
    delegate: Address,
    contract_address: Address,
    delegation_counter: u64,
    old_expiration_date: u64,
    new_expiration_date: u64,
    chain_id: u64,
    block_hash: &[u8],
    block_number: u64,
    transaction_id: Option<Vec<u8>>,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        "INSERT INTO delegate_user_decrypt(delegator, delegate, contract_address, delegation_counter, old_expiration_date, new_expiration_date, host_chain_id, block_number, block_hash, transaction_id, on_gateway, reorg_out)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, false, false)
        ON CONFLICT DO NOTHING",
        &delegator.into_array(),
        &delegate.into_array(),
        &contract_address.into_array(),
        delegation_counter as i64,
        old_expiration_date as i64,
        new_expiration_date as i64,
        chain_id as i64,
        block_number as i64,
        block_hash,
        transaction_id,
    );
    query.execute(pool).await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn delegate_user_decrypt_life_cycle(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let already_allowed_revert = false;
    delegate_user_decrypt_life_cycle_aux(signer_type, already_allowed_revert).await
}

async fn delegate_user_decrypt_life_cycle_aux(
    signer_type: SignerType,
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

    let config = ConfigSettings {
        delegation_block_delay: 2,
        delegation_clear_after_n_blocks: 5,
        delegation_fallback_polling: 1000, // disable
        ..env.conf.clone()
    };

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *multichain_acl.address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(), // shared blockchain
        env.cancel_token.clone(),
        config.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let chain_id = provider.get_chain_id().await?;

    let block = provider
        .inner()
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .unwrap(); // make sure the provider is warmed up
    error!("Current block: {:?}", block);
    let start_block = block.number();
    for _ in 1..2 {
        // check deduplication based on unique constraint
        insert_delegate_user_decrypt(
            &env.db_pool,
            *multichain_acl.address(),
            *multichain_acl.address(),
            *multichain_acl.address(),
            0,
            0,
            2,
            chain_id,
            block.hash().as_ref(),
            start_block,
            None,
        )
        .await?;
        insert_delegate_user_decrypt(
            &env.db_pool,
            *multichain_acl.address(),
            *multichain_acl.address(),
            *multichain_acl.address(),
            0,
            0,
            3,
            chain_id,
            &[], // reorg out for bad hash
            start_block + 1,
            Some(vec![]),
        )
        .await?;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    for i in 0..(config.delegation_clear_after_n_blocks + 1) {
        sqlx::query!(
            "SELECT pg_notify($1, $2)",
            "new_host_block",
            (start_block + i).to_string()
        )
        .execute(&env.db_pool)
        .await?;
        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let present = sqlx::query!("SELECT COUNT(*) FROM delegate_user_decrypt")
            .fetch_one(&env.db_pool)
            .await?
            .count
            .unwrap_or(0);
        let on_gateway =
            sqlx::query!("SELECT COUNT(*) FROM delegate_user_decrypt WHERE on_gateway = true")
                .fetch_one(&env.db_pool)
                .await?
                .count
                .unwrap_or(0);
        let reorg_out =
            sqlx::query!("SELECT COUNT(*) FROM delegate_user_decrypt WHERE reorg_out = true")
                .fetch_one(&env.db_pool)
                .await?
                .count
                .unwrap_or(0);
        if i < config.delegation_block_delay {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 0);
            assert_eq!(on_gateway, 0);
        } else if i == config.delegation_block_delay {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 0);
            assert_eq!(on_gateway, 1);
        } else if i == config.delegation_block_delay + 1 {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 1);
            assert_eq!(on_gateway, 1);
        } else if i > config.delegation_clear_after_n_blocks {
            assert_eq!(present, 0);
        } else {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 1);
            assert_eq!(on_gateway, 1);
        }
    }
    env.cancel_token.cancel();
    let _ = run_handle.await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn delegate_user_decrypt_idempotent_error(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let already_allowed_revert = false;
    delegate_user_decrypt_idempotent_error_call(signer_type, already_allowed_revert).await
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn delegate_user_decrypt_idempotent_error_no_nonce_retry(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let already_allowed_revert = false;
    delegate_user_decrypt_idempotent_error_call(signer_type, already_allowed_revert).await
}

async fn delegate_user_decrypt_idempotent_error_call(
    signer_type: SignerType,
    already_allowed_revert: bool,
) -> anyhow::Result<()> {
    // simulate a host listener during catchup where some delegation are already part of gateway
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

    let config = ConfigSettings {
        delegation_block_delay: 2,
        delegation_clear_after_n_blocks: 5,
        delegation_fallback_polling: 1000, // disable
        ..env.conf.clone()
    };

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        *multichain_acl.address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(), // shared blockchain
        env.cancel_token.clone(),
        config.clone(),
        None,
    )
    .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let chain_id = provider.get_chain_id().await?;

    let block = provider
        .inner()
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .unwrap(); // make sure the provider is warmed up
    error!("Current block: {:?}", block);
    let start_block = block.number();
    for _ in 1..2 {
        // check deduplication based on unique constraint
        // This insert is a revocation request.
        insert_delegate_user_decrypt(
            &env.db_pool,
            *multichain_acl.address(),
            *multichain_acl.address(),
            *multichain_acl.address(),
            0,
            0,
            0, // will fail either Nonce too high or counter too low
            chain_id,
            block.hash().as_ref(),
            start_block,
            None,
        )
        .await?;
        // This one is a delegation request.
        insert_delegate_user_decrypt(
            &env.db_pool,
            *multichain_acl.address(),
            *multichain_acl.address(),
            *multichain_acl.address(),
            0,
            0,
            1, // will fail either Nonce too high or already known
            chain_id,
            block.hash().as_ref(),
            start_block,
            None,
        )
        .await?;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    for i in 0..(config.delegation_clear_after_n_blocks + 1) {
        sqlx::query!(
            "SELECT pg_notify($1, $2)",
            "new_host_block",
            (start_block + i).to_string()
        )
        .execute(&env.db_pool)
        .await?;
        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let present = sqlx::query!("SELECT COUNT(*) FROM delegate_user_decrypt")
            .fetch_one(&env.db_pool)
            .await?
            .count
            .unwrap_or(0);
        let on_gateway =
            sqlx::query!("SELECT COUNT(*) FROM delegate_user_decrypt WHERE on_gateway = true")
                .fetch_one(&env.db_pool)
                .await?
                .count
                .unwrap_or(0);
        let reorg_out =
            sqlx::query!("SELECT COUNT(*) FROM delegate_user_decrypt WHERE reorg_out = true")
                .fetch_one(&env.db_pool)
                .await?
                .count
                .unwrap_or(0);
        let error = sqlx::query!(
            "SELECT COUNT(*) FROM delegate_user_decrypt WHERE gateway_nb_attempts > 0"
        )
        .fetch_one(&env.db_pool)
        .await?
        .count
        .unwrap_or(0);
        error!("{i} {present} {on_gateway} {reorg_out} {error}");
        if i < config.delegation_block_delay {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 0);
            assert_eq!(on_gateway, 0);
        } else if i == config.delegation_block_delay {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 0);
            assert_eq!(on_gateway, 2);
        } else if i > config.delegation_clear_after_n_blocks {
            assert_eq!(present, 0);
        } else {
            assert_eq!(present, 2);
            assert_eq!(reorg_out, 0);
            assert_eq!(on_gateway, 2);
        }
    }
    env.cancel_token.cancel();
    let _ = run_handle.await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn delegate_user_decrypt_terminal_on_gw_config_error(
    #[case] signer_type: SignerType,
    #[values(1u8, 2, 3)] config_error_mode: u8,
) -> anyhow::Result<()> {
    let base_conf = ConfigSettings {
        delegation_block_delay: 0,
        delegation_clear_after_n_blocks: 5,
        delegation_fallback_polling: 1,
        delegation_max_retry: 3,
        ..Default::default()
    };
    let force_per_test_localstack = false;
    let env =
        TestEnvironment::new_with_config(signer_type, base_conf, force_per_test_localstack).await?;
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
    provider_deploy
        .send_transaction_sync(
            multichain_acl
                .setConfigErrorMode(config_error_mode)
                .into_transaction_request(),
        )
        .await?;

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });
    let chain_id = provider.get_chain_id().await?;
    let block = provider
        .inner()
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .unwrap();
    let start_block = block.number();

    insert_delegate_user_decrypt(
        &env.db_pool,
        *multichain_acl.address(),
        *multichain_acl.address(),
        *multichain_acl.address(),
        0,
        0,
        2,
        chain_id,
        block.hash().as_ref(),
        start_block,
        None,
    )
    .await?;

    sqlx::query!(
        "SELECT pg_notify($1, $2)",
        "new_host_block",
        start_block.to_string()
    )
    .execute(&env.db_pool)
    .await?;

    let mut attempts = 0;
    let row = loop {
        sqlx::query!(
            "SELECT pg_notify($1, $2)",
            "new_host_block",
            start_block.to_string()
        )
        .execute(&env.db_pool)
        .await?;

        let row = sqlx::query!(
            "SELECT on_gateway, gateway_nb_attempts, gateway_last_error
             FROM delegate_user_decrypt
             WHERE block_number = $1",
            start_block as i64,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if !row.on_gateway
            && row.gateway_nb_attempts == (env.conf.delegation_max_retry + 1) as i64
            && row
                .gateway_last_error
                .as_deref()
                .is_some_and(is_coprocessor_config_error)
        {
            break row;
        }
        attempts += 1;
        assert!(
            attempts < 80,
            "timed out waiting for terminal state; gateway_nb_attempts={}, last_error={:?}",
            row.gateway_nb_attempts,
            row.gateway_last_error
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
    };
    assert!(!row.on_gateway);
    assert_eq!(
        row.gateway_nb_attempts,
        (env.conf.delegation_max_retry + 1) as i64
    );
    assert!(
        row.gateway_last_error
            .as_deref()
            .is_some_and(is_coprocessor_config_error),
        "Expected terminal gateway config error, got {:?}",
        row.gateway_last_error
    );

    let tx_count = provider.get_transaction_count(env.signer.address()).await?;
    assert_eq!(
        tx_count, initial_tx_count,
        "Expected no transaction to be sent for gateway config errors detected before send"
    );

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}
