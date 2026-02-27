use crate::core::{Config, publish::publish_context_id};
use alloy::{network::Ethereum, providers::Provider};
use fhevm_host_bindings::kms_verifier::KMSVerifier::{self, KMSVerifierInstance};
use sqlx::{Pool, Postgres};
use tracing::info;

pub struct EthereumListener<P> {
    /// The database pool for storing Ethereum's events.
    db_pool: Pool<Postgres>,

    /// The `KMSVerifier` contract instance on Ethereum.
    kms_verifier_contract: KMSVerifierInstance<P>,
}

impl<P> EthereumListener<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Creates a new `EthereumListener` instance.
    pub fn new(db_pool: Pool<Postgres>, provider: P, config: &Config) -> Self {
        let kms_verifier_contract = KMSVerifier::new(config.kms_verifier_address, provider);
        Self {
            db_pool,
            kms_verifier_contract,
        }
    }

    /// Starts the `EthereumListener`.
    pub async fn start(self) {
        // No listening for now, will be done when implementing RFC-005.

        info!("EthereumListener stopped successfully!");
    }

    /// Stores the current context ID found on-chain in the database.
    pub async fn store_on_chain_context(&self) -> anyhow::Result<()> {
        let current_context_id = self
            .kms_verifier_contract
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
