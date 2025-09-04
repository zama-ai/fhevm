import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';

task('task:verifyACL').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvACL = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyACLAddress = parsedEnvACL.ACL_CONTRACT_ADDRESS;
  const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyACLAddress);
  await run('verify:verify', {
    address: implementationACLAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyACLAddress,
    constructorArguments: [],
  });
});

task('task:verifyFHEVMExecutor').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvFHEVMExecutor = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyFHEVMExecutorAddress = parsedEnvFHEVMExecutor.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const implementationFHEVMExecutorAddress = await upgrades.erc1967.getImplementationAddress(proxyFHEVMExecutorAddress);
  await run('verify:verify', {
    address: implementationFHEVMExecutorAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyFHEVMExecutorAddress,
    constructorArguments: [],
  });
});

task('task:verifyKMSVerifier').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvKMSVerifier = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyKMSVerifier = parsedEnvKMSVerifier.KMS_VERIFIER_CONTRACT_ADDRESS;
  const implementationKMSVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyKMSVerifier);
  await run('verify:verify', {
    address: implementationKMSVerifierAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyKMSVerifier,
    constructorArguments: [],
  });
});

task('task:verifyInputVerifier').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvInputVerifier = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyInputVerifier = parsedEnvInputVerifier.INPUT_VERIFIER_CONTRACT_ADDRESS;
  const implementationInputVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyInputVerifier);
  await run('verify:verify', {
    address: implementationInputVerifierAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyInputVerifier,
    constructorArguments: [],
  });
});

task('task:verifyHCULimit').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvHCULimit = dotenv.parse(fs.readFileSync('addresses/.env.host'));
  const proxyHCULimit = parsedEnvHCULimit.HCU_LIMIT_CONTRACT_ADDRESS;
  const implementationHCULimitAddress = await upgrades.erc1967.getImplementationAddress(proxyHCULimit);
  await run('verify:verify', {
    address: implementationHCULimitAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyHCULimit,
    constructorArguments: [],
  });
});

task('task:verifyDecryptionOracle').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvDecryptionOracle = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
  const proxyDecryptionOracle = parsedEnvDecryptionOracle.DECRYPTION_ORACLE_ADDRESS;
  const implementationDecryptionOracleAddress = await upgrades.erc1967.getImplementationAddress(proxyDecryptionOracle);
  await run('verify:verify', {
    address: implementationDecryptionOracleAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyDecryptionOracle,
    constructorArguments: [],
  });
});

task('task:verifyAllHostContracts')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, hre) {
    console.log('Verify ACL contract:');
    await hre.run('task:verifyACL', { useInternalProxyAddress });

    console.log('Verify FHEVMExecutor contract:');
    await hre.run('task:verifyFHEVMExecutor', { useInternalProxyAddress });

    console.log('Verify KMSVerifier contract:');
    await hre.run('task:verifyKMSVerifier', { useInternalProxyAddress });

    console.log('Verify InputVerifier contract:');
    await hre.run('task:verifyInputVerifier', { useInternalProxyAddress });

    console.log('Verify HCULimit contract:');
    await hre.run('task:verifyHCULimit', { useInternalProxyAddress });

    console.log('Verify DecryptionOracle contract:');
    await hre.run('task:verifyDecryptionOracle', { useInternalProxyAddress });

    console.log('Contract verification done!');
  });
