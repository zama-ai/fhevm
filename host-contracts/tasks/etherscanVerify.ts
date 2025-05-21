import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';

task('task:verifyACL').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvACL = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
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
  const parsedEnvFHEVMExecutor = dotenv.parse(fs.readFileSync('addresses/.env.exec'));
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
  const parsedEnvKMSVerifier = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier'));
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
  const parsedEnvInputVerifier = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier'));
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

task('task:verifyFHEGasLimit').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvFHEGasLimit = dotenv.parse(fs.readFileSync('addresses/.env.fhegaslimit'));
  const proxyFHEGasLimit = parsedEnvFHEGasLimit.FHE_GASLIMIT_CONTRACT_ADDRESS;
  const implementationFHEGasLimitAddress = await upgrades.erc1967.getImplementationAddress(proxyFHEGasLimit);
  await run('verify:verify', {
    address: implementationFHEGasLimitAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyFHEGasLimit,
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
