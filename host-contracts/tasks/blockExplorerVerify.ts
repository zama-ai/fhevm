import dotenv from 'dotenv';
import { task, types } from 'hardhat/config';

import { getRequiredEnvVar } from './utils/loadVariables';

task('task:verifyACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: 'addresses/.env.host' });
    }
    const proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyFHEVMExecutor')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: 'addresses/.env.host' });
    }
    const proxyAddress = getRequiredEnvVar('FHEVM_EXECUTOR_CONTRACT_ADDRESS');
    const implementationFHEVMExecutorAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationFHEVMExecutorAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyKMSVerifier')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: 'addresses/.env.host' });
    }
    const proxyAddress = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');
    const implementationKMSVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationKMSVerifierAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyInputVerifier')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: 'addresses/.env.host' });
    }
    const proxyAddress = getRequiredEnvVar('INPUT_VERIFIER_CONTRACT_ADDRESS');
    const implementationInputVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationInputVerifierAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyHCULimit')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: 'addresses/.env.host' });
    }
    const proxyAddress = getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS');
    const implementationHCULimitAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationHCULimitAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });

task('task:verifyDecryptionOracle')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      dotenv.config({ path: 'addresses/.env.host' });
    }
    const proxyAddress = getRequiredEnvVar('DECRYPTION_ORACLE_ADDRESS');
    const implementationDecryptionOracleAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationDecryptionOracleAddress,
      constructorArguments: [],
    });
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });
  });
