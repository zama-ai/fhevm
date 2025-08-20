use crate::{
    config::KmsWallet,
    conn::WalletGatewayProvider,
    provider::{FillersWithoutNonceManagement, NonceManagedProvider},
    tests::setup::pick_free_port,
    // tests::setup::{ROOT_CARGO_TOML, pick_free_port},
};
use alloy::{
    primitives::{Address, ChainId, FixedBytes},
    providers::{ProviderBuilder, WsConnect},
};
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    gateway_config::GatewayConfig::{self, GatewayConfigInstance},
    kms_management::KmsManagement::{self, KmsManagementInstance},
};
use std::{sync::LazyLock, time::Duration};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{WaitFor, client::docker_client_instance},
    runners::AsyncRunner,
};
use tracing::info;

pub const DECRYPTION_MOCK_ADDRESS: Address = Address(FixedBytes([
    184, 174, 68, 54, 92, 69, 167, 197, 37, 107, 20, 246, 7, 202, 226, 59, 192, 64, 195, 84,
]));
pub const GATEWAY_CONFIG_MOCK_ADDRESS: Address = Address(FixedBytes([
    159, 167, 153, 249, 90, 114, 37, 140, 4, 21, 223, 237, 216, 207, 118, 210, 97, 60, 117, 15,
]));
pub const KMS_MANAGEMENT_MOCK_ADDRESS: Address = Address(FixedBytes([
    200, 27, 227, 169, 24, 21, 210, 212, 9, 109, 174, 8, 26, 113, 22, 201, 250, 123, 223, 8,
]));

pub const TEST_MNEMONIC: &str =
    "coyote sketch defense hover finger envelope celery urge panther venue verb cheese";

pub static CHAIN_ID: LazyLock<u32> = LazyLock::new(rand::random::<u32>);

pub const DEPLOYER_PRIVATE_KEY: &str =
    "0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c";

const ANVIL_PORT: u16 = 8545;

pub struct GatewayInstance {
    pub provider: WalletGatewayProvider,
    pub decryption_contract: DecryptionInstance<WalletGatewayProvider>,
    pub gateway_config_contract: GatewayConfigInstance<WalletGatewayProvider>,
    pub kms_management_contract: KmsManagementInstance<WalletGatewayProvider>,
    pub anvil: ContainerAsync<GenericImage>,
    pub anvil_host_port: u16,
    pub block_time: u64,
}

impl GatewayInstance {
    pub fn new(
        anvil: ContainerAsync<GenericImage>,
        anvil_host_port: u16,
        provider: WalletGatewayProvider,
        block_time: u64,
    ) -> Self {
        let decryption_contract = Decryption::new(DECRYPTION_MOCK_ADDRESS, provider.clone());
        let gateway_config_contract =
            GatewayConfig::new(GATEWAY_CONFIG_MOCK_ADDRESS, provider.clone());
        let kms_management_contract =
            KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, provider.clone());

        GatewayInstance {
            provider,
            decryption_contract,
            gateway_config_contract,
            kms_management_contract,
            anvil,
            anvil_host_port,
            block_time,
        }
    }

    pub async fn setup() -> anyhow::Result<Self> {
        let block_time = 1;
        let anvil_host_port = pick_free_port();
        let anvil: ContainerAsync<GenericImage> =
            setup_anvil_gateway(anvil_host_port, block_time).await?;
        let wallet = KmsWallet::from_private_key_str(
            DEPLOYER_PRIVATE_KEY,
            Some(ChainId::from(*CHAIN_ID as u64)),
        )?;
        let wallet_addr = wallet.address();

        let inner_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet)
            .connect_ws(WsConnect::new(Self::anvil_ws_endpoint_impl(
                anvil_host_port,
            )))
            .await?;
        let provider = NonceManagedProvider::new(inner_provider, wallet_addr);

        Ok(GatewayInstance::new(
            anvil,
            anvil_host_port,
            provider,
            block_time,
        ))
    }

    pub fn anvil_block_time(&self) -> Duration {
        Duration::from_secs(self.block_time)
    }

    fn anvil_ws_endpoint_impl(anvil_host_port: u16) -> String {
        format!("ws://localhost:{anvil_host_port}")
    }

    pub fn anvil_ws_endpoint(&self) -> String {
        Self::anvil_ws_endpoint_impl(self.anvil_host_port)
    }
}

pub async fn setup_anvil_gateway(
    host_port: u16,
    block_time: u64,
) -> anyhow::Result<ContainerAsync<GenericImage>> {
    info!("Starting Anvil...");
    let anvil = GenericImage::new("ghcr.io/foundry-rs/foundry", "v1.2.3")
        .with_wait_for(WaitFor::message_on_stdout("Listening"))
        .with_entrypoint("anvil")
        .with_cmd([
            "--host",
            "0.0.0.0",
            "--port",
            ANVIL_PORT.to_string().as_str(),
            "--chain-id",
            CHAIN_ID.to_string().as_str(),
            "--mnemonic",
            TEST_MNEMONIC,
            "--block-time",
            &format!("{block_time}"),
        ])
        .with_mapped_port(host_port, ANVIL_PORT.into())
        .start()
        .await?;

    let docker = docker_client_instance().await?;
    let inspect = docker.inspect_container(anvil.id(), None).await?;
    let networks = inspect.network_settings.unwrap().networks.unwrap();
    let endpoint_settings = networks.values().next().unwrap();
    let anvil_internal_ip = endpoint_settings.ip_address.clone().unwrap();

    info!("Deploying Gateway mock contracts...");
    // TODO: fix this
    // let version = ROOT_CARGO_TOML.get_gateway_bindings_version();
    let _deploy_mock_container =
        // GenericImage::new("ghcr.io/zama-ai/fhevm/gateway-contracts", &version)
        GenericImage::new("ghcr.io/zama-ai/fhevm/gateway-contracts", "b7da937")
            .with_wait_for(WaitFor::message_on_stdout("Mock contract deployment done!"))
            .with_env_var("HARDHAT_NETWORK", "staging")
            .with_env_var(
                "RPC_URL",
                format!("http://{anvil_internal_ip}:{ANVIL_PORT}"),
            )
            .with_env_var("CHAIN_ID_GATEWAY", format!("{}", *CHAIN_ID))
            .with_env_var("MNEMONIC", TEST_MNEMONIC)
            .with_env_var(
                "DEPLOYER_ADDRESS",
                "0xCf28E90D4A6dB23c34E1881aEF5fd9fF2e478634",
            ) // accounts[1]
            .with_env_var("DEPLOYER_PRIVATE_KEY", DEPLOYER_PRIVATE_KEY) // accounts[1]
            .with_env_var(
                "PAUSER_ADDRESS",
                "0xfCefe53c7012a075b8a711df391100d9c431c468",
            )
            .with_cmd(["npx hardhat task:deployGatewayMockContracts"])
            .start()
            .await?;
    info!("Mock contract successfully deployed on Anvil!");

    Ok(anvil)
}
