import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment, RunTaskFunction, TaskArguments } from 'hardhat/types';

import { getRequiredEnvVar } from './utils/loadVariables';

const REINITIALIZE_FUNCTION_PREFIX = 'reinitializeV'; // Prefix for reinitialize functions

// This file defines generic tasks that can be used to upgrade the implementation of already deployed contracts.

function getImplementationDirectory(input: string): string {
  const colonIndex = input.lastIndexOf('/');
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

async function upgradeCurrentToNew(
  proxyAddress: string,
  currentImplementation: string,
  newImplementation: string,
  verifyContract: boolean,
  hre: HardhatRuntimeEnvironment,
  reinitializeArgs: unknown[] = [],
) {
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log(`Importing ${currentImplementation} contract implementation at address ${proxyAddress}...`);
  const currentImplementationFactory = await hre.ethers.getContractFactory(currentImplementation, deployer);
  const currentProxyContract = await hre.upgrades.forceImport(proxyAddress, currentImplementationFactory);
  console.log('Proxy contract successfully loaded!');

  console.log(
    `Upgrading proxy to "${newImplementation}" implementation with reinitialize arguments:`,
    reinitializeArgs,
  );

  // Get reinitialize function from the new implementation artifact
  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  const reinitializeFunction = newImplementationArtifact.abi.find(
    (item) => item.type === 'function' && item.name.includes(REINITIALIZE_FUNCTION_PREFIX),
  );

  // Prepare the new implementation factory and execute the upgrade by calling the reinitialize function
  const newImplementationFactory = await hre.ethers.getContractFactory(newImplementation, deployer);

  await hre.upgrades.upgradeProxy(currentProxyContract, newImplementationFactory, {
    call: {
      fn: reinitializeFunction.name,
      args: reinitializeArgs,
    },
  });
  console.log('Proxy contract successfully upgraded!');

  if (verifyContract) {
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationAddress = await hre.upgrades.erc1967.getImplementationAddress(proxyAddress);
    await hre.run('verify:verify', {
      address: implementationAddress,
      contract: newImplementation,
      constructorArguments: [],
    });
  }
}

async function compileImplementations(
  currentImplementation: string,
  newImplementation: string,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  await hre.run('compile:specific', { contract: getImplementationDirectory(currentImplementation) });
  await hre.run('compile:specific', { contract: getImplementationDirectory(newImplementation) });
}

async function checkImplementationArtifacts(
  expectedArtifactName: string,
  currentImplementation: string,
  newImplementation: string,
  hre: HardhatRuntimeEnvironment,
): Promise<void> {
  const currentImplementationArtifact = await hre.artifacts.readArtifact(currentImplementation);
  if (currentImplementationArtifact.contractName !== expectedArtifactName) {
    throw new Error(
      `The current implementation artifact does not match the expected contract name "${expectedArtifactName}". Found: ${currentImplementationArtifact.contractName}`,
    );
  }

  const newImplementationArtifact = await hre.artifacts.readArtifact(newImplementation);
  if (newImplementationArtifact.contractName !== expectedArtifactName) {
    throw new Error(
      `The new implementation artifact does not match the expected contract name "${expectedArtifactName}". Found: ${newImplementationArtifact.contractName}`,
    );
  }

  const hasReinitializeFunction = newImplementationArtifact.abi.some(
    (item) => item.type === 'function' && item.name.includes(REINITIALIZE_FUNCTION_PREFIX),
  );
  if (!hasReinitializeFunction) {
    throw new Error(
      `The new implementation artifact does not contain a reinitialize function. Please ensure the contract has a reinitialize function defined.`,
    );
  }
}

task('task:upgradeACL')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/ACL.sol:ACL',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/ACLUpgradedExample.sol:ACLUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts('ACL', currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
      proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar('ACL_CONTRACT_ADDRESS');
    }

    const pauserAddress = getRequiredEnvVar('PAUSER_ADDRESS');

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre, [
      pauserAddress,
    ]);
  });

task('task:upgradeFHEVMExecutor')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/FHEVMExecutor.sol:FHEVMExecutor',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/FHEVMExecutorUpgradedExample.sol:FHEVMExecutorUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts('FHEVMExecutor', currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.exec'));
      proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar('FHEVM_EXECUTOR_CONTRACT_ADDRESS');
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task('task:upgradeKMSVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/KMSVerifier.sol:KMSVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/KMSVerifierUpgradedExample.sol:KMSVerifierUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts('KMSVerifier', currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier'));
      proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar('KMS_VERIFIER_CONTRACT_ADDRESS');
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task('task:upgradeInputVerifier')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/InputVerifier.sol:InputVerifier',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: contracts/InputVerifier2.sol:InputVerifier',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts('InputVerifier', currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier'));
      proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar('INPUT_VERIFIER_CONTRACT_ADDRESS');
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });

task('task:upgradeHCULimit')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/HCULimit.sol:HCULimit',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/HCULimitUpgradedExample.sol:HCULimitUpgradedExample',
  )
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (
    { currentImplementation, newImplementation, useInternalProxyAddress, verifyContract }: TaskArguments,
    hre,
  ) {
    await compileImplementations(currentImplementation, newImplementation, hre);

    await checkImplementationArtifacts('HCULimit', currentImplementation, newImplementation, hre);

    let proxyAddress: string;
    if (useInternalProxyAddress) {
      const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.hculimit'));
      proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
    } else {
      proxyAddress = getRequiredEnvVar('HCU_LIMIT_CONTRACT_ADDRESS');
    }

    await upgradeCurrentToNew(proxyAddress, currentImplementation, newImplementation, verifyContract, hre);
  });
