use crate::{
    config::KmsWallet,
    conn::WalletProvider,
    provider::{FillersWithoutNonceManagement, NonceManagedProvider},
};
use alloy::{
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, ChainId},
    providers::ProviderBuilder,
    transports::http::reqwest::Url,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    gateway_config::GatewayConfig::{self, GatewayConfigInstance},
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::{self, KMSGenerationInstance},
    protocol_config::ProtocolConfig::ProtocolConfigInstance,
};
use std::{
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    sync::LazyLock,
    time::Duration,
};
use tracing::info;

pub const TEST_MNEMONIC: &str =
    "coyote sketch defense hover finger envelope celery urge panther venue verb cheese";

pub static CHAIN_ID: LazyLock<u32> = LazyLock::new(rand::random::<u32>);

pub const DEPLOYER_PRIVATE_KEY: &str =
    "0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c";

pub struct BlockchainInstance {
    pub provider: WalletProvider,
    pub decryption_contract: DecryptionInstance<WalletProvider>,
    pub gateway_config_contract: GatewayConfigInstance<WalletProvider>,
    pub kms_generation_contract: KMSGenerationInstance<WalletProvider>,
    pub protocol_config_contract: ProtocolConfigInstance<WalletProvider>,
    pub anvil: AnvilInstance,
    pub block_time: u64,
}

impl BlockchainInstance {
    pub fn new(
        anvil: AnvilInstance,
        provider: WalletProvider,
        decryption_address: Address,
        gateway_config_address: Address,
        kms_generation_address: Address,
        protocol_config_address: Address,
        block_time: u64,
    ) -> Self {
        let decryption_contract = Decryption::new(decryption_address, provider.clone());
        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider.clone());
        let kms_generation_contract = KMSGeneration::new(kms_generation_address, provider.clone());
        let protocol_config_contract =
            ProtocolConfigInstance::new(protocol_config_address, provider.clone());

        BlockchainInstance {
            provider,
            decryption_contract,
            gateway_config_contract,
            kms_generation_contract,
            protocol_config_contract,
            anvil,
            block_time,
        }
    }

    pub async fn setup() -> anyhow::Result<Self> {
        let block_time = 1;

        let anvil = setup_anvil(block_time)?;
        let anvil_endpoint = anvil.endpoint_url();

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
            .connect_http(anvil_endpoint.clone());
        let provider = NonceManagedProvider::new(inner_provider, wallet_addr);

        // Deploy mock contracts via forge create
        info!("Deploying Gateway mock contracts via forge...");

        let gateway_contracts_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../gateway-contracts")
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Could not find gateway-contracts directory: {e}"))?;

        let kms_connector_tests_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests")
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Could not find kms-connector tests: {e}"))?;

        let rpc_url = anvil_endpoint.to_string();
        let private_key = DEPLOYER_PRIVATE_KEY.to_string();

        let (decryption_addr, gateway_config_addr, kms_generation_addr, protocol_config_addr) =
            tokio::task::spawn_blocking(move || {
                let decryption = deploy_contract(
                    "contracts/DecryptionMock.sol",
                    "DecryptionMock",
                    &rpc_url,
                    &private_key,
                    &kms_connector_tests_path,
                )?;

                let gateway_config = deploy_contract(
                    "contracts/mocks/GatewayConfigMock.sol",
                    "GatewayConfigMock",
                    &rpc_url,
                    &private_key,
                    &gateway_contracts_path,
                )?;

                let kms_generation = deploy_contract(
                    "contracts/KMSGenerationMock.sol",
                    "KMSGenerationMock",
                    &rpc_url,
                    &private_key,
                    &kms_connector_tests_path,
                )?;

                let protocol_config = deploy_contract(
                    "contracts/ProtocolConfigMock.sol",
                    "ProtocolConfigMock",
                    &rpc_url,
                    &private_key,
                    &kms_connector_tests_path,
                )?;

                Ok::<_, anyhow::Error>((
                    decryption,
                    gateway_config,
                    kms_generation,
                    protocol_config,
                ))
            })
            .await??;

        info!("DecryptionMock deployed at: {}", decryption_addr);
        info!("GatewayConfigMock deployed at: {}", gateway_config_addr);
        info!("KMSGenerationMock deployed at: {}", kms_generation_addr);
        info!("ProtocolConfigMock deployed at: {}", protocol_config_addr);

        Ok(BlockchainInstance::new(
            anvil,
            provider,
            decryption_addr,
            gateway_config_addr,
            kms_generation_addr,
            protocol_config_addr,
            block_time,
        ))
    }

    pub fn anvil_block_time(&self) -> Duration {
        Duration::from_secs(self.block_time)
    }

    pub fn anvil_http_endpoint(&self) -> Url {
        self.anvil.endpoint_url()
    }

    /// Freezes the Anvil process via `SIGSTOP`.
    pub fn pause_anvil(&self) -> anyhow::Result<()> {
        signal_anvil(&self.anvil, "-STOP")
    }

    /// Resumes a previously paused Anvil process via `SIGCONT`.
    pub fn unpause_anvil(&self) -> anyhow::Result<()> {
        signal_anvil(&self.anvil, "-CONT")
    }
}

fn setup_anvil(block_time: u64) -> anyhow::Result<AnvilInstance> {
    info!("Starting Anvil...");
    // The port is left unset so Anvil binds a random free port, avoiding collisions between
    // concurrently running tests.
    let anvil = Anvil::new()
        .chain_id(*CHAIN_ID as u64)
        .mnemonic(TEST_MNEMONIC)
        .block_time(block_time)
        // Reduce number of slots in an epoch to consider transaction as finalized ASAP.
        // A tx is generally considered finalized after two epochs.
        .args(["--slots-in-an-epoch", "1"])
        .try_spawn()?;

    Ok(anvil)
}

fn signal_anvil(anvil: &AnvilInstance, signal_flag: &str) -> anyhow::Result<()> {
    let pid = anvil.child().id();
    let kill_status = Command::new("kill")
        .args([signal_flag, &pid.to_string()])
        .status()
        .map_err(|e| anyhow!("Failed to run `kill {signal_flag}` on Anvil (pid {pid}): {e}"))?;
    if !kill_status.success() {
        return Err(anyhow!(
            "`kill {signal_flag}` on Anvil (pid {pid}) failed with status: {kill_status}"
        ));
    }
    Ok(())
}

fn deploy_contract(
    contract_path: &str,
    contract_name: &str,
    rpc_url: &str,
    private_key: &str,
    contracts_root: &Path,
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
            contracts_root
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid contracts root path"))?,
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
