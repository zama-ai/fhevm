import type { HardhatEthersHelpers } from '@nomicfoundation/hardhat-ethers/types';
import { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import * as fs from 'fs-extra';
import { task, types } from 'hardhat/config';
import type { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import path from 'path';

import { getRequiredEnvVar } from './utils/loadVariables';

// OZ upgrades' upgradeProxy can return before the upgradeToAndCall tx is mined on
// interval-mining networks. Poll until the new implementation answers a
// state-dependent view.
async function waitForUpgradeLanded(
  hre: HardhatRuntimeEnvironment,
  proxyAddress: string,
  contractLabel: string,
): Promise<void> {
  const proxy = new hre.ethers.Contract(
    proxyAddress,
    ['function getCurrentKmsContextId() view returns (uint256)'],
    hre.ethers.provider,
  );
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      await proxy.getCurrentKmsContextId();
      return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  }
  throw new Error(`${contractLabel} upgrade did not land after 30s of polling`);
}

const MISSING_WITH_KMS_GENERATION = '__missing_with_kms_generation__';
const WITH_KMS_GENERATION_HELP = `Missing or invalid required --with-kms-generation flag.

KMSGeneration is deployed only on the canonical host chain. Use:
  --with-kms-generation true   when deploying the canonical host chain
  --with-kms-generation false  when deploying a non-canonical host chain

The no-flag behavior was removed in v0.13 to avoid accidentally deploying KMSGeneration on every host chain.`;
const LEGACY_DEPLOY_ALL_HOST_CONTRACTS_WARNING = `task:deployLegacyAllHostContracts is deprecated and will be removed after the v0.13 rollout.
It deploys KMSGeneration and is valid only for canonical-host deployments.
Use task:deployAllHostContracts --with-kms-generation true instead.`;

function parseWithKmsGeneration(value: unknown): boolean {
  if (value === 'true') {
    return true;
  }
  if (value === 'false') {
    return false;
  }
  throw new Error(WITH_KMS_GENERATION_HELP);
}

////////////////////////////////////////////////////////////////////////////////
// All Host Contracts
////////////////////////////////////////////////////////////////////////////////

task('task:deployAllHostContracts')
  .addOptionalParam(
    'withKmsGeneration',
    'Whether to deploy canonical-host-only KMSGeneration. Required: true for canonical host, false for non-canonical host.',
    MISSING_WITH_KMS_GENERATION,
    types.string,
  )
  .setAction(async function ({ withKmsGeneration }, hre) {
    const deployKmsGeneration = parseWithKmsGeneration(withKmsGeneration);

    if (process.env.SOLIDITY_COVERAGE !== 'true') {
      await hre.run('clean');
    }

    await hre.run('task:deployEmptyUUPSProxies', { withKmsGeneration });
    await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/immutable' });
    await hre.run('task:deployPauserSet');

    // The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
    // Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
    // used.
    await hre.run('compile:specific', { contract: 'fhevmTemp/contracts' });

    await hre.run('task:deployACL');
    await hre.run('task:deployFHEVMExecutor');
    await hre.run('task:deployProtocolConfig');
    if (deployKmsGeneration) {
      await hre.run('task:deployKMSGeneration');
    }
    await hre.run('task:deployKMSVerifier');
    await hre.run('task:deployInputVerifier');
    await hre.run('task:deployHCULimit');

    // Compile examples
    await hre.run('compile:specific', { contract: 'examples' });
  });

task('task:deployLegacyAllHostContracts').setAction(async function (_, hre) {
  console.warn(LEGACY_DEPLOY_ALL_HOST_CONTRACTS_WARNING);
  await hre.run('task:deployAllHostContracts', { withKmsGeneration: 'true' });
});

////////////////////////////////////////////////////////////////////////////////
// UUPS
////////////////////////////////////////////////////////////////////////////////

async function deployEmptyUUPSForACL(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.log('Deploying an EmptyUUPSProxyACL proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const UUPSEmptyACL = await upgrades.deployProxy(factory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmptyACL.waitForDeployment();
  const UUPSEmptyACLAddress = await UUPSEmptyACL.getAddress();
  console.log('EmptyUUPSProxyACL proxy contract successfully deployed!');
  return UUPSEmptyACLAddress;
}

async function deployEmptyUUPS(ethers: HardhatEthersHelpers, upgrades: HardhatUpgrades, deployer: Wallet) {
  console.info('Deploying an EmptyUUPSProxy proxy contract...');
  const factory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const UUPSEmpty = await upgrades.deployProxy(factory, {
    initializer: 'initialize',
    kind: 'uups',
  });
  await UUPSEmpty.waitForDeployment();
  const UUPSEmptyAddress = await UUPSEmpty.getAddress();
  console.info('EmptyUUPSProxy proxy contract successfully deployed!');
  return UUPSEmptyAddress;
}

task('task:deployEmptyUUPSProxies')
  .addOptionalParam(
    'withKmsGeneration',
    'Whether to deploy the canonical-host-only KMSGeneration proxy. Required: true for canonical host, false for non-canonical host.',
    MISSING_WITH_KMS_GENERATION,
    types.string,
  )
  .setAction(async function ({ withKmsGeneration }, { ethers, upgrades, run }) {
    const deployKmsGeneration = parseWithKmsGeneration(withKmsGeneration);

    // Compile the EmptyUUPS proxy contract for ACL
    await run('compile:specific', { contract: 'fhevmTemp/contracts/emptyProxyACL' });
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    // Ensure the addresses directory exists.
    fs.mkdirSync(path.join(__dirname, '../fhevmTemp/addresses'), { recursive: true });

    const aclAddress = await deployEmptyUUPSForACL(ethers, upgrades, deployer);
    await run('task:setACLAddress', { address: aclAddress });

    // Compile the EmptyUUPS proxy contract for other contracts
    await run('compile:specific', { contract: 'fhevmTemp/contracts/emptyProxy' });

    const fhevmExecutorAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setFHEVMExecutorAddress', { address: fhevmExecutorAddress });

    const kmsVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setKMSVerifierAddress', { address: kmsVerifierAddress });

    const inputVerifierAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setInputVerifierAddress', { address: inputVerifierAddress });

    const HCULimitAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setHCULimitAddress', { address: HCULimitAddress });

    const protocolConfigAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setProtocolConfigAddress', { address: protocolConfigAddress });

    if (deployKmsGeneration) {
      const kmsGenerationAddress = await deployEmptyUUPS(ethers, upgrades, deployer);
      await run('task:setKMSGenerationAddress', { address: kmsGenerationAddress });
    }
  });

////////////////////////////////////////////////////////////////////////////////
// ACL
////////////////////////////////////////////////////////////////////////////////

task('task:deployACL').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const newImplem = await ethers.getContractFactory('ACL', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('ACL code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// FHEVMExecutor
////////////////////////////////////////////////////////////////////////////////

task('task:deployFHEVMExecutor').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  let newImplem;
  newImplem = await ethers.getContractFactory('fhevmTemp/contracts/FHEVMExecutor.sol:FHEVMExecutor', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('FHEVMExecutor code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// KMSVerifier
////////////////////////////////////////////////////////////////////////////////

function buildKmsNodes() {
  const numSigners = +getRequiredEnvVar('NUM_KMS_NODES');
  const nodes = [];
  for (let idx = 0; idx < numSigners; idx++) {
    const signerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
    nodes.push({
      txSenderAddress: signerAddress,
      signerAddress,
      ipAddress: '',
      storageUrl: '',
    });
  }
  return nodes;
}

function buildKmsThresholds() {
  const threshold = +getRequiredEnvVar('KMS_THRESHOLD');
  return {
    publicDecryption: threshold,
    userDecryption: threshold,
    kmsGen: threshold,
    mpc: threshold,
  };
}

task('task:deployKMSVerifier').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.KMS_VERIFIER_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const verifyingContractSource = getRequiredEnvVar('DECRYPTION_ADDRESS');
  const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [verifyingContractSource, chainIDSource],
    },
  });
  console.info('KMSVerifier code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// ProtocolConfig
////////////////////////////////////////////////////////////////////////////////

task('task:deployProtocolConfig').setAction(async function (_taskArguments: TaskArguments, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/ProtocolConfig.sol:ProtocolConfig', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [buildKmsNodes(), buildKmsThresholds()],
    },
  });
  // upgrades.upgradeProxy can return before the upgradeToAndCall tx is mined on interval-mining
  // networks. Wait until the new implementation is observable on-chain.
  await waitForUpgradeLanded(hre, proxyAddress, 'ProtocolConfig');
  console.info('ProtocolConfig code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// KMSGeneration
////////////////////////////////////////////////////////////////////////////////

task('task:deployKMSGeneration').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/KMSGeneration.sol:KMSGeneration', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  console.info('KMSGeneration code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// InputVerifier
////////////////////////////////////////////////////////////////////////////////

task('task:deployInputVerifier')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    true,
    types.boolean,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers, upgrades }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));

    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = getRequiredEnvVar('INPUT_VERIFICATION_ADDRESS');
    const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');

    let initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    for (let idx = 0; idx < +numSigners; idx++) {
      if (!taskArguments.useAddress) {
        const privKeySigner = getRequiredEnvVar(`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${idx}`);
        const inputSigner = new ethers.Wallet(privKeySigner).connect(ethers.provider);
        initialSigners.push(inputSigner.address);
      } else {
        const inputSignerAddress = getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`);
        initialSigners.push(inputSignerAddress);
      }
    }

    const initialThreshold = getRequiredEnvVar('COPROCESSOR_THRESHOLD');

    await upgrades.upgradeProxy(proxy, newImplem, {
      call: {
        fn: 'initializeFromEmptyProxy',
        args: [verifyingContractSource, chainIDSource, initialSigners, initialThreshold],
      },
    });
    console.info('InputVerifier code set successfully at address:', proxyAddress);
  });

////////////////////////////////////////////////////////////////////////////////
// HCULimit
////////////////////////////////////////////////////////////////////////////////

task('task:deployHCULimit').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('HCULimit', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync('fhevmTemp/addresses/.env.host'));
  const proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy', args: [BigInt('281474976710655'), BigInt('5000000'), BigInt('20000000')] },
  });
  console.info('HCULimit code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// PauserSet
////////////////////////////////////////////////////////////////////////////////

task('task:deployPauserSet').setAction(async function (_, hre) {
  // Get a deployer wallet
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new Wallet(deployerPrivateKey).connect(hre.ethers.provider);

  console.log('Deploying PauserSet...');
  const pauserSetFactory = await hre.ethers.getContractFactory('PauserSet', deployer);
  const pauserSet = await pauserSetFactory.deploy();
  const pauserSetAddress = await pauserSet.getAddress();

  await hre.run('task:setPauserSetAddress', {
    address: pauserSetAddress,
  });
});

////////////////////////////////////////////////////////////////////////////////
// Setup ACL Address
////////////////////////////////////////////////////////////////////////////////

task('task:setACLAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `ACL_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.writeFileSync(envFilePath, content, { flag: 'w' });
      console.info(`ACL address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write ACL address:', err);
    }

    const solidityTemplate = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

address constant aclAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'w',
      });
      console.info('./fhevmTemp/addresses/FHEVMHostAddresses.sol file generated successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup FHEVMExecutor Address
////////////////////////////////////////////////////////////////////////////////

task('task:setFHEVMExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `FHEVM_EXECUTOR_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.info(`FHEVMExecutor address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write FHEVMExecutor address:', err);
    }

    const solidityTemplate = `
address constant fhevmExecutorAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.info('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with fhevmExecutorAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup KMSVerifier Address
////////////////////////////////////////////////////////////////////////////////

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `KMS_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.info(`KMSVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write KMSVerifier address:', err);
    }

    const solidityTemplate = `
address constant kmsVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.info('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with kmsVerifierAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup InputVerifier Address
////////////////////////////////////////////////////////////////////////////////

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    // this script also computes the coprocessor address from its private key
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `INPUT_VERIFIER_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`InputVerifier address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write InputVerifier address:', err);
    }

    const solidityTemplate = `
address constant inputVerifierAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with inputVerifierAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup HCULimit Address
////////////////////////////////////////////////////////////////////////////////

task('task:setHCULimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `HCU_LIMIT_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`HCULimit address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write HCULimit address:', err);
    }

    const solidityTemplate = `
address constant hcuLimitAdd = ${taskArguments.address};\n`;

    try {
      fs.writeFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol file appended with hcuLimitAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup PauserSet Address
////////////////////////////////////////////////////////////////////////////////

task('task:setPauserSetAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `PAUSER_SET_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`PauserSet address ${taskArguments.address} written successfully!`);
    } catch (err) {
      console.error('Failed to write PauserSet address:', err);
    }

    const solidityTemplate = `
address constant pauserSetAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol appended with hcuLimitAdd successfully!');
    } catch (error) {
      console.error('Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol', error);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup ProtocolConfig Address
////////////////////////////////////////////////////////////////////////////////

task('task:setProtocolConfigAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `PROTOCOL_CONFIG_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`ProtocolConfig address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write ProtocolConfig address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant protocolConfigAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol appended with protocolConfigAdd successfully!');
    } catch (error) {
      throw new Error(`Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol: ${String(error)}`);
    }
  });

////////////////////////////////////////////////////////////////////////////////
// Setup KMSGeneration Address
////////////////////////////////////////////////////////////////////////////////

task('task:setKMSGenerationAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const envFilePath = path.join(__dirname, '../fhevmTemp/addresses/.env.host');
    const content = `KMS_GENERATION_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      fs.appendFileSync(envFilePath, content, { flag: 'a' });
      console.log(`KMSGeneration address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write KMSGeneration address: ${String(err)}`);
    }

    const solidityTemplate = `
address constant kmsGenerationAdd = ${taskArguments.address};\n`;

    try {
      fs.appendFileSync('./fhevmTemp/addresses/FHEVMHostAddresses.sol', solidityTemplate, {
        encoding: 'utf8',
        flag: 'a',
      });
      console.log('./fhevmTemp/addresses/FHEVMHostAddresses.sol appended with kmsGenerationAdd successfully!');
    } catch (error) {
      throw new Error(`Failed to write ./fhevmTemp/addresses/FHEVMHostAddresses.sol: ${String(error)}`);
    }
  });
