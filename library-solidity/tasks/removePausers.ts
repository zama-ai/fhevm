import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Remove pausers from the PauserSet contract
// Note: Internal PauserSet address is defined in the `addresses/` directory. It should be used
// for local testing. By default, we use the PAUSER_SET_ADDRESS env var, as done in deployment
task('task:removeHostPausers')
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/contracts/immutable' });
    console.log('Removing pausers from PauserSet contract');

    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const numPausers = parseInt(getRequiredEnvVar('NUM_PAUSERS'));
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    const pausers = [];
    for (let idx = 0; idx < numPausers; idx++) {
      pausers.push(getRequiredEnvVar(`PAUSER_ADDRESS_${idx}`));
    }

    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const pauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');

    const pauserSet = await hre.ethers.getContractAt('PauserSet', pauserSetAddress, deployer);
    for (const pauser of pausers) {
      await pauserSet.removePauser(pauser);
    }

    console.log('In PauserSet contract:', pauserSetAddress, '\n');
    console.log('Removed pausers:', pausers, '\n');
    console.log('Pausers removal done!');
  });
