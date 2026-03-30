import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Swap pausers in the PauserSet contract
// Note: Internal PauserSet address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the PAUSER_SET_ADDRESS env var, as done in deployment
task('task:swapHostPausers')
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await hre.run('compile:specific', { contract: 'contracts/immutable' });
    console.log('Swapping pausers in PauserSet contract');

    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const numPausers = parseInt(getRequiredEnvVar('NUM_PAUSERS'));
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    const pauserSwaps = [];
    for (let idx = 0; idx < numPausers; idx++) {
      pauserSwaps.push({
        oldPauser: getRequiredEnvVar(`OLD_PAUSER_ADDRESS_${idx}`),
        newPauser: getRequiredEnvVar(`NEW_PAUSER_ADDRESS_${idx}`),
      });
    }

    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const pauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');

    const pauserSet = await hre.ethers.getContractAt('PauserSet', pauserSetAddress, deployer);
    for (const { oldPauser, newPauser } of pauserSwaps) {
      await pauserSet.swapPauser(oldPauser, newPauser);
    }

    console.log('In PauserSet contract:', pauserSetAddress, '\n');
    console.log('Swapped pausers:', pauserSwaps, '\n');
    console.log('Pausers swap done!');
  });
