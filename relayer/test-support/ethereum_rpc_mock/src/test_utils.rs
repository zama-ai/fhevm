//! Test utilities and helpers
//!
//! This module provides utilities specifically for testing with ethereum_rpc_mock.
//! While technically available in all builds, these functions are intended only for testing.

use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use rand::{rngs::StdRng, RngExt, SeedableRng};

use crate::{MockConfig, MockServer};

/// Generate n test addresses with fixed seed for reproducibility
///
/// # For Testing Only  
/// This function is intended for test usage only.
pub fn generate_test_addresses(count: usize) -> Vec<Address> {
    let mut rng = StdRng::seed_from_u64(12345);
    (0..count)
        .map(|_| {
            let bytes: [u8; 20] = rng.random();
            Address::from(bytes)
        })
        .collect()
}

/// Create a test wallet with deterministic private key
///
/// # For Testing Only
/// This function is intended for test usage only.
pub fn create_test_wallet() -> (PrivateKeySigner, Address) {
    let mut rng = StdRng::seed_from_u64(54321);
    let private_key_bytes: [u8; 32] = rng.random();
    let signer = PrivateKeySigner::from_bytes(&private_key_bytes.into()).unwrap();
    let address = signer.address();
    (signer, address)
}

/// Test with both HTTP and WebSocket providers
///
/// # For Testing Only
/// This function is intended for test usage only.
pub async fn test_with_both_transports<F, Fut>(
    test_name: &str,
    test_fn: F,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: Fn(Box<dyn Provider<AnyNetwork> + Send + Sync>, &str) -> Fut + Send + Sync + Copy,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
{
    let transports = [("HTTP", false), ("WebSocket", true)];

    for (name, is_ws) in transports {
        println!("Running {} with {}", test_name, name);

        let config = MockConfig {
            port: 0,
            chain_id: 1337,
            ..MockConfig::new()
        };
        let server = MockServer::new(config);
        let handle = server.clone().start().await?;
        let port = handle.port();

        // Wait a bit for server to start
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let provider: Box<dyn Provider<AnyNetwork> + Send + Sync> = if is_ws {
            let url = format!("ws://127.0.0.1:{}/ws", port);
            Box::new(
                ProviderBuilder::new()
                    .network::<AnyNetwork>()
                    .connect(&url)
                    .await?,
            )
        } else {
            let url = format!("http://127.0.0.1:{}", port);
            Box::new(
                ProviderBuilder::new()
                    .network::<AnyNetwork>()
                    .connect_http(url.parse()?),
            )
        };

        let result = test_fn(provider, name).await;
        handle.shutdown().await?;
        result?;
    }
    Ok(())
}
