use anyhow::anyhow;
use std::sync::Once;

pub fn install_crypto_provider() {
    INSTALL_CRYPTO_PROVIDER_ONCE.call_once(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .map_err(|e| anyhow!("Failed to install AWS-LC crypto provider: {e:?}"))
            .unwrap()
    });
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();
