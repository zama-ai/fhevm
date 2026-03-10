mod common;

use alloy::network::TxSigner;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use common::{is_coprocessor_config_error, CiphertextCommits, TestEnvironment};

use common::SignerType;
use rand::{random, Rng};
use rstest::*;
use serial_test::serial;
use std::time::Duration;
use test_harness::db_utils::{insert_ciphertext_digest, insert_random_keys_and_host_chain};
use tokio::time::sleep;
use transaction_sender::{
    is_backend_gone, ConfigSettings, FillersWithoutNonceManagement, NonceManagedProvider,
    TransactionSender,
};

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn add_ciphertext_digests(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    use test_harness::db_utils::insert_ciphertext_digest;

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

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    //  Add a ciphertext digest to database
    let handle = random::<[u8; 32]>();
    // Record initial transaction count.
    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn ciphertext_digest_already_added(#[case] signer_type: SignerType) -> anyhow::Result<()> {
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

    let already_added_revert = true;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    // Record initial transaction count.
    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    //  Add a ciphertext digest to database
    let handle = random::<[u8; 32]>();

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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

    let tx_count = provider.get_transaction_count(env.signer.address()).await?;
    assert_eq!(
        tx_count, initial_tx_count,
        "Expected no new transaction to be sent due to revert"
    );

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn recover_from_transport_error(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let mut env = TestEnvironment::new(signer_type).await?;
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

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Record a transaction count, to make sure the provider is connected before the transport error.
    let _ = provider.get_transaction_count(env.signer.address()).await?;

    // Simulate a transport error by recreating the anvil instance.
    env.recreate_anvil()?;

    // Insert a ciphertext digest into the database.
    let handle = random::<[u8; 32]>();
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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
        add_ciphertexts_max_retries: 2,
        graceful_shutdown_timeout: Duration::from_secs(2),
        ..Default::default()
    };

    let force_per_test_localstack = false;
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

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Simulate a transport error by stopping the anvil instance.
    env.drop_anvil();

    // Insert a ciphertext digest into the database.
    let handle = random::<[u8; 32]>();
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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

    // Make sure the digest is not sent, the retry count is 0 and the unlimited retry count is 1.
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent, txn_limited_retries_count, txn_unlimited_retries_count
             FROM ciphertext_digest
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

    // Expect that the sender will stop on its own due to BackendGone.
    let err = run_handle.await?.err().unwrap();
    assert!(is_backend_gone(&err));
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn retry_mechanism(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    use alloy::network::EthereumWallet;

    let conf = ConfigSettings {
        add_ciphertexts_max_retries: 3,
        ..Default::default()
    };

    let force_per_test_localstack = false;
    let env =
        TestEnvironment::new_with_config(signer_type, conf, force_per_test_localstack).await?;

    // Create a provider with a random wallet without funds.
    let wallet: EthereumWallet = PrivateKeySigner::random().into();
    let provider = NonceManagedProvider::new(
        ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet)
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    let mut rng = rand::rng();
    let handle = rng.random::<[u8; 32]>();

    // Insert a ciphertext digest into the database.
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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
            "SELECT txn_is_sent, txn_limited_retries_count
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;

        if rows.txn_is_sent {
            panic!("Expected txn_is_sent to be false");
        } else {
            println!("txn_retry_count: {}", rows.txn_limited_retries_count);
            if rows.txn_limited_retries_count == env.conf.add_ciphertexts_max_retries - 1 {
                valid_retries_count = true;
                break;
            }
        }

        sleep(Duration::from_millis(500)).await;
    }

    assert!(
        valid_retries_count,
        "Expected the retry count to be greater than 0"
    );

    env.cancel_token.cancel();
    txn_sender_task.await??;
    Ok(())
}

#[rstest]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn retry_on_aws_kms_error(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let conf = ConfigSettings {
        add_ciphertexts_max_retries: 2,
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

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;

    // Simulate an AWS KMS error by stopping the localstack instance.
    env.stop_localstack().await;

    // Insert a ciphertext digest into the database.
    let handle = random::<[u8; 32]>();
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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

    // Make sure the digest is not sent, the retry count is 0 and the unlimited retry count is greater than the txn max retry count.
    loop {
        let rows = sqlx::query!(
            "SELECT txn_is_sent, txn_limited_retries_count, txn_unlimited_retries_count
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle,
        )
        .fetch_one(&env.db_pool)
        .await?;
        if !rows.txn_is_sent
            && rows.txn_limited_retries_count == 0
            && rows.txn_unlimited_retries_count > conf.add_ciphertexts_max_retries
        {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[tokio::test]
#[serial(db)]
async fn stop_retrying_add_ciphertext_on_gw_config_error(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let config_error_mode: u8 = 1;
    let conf = ConfigSettings {
        add_ciphertexts_max_retries: 3,
        ..Default::default()
    };
    let force_per_test_localstack = false;
    let env =
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

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    provider_deploy
        .send_transaction_sync(
            ciphertext_commits
                .setConfigErrorMode(config_error_mode)
                .into_transaction_request(),
        )
        .await?;

    let txn_sender = TransactionSender::new(
        env.db_pool.clone(),
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

    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let (host_chain_id, key_id) = insert_random_keys_and_host_chain(&env.db_pool).await?;
    let handle = random::<[u8; 32]>();
    insert_ciphertext_digest(
        &env.db_pool,
        host_chain_id,
        key_id,
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

    let mut attempts = 0;
    let row = loop {
        let row = sqlx::query!(
            "SELECT txn_is_sent, txn_limited_retries_count, txn_last_error
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle[..],
        )
        .fetch_one(&env.db_pool)
        .await?;
        if !row.txn_is_sent
            && row.txn_limited_retries_count == conf.add_ciphertexts_max_retries
            && row
                .txn_last_error
                .as_deref()
                .is_some_and(is_coprocessor_config_error)
        {
            break row;
        }
        attempts += 1;
        assert!(
            attempts < 60,
            "timed out waiting for non-retryable state; retries={}, last_error={:?}",
            row.txn_limited_retries_count,
            row.txn_last_error
        );
        sleep(Duration::from_millis(250)).await;
    };
    assert!(!row.txn_is_sent);
    assert_eq!(
        row.txn_limited_retries_count,
        conf.add_ciphertexts_max_retries
    );
    assert!(
        row.txn_last_error
            .as_deref()
            .is_some_and(is_coprocessor_config_error),
        "Expected non-retryable gateway config error, got {:?}",
        row.txn_last_error
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
