import dotenv from 'dotenv';
import fs from 'fs';
import { task } from 'hardhat/config';

task('task:verifyContracts').setAction(async function (taskArguments, { upgrades, run }) {
  const parsedEnvACL = dotenv.parse(fs.readFileSync('lib/.env.acl'));
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

  const parsedEnvTFHEExecutor = dotenv.parse(fs.readFileSync('lib/.env.exec'));
  const proxyTFHEExecutorAddress = parsedEnvTFHEExecutor.TFHE_EXECUTOR_CONTRACT_ADDRESS;
  const implementationTFHEExecutorAddress = await upgrades.erc1967.getImplementationAddress(proxyTFHEExecutorAddress);
  await run('verify:verify', {
    address: implementationTFHEExecutorAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyTFHEExecutorAddress,
    constructorArguments: [],
  });

  const parsedEnvKMSVerifier = dotenv.parse(fs.readFileSync('lib/.env.kmsverifier'));
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

  const parsedEnvInputVerifier = dotenv.parse(fs.readFileSync('lib/.env.inputverifier'));
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

  const parsedEnvFHEPayment = dotenv.parse(fs.readFileSync('lib/.env.fhepayment'));
  const proxyFHEPayment = parsedEnvFHEPayment.FHE_PAYMENT_CONTRACT_ADDRESS;
  const implementationFHEPaymentAddress = await upgrades.erc1967.getImplementationAddress(proxyFHEPayment);
  await run('verify:verify', {
    address: implementationFHEPaymentAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyFHEPayment,
    constructorArguments: [],
  });

  const parsedEnvGateway = dotenv.parse(fs.readFileSync('gateway/.env.gateway'));
  const proxyGateway = parsedEnvGateway.GATEWAY_CONTRACT_PREDEPLOY_ADDRESS;
  const implementationGatewayAddress = await upgrades.erc1967.getImplementationAddress(proxyGateway);
  await run('verify:verify', {
    address: implementationGatewayAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: proxyGateway,
    constructorArguments: [],
  });
});
