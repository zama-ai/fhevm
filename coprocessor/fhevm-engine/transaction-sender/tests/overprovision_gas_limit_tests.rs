mod common;

use alloy::primitives::{FixedBytes, U256};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use common::SignerType;
use common::{CiphertextCommits, TestEnvironment};
use rstest::*;
use serial_test::serial;
use std::time::Duration;
use transaction_sender::NonceManagedProvider;

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn overprovision_gas_limit(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let env = TestEnvironment::new(signer_type).await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::new()
            .wallet(env.wallet.clone())
            .connect_ws(WsConnect::new(env.ws_endpoint_url()))
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(provider.inner().clone(), already_added_revert).await?;

    let txn_req = ciphertext_commits
        .addCiphertextMaterial(
            FixedBytes([1u8; 32]),
            U256::from(1),
            FixedBytes([2u8; 32]),
            FixedBytes([3u8; 32]),
        )
        .into_transaction_request();

    assert!(
        txn_req.gas.is_none(),
        "Gas limit should not be set initially"
    );

    let without_overprovision = provider.inner().estimate_gas(txn_req.clone()).await?;
    let with_overprovision = provider
        .overprovision_gas_limit(txn_req, 120)
        .await?
        .gas
        .expect("Gas limit is set after overprovisioning");

    assert_eq!(
        with_overprovision,
        (without_overprovision * 120) / 100,
        "Overprovisioned gas limit should be greater than the estimated gas limit"
    );

    Ok(())
}

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn overprovision_estimate_failure(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let mut env = TestEnvironment::new(signer_type).await?;
    let provider = NonceManagedProvider::new(
        ProviderBuilder::new()
            .wallet(env.wallet.clone())
            .connect_ws(
                // Reduce the retries count and the interval for alloy's internal retry to make this test faster.
                WsConnect::new(env.ws_endpoint_url())
                    .with_max_retries(2)
                    .with_retry_interval(Duration::from_millis(100)),
            )
            .await?,
        Some(env.wallet.default_signer().address()),
    );

    let already_added_revert = false;
    let ciphertext_commits =
        CiphertextCommits::deploy(provider.inner().clone(), already_added_revert).await?;

    let txn_req = ciphertext_commits
        .addCiphertextMaterial(
            FixedBytes([1u8; 32]),
            U256::from(1),
            FixedBytes([2u8; 32]),
            FixedBytes([3u8; 32]),
        )
        .into_transaction_request();

    assert!(
        txn_req.gas.is_none(),
        "Gas limit should not be set initially"
    );

    env.drop_anvil();

    let with_overprovision = provider.overprovision_gas_limit(txn_req, 120).await;

    assert!(with_overprovision.is_err(), "Gas limit should not be set");

    Ok(())
}
