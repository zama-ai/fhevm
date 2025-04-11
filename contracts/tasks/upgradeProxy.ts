import { HardhatEthersHelpers } from '@nomicfoundation/hardhat-ethers/types';
import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import fs from 'fs';
import { task, types } from 'hardhat/config';
import type { RunTaskFunction, TaskArguments } from 'hardhat/types';

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
  await upgrades.upgradeProxy(proxy, newImplementationFactory);
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
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.acl'));
    const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task('task:upgradeHTTPZExecutor')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/HTTPZExecutor.sol:HTTPZExecutor',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/HTTPZExecutorUpgradedExample.sol:HTTPZExecutorUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.exec'));
    const proxyAddress = parsedEnv.TFHE_EXECUTOR_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
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
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.kmsverifier'));
    const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
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
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.inputverifier'));
    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });

task('task:upgradeFHEGasLimit')
  .addParam(
    'currentImplementation',
    'The currently deployed implementation solidity contract path and name, eg: contracts/FHEGasLimit.sol:FHEGasLimit',
  )
  .addParam(
    'newImplementation',
    'The new implementation solidity contract path and name, eg: examples/FHEGasLimitUpgradedExample.sol:FHEGasLimitUpgradedExample',
  )
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.fhegaslimit'));
    const proxyAddress = parsedEnv.FHE_GASLIMIT_CONTRACT_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
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
  .addParam('privateKey', 'The deployer private key')
  .addOptionalParam(
    'verifyContract',
    'Verify new implementation on Etherscan (for eg if deploying on Sepolia or Mainnet)',
    false,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades, run }) {
    const parsedEnv = dotenv.parse(fs.readFileSync('addresses/.env.decryptionoracle'));
    const proxyAddress = parsedEnv.DECRYPTION_ORACLE_ADDRESS;
    await upgradeCurrentToNew(
      taskArguments.privateKey,
      proxyAddress,
      taskArguments.currentImplementation,
      taskArguments.newImplementation,
      taskArguments.verifyContract,
      upgrades,
      run,
      ethers,
    );
  });
