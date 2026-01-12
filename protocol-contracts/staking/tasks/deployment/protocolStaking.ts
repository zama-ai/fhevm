import { getRequiredEnvVar } from '../utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

export const PROTOCOL_STAKING_CONTRACT_NAME = 'ProtocolStaking';

// Get the name of the proxy contract to save in the deployments
export function getProtocolStakingProxyName(tokenName: string): string {
  return tokenName + '_Proxy';
}

// Get the name of the implementation contract to save in the deployments
export function getProtocolStakingImplName(tokenName: string): string {
  return tokenName + '_Impl';
}

// Deploy a ProtocolStaking contract
async function deployProtocolStaking(
  tokenName: string,
  symbol: string,
  version: string,
  cooldown: number,
  rewardRate: bigint,
  hre: HardhatRuntimeEnvironment,
) {
  const { getNamedAccounts, ethers, deployments, upgrades, network } = hre;
  const { save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get the env vars shared by both protocol staking contracts
  const zamaTokenAddress = getRequiredEnvVar('ZAMA_TOKEN_ADDRESS');

  // Get the contract factory and deploy the proxy + the implementation
  // At deployment, the governor and manager roles are set to the deployer address to ease initial
  // configuration. They should be transferred to the DAO address soon after.
  const protocolStakingFactory = await ethers.getContractFactory(PROTOCOL_STAKING_CONTRACT_NAME, deployerSigner);
  const proxy = await upgrades.deployProxy(
    protocolStakingFactory,
    [tokenName, symbol, version, zamaTokenAddress, deployer, deployer, cooldown, rewardRate],
    { kind: 'uups', initializer: 'initialize' },
  );
  await proxy.waitForDeployment();

  // Get the proxy address
  const proxyAddress = await proxy.getAddress();
  console.log(
    [
      `✅ Deployed ${tokenName} ProtocolStaking:`,
      `  - Protocol staking proxy address: ${proxyAddress}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  // Save the proxy and implementation contract artifacts
  const artifact = await getArtifact(PROTOCOL_STAKING_CONTRACT_NAME);
  const implAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
  await save(getProtocolStakingProxyName(tokenName), { address: proxyAddress, abi: artifact.abi });
  await save(getProtocolStakingImplName(tokenName), { address: implAddress, abi: artifact.abi });
}

// Deploy the coprocessor ProtocolStaking contracts
// Example usage:
// npx hardhat task:deployProtocolStakingCopro --network testnet
task('task:deployProtocolStakingCopro').setAction(async function (_, hre) {
  console.log('Deploying coprocessor protocol staking contracts...\n');

  // Get the env vars for the coprocessor protocol staking contract
  const coproTokenName = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_NAME');
  const coproTokenSymbol = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_TOKEN_SYMBOL');
  const coproVersion = getRequiredEnvVar('PROTOCOL_STAKING_COPRO_VERSION');
  const coproCooldown = parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_COOLDOWN_PERIOD'));
  const coproRewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_REWARD_RATE')));

  // Deploy the coprocessor protocol staking contract
  await deployProtocolStaking(coproTokenName, coproTokenSymbol, coproVersion, coproCooldown, coproRewardRate, hre);
});

// Deploy the KMS ProtocolStaking contracts
// Example usage:
// npx hardhat task:deployProtocolStakingKMS --network testnet
task('task:deployProtocolStakingKMS').setAction(async function (_, hre) {
  console.log('Deploying KMS protocol staking contracts...');

  // Get the env vars for the KMS protocol staking contract
  const kmsTokenName = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_NAME');
  const kmsTokenSymbol = getRequiredEnvVar('PROTOCOL_STAKING_KMS_TOKEN_SYMBOL');
  const kmsVersion = getRequiredEnvVar('PROTOCOL_STAKING_KMS_VERSION');
  const kmsCooldown = parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_COOLDOWN_PERIOD'));
  const kmsRewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_REWARD_RATE')));

  // Deploy the KMS protocol staking contract
  await deployProtocolStaking(kmsTokenName, kmsTokenSymbol, kmsVersion, kmsCooldown, kmsRewardRate, hre);
});

// Deploy the ProtocolStaking contracts
// Example usage:
// npx hardhat task:deployAllProtocolStakingContracts --network testnet
task('task:deployAllProtocolStakingContracts').setAction(async function (_, hre) {
  console.log('Deploying protocol staking contracts...');

  await hre.run('task:deployProtocolStakingCopro');
  await hre.run('task:deployProtocolStakingKMS');

  console.log('✅ All protocol staking contracts deployed\n');
});
