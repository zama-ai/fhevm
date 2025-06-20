use alloy::{
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, FixedBytes},
};
use connector_utils::conn::WalletGatewayProvider;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    gatewayconfig::GatewayConfig::{self, GatewayConfigInstance},
    kmsmanagement::KmsManagement::{self, KmsManagementInstance},
};
use std::sync::LazyLock;
use testcontainers::{GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};

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

pub async fn setup_anvil_gateway() -> anyhow::Result<AnvilInstance> {
    let anvil = Anvil::new()
        .mnemonic(TEST_MNEMONIC)
        .block_time(1)
        .chain_id(*CHAIN_ID as u64)
        .try_spawn()?;
    println!("Anvil started...");

    let _deploy_mock_container =
        GenericImage::new("ghcr.io/zama-ai/fhevm/gateway-contracts", "v0.7.0-rc0")
            .with_wait_for(WaitFor::message_on_stdout("Mock contract deployment done!"))
            .with_env_var("HARDHAT_NETWORK", "staging")
            .with_env_var("RPC_URL", anvil.endpoint_url().as_str())
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
            .with_network("host")
            .with_cmd(["npx hardhat task:deployGatewayMockContracts"])
            .start()
            .await?;
    println!("Mock contract successfully deployed on Anvil!");

    Ok(anvil)
}

pub struct GatewayInstance {
    pub anvil: AnvilInstance,
    pub provider: WalletGatewayProvider,
    pub decryption_contract: DecryptionInstance<(), WalletGatewayProvider>,
    pub gateway_config_contract: GatewayConfigInstance<(), WalletGatewayProvider>,
    pub kms_management_contract: KmsManagementInstance<(), WalletGatewayProvider>,
}

impl GatewayInstance {
    pub fn new(anvil: AnvilInstance, provider: WalletGatewayProvider) -> Self {
        let decryption_contract = Decryption::new(DECRYPTION_MOCK_ADDRESS, provider.clone());
        let gateway_config_contract =
            GatewayConfig::new(GATEWAY_CONFIG_MOCK_ADDRESS, provider.clone());
        let kms_management_contract =
            KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, provider.clone());

        GatewayInstance {
            anvil,
            provider,
            decryption_contract,
            gateway_config_contract,
            kms_management_contract,
        }
    }
}
