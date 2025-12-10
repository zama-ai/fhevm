import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task, types } from 'hardhat/config';
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
  beneficiaryAddress: string,
  initialMaxFeeBasisPoints: number,
  initialFeeBasisPoints: number,
  hre: HardhatRuntimeEnvironment,
) {
  const { getNamedAccounts, ethers, deployments, network } = hre;
  const { save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get the contract factory and deploy the operator staking and rewarder contracts
  const operatorStakingFactory = await ethers.getContractFactory(OPERATOR_STAKING_CONTRACT_NAME, deployerSigner);
  const operatorStaking = await operatorStakingFactory.deploy(
    tokenName,
    symbol,
    protocolStakingAddress,
    beneficiaryAddress,
    initialMaxFeeBasisPoints,
    initialFeeBasisPoints,
  );
  await operatorStaking.waitForDeployment();

  // Get the operator staking and rewarder addresses
  const operatorStakingAddress = await operatorStaking.getAddress();
  const operatorRewarderAddress = await operatorStaking.rewarder();

  console.log(
    [
      `✅ Deployed ${tokenName} OperatorStaking:`,
      `  - Operator staking address:  ${operatorStakingAddress}`,
      `  - Operator rewarder address: ${operatorRewarderAddress}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  // Save the OperatorStaking and OperatorRewarder contract artifacts
  const operatorStakingArtifact = await getArtifact(OPERATOR_STAKING_CONTRACT_NAME);
  await save(getOperatorStakingName(tokenName), { address: operatorStakingAddress, abi: operatorStakingArtifact.abi });
  const operatorRewarderArtifact = await getArtifact(OPERATOR_REWARDER_CONTRACT_NAME);
  await save(getOperatorRewarderName(tokenName), {
    address: operatorRewarderAddress,
    abi: operatorRewarderArtifact.abi,
  });
}

// Deploy a coprocessor OperatorStaking contracts
// Example usage:
// npx hardhat task:deployOperatorStakingCopro --index 0 --network testnet
task('task:deployOperatorStakingCopro')
  .addParam('index', 'The index of the coprocessor operator staking contract to deploy', 0, types.int)
  .setAction(async function ({ index }, hre) {
    // Get the number of operator staking contracts for coprocessors and check if the index is in bounds
    const numOperatorStakingCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));

    if (index >= numOperatorStakingCopro || index < 0) {
      throw new Error(
        `Index ${index} is out of bounds for the number of coprocessor operator staking contracts: ${numOperatorStakingCopro}`,
      );
    }

    // Get the coprocessor protocol staking proxy address
    const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

    // Get the env vars for the coprocessor operator staking contract
    const coproTokenName = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_NAME_${index}`);
    const coproTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_COPRO_TOKEN_SYMBOL_${index}`);

    // Get the env vars for the coprocessor operator rewarder contract (deployed at the same time)
    const coproBeneficiaryAddress = getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_BENEFICIARY_${index}`);
    const coproInitialMaxFeeBasisPoints = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_MAX_FEE_${index}`));
    const coproInitialFeeBasisPoints = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_FEE_${index}`));

    await deployOperatorStaking(
      coproTokenName,
      coproTokenSymbol,
      protocolStakingCoproProxyAddress,
      coproBeneficiaryAddress,
      coproInitialMaxFeeBasisPoints,
      coproInitialFeeBasisPoints,
      hre,
    );
  });

// Deploy a KMS OperatorStaking contracts
// Example usage:
// npx hardhat task:deployOperatorStakingKMS --index 0 --network testnet
task('task:deployOperatorStakingKMS')
  .addParam('index', 'The index of the KMS operator staking contract to deploy', 0, types.int)
  .setAction(async function ({ index }, hre) {
    // Get the number of operator staking contracts for KMS and check if the index is in bounds
    const numOperatorStakingKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));

    if (index >= numOperatorStakingKms || index < 0) {
      throw new Error(
        `Index ${index} is out of bounds for the number of KMS operator staking contracts: ${numOperatorStakingKms}`,
      );
    }

    // Get the KMS protocol staking proxy address
    const protocolStakingKMSProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

    // Get the env vars for the KMS operator staking contract
    const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_${index}`);
    const kmsTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_SYMBOL_${index}`);

    // Get the env vars for the KMS operator rewarder contract (deployed at the same time)
    const kmsBeneficiaryAddress = getRequiredEnvVar(`OPERATOR_REWARDER_KMS_BENEFICIARY_${index}`);
    const kmsInitialMaxFeeBasisPoints = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_MAX_FEE_${index}`));
    const kmsInitialFeeBasisPoints = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_FEE_${index}`));

    await deployOperatorStaking(
      kmsTokenName,
      kmsTokenSymbol,
      protocolStakingKMSProxyAddress,
      kmsBeneficiaryAddress,
      kmsInitialMaxFeeBasisPoints,
      kmsInitialFeeBasisPoints,
      hre,
    );
  });

// Deploy the coprocessor OperatorStaking contracts
// Example usage:
// npx hardhat task:deployAllOperatorStakingCoproContracts --network testnet
task('task:deployAllOperatorStakingCoproContracts').setAction(async function (_, hre) {
  console.log('Deploying coprocessor operator staking contracts...\n');

  // Get the number of operator staking contracts for coprocessors to deploy
  const numOperatorStakingCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));

  for (let i = 0; i < numOperatorStakingCopro; i++) {
    await hre.run('task:deployOperatorStakingCopro', { index: i });
  }

  console.log('All coprocessor operator staking contracts deployed');
});

// Deploy the KMS OperatorStaking contracts
// Example usage:
// npx hardhat task:deployAllOperatorStakingKMSContracts --network testnet
task('task:deployAllOperatorStakingKMSContracts').setAction(async function (_, hre) {
  console.log('Deploying KMS operator staking contracts...');

  // Get the number of operator staking contracts for KMS to deploy
  const numOperatorStakingKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));

  for (let i = 0; i < numOperatorStakingKms; i++) {
    await hre.run('task:deployOperatorStakingKMS', { index: i });
  }

  console.log('All KMS operator staking contracts deployed');
});

// Deploy the OperatorStaking contracts
// Example usage:
// npx hardhat task:deployAllOperatorStakingContracts --network testnet
task('task:deployAllOperatorStakingContracts').setAction(async function (_, hre) {
  console.log('Deploying operator staking contracts...');

  await hre.run('task:deployAllOperatorStakingCoproContracts');
  await hre.run('task:deployAllOperatorStakingKMSContracts');

  console.log('✅ All operator staking contracts deployed\n');
});
