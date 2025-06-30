mod common;

use alloy::primitives::{FixedBytes, U256};
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use common::SignerType;
use common::{CiphertextCommits, TestEnvironment};
use rstest::*;
use serial_test::serial;
use std::time::Duration;
use transaction_sender::overprovision_gas_limit::try_overprovision_gas_limit;

#[rstest]
#[case::private_key(SignerType::PrivateKey)]
#[case::aws_kms(SignerType::AwsKms)]
#[tokio::test]
#[serial(db)]
async fn overprovision_gas_limit(#[case] signer_type: SignerType) -> anyhow::Result<()> {
    let env = TestEnvironment::new(signer_type).await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.ws_endpoint_url()))
        .await?;

    let already_added_revert = false;
    let ciphertext_commits = CiphertextCommits::deploy(&provider, already_added_revert).await?;

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

    let without_overprovision = provider.estimate_gas(txn_req.clone()).await?;
    let with_overprovision = try_overprovision_gas_limit(txn_req, &provider, 120)
        .await
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
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(
            // Reduce the retries count and the interval for alloy's internal retry to make this test faster.
            WsConnect::new(env.ws_endpoint_url())
                .with_max_retries(2)
                .with_retry_interval(Duration::from_millis(100)),
        )
        .await?;

    let already_added_revert = false;
    let ciphertext_commits = CiphertextCommits::deploy(&provider, already_added_revert).await?;

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

    let with_overprovision = try_overprovision_gas_limit(txn_req, &provider, 120)
        .await
        .gas;

    assert!(with_overprovision.is_none(), "Gas limit should not be set");

    Ok(())
}
