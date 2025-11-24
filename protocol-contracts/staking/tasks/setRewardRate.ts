import { PROTOCOL_STAKING_CONTRACT_NAME } from './deployment';
import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from './utils/getAddresses';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Set the reward rate for the protocol staking contract
// Note: The reward rate is in tokens (using 18 decimals) per second
task('task:setRewardRate')
  .addParam(
    'rewardRate',
    'The reward rate to set for the protocol staking contract (in tokens per second)',
    0n,
    types.bigint,
  )
  .addParam(
    'protocolStakingProxyAddress',
    'The address of the protocol staking contract to set the reward rate for',
    '',
    types.string,
  )
  .setAction(async function ({ rewardRate, protocolStakingProxyAddress }, hre: HardhatRuntimeEnvironment) {
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

    // Set the reward rate
    await protocolStaking.setRewardRate(rewardRate);

    log(`Reward rate set to ${rewardRate} for protocol staking contract at 
        address ${protocolStakingProxyAddress} on network ${network.name}`);
  });

// Set the reward rate for the coprocessor protocol staking contract
// Note: The reward rate is in tokens (using 18 decimals) per second
// Example usage:
// npx hardhat task:setCoprocessorRewardRate --rewardRate 1000000000000 --network ethereum-testnet
task('task:setCoprocessorRewardRate')
  .addParam(
    'rewardRate',
    'The reward rate to set for the coprocessor protocol staking contract (in tokens per second)',
    0n,
    types.bigint,
  )
  .setAction(async function ({ rewardRate }, hre: HardhatRuntimeEnvironment) {
    const { log } = hre.deployments;

    log('Setting reward rate for coprocessor protocol staking contract...');

    const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);
    await hre.run('task:setRewardRate', { rewardRate, protocolStakingProxyAddress: protocolStakingCoproProxyAddress });
  });

// Set the reward rate for the KMS protocol staking contract
// Example usage:
// npx hardhat task:setKMSRewardRate --network ethereum-testnet
task('task:setKMSRewardRate').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Setting reward rate for KMS protocol staking contract...');

  // Get the reward rate for the KMS protocol staking contract
  const rewardRate = getRequiredEnvVar('PROTOCOL_STAKING_REWARD_RATE');

  // Get the address of the KMS protocol staking contract
  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  await hre.run('task:setRewardRate', { rewardRate, protocolStakingProxyAddress: protocolStakingKmsProxyAddress });
});

task('task:setAllRewardRates').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Setting reward rate for all protocol staking contracts...');

  await hre.run('task:setCoprocessorRewardRate');
  await hre.run('task:setKMSRewardRate');
});
