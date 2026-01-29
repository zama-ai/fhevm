use crate::{
    config::KmsWallet,
    provider::{FillersWithoutNonceManagement, NonceManagedProvider},
};
use alloy::{
    network::EthereumWallet,
    providers::{
        Identity, ProviderBuilder, ProviderLayer, RootProvider,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, TxFiller,
            WalletFiller,
        },
    },
    transports::http::reqwest::Url,
};
use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{sync::Once, time::Duration};
use tracing::{info, warn};

/// The number of connection retry to connect to the database or to a RPC node.
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

/// The default `alloy::Provider` used to interact with the Gateway/Host chain.
pub type DefaultProvider = FillProvider<JoinFill<DefaultFillers, ChainIdFiller>, RootProvider>;

/// The default `alloy::Provider` used to interact with the Gateway/Host chain using a wallet.
pub type WalletProvider = NonceManagedProvider<WalletProviderFillers, RootProvider>;
pub type WalletProviderFillers = JoinFill<
    JoinFill<JoinFill<Identity, ChainIdFiller>, FillersWithoutNonceManagement>,
    WalletFiller<EthereumWallet>,
>;

/// Tries to establish the connection with a RPC node.
pub async fn connect_to_rpc_node(
    rpc_node_url: Url,
    chain_id: u64,
) -> anyhow::Result<DefaultProvider> {
    connect_to_rpc_node_inner(rpc_node_url, || {
        ProviderBuilder::new().with_chain_id(chain_id)
    })
    .await
}

/// Tries to establish the connection with a RPC node, with a `WalletFiller`.
pub async fn connect_to_rpc_node_with_wallet(
    rpc_node_url: Url,
    chain_id: u64,
    wallet: KmsWallet,
) -> anyhow::Result<WalletProvider> {
    let provider = connect_to_rpc_node_inner(rpc_node_url, || {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .with_chain_id(chain_id)
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
    })
    .await?;
    Ok(NonceManagedProvider::new(provider, wallet.address()))
}

/// Tries to establish the connection with a RPC node.
async fn connect_to_rpc_node_inner<L, F>(
    rpc_node_url: Url,
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

    let provider = provider_builder_new().connect_http(rpc_node_url);
    info!("Connected to RPC node successfully");
    Ok(provider)
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();
