import { PROTOCOL_STAKING_CONTRACT_NAME } from '../deployment';
import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Computed as `keccak256("MANAGER_ROLE")` as done in the ProtocolStaking contract
const MANAGER_ROLE = '0x241ecf16d79d0f8dbfb92cbc07fe17840425976cf0667f022fe9877caa831b08';

// Grant the protocol staking contract's manager role to the DAO using the deployer account
async function grantManagerRole(protocolStakingProxyAddress: string, hre: HardhatRuntimeEnvironment) {
  const { ethers, deployments, network, getNamedAccounts } = hre;
  const { log } = deployments;

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
  await protocolStaking.grantRole(MANAGER_ROLE, DAO_ADDRESS);

  log(`Manager role of protocol staking contract at address ${protocolStakingProxyAddress} 
      granted to DAO address ${DAO_ADDRESS} on network ${network.name}`);
}

// Renounce the protocol staking contract's manager role from the deployer account
async function renounceManagerRole(protocolStakingProxyAddress: string, hre: HardhatRuntimeEnvironment) {
  const { ethers, deployments, network, getNamedAccounts } = hre;
  const { log } = deployments;

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
  await protocolStaking.grantRole(MANAGER_ROLE, deployerSigner);

  log(`Manager role of protocol staking contract at address ${protocolStakingProxyAddress} 
      renounced from deployer address ${deployer} on network ${network.name}`);
}

// Grant all protocol staking contracts' manager roles to the DAO
// Example usage:
// npx hardhat task:grantProtocolStakingManagerRolesToDAO --network ethereum-testnet
task('task:grantProtocolStakingManagerRolesToDAO').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log("Granting all coprocessor protocol staking contracts' manager roles to the DAO...");

  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  grantManagerRole(protocolStakingCoproProxyAddress, hre);

  log("Granting all KMS protocol staking contracts' manager roles to the DAO...");

  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  grantManagerRole(protocolStakingKmsProxyAddress, hre);
});

// Renounce all protocol staking contracts' manager roles from the deployer
// Example usage:
// npx hardhat task:renounceProtocolStakingManagerRolesFromDeployer --network ethereum-testnet
task('task:renounceProtocolStakingManagerRolesFromDeployer').setAction(async function (
  _,
  hre: HardhatRuntimeEnvironment,
) {
  const { log } = hre.deployments;

  log("Renouncing all coprocessor protocol staking contracts' manager roles from the deployer...");

  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);
  await renounceManagerRole(protocolStakingCoproProxyAddress, hre);

  log("Renouncing all KMS protocol staking contracts' manager roles from the deployer...");

  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  await renounceManagerRole(protocolStakingKmsProxyAddress, hre);
});
