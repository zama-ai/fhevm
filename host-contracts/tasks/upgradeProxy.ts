import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { RunTaskFunction, TaskArguments } from 'hardhat/types';

import { getRequiredEnvVar } from './utils/loadVariables';

function stripContractName(input: string): string {
  const colonIndex = input.lastIndexOf('/');
  if (colonIndex !== -1) {
    return input.substring(0, colonIndex);
  }
  return input;
}

async function upgradeCurrentToNew(
  privateKey: string,
  proxyAddress: string,
  currentImplem: string,
  newImplem: string,
  verifyContract: boolean,
  upgrades: HardhatUpgrades,
  run: RunTaskFunction,
  ethers: any,
) {
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  await run('compile:specific', { contract: stripContractName(currentImplem) });
  await run('compile:specific', { contract: stripContractName(newImplem) });
  const currentImplementation = await ethers.getContractFactory(currentImplem, deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const newImplementationFactory = await ethers.getContractFactory(newImplem, deployer);
  await upgrades.upgradeProxy(proxy, newImplementationFactory, {
    call: { fn: 'reinitializeV2' },
  });
  if (verifyContract) {
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
  }
}

async function upgradeCurrentToNewACL(
  privateKey: string,
  proxyAddress: string,
  currentImplem: string,
  newImplem: string,
  verifyContract: boolean,
  upgrades: HardhatUpgrades,
  run: RunTaskFunction,
  ethers: any,
) {
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  await run('compile:specific', { contract: stripContractName(currentImplem) });
  await run('compile:specific', { contract: stripContractName(newImplem) });
  const currentImplementation = await ethers.getContractFactory(currentImplem, deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const newImplementationFactory = await ethers.getContractFactory(newImplem, deployer);
  await upgrades.upgradeProxy(proxy, newImplementationFactory, {
    call: { fn: 'reinitializeV2', args: [getRequiredEnvVar('PAUSER_ADDRESS')] },
  });
  if (verifyContract) {
    console.log('Waiting 2 minutes before contract verification... Please wait...');
    await new Promise((resolve) => setTimeout(resolve, 2 * 60 * 1000));
    const implementationACLAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
    await run('verify:verify', {
      address: implementationACLAddress,
      constructorArguments: [],
    });
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
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
    const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
    await upgradeCurrentToNewACL(
      privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
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
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.exec'));
    const proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
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
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier'));
    const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
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
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier'));
    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
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
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.hculimit'));
    const proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task('task:upgradeDecryptionOracleContract')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: decryptionOracle/DecryptionOracle.sol:DecryptionOracle',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: example/DecryptionOracleUpgradedExample.sol:DecryptionOracleUpgradedExample',
  )
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
    const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
    await upgradeCurrentToNew(
      privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });
