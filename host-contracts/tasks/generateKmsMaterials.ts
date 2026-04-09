import { task, types } from 'hardhat/config';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

task('task:triggerKeygen')
  .addParam('paramsType', 'The type of the parameters to use for the key generation.')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used.',
    false,
    types.boolean,
  )
  .setAction(async function ({ paramsType, useInternalProxyAddress }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    console.log('Trigger key generation in KMSGeneration contract.');

    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      loadHostAddresses();
    }

    const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
    const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', proxyAddress, deployer);

    const keygenTx = await kmsGeneration.keygen(paramsType);
    await keygenTx.wait();

    console.log('Keygen triggering done!');
  });

task('task:triggerCrsgen')
  .addParam('maxBitLength', 'The maximum bit length for the CRS generation.')
  .addParam('paramsType', 'The type of the parameters to use for the CRS generation.')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used.',
    false,
    types.boolean,
  )
  .setAction(async function ({ maxBitLength, paramsType, useInternalProxyAddress }, hre) {
    await hre.run('compile:specific', { contract: 'contracts' });
    console.log('Trigger CRS generation in KMSGeneration contract.');

    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new hre.ethers.Wallet(deployerPrivateKey).connect(hre.ethers.provider);

    if (useInternalProxyAddress) {
      loadHostAddresses();
    }

    const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
    const kmsGeneration = await hre.ethers.getContractAt('KMSGeneration', proxyAddress, deployer);

    const crsgenTx = await kmsGeneration.crsgenRequest(maxBitLength, paramsType);
    await crsgenTx.wait();

    console.log('Crsgen triggering done!');
  });
