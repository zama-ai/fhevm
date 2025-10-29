mod common;

use alloy::primitives::Address;
use alloy::providers::fillers::NonceManager;
use alloy::providers::ProviderBuilder;
use fhevm_relayer::blockchain::gateway::arbitrum::transaction::nonce::CachedNonceManagerWithRefresh;
use reqwest::Url;

#[test]
fn test() {
    let val = true;
    assert!(val);
}

#[tokio::test]
async fn cloned_managers() {
    let setup = common::utils::TestSetup::new()
        .await
        .expect("Failed to create test setup");
    let cnm1 = CachedNonceManagerWithRefresh::default();
    let cnm2 = cnm1.clone();

    let provider = ProviderBuilder::new()
        .connect_http(Url::parse(&setup.settings.networks.fhevm.http_url).unwrap());
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
