import { getAllOperatorStakingAddresses, getAllOperatorRewarderAddresses } from '../utils/getAddresses';
import { task } from 'hardhat/config';

// Verify all operator staking contracts
// Example usage:
// npx hardhat task:verifyAllOperatorStakingContracts --network ethereum-testnet
task('task:verifyAllOperatorStakingContracts').setAction(async function (_, hre) {
  const { run } = hre;

  const operatorStakingAddresses = await getAllOperatorStakingAddresses(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await run('verify:verify', {
      address: operatorStakingAddress,
      constructorArguments: [],
    });
  }
});

// Verify all operator rewarder contracts
// Example usage:
// npx hardhat task:verifyAllOperatorRewarderContracts --network ethereum-testnet
task('task:verifyAllOperatorRewarderContracts').setAction(async function (_, hre) {
  const { run } = hre;

  const operatorRewarderAddresses = await getAllOperatorRewarderAddresses(hre);

  for (let i = 0; i < operatorRewarderAddresses.length; i++) {
    const operatorRewarderAddress = operatorRewarderAddresses[i];
    await run('verify:verify', {
      address: operatorRewarderAddress,
      constructorArguments: [],
    });
  }
});
