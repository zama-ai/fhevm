use crate::{
    config::KmsWallet,
    provider::{FillersWithoutNonceManagement, NonceManagedProvider},
};
use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, ProviderLayer, RootProvider, WsConnect,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, TxFiller,
            WalletFiller,
        },
    },
};
use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{sync::Once, time::Duration};
use tracing::{info, warn};

/// The number of connection retry to connect to the database or the Gateway RPC node.
pub const CONNECTION_RETRY_NUMBER: usize = 5;

/// The delay between two connection attempts.
pub const CONNECTION_RETRY_DELAY: Duration = Duration::from_secs(2);

/// Tries to establish the connection with Postgres database.
pub async fn connect_to_db(db_url: &str, db_pool_size: u32) -> anyhow::Result<Pool<Postgres>> {
    for i in 1..=CONNECTION_RETRY_NUMBER {
        info!("Attempting connection to DB... ({i}/{CONNECTION_RETRY_NUMBER})");

        let options = PgPoolOptions::new().max_connections(db_pool_size);
        match options.connect(db_url).await {
            Ok(db_pool) => {
                info!("Connected to Postgres database successfully");
                return Ok(db_pool);
            }
            Err(e) => warn!("DB connection attempt #{i} failed: {e}"),
        }

        if i != CONNECTION_RETRY_NUMBER {
            tokio::time::sleep(CONNECTION_RETRY_DELAY).await;
        }
    }
    Err(anyhow!("Could not connect to Postgres DB at url {db_url}"))
}

/// The default `Filler`s for an `alloy::Provider`.
type DefaultFillers = JoinFill<
    Identity,
    JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
>;

/// The default `alloy::Provider` used to interact with the Gateway.
pub type GatewayProvider = FillProvider<DefaultFillers, RootProvider>;

/// The default `alloy::Provider` used to interact with the Gateway using a wallet.
pub type WalletGatewayProvider = NonceManagedProvider<
    FillProvider<
        JoinFill<JoinFill<Identity, FillersWithoutNonceManagement>, WalletFiller<EthereumWallet>>,
        RootProvider,
    >,
>;

/// Tries to establish the connection with a RPC node of the Gateway.
pub async fn connect_to_gateway(gateway_url: &str) -> anyhow::Result<GatewayProvider> {
    connect_to_gateway_inner(gateway_url, ProviderBuilder::new).await
}

/// Tries to establish the connection with a RPC node of the Gateway, with a `WalletFiller`.
pub async fn connect_to_gateway_with_wallet(
    gateway_url: &str,
    wallet: KmsWallet,
) -> anyhow::Result<WalletGatewayProvider> {
    let provider = connect_to_gateway_inner(gateway_url, || {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
    })
    .await?;
    Ok(NonceManagedProvider::new(provider, wallet.address()))
}

/// Tries to establish the connection with a RPC node of the Gateway.
async fn connect_to_gateway_inner<L, F>(
    gateway_url: &str,
    provider_builder_new: impl Fn() -> ProviderBuilder<L, F>,
) -> anyhow::Result<F::Provider>
where
    L: ProviderLayer<RootProvider>,
    F: ProviderLayer<L::Provider> + TxFiller,
{
    INSTALL_CRYPTO_PROVIDER_ONCE.call_once(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .map_err(|e| anyhow!("Failed to install AWS-LC crypto provider: {e:?}"))
            .unwrap()
    });

    for i in 1..=CONNECTION_RETRY_NUMBER {
        info!("Attempting connection to Gateway... ({i}/{CONNECTION_RETRY_NUMBER})");

        let ws_endpoint = WsConnect::new(gateway_url);
        match provider_builder_new().connect_ws(ws_endpoint).await {
            Ok(provider) => {
                info!("Connected to Gateway's RPC node successfully");
                return Ok(provider);
            }
            Err(e) => warn!("Gateway connection attempt #{i} failed: {e}"),
        }

        if i != CONNECTION_RETRY_NUMBER {
            tokio::time::sleep(CONNECTION_RETRY_DELAY).await;
        }
    }
    Err(anyhow!("Could not connect to Gateway at url {gateway_url}"))
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();
