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

    // Set the reward rate
    const tx = await protocolStaking.setRewardRate(rewardRate);
    await tx.wait();

    console.log(
      [
        `👉 Set reward rate:`,
        `  - Reward rate in tokens per second: ${rewardRate}`,
        `  - Protocol staking proxy address: ${protocolStakingProxyAddress}`,
        `  - Initiated by manager (deployer): ${deployer}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });

// Set the reward rate for the coprocessor protocol staking contract
// Note: The reward rate is in tokens (using 18 decimals) per second
// Example usage:
// npx hardhat task:setCoprocessorRewardRate --rewardRate 1000000000000 --network testnet
task('task:setCoprocessorRewardRate').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting reward rate for coprocessor protocol staking contract...\n');

  // Get the reward rate for the coprocessor protocol staking contract
  const rewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_COPRO_REWARD_RATE')));

  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);
  await hre.run('task:setRewardRate', { rewardRate, protocolStakingProxyAddress: protocolStakingCoproProxyAddress });
});

// Set the reward rate for the KMS protocol staking contract
// Example usage:
// npx hardhat task:setKMSRewardRate --network testnet
task('task:setKMSRewardRate').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting reward rate for KMS protocol staking contract...\n');

  // Get the reward rate for the KMS protocol staking contract
  const rewardRate = BigInt(parseInt(getRequiredEnvVar('PROTOCOL_STAKING_KMS_REWARD_RATE')));

  // Get the address of the KMS protocol staking contract
  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  await hre.run('task:setRewardRate', { rewardRate, protocolStakingProxyAddress: protocolStakingKmsProxyAddress });
});

// Set the reward rate for all protocol staking contracts
// Example usage:
// npx hardhat task:setAllRewardRates --network testnet
task('task:setAllRewardRates').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting reward rate for all protocol staking contracts...\n');

  await hre.run('task:setCoprocessorRewardRate');
  await hre.run('task:setKMSRewardRate');

  console.log('✅ Reward rates for all protocol staking contracts have been set\n');
});
