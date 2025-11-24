import { getProtocolStakingCoproProxyAddress, getProtocolStakingKMSProxyAddress } from '../utils/getAddresses';
import { task } from 'hardhat/config';

// Verify the coprocessor protocol staking contract
// Example usage:
// npx hardhat task:verifyProtocolStakingCopro --network ethereum-testnet
task('task:verifyProtocolStakingCopro').setAction(async function (_, hre) {
  const { upgrades, run } = hre;
  const proxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

  await run('verify:verify', {
    address: proxyAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: implementationAddress,
    constructorArguments: [],
  });
});

// Verify the KMS protocol staking contract
// Example usage:
// npx hardhat task:verifyProtocolStakingKMS --network ethereum-testnet
task('task:verifyProtocolStakingKMS').setAction(async function (_, hre) {
  const { upgrades, run } = hre;
  const proxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

  await run('verify:verify', {
    address: proxyAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: implementationAddress,
    constructorArguments: [],
  });
});

// Verify all protocol staking contracts
// Example usage:
// npx hardhat task:verifyAllProtocolStakingContracts --network ethereum-testnet
task('task:verifyAllProtocolStakingContracts').setAction(async function (_, hre) {
  console.log('Verify Protocol Staking Copro contract:');

  await hre.run('task:verifyProtocolStakingCopro');
  await hre.run('task:verifyProtocolStakingKMS');

  console.log('Protocol staking contracts verification done!');
});
