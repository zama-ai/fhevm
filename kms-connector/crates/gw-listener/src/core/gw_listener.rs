use super::{Config, EventPublisher};
use alloy::{contract::Event, network::Ethereum, providers::Provider, sol_types::SolEvent};
use connector_utils::types::GatewayEvent;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    kmsmanagement::KmsManagement::{self, KmsManagementInstance},
};
use tokio::task::JoinSet;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// Struct monitoring and storing Gateway's events.
#[derive(Clone)]
pub struct GatewayListener<Prov, Publ>
where
    Prov: Provider,
{
    /// The Gateway's `Decryption` contract instance which is monitored.
    decryption_contract: DecryptionInstance<(), Prov>,

    /// The Gateway's `KmsManagement` contract instance which is monitored.
    kms_management_contract: KmsManagementInstance<(), Prov>,

    /// The entity responsible of events publication to some external storage.
    publisher: Publ,
}

impl<Prov, Publ> GatewayListener<Prov, Publ>
where
    Prov: Provider<Ethereum> + Clone + 'static,
    Publ: EventPublisher + 'static,
{
    /// Creates a new `GatewayListener` instance.
    pub fn new(config: &Config, provider: Prov, publisher: Publ) -> Self {
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, provider.clone());
        let kms_management_contract =
            KmsManagement::new(config.kms_management_contract.address, provider);

        Self {
            decryption_contract,
            kms_management_contract,
            publisher,
        }
    }

    /// Starts the `GatewayListener`.
    pub async fn start(self, cancel_token: CancellationToken) {
        tokio::select! {
            _ = cancel_token.cancelled() => info!("GatewayListener cancelled..."),
            _ = self.run() => (),
        }
        info!("GatewayListener stopped successfully!");
    }

    /// Spawns and joins the `GatewayListener` event monitoring tasks.
    async fn run(self) {
        let mut tasks = JoinSet::new();

        tasks.spawn(self.clone().subscribe_to_public_decryption_requests());
        tasks.spawn(self.clone().subscribe_to_user_decryption_requests());
        tasks.spawn(self.clone().subscribe_to_preprocess_keygen_requests());
        tasks.spawn(self.clone().subscribe_to_preprocess_kskgen_requests());
        tasks.spawn(self.clone().subscribe_to_keygen_requests());
        tasks.spawn(self.clone().subscribe_to_kskgen_requests());
        tasks.spawn(self.subscribe_to_crsgen_requests());

        tasks.join_all().await;
    }

    /// Subscribes to a particular set of events.
    ///
    /// Each event received from the `event_filer` is then published using the `EventPublisher` of
    /// the `GatewayListener`.
    async fn subscribe_to_events<'a, E>(
        &'a self,
        event_name: &str,
        event_filter: Event<(), &'a Prov, E>,
    ) where
        E: Into<GatewayEvent> + SolEvent + Send + Sync,
    {
        info!("Starting {event_name} event subscriptions...");
        let mut events = match event_filter.watch().await {
            Ok(filter) => filter.into_stream(),
            Err(err) => {
                return error!("Failed to subscribe to {event_name} events: {err}");
            }
        };
        info!("✓ Subscribed to {event_name} events");

        loop {
            let req = match events.next().await {
                Some(Ok((req, _log))) => req,
                Some(Err(err)) => {
                    error!("Error while listening for {event_name} events: {err}");
                    continue;
                }
                None => break error!("Alloy Provider was dropped"),
            };
            if let Err(err) = self.publisher.publish(req).await {
                error!("Failed to publish {event_name}: {err}");
            }
        }
    }

    async fn subscribe_to_public_decryption_requests(self) {
        let public_decryption_filter = self.decryption_contract.PublicDecryptionRequest_filter();
        self.subscribe_to_events("PublicDecryptionRequest", public_decryption_filter)
            .await;
    }

    async fn subscribe_to_user_decryption_requests(self) {
        let user_decryption_filter = self.decryption_contract.UserDecryptionRequest_filter();
        self.subscribe_to_events("UserDecryptionRequest", user_decryption_filter)
            .await;
    }

    async fn subscribe_to_preprocess_keygen_requests(self) {
        let preprocess_keygen_filter = self
            .kms_management_contract
            .PreprocessKeygenRequest_filter();
        self.subscribe_to_events("PreprocessKeygenRequest", preprocess_keygen_filter)
            .await;
    }

    async fn subscribe_to_preprocess_kskgen_requests(self) {
        let preprocess_kskgen_filter = self
            .kms_management_contract
            .PreprocessKskgenRequest_filter();
        self.subscribe_to_events("PreprocessKskgenRequest", preprocess_kskgen_filter)
            .await;
    }

    async fn subscribe_to_keygen_requests(self) {
        let keygen_filter = self.kms_management_contract.KeygenRequest_filter();
        self.subscribe_to_events("KeygenRequest", keygen_filter)
            .await;
    }

    async fn subscribe_to_kskgen_requests(self) {
        let kskgen_filter = self.kms_management_contract.KskgenRequest_filter();
        self.subscribe_to_events("KskgenRequest", kskgen_filter)
            .await;
    }

    async fn subscribe_to_crsgen_requests(self) {
        let crsgen_filter = self.kms_management_contract.CrsgenRequest_filter();
        self.subscribe_to_events("CrsgenRequest", crsgen_filter)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Address, IntoLogData},
        providers::{
            Identity, ProviderBuilder, RootProvider,
            fillers::{
                BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            },
            mock::Asserter,
        },
    };
    use anyhow::Result;
    use fhevm_gateway_rust_bindings::{
        decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
        kmsmanagement::KmsManagement::{
            CrsgenRequest, KeygenRequest, KskgenRequest, PreprocessKeygenRequest,
            PreprocessKskgenRequest,
        },
    };
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_public_decryption_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(PublicDecryptionRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_public_decryption_requests().await;
        assert!(logs_contain("PublicDecryptionRequest published!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_user_decryption_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(UserDecryptionRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_user_decryption_requests().await;
        assert!(logs_contain("UserDecryptionRequest published!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_preprocess_keygen_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(PreprocessKeygenRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_preprocess_keygen_requests().await;
        assert!(logs_contain("PreprocessKeygenRequest published!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_preprocess_kskgen_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(PreprocessKskgenRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_preprocess_kskgen_requests().await;
        assert!(logs_contain("PreprocessKskgenRequest published!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_keygen_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(KeygenRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_keygen_requests().await;
        assert!(logs_contain("KeygenRequest published!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_kskgen_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(KskgenRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_kskgen_requests().await;
        assert!(logs_contain("KskgenRequest published!"));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_crsgen_requests_subscription() {
        let (asserter, gw_listener) = test_setup().await;

        // Used to mock a new event
        let rpc_event_log = mock_rpc_event_log(CrsgenRequest::default());
        asserter.push_success(&[rpc_event_log]);

        gw_listener.subscribe_to_crsgen_requests().await;
        assert!(logs_contain("CrsgenRequest published!"));
    }

    /// Mock the log generated by the publication of a Gateway event.
    fn mock_rpc_event_log(event: impl IntoLogData) -> alloy::rpc::types::Log {
        let event_log = alloy::primitives::Log {
            address: Address::default(),
            data: event.to_log_data(),
        };
        alloy::rpc::types::Log {
            inner: event_log,
            block_number: Some(0),
            ..Default::default()
        }
    }

    type MockProvider = FillProvider<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >;

    async fn test_setup() -> (Asserter, GatewayListener<MockProvider, MockPublisher>) {
        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().on_mocked_client(asserter.clone());

        // Used to mock response of `filter.watch()` operation
        let mocked_eth_get_filter_changes_result = Address::default();
        asserter.push_success(&mocked_eth_get_filter_changes_result);

        let mock_publisher = MockPublisher::new();
        let config = Config::default();
        let listener = GatewayListener::new(&config, mock_provider, mock_publisher);
        (asserter, listener)
    }

    #[derive(Clone)]
    struct MockPublisher;

    impl MockPublisher {
        pub fn new() -> Self {
            MockPublisher {}
        }
    }

    impl EventPublisher for MockPublisher {
        async fn publish(&self, event: impl Into<GatewayEvent>) -> Result<()> {
            match event.into() {
                GatewayEvent::PublicDecryption(_) => info!("PublicDecryptionRequest published!"),
                GatewayEvent::UserDecryption(_) => info!("UserDecryptionRequest published!"),
                GatewayEvent::PreprocessKeygen(_) => info!("PreprocessKeygenRequest published!"),
                GatewayEvent::PreprocessKskgen(_) => info!("PreprocessKskgenRequest published!"),
                GatewayEvent::Keygen(_) => info!("KeygenRequest published!"),
                GatewayEvent::Kskgen(_) => info!("KskgenRequest published!"),
                GatewayEvent::Crsgen(_) => info!("CrsgenRequest published!"),
            }
            Ok(())
        }
    }
}
