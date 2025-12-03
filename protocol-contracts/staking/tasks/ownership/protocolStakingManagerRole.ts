import { PROTOCOL_STAKING_CONTRACT_NAME } from '../deployment';
import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Computed as `keccak256("MANAGER_ROLE")` as done in the ProtocolStaking contract
export const MANAGER_ROLE = '0x241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08';

// Grant the protocol staking contract's manager role to the DAO using the deployer account
async function grantManagerRole(protocolStakingProxyAddress: string, hre: HardhatRuntimeEnvironment) {
  const { ethers, network, getNamedAccounts } = hre;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Load the protocol staking contract
  const protocolStaking = await ethers.getContractAt(
    PROTOCOL_STAKING_CONTRACT_NAME,
    protocolStakingProxyAddress,
    deployerSigner,
  );

  // Get the DAO address
  const DAO_ADDRESS = getRequiredEnvVar('DAO_ADDRESS');

  // Grant the manager role to the DAO
  const tx = await protocolStaking.grantRole(MANAGER_ROLE, DAO_ADDRESS);
  await tx.wait();

  console.log(
    [
      `🔑 Granted manager role of ProtocolStaking contract:`,
      `  - Staking proxy address: ${protocolStakingProxyAddress}`,
      `  - New role holder (DAO): ${DAO_ADDRESS}`,
      `  - Granted by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );
}

// Renounce the protocol staking contract's manager role from the deployer account
async function renounceManagerRole(protocolStakingProxyAddress: string, hre: HardhatRuntimeEnvironment) {
  const { ethers, network, getNamedAccounts } = hre;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Load the protocol staking contract
  const protocolStaking = await ethers.getContractAt(
    PROTOCOL_STAKING_CONTRACT_NAME,
    protocolStakingProxyAddress,
    deployerSigner,
  );

  // Renounce the manager role from the deployer
  const tx = await protocolStaking.renounceRole(MANAGER_ROLE, deployerSigner);
  await tx.wait();

  console.log(
    [
      `🔑 Renounced manager role of ProtocolStaking contract:`,
      `  - Protocol staking proxy address: ${protocolStakingProxyAddress}`,
      `  - Renounced by governor (deployer): ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );
}

// Grant all protocol staking contracts' manager roles to the DAO
// Example usage:
// npx hardhat task:grantProtocolStakingManagerRolesToDAO --network testnet
task('task:grantProtocolStakingManagerRolesToDAO').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log("Granting all coprocessor protocol staking contracts' manager roles to the DAO...\n");

  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  await grantManagerRole(protocolStakingCoproProxyAddress, hre);

  console.log("Granting all KMS protocol staking contracts' manager roles to the DAO...\n");

  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  await grantManagerRole(protocolStakingKmsProxyAddress, hre);
});

// Renounce all protocol staking contracts' manager roles from the deployer
// Example usage:
// npx hardhat task:renounceProtocolStakingManagerRolesFromDeployer --network testnet
task('task:renounceProtocolStakingManagerRolesFromDeployer').setAction(async function (
  _,
  hre: HardhatRuntimeEnvironment,
) {
  console.log("Renouncing all coprocessor protocol staking contracts' manager roles from the deployer...\n");

  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  await renounceManagerRole(protocolStakingCoproProxyAddress, hre);

  console.log("Renouncing all KMS protocol staking contracts' manager roles from the deployer...\n");

  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  await renounceManagerRole(protocolStakingKmsProxyAddress, hre);

  console.log("✅ All protocol staking contracts' manager roles have been renounced from the deployer\n");
});
