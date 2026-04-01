import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Swap a pauser in the PauserSet contract
// Note: Internal PauserSet address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the PAUSER_SET_ADDRESS env var, as done in deployment
task('task:swapHostPauser')
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean
  )
  .addParam('oldPauserAddress', 'Address of the pauser to replace', undefined, types.string)
  .addParam('newPauserAddress', 'Address of the new pauser', undefined, types.string)
  .setAction(async function ({ useInternalProxyAddress, oldPauserAddress, newPauserAddress }, hre) {
    await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/contracts/immutable' });
    console.log('Swapping pauser in PauserSet contract');

    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const pauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');

    const pauserSet = await hre.ethers.getContractAt('PauserSet', pauserSetAddress, deployer);
    await pauserSet.swapPauser(oldPauserAddress, newPauserAddress);

    console.log('In PauserSet contract:', pauserSetAddress, '\n');
    console.log('Swapped pauser:', oldPauserAddress, '->', newPauserAddress, '\n');
    console.log('Pauser swap done!');
  });
