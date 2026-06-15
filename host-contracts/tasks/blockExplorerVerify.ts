import { task, types } from 'hardhat/config';
import type { RunTaskFunction } from 'hardhat/types';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Verifies a single contract on the block explorer, skipping the benign "already verified" response.
//
// `@nomicfoundation/hardhat-verify` rethrows "Already Verified" as a hard error — for the auto-matched
// ERC1967 proxy, and for the deterministic implementation when a prior deploy already verified it.
// When a per-contract `task:verify*` is called straight from a deploy script (the gitops sc-deploy
// pattern), that error combines with `set -eo pipefail` to abort the whole deploy. Genuine failures
// (bad API key, explorer down, bytecode mismatch) are rethrown unchanged.
export async function verifyContract(run: RunTaskFunction, address: string): Promise<void> {
  try {
    await run('verify:verify', { address, constructorArguments: [] });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (/already verified/i.test(message)) {
      console.log(`${address} is already verified — skipping.`);
    } else {
      throw error;
    }
  }
}

task('task:verifyACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationACLAddress);
    await verifyContract(run, proxyAddress);
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
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('FHEVM_EXECUTOR_CONTRACT_ADDRESS');
    const implementationFHEVMExecutorAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationFHEVMExecutorAddress);
    await verifyContract(run, proxyAddress);
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
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');
    const implementationKMSVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationKMSVerifierAddress);
    await verifyContract(run, proxyAddress);
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
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('INPUT_VERIFIER_CONTRACT_ADDRESS');
    const implementationInputVerifierAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationInputVerifierAddress);
    await verifyContract(run, proxyAddress);
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
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS');
    const implementationHCULimitAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationHCULimitAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyPauserSet')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const implementationPauserSetAddress = getRequiredEnvVar('PAUSER_SET_CONTRACT_ADDRESS');
    await verifyContract(run, implementationPauserSetAddress);
  });

task('task:verifyProtocolConfig')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('PROTOCOL_CONFIG_CONTRACT_ADDRESS');
    const implementationProtocolConfigAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationProtocolConfigAddress);
    await verifyContract(run, proxyAddress);
  });

task('task:verifyKMSGeneration')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { upgrades, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const proxyAddress = getRequiredEnvVar('KMS_GENERATION_CONTRACT_ADDRESS');
    const implementationKMSGenerationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await verifyContract(run, implementationKMSGenerationAddress);
    await verifyContract(run, proxyAddress);
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
    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      await hre.run('task:verifyACL', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify FHEVMExecutor contract:');
      await hre.run('task:verifyFHEVMExecutor', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify KMSVerifier contract:');
      await hre.run('task:verifyKMSVerifier', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify InputVerifier contract:');
      await hre.run('task:verifyInputVerifier', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify HCULimit contract:');
      await hre.run('task:verifyHCULimit', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify PauserSet contract:');
      await hre.run('task:verifyPauserSet', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify ProtocolConfig contract:');
      await hre.run('task:verifyProtocolConfig', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    try {
      // to not panic if Etherscan throws an error due to already verified implementation
      console.log('Verify KMSGeneration contract:');
      await hre.run('task:verifyKMSGeneration', { useInternalProxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }

    console.log('Contract verification done!');
  });
