use alloy::network::TxSigner;
use alloy::primitives::FixedBytes;
use alloy::primitives::U256;
use alloy::providers::WsConnect;
use alloy::signers::local::PrivateKeySigner;
use alloy::{providers::ProviderBuilder, sol};
use common::SignerType;
use common::{CiphertextCommits, InputVerification, TestEnvironment};
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use rand::random;
use rstest::*;
use serial_test::serial;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use transaction_sender::{FillersWithoutNonceManagement, NonceManagedProvider, TransactionSender};
mod common;

sol! {
    struct CiphertextVerification {
        bytes32[] ctHandles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
    }
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn verify_proof_response_success(#[case] signer_type: SignerType) -> anyhow::Result<()> {
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = false;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let event_filter = input_verification
        .VerifyProofResponse_filter()
        .watch()
        .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let event_handle = tokio::spawn(async move {
        event_filter
            .into_stream()
            .take(1)
            .collect::<Vec<_>>()
            .await
            .first()
            .unwrap()
            .clone()
            .unwrap()
    });

    let contract_chain_id = 42u64;

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        contract_chain_id as i64,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[1u8; 64],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    let event = event_handle.await?;

    let expected_proof_id = U256::from(proof_id);
    let expected_handles: Vec<FixedBytes<32>> = vec![FixedBytes([1u8; 32]), FixedBytes([1u8; 32])];

    // Make sure data in the event is correct.
    assert_eq!(event.0.zkProofId, expected_proof_id);
    assert_eq!(event.0.ctHandles, expected_handles);

    // Make sure the proof is removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
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
async fn verify_proof_response_empty_handles_success(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = false;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let event_filter = input_verification
        .VerifyProofResponse_filter()
        .watch()
        .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let event_handle = tokio::spawn(async move {
        event_filter
            .into_stream()
            .take(1)
            .collect::<Vec<_>>()
            .await
            .first()
            .unwrap()
            .clone()
            .unwrap()
    });

    let contract_chain_id = 42u64;

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        contract_chain_id as i64,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    let event: (
        InputVerification::VerifyProofResponse,
        alloy::rpc::types::Log,
    ) = event_handle.await?;

    let expected_proof_id = U256::from(proof_id);
    let expected_handles: Vec<FixedBytes<32>> = vec![];

    // Make sure data in the event is correct.
    assert_eq!(event.0.zkProofId, expected_proof_id);
    assert_eq!(event.0.ctHandles, expected_handles);

    // Make sure the proof is removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
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
async fn verify_proof_response_concurrent_success(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = false;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let event_filter = input_verification
        .VerifyProofResponse_filter()
        .watch()
        .await?;

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let count = 32;

    let events_handle = tokio::spawn(async move {
        event_filter
            .into_stream()
            .take(count)
            .map_ok(|event| (event.0.zkProofId, event))
            .try_collect::<HashMap<_, _>>()
            .await
    });

    let contract_chain_id = 42u64;

    let mut query_builder = QueryBuilder::<Postgres>::new("WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)");
    query_builder.push_values(0..count, |mut b, i| {
        b.push_bind(i as i64);
        b.push_bind(contract_chain_id as i64);
        b.push_bind(env.contract_address.to_string());
        b.push_bind(env.user_address.to_string());
        b.push_bind([1u8; 64]);
        b.push_bind(true);
    });
    query_builder.push(")");
    query_builder.push("SELECT pg_notify(");
    query_builder.push_bind(env.conf.verify_proof_resp_db_channel);
    query_builder
        .push(", '')")
        .build()
        .execute(&env.db_pool)
        .await?;

    let events: HashMap<U256, _> = events_handle.await??;
    for proof_id in 0..count {
        let event = events
            .get(&U256::from(proof_id))
            .expect("Event for proof ID not found");

        let expected_proof_id = U256::from(proof_id);
        let expected_handles: Vec<FixedBytes<32>> =
            vec![FixedBytes([1u8; 32]), FixedBytes([1u8; 32])];

        // Make sure data in the event is correct.
        assert_eq!(event.0.zkProofId, expected_proof_id);
        assert_eq!(event.0.ctHandles, expected_handles);
    }

    // Make sure the proofs are removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs"
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
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
async fn reject_proof_response_success(#[case] signer_type: SignerType) -> anyhow::Result<()> {
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = false;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let event_filter = input_verification
        .RejectProofResponse_filter()
        .watch()
        .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    let event_handle = tokio::spawn(async move {
        event_filter
            .into_stream()
            .take(1)
            .collect::<Vec<_>>()
            .await
            .first()
            .unwrap()
            .clone()
            .unwrap()
    });

    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, false)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42 as i64,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    let event = event_handle.await?;

    let expected_proof_id = U256::from(proof_id);

    assert_eq!(event.0.zkProofId, expected_proof_id);

    // Make sure the proof is removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
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
async fn verify_proof_response_reversal_already_verified(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = true;
    let already_rejected_revert = false;
    let other_revert = false;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Record initial transaction count.
    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[1u8; 64],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof is removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    // Verify that no transaction has been sent.
    let final_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;
    assert_eq!(
        final_tx_count, initial_tx_count,
        "Expected no new transaction to be sent"
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
async fn reject_proof_response_reversal_already_rejected(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = true;
    let other_revert = false;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Record initial transaction count.
    let initial_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;

    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, false)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof is removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    // Verify that no transaction has been sent.
    let final_tx_count = provider
        .get_transaction_count(TxSigner::address(&env.signer))
        .await?;
    assert_eq!(
        final_tx_count, initial_tx_count,
        "Expected no new transaction to be sent"
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
async fn verify_proof_response_other_reversal(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = true;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    // Create the sender with a gas limit such that no gas estimation is done, forcing failure at receipt (after the txn has been sent).
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        Some(1_000_000_000_000_000),
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[1u8; 64],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof retry count is incremented.
    //
    // Note this is a racy test, because the retry count is incremented by the transaction sender and it might
    // get to a point where retry count reaches max retries - then, transaction sender gives up and deletes the entry.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        assert_eq!(rows.len(), 1);
        if rows.first().unwrap().retry_count > 0 {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    env.cancel_token.cancel();
    run_handle.await??;

    // Make sure the entry is removed at the end of the test.
    sqlx::query("DELETE FROM verify_proofs WHERE zk_proof_id = $1")
        .bind(proof_id as i64)
        .execute(&env.db_pool)
        .await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn reject_proof_response_other_reversal(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = true;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    // Create the sender with a gas limit such that no gas estimation is done, forcing failure at receipt (after the txn has been sent).
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        Some(1_000_000_000_000_000),
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, false)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof retry count is incremented.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        assert_eq!(rows.len(), 1);
        if rows.first().unwrap().retry_count > 0 {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    env.cancel_token.cancel();
    run_handle.await??;

    // Make sure the entry is removed at the end of the test.
    sqlx::query("DELETE FROM verify_proofs WHERE zk_proof_id = $1")
        .bind(proof_id as i64)
        .execute(&env.db_pool)
        .await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn verify_proof_response_other_reversal_gas_estimation(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = true;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[1u8; 64],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof retry count is incremented.
    //
    // Note this is a racy test, because the retry count is incremented by the transaction sender and it might
    // get to a point where retry count reaches max retries - then, transaction sender gives up and deletes the entry.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        assert_eq!(rows.len(), 1);
        if rows.first().unwrap().retry_count > 0 {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    env.cancel_token.cancel();
    run_handle.await??;

    // Make sure the entry is removed at the end of the test.
    sqlx::query("DELETE FROM verify_proofs WHERE zk_proof_id = $1")
        .bind(proof_id as i64)
        .execute(&env.db_pool)
        .await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn reject_proof_response_other_reversal_gas_estimation(
    #[case] signer_type: SignerType,
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = true;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, false)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof retry count is incremented.
    //
    // Note this is a racy test, because the retry count is incremented by the transaction sender and it might
    // get to a point where retry count reaches max retries - then, transaction sender gives up and deletes the entry.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        assert_eq!(rows.len(), 1);
        if rows.first().unwrap().retry_count > 0 {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    env.cancel_token.cancel();
    run_handle.await??;

    // Make sure the entry is removed at the end of the test.
    sqlx::query("DELETE FROM verify_proofs WHERE zk_proof_id = $1")
        .bind(proof_id as i64)
        .execute(&env.db_pool)
        .await?;
    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn verify_proof_max_retries_remove_entry(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let mut env = TestEnvironment::new(signer_type).await?;
    env.conf.verify_proof_remove_after_max_retries = true;
    env.conf.verify_proof_resp_max_retries = 2;
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = true;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[1u8; 64],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Make sure the proof is removed from the database.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if rows.is_empty() {
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
async fn verify_proof_max_retries_do_not_remove_entry(
    #[case] signer_type: SignerType,
) -> anyhow::Result<()> {
    let mut env = TestEnvironment::new(signer_type).await?;
    env.conf.verify_proof_remove_after_max_retries = false;
    env.conf.verify_proof_resp_max_retries = 2;
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
    let already_verified_revert = false;
    let already_rejected_revert = false;
    let other_revert = true;
    let input_verification = InputVerification::deploy(
        &provider_deploy,
        already_verified_revert,
        already_rejected_revert,
        other_revert,
    )
    .await?;
    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(&provider_deploy, already_added_revert).await?;
    let txn_sender = TransactionSender::new(
        *input_verification.address(),
        *ciphertext_commits.address(),
        PrivateKeySigner::random().address(),
        env.signer.clone(),
        provider.clone(),
        provider.inner().clone(),
        env.cancel_token.clone(),
        env.conf.clone(),
        None,
    )
    .await?;

    let proof_id: u32 = random();

    let run_handle = tokio::spawn(async move { txn_sender.run().await });

    // Insert a proof into the database and notify the sender.
    sqlx::query!(
        "WITH ins AS (
            INSERT INTO verify_proofs (zk_proof_id, chain_id, contract_address, user_address, handles, verified)
            VALUES ($1, $2, $3, $4, $5, true)
        )
        SELECT pg_notify($6, '')",
        proof_id as i64,
        42,
        env.contract_address.to_string(),
        env.user_address.to_string(),
        &[1u8; 64],
        env.conf.verify_proof_resp_db_channel
    )
    .execute(&env.db_pool)
    .await?;

    // Wait until retry_count = 2.
    loop {
        let rows = sqlx::query!(
            "SELECT *
             FROM verify_proofs
             WHERE zk_proof_id = $1 AND retry_count = 2 AND verified = true",
            proof_id as i64,
        )
        .fetch_all(&env.db_pool)
        .await?;
        if !rows.is_empty() {
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    // Stop the transaction sender.
    env.cancel_token.cancel();
    run_handle.await??;

    // Make sure the entry is not removed.
    let rows = sqlx::query!(
        "SELECT *
         FROM verify_proofs
         WHERE zk_proof_id = $1 AND retry_count = 2 AND verified = true",
        proof_id as i64,
    )
    .fetch_all(&env.db_pool)
    .await?;
    assert_eq!(rows.len(), 1);

    Ok(())
}
