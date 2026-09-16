import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

export const LUGANODES_OPERATOR_STAKING_V2_CONTRACT = 'OperatorStakingRenameTokenV2';

// Get the name for saving the implementation in deployments
export function getLuganodesOperatorStakingV2ImplName(networkName: string): string {
  const contractName = LUGANODES_OPERATOR_STAKING_V2_CONTRACT;
  return contractName + '_Impl';
}

// Deploy the LuganodesOperatorStakingV2 implementation contract (no proxy)
// This is used for DAO proposals to upgrade the existing proxy
async function deployLuganodesOperatorStakingV2Impl(hre: HardhatRuntimeEnvironment) {
  const { getNamedAccounts, ethers, deployments, network } = hre;
  const { save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get contract name
  const contractName = LUGANODES_OPERATOR_STAKING_V2_CONTRACT;

  // Get the contract factory and deploy the implementation only (no proxy)
  const factory = await ethers.getContractFactory(contractName, deployerSigner);
  const implementation = await factory.deploy();
  await implementation.waitForDeployment();

  const implementationAddress = await implementation.getAddress();

  console.log(
    [
      `✅ Deployed ${contractName} implementation:`,
      `  - Implementation address: ${implementationAddress}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  // Save the implementation contract artifact
  const artifact = await getArtifact(contractName);
  await save(getLuganodesOperatorStakingV2ImplName(network.name), {
    address: implementationAddress,
    abi: artifact.abi,
  });

  return implementationAddress;
}

// Deploy the LuganodesOperatorStakingV2 implementation contract
// After deploying the implementation, the owner should call `upagradeToAndCall(address,bytes)` function on the Proxy,
// with the address of the implementation as first argument,
// and second argument the calldata returned by, for e.g: `cast calldata "reinitializeV2(string,string)" "Mock Luganodes Staked ZAMA (Coprocessor)" "stZAMA-Mock-Luganodes-Coprocessor"`
// Example usage:
// npx hardhat task:deployLuganodesOperatorStakingV2Impl --network testnet
task('task:deployLuganodesOperatorStakingV2Impl').setAction(async function (_, hre) {
  console.log('Deploying LuganodesOperatorStakingV2 implementation...\n');
  await deployLuganodesOperatorStakingV2Impl(hre);
});

// Verify the LuganodesOperatorStakingV2 implementation contract
// Example usage:
// npx hardhat task:verifyLuganodesOperatorStakingV2Impl --impl-address 0x... --network testnet
task('task:verifyLuganodesOperatorStakingV2Impl')
  .addParam('implAddress', 'The address of the implementation contract to verify', '', types.string)
  .setAction(async function ({ implAddress }, hre) {
    const { run } = hre;

    console.log(`Verifying LuganodesOperatorStakingV2 implementation at ${implAddress}...\n`);
    await run('verify:verify', {
      address: implAddress,
      constructorArguments: [],
    });
  });
