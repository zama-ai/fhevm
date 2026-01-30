use crate::{
    config::KmsWallet,
    conn::WalletProvider,
    provider::{FillersWithoutNonceManagement, NonceManagedProvider},
};
use alloy::{
    primitives::{Address, ChainId},
    providers::ProviderBuilder,
    transports::http::reqwest::Url,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    gateway_config::GatewayConfig::{self, GatewayConfigInstance},
    kms_generation::KMSGeneration::{self, KMSGenerationInstance},
};
use std::{path::PathBuf, process::Command, str::FromStr, sync::LazyLock, time::Duration};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tracing::info;

pub const TEST_MNEMONIC: &str =
    "coyote sketch defense hover finger envelope celery urge panther venue verb cheese";

pub static CHAIN_ID: LazyLock<u32> = LazyLock::new(rand::random::<u32>);

pub const DEPLOYER_PRIVATE_KEY: &str =
    "0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c";

const ANVIL_PORT: u16 = 8545;

pub struct GatewayInstance {
    pub provider: WalletProvider,
    pub decryption_contract: DecryptionInstance<WalletProvider>,
    pub gateway_config_contract: GatewayConfigInstance<WalletProvider>,
    pub kms_generation_contract: KMSGenerationInstance<WalletProvider>,
    pub anvil: ContainerAsync<GenericImage>,
    pub anvil_host_port: u16,
    pub block_time: u64,
}

impl GatewayInstance {
    pub fn new(
        anvil: ContainerAsync<GenericImage>,
        anvil_host_port: u16,
        provider: WalletProvider,
        decryption_address: Address,
        gateway_config_address: Address,
        kms_generation_address: Address,
        block_time: u64,
    ) -> Self {
        let decryption_contract = Decryption::new(decryption_address, provider.clone());
        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider.clone());
        let kms_generation_contract = KMSGeneration::new(kms_generation_address, provider.clone());

        GatewayInstance {
            provider,
            decryption_contract,
            gateway_config_contract,
            kms_generation_contract,
            anvil,
            anvil_host_port,
            block_time,
        }
    }

    pub async fn setup() -> anyhow::Result<Self> {
        let block_time = 1;

        let anvil = setup_anvil(block_time).await?;
        let anvil_host_port = anvil.get_host_port_ipv4(ANVIL_PORT).await?;

        let wallet = KmsWallet::from_private_key_str(
            DEPLOYER_PRIVATE_KEY,
            Some(ChainId::from(*CHAIN_ID as u64)),
        )?;
        let wallet_addr = wallet.address();

        let inner_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .with_chain_id(*CHAIN_ID as u64)
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet)
            .connect_http(Self::anvil_http_endpoint_impl(anvil_host_port));
        let provider = NonceManagedProvider::new(inner_provider, wallet_addr);

        // Deploy mock contracts via forge create
        info!("Deploying Gateway mock contracts via forge...");

        let gateway_contracts_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../gateway-contracts")
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Could not find gateway-contracts directory: {e}"))?;

        let rpc_url = Self::anvil_http_endpoint_impl(anvil_host_port).to_string();
        let private_key = DEPLOYER_PRIVATE_KEY.to_string();

        let (decryption_addr, gateway_config_addr, kms_generation_addr) =
            tokio::task::spawn_blocking(move || {
                let decryption = deploy_contract(
                    "contracts/mocks/DecryptionMock.sol",
                    "DecryptionMock",
                    &rpc_url,
                    &private_key,
                    &gateway_contracts_path,
                )?;

                let gateway_config = deploy_contract(
                    "contracts/mocks/GatewayConfigMock.sol",
                    "GatewayConfigMock",
                    &rpc_url,
                    &private_key,
                    &gateway_contracts_path,
                )?;

                let kms_generation = deploy_contract(
                    "contracts/mocks/KMSGenerationMock.sol",
                    "KMSGenerationMock",
                    &rpc_url,
                    &private_key,
                    &gateway_contracts_path,
                )?;

                Ok::<_, anyhow::Error>((decryption, gateway_config, kms_generation))
            })
            .await??;

        info!("DecryptionMock deployed at: {}", decryption_addr);
        info!("GatewayConfigMock deployed at: {}", gateway_config_addr);
        info!("KMSGenerationMock deployed at: {}", kms_generation_addr);

        Ok(GatewayInstance::new(
            anvil,
            anvil_host_port,
            provider,
            decryption_addr,
            gateway_config_addr,
            kms_generation_addr,
            block_time,
        ))
    }

    pub fn anvil_block_time(&self) -> Duration {
        Duration::from_secs(self.block_time)
    }

    fn anvil_http_endpoint_impl(anvil_host_port: u16) -> Url {
        format!("http://localhost:{anvil_host_port}")
            .parse()
            .unwrap()
    }

    pub fn anvil_http_endpoint(&self) -> Url {
        Self::anvil_http_endpoint_impl(self.anvil_host_port)
    }
}

async fn setup_anvil(block_time: u64) -> anyhow::Result<ContainerAsync<GenericImage>> {
    info!("Starting Anvil...");
    let anvil = GenericImage::new("ghcr.io/foundry-rs/foundry", "v1.3.5")
        .with_exposed_port(ContainerPort::Tcp(ANVIL_PORT))
        .with_wait_for(WaitFor::message_on_stdout("Listening"))
        .with_entrypoint("anvil")
        .with_cmd([
            "--host",
            "0.0.0.0",
            "--chain-id",
            CHAIN_ID.to_string().as_str(),
            "--mnemonic",
            TEST_MNEMONIC,
            "--block-time",
            &format!("{block_time}"),
        ])
        .start()
        .await?;

    Ok(anvil)
}

fn deploy_contract(
    contract_path: &str,
    contract_name: &str,
    rpc_url: &str,
    private_key: &str,
    gateway_contracts_root: &std::path::Path,
) -> anyhow::Result<Address> {
    info!("Deploying {} via forge create...", contract_name);
    let contract_spec = format!("{}:{}", contract_path, contract_name);

    let output = Command::new("forge")
        .args([
            "create",
            "--broadcast",
            "--rpc-url",
            rpc_url,
            "--private-key",
            private_key,
            "--root",
            gateway_contracts_root
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid gateway-contracts path"))?,
            &contract_spec,
        ])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute forge: {e}"))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        return Err(anyhow!(
            "Error while deploying {contract_name}.\nstdout: {stdout}\nstderr: {stderr}"
        ));
    }
    parse_deployed_address(&stdout).map_err(|e| {
        anyhow!("Error while deploying {contract_name}: {e}.\nstdout: {stdout}\nstderr: {stderr}")
    })
}

fn parse_deployed_address(output: &str) -> anyhow::Result<Address> {
    for line in output.lines() {
        if let Some(addr_part) = line.strip_prefix("Deployed to: ") {
            let addr_str = addr_part.trim();
            return Address::from_str(addr_str)
                .map_err(|e| anyhow!("Invalid address '{addr_str}': {e}"));
        }
    }
    anyhow::bail!("Could not find 'Deployed to:' in forge output:\n{}", output)
}
