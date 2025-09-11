mod utils;

use alloy::consensus::Transaction;
use alloy::primitives::address;
use alloy::primitives::Address;
use alloy::primitives::U256;
use alloy::providers::fillers::NonceFiller;
use alloy::providers::fillers::NonceManager;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::WalletProvider;
use alloy::rpc::types::TransactionRequest;
use fhevm_relayer::transaction::nonce::CachedNonceManagerWithRefresh;
use reqwest::Url;

#[test]
fn test() {
    let val = true;
    assert!(val);
}

// TODO: Remove usage of `connect_anvil_with_wallet` and `connect_anvil` that relies on
// `anvil` being available on the test machine.
#[tokio::test]
async fn increments_nonce() {
    let _ = crate::utils::ensure_relayer_started().await;
    let cnm1 = CachedNonceManagerWithRefresh::default();
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .filler(NonceFiller::new(cnm1))
        .connect_anvil_with_wallet();

    let from = provider.default_signer_address();
    let tx = TransactionRequest {
        from: Some(from),
        value: Some(U256::from(100)),
        to: Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into()),
        gas_price: Some(20e9 as u128),
        gas: Some(21000),
        ..Default::default()
    };

    let pending = provider.send_transaction(tx.clone()).await.unwrap();
    let tx_hash = pending.watch().await.unwrap();
    let mined_tx = provider
        .get_transaction_by_hash(tx_hash)
        .await
        .expect("failed to fetch tx")
        .expect("tx not included");
    assert_eq!(mined_tx.nonce(), 0);

    let pending = provider.send_transaction(tx).await.unwrap();
    let tx_hash = pending.watch().await.unwrap();
    let mined_tx = provider
        .get_transaction_by_hash(tx_hash)
        .await
        .expect("fail to fetch tx")
        .expect("tx didn't finalize");
    assert_eq!(mined_tx.nonce(), 1);
}

#[tokio::test]
async fn cloned_managers() {
    let _ = crate::utils::ensure_relayer_started().await;
    let cnm1 = CachedNonceManagerWithRefresh::default();
    let cnm2 = cnm1.clone();

    let provider =
        ProviderBuilder::new().connect_http(Url::parse("http://localhost:8756").unwrap());
    let address = Address::ZERO;

    assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 0);
    assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 1);
    assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 2);
    assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 3);

    let _ = cnm1.sync_nonce(&provider, address).await;

    assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 0);
    assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 1);
    assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 2);
    assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 3);
}
