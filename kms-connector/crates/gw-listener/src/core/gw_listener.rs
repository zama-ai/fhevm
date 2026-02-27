use crate::{
    core::{Config, ethereum::EthereumListener, gateway::GatewayListener},
    monitoring::health::State,
};
use alloy::{network::Ethereum, providers::Provider};
use connector_utils::conn::{DefaultProvider, connect_to_db, connect_to_rpc_node};
use tokio::join;
use tokio_util::sync::CancellationToken;
use tracing::info;

/// Struct monitoring and storing events of the Zama Protocol.
pub struct EventListener<GP, HP>
where
    GP: Provider,
    HP: Provider,
{
    /// The listener of Gateway's events.
    gateway_listener: GatewayListener<GP>,

    /// The listener of Ethereum's events.
    ethereum_listener: EthereumListener<HP>,
}

impl<GP, HP> EventListener<GP, HP>
where
    GP: Provider<Ethereum> + Clone + 'static,
    HP: Provider<Ethereum> + Clone + 'static,
{
    pub fn new(
        gateway_listener: GatewayListener<GP>,
        ethereum_listener: EthereumListener<HP>,
    ) -> Self {
        Self {
            gateway_listener,
            ethereum_listener,
        }
    }

    pub async fn start(self) {
        join!(
            self.gateway_listener.start(),
            self.ethereum_listener.start()
        );
        info!("EventListener stopped successfully!");
    }
}

impl EventListener<DefaultProvider, DefaultProvider> {
    /// Creates a new `EventListener` instance from a valid `Config`.
    pub async fn from_config(
        config: Config,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<(Self, State<DefaultProvider>)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let gateway_provider =
            connect_to_rpc_node(config.gateway_url.clone(), config.gateway_chain_id).await?;
        let ethereum_provider =
            connect_to_rpc_node(config.ethereum_url.clone(), config.ethereum_chain_id).await?;

        let state = State::new(
            db_pool.clone(),
            gateway_provider.clone(),
            config.healthcheck_timeout,
        );

        let gateway_listener =
            GatewayListener::new(db_pool.clone(), gateway_provider, &config, cancel_token);
        let ethereum_listener = EthereumListener::new(db_pool, ethereum_provider, &config);
        let event_listener = EventListener::new(gateway_listener, ethereum_listener);
        Ok((event_listener, state))
    }
}
