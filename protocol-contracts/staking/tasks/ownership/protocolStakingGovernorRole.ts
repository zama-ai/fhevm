import { PROTOCOL_STAKING_CONTRACT_NAME } from '../deployment';
import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Begin a transfer of a protocol staking contract's governor role from the deployer to the DAO
async function transferGovernorRole(protocolStakingProxyAddress: string, hre: HardhatRuntimeEnvironment) {
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

  // Begin the transfer of the governor role to the DAO
  await protocolStaking.beginDefaultAdminTransfer(DAO_ADDRESS);

  log(`Governor role of protocol staking contract at address ${protocolStakingProxyAddress} 
      being transferred to DAO address ${DAO_ADDRESS} on network ${network.name}`);
}

// Begin a transfer of all protocol staking contracts' governor roles from the deployer to the DAO
// This is te first step of a 2-step process to transfer the governor role to the DAO
// The DAO then needs to accept the transfer by calling `acceptDefaultAdminTransfer()` function on
// both protocol staking contracts
// Example usage:
// npx hardhat task:beginTransferProtocolStakingGovernorRolesToDAO --network testnet
task('task:beginTransferProtocolStakingGovernorRolesToDAO').setAction(async function (
  _,
  hre: HardhatRuntimeEnvironment,
) {
  const { log } = hre.deployments;

  log("Begin a transfer of all coprocessor protocol staking contracts' governor roles to the DAO...");

  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  transferGovernorRole(protocolStakingCoproProxyAddress, hre);

  log("Begin a transfer of all KMS protocol staking contracts' governor roles to the DAO...");

  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  transferGovernorRole(protocolStakingKmsProxyAddress, hre);
});
