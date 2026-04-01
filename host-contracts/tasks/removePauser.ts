import { task, types } from 'hardhat/config';

import { getPauserSetContract } from './utils/loadVariables';

task('task:removeHostPauser')
  .addParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean
  )
  .addParam('pauserAddress', 'Address of the pauser to remove', undefined, types.string)
  .setAction(async function ({ useInternalProxyAddress, pauserAddress }, hre) {
    const pauserSet = await getPauserSetContract(useInternalProxyAddress, hre);
    await pauserSet.removePauser(pauserAddress);
    console.log('Removed pauser:', pauserAddress);
  });
