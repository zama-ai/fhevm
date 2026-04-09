use crate::core::{Config, publish::publish_context_id};
use alloy::{network::Ethereum, providers::Provider};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use sqlx::{Pool, Postgres};
use tokio::time::{Duration, sleep};
use tracing::{info, warn};

pub struct EthereumListener<P> {
    /// The database pool for storing Ethereum's events.
    db_pool: Pool<Postgres>,

    /// The `GatewayConfig` contract instance on Ethereum.
    gateway_config_contract: GatewayConfigInstance<P>,

    /// Whether to skip the legacy host-side context bootstrap.
    skip_context_bootstrap: bool,
}

impl<P> EthereumListener<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Creates a new `EthereumListener` instance.
    pub fn new(db_pool: Pool<Postgres>, provider: P, config: &Config) -> Self {
        let gateway_config_contract =
            GatewayConfig::new(config.gateway_config_contract.address, provider);
        Self {
            db_pool,
            gateway_config_contract,
            skip_context_bootstrap: config.skip_ethereum_context_bootstrap,
        }
    }

    /// Starts the `EthereumListener`.
    pub async fn start(self) {
        if self.skip_context_bootstrap {
            info!("Skipping legacy host-side KMS context bootstrap");
            info!("EthereumListener stopped successfully!");
            return;
        }

        loop {
            match self.store_on_chain_context().await {
                Ok(()) => break,
                Err(error) => {
                    warn!(
                        "Failed to store current context yet: {error}; retrying in 5s"
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }

        info!("EthereumListener stopped successfully!");
    }

    /// Stores the current context ID found on-chain in the database.
    pub async fn store_on_chain_context(&self) -> anyhow::Result<()> {
        let current_context_id = self
            .gateway_config_contract
            .getCurrentKmsContextId()
            .call()
            .await?;

        publish_context_id(&self.db_pool, current_context_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::U256,
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };
    use connector_utils::tests::setup::TestInstanceBuilder;
    use sqlx::Row;
    use std::time::Duration;

    #[rstest::rstest]
    #[timeout(Duration::from_secs(90))]
    #[tokio::test]
    async fn test_store_current_context_id() {
        let test_instance = TestInstanceBuilder::db_setup().await.unwrap();
        let context_id = U256::from(79_u64);

        let asserter = Asserter::new();
        asserter.push_success(&context_id.abi_encode());

        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);

        let config = Config::default();
        let listener = EthereumListener::new(test_instance.db().clone(), mock_provider, &config);

        listener.store_on_chain_context().await.unwrap();

        let row = sqlx::query("SELECT is_valid FROM kms_context WHERE id = $1")
            .bind(context_id.as_le_slice())
            .fetch_one(test_instance.db())
            .await
            .unwrap();

        assert!(row.try_get::<bool, _>("is_valid").unwrap());
    }
}
