import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Remove a pauser from the PauserSet contract
// Note: Internal PauserSet address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the PAUSER_SET_ADDRESS env var, as done in deployment
task('task:removeHostPauser')
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean
  )
  .addParam('pauserAddress', 'Address of the pauser to remove', undefined, types.string)
  .setAction(async function ({ useInternalProxyAddress, pauserAddress }, hre) {
    await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/contracts/immutable' });
    console.log('Removing pauser from PauserSet contract');

    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const pauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');

    const pauserSet = await hre.ethers.getContractAt('PauserSet', pauserSetAddress, deployer);
    await pauserSet.removePauser(pauserAddress);

    console.log('In PauserSet contract:', pauserSetAddress, '\n');
    console.log('Removed pauser:', pauserAddress, '\n');
    console.log('Pauser removal done!');
  });
