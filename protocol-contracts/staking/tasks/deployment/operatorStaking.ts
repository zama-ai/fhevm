import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

export const OPERATOR_STAKING_CONTRACT_NAME = 'OperatorStaking';
export const OPERATOR_REWARDER_CONTRACT_NAME = 'OperatorRewarder';

// Get the name of the operator staking contract to save in the deployments
export function getOperatorStakingName(tokenName: string): string {
  return tokenName + '_Staking';
}

// Get the name of the operator rewarder contract to save in the deployments
export function getOperatorRewarderName(tokenName: string): string {
  return tokenName + '_Rewarder';
}

// Deploy an OperatorStaking contract
async function deployOperatorStaking(
  tokenName: string,
  symbol: string,
  protocolStakingAddress: string,
  ownerAddress: string,
  hre: HardhatRuntimeEnvironment,
) {
  const { getNamedAccounts, ethers, deployments, network } = hre;
  const { log, save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get the contract factory and deploy the operator staking and rewarder contracts
  const operatorStakingFactory = await ethers.getContractFactory(OPERATOR_STAKING_CONTRACT_NAME, deployerSigner);
  const operatorStaking = await operatorStakingFactory.deploy(tokenName, symbol, protocolStakingAddress, ownerAddress);
  await operatorStaking.waitForDeployment();

  // Get the operator staking and rewarder addresses
  const operatorStakingAddress = await operatorStaking.getAddress();
  const operatorRewarderAddress = await operatorStaking.rewarder();

  log(`${tokenName} operator staking deployed at address ${operatorStakingAddress} on network ${network.name}`);
  log(`${tokenName} operator rewarder deployed at address ${operatorRewarderAddress} on network ${network.name}`);

  // Save the OperatorStaking and OperatorRewarder contract artifacts
  const operatorStakingArtifact = await getArtifact(OPERATOR_STAKING_CONTRACT_NAME);
  await save(getOperatorStakingName(tokenName), { address: operatorStakingAddress, abi: operatorStakingArtifact.abi });
  const operatorRewarderArtifact = await getArtifact(OPERATOR_REWARDER_CONTRACT_NAME);
  await save(getOperatorRewarderName(tokenName), {
    address: operatorRewarderAddress,
    abi: operatorRewarderArtifact.abi,
  });
}

// Deploy the coprocessor OperatorStaking contracts
// Example usage:
// npx hardhat task:deployAllOperatorStakingCoproContracts --network ethereum-testnet
task('task:deployAllOperatorStakingCoproContracts').setAction(async function (_, hre) {
  const { log } = hre.deployments;

  // Load the coprocessor protocol staking contract address
  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  // Get the number of operator staking contracts for coprocessors to deploy
  const numOperatorStakingCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));

  log('Deploying coprocessor operator staking contracts...');

  for (let i = 0; i < numOperatorStakingCopro; i++) {
    // Get the env vars for the coprocessor operator staking contract
    const coproTokenName = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_NAME_${i}`);
    const coproTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_SYMBOL_${i}`);
    const coproOwnerAddress = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_OWNER_ADDRESS_${i}`);
    await deployOperatorStaking(
      coproTokenName,
      coproTokenSymbol,
      protocolStakingCoproProxyAddress,
      coproOwnerAddress,
      hre,
    );
  }

  log('All coprocessor operator staking contracts deployed');
});

// Deploy the KMS OperatorStaking contracts
// Example usage:
// npx hardhat task:deployAllOperatorStakingKMSContracts --network ethereum-testnet
task('task:deployAllOperatorStakingKMSContracts').setAction(async function (_, hre) {
  const { log } = hre.deployments;

  // Load the KMS protocol staking contract address
  const protocolStakingKMSProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  // Get the number of operator staking contracts for KMS to deploy
  const numOperatorStakingKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));

  log('Deploying KMS operator staking contracts...');

  for (let i = 0; i < numOperatorStakingKms; i++) {
    // Get the env vars for the KMS operator staking contract
    const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_${i}`);
    const kmsTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_SYMBOL_${i}`);
    const kmsOwnerAddress = getRequiredEnvVar(`OPERATOR_STAKING_KMS_OWNER_ADDRESS_${i}`);
    await deployOperatorStaking(kmsTokenName, kmsTokenSymbol, protocolStakingKMSProxyAddress, kmsOwnerAddress, hre);
  }

  log('All KMS operator staking contracts deployed');
});

// Deploy the OperatorStaking contracts
// Example usage:
// npx hardhat task:deployAllOperatorStakingContracts --network ethereum-testnet
task('task:deployAllOperatorStakingContracts').setAction(async function (_, hre) {
  const { log } = hre.deployments;

  log('Deploying operator staking contracts...');

  hre.run('task:deployAllOperatorStakingCoproContracts');
  hre.run('task:deployAllOperatorStakingKMSContracts');

  log('All operator staking contracts deployed');
});
