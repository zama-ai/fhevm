use alloy::providers::{
    Identity, ProviderBuilder, RootProvider, WsConnect,
    fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
};
use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use tracing::{info, warn};

/// The number of connection retry to connect to the database or the Gateway RPC node.
const RETRY_NUMBER: usize = 5;

/// The delay between two connection attempts.
const RETRY_DELAY: Duration = Duration::from_secs(2);

/// Tries to establish the connection with Postgres database.
pub async fn connect_to_db(db_url: &str, db_pool_size: u32) -> anyhow::Result<Pool<Postgres>> {
    for i in 1..=RETRY_NUMBER {
        info!("Attempting connection to DB... ({i}/{RETRY_NUMBER})");

        let options = PgPoolOptions::new().max_connections(db_pool_size);
        match options.connect(db_url).await {
            Ok(db_pool) => {
                info!("Connected to Postgres database successfully");
                return Ok(db_pool);
            }
            Err(e) => warn!("DB connection attempt #{i} failed: {e}"),
        }

        if i != RETRY_NUMBER {
            tokio::time::sleep(RETRY_DELAY).await;
        }
    }
    Err(anyhow!("Could not connect to Postgres DB at url {db_url}"))
}

pub type GatewayProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

/// Tries to establish the connection with a RPC node of the Gateway.
pub async fn connect_to_gateway(gateway_url: &str) -> anyhow::Result<GatewayProvider> {
    for i in 1..=RETRY_NUMBER {
        info!("Attempting connection to Gateway... ({i}/{RETRY_NUMBER})");

        let ws_endpoint = WsConnect::new(gateway_url);
        match ProviderBuilder::new().on_ws(ws_endpoint).await {
            Ok(provider) => {
                info!("Connected to Gateway's RPC node successfully");
                return Ok(provider);
            }
            Err(e) => warn!("Gateway connection attempt #{i} failed: {e}"),
        }

        if i != RETRY_NUMBER {
            tokio::time::sleep(RETRY_DELAY).await;
        }
    }
    Err(anyhow!("Could not connect to Gateway at url {gateway_url}"))
}
