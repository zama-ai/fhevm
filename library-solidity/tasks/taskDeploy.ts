import type { HardhatEthersHelpers } from '@nomicfoundation/hardhat-ethers/types';
import type { HardhatUpgrades } from '@openzeppelin/hardhat-upgrades';
import type { HardhatRuntimeEnvironment, TaskArguments } from 'hardhat/types';
import dotenv from 'dotenv';
import path from 'path';
import * as fs from 'fs';
import { getAddress, isAddress, Wallet } from 'ethers';
import { task, types } from 'hardhat/config';

import { getRequiredEnvVar } from './utils/loadVariables';
import { getZamaConfigAddresses } from './utils/zamaConfig';

// Generated files holding the host contract addresses. Absolute paths, resolved from this file's location
// so they do not depend on the directory the process runs from.
const HOST_ADDRESSES_DIR = path.resolve(__dirname, '../fhevmTemp/addresses');
const HOST_ADDRESSES_ENV_PATH = path.join(HOST_ADDRESSES_DIR, '.env.host');
const FHEVM_HOST_ADDRESSES_SOL_PATH = path.join(HOST_ADDRESSES_DIR, 'FHEVMHostAddresses.sol');

const LEGACY_DEPLOY_ALL_HOST_CONTRACTS_WARNING = `task:deployLegacyAllHostContracts is deprecated and will be removed after the v0.13 rollout.
It deploys KMSGeneration and is valid only for canonical-host deployments.
Use task:deployAllHostContracts --with-kms-generation true instead.`;

////////////////////////////////////////////////////////////////////////////////
// All Host Contracts
////////////////////////////////////////////////////////////////////////////////

task('task:deployAllHostContracts')
  .addParam(
    'withKmsGeneration',
    'Whether to deploy canonical-host-only KMSGeneration. Required: true for canonical host, false for non-canonical host.',
    undefined,
    types.boolean,
  )
  .setAction(async function ({ withKmsGeneration }: { withKmsGeneration: boolean }, hre) {
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
    if (withKmsGeneration) {
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
  await hre.run('task:deployAllHostContracts', { withKmsGeneration: true });
});

////////////////////////////////////////////////////////////////////////////////
// UUPS Helpers
////////////////////////////////////////////////////////////////////////////////

async function _deployEmptyUUPSForACL(
  ethers: HardhatEthersHelpers,
  upgrades: HardhatUpgrades,
  deployer: Wallet,
): Promise<string> {
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

async function _deployEmptyUUPS(
  ethers: HardhatEthersHelpers,
  upgrades: HardhatUpgrades,
  deployer: Wallet,
): Promise<string> {
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

////////////////////////////////////////////////////////////////////////////////
// Deploy Empty UUPS Proxies
////////////////////////////////////////////////////////////////////////////////

task('task:deployEmptyUUPSProxies')
  .addParam(
    'withKmsGeneration',
    'Whether to deploy the canonical-host-only KMSGeneration proxy. Required: true for canonical host, false for non-canonical host.',
    undefined,
    types.boolean,
  )
  .setAction(async function ({ withKmsGeneration }: { withKmsGeneration: boolean }, { ethers, upgrades, run }) {
    // Compile the EmptyUUPS proxy contract for ACL
    await run('compile:specific', { contract: 'fhevmTemp/contracts/emptyProxyACL' });
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    // Ensure the addresses directory exists.
    fs.mkdirSync(HOST_ADDRESSES_DIR, { recursive: true });

    const aclAddress = await _deployEmptyUUPSForACL(ethers, upgrades, deployer);
    await run('task:setACLAddress', { address: aclAddress });

    // Compile the EmptyUUPS proxy contract for other contracts
    await run('compile:specific', { contract: 'fhevmTemp/contracts/emptyProxy' });

    const fhevmExecutorAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setFHEVMExecutorAddress', { address: fhevmExecutorAddress });

    const kmsVerifierAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setKMSVerifierAddress', { address: kmsVerifierAddress });

    const inputVerifierAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setInputVerifierAddress', { address: inputVerifierAddress });

    const HCULimitAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setHCULimitAddress', { address: HCULimitAddress });

    const protocolConfigAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setProtocolConfigAddress', { address: protocolConfigAddress });

    if (withKmsGeneration) {
      const kmsGenerationAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
      await run('task:setKMSGenerationAddress', { address: kmsGenerationAddress });
    }

    const confidentialBridgeAddress = await _deployEmptyUUPS(ethers, upgrades, deployer);
    await run('task:setBridgeAddress', { address: confidentialBridgeAddress });
  });

////////////////////////////////////////////////////////////////////////////////
// Deploy ACL
////////////////////////////////////////////////////////////////////////////////

task('task:deployACL').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const newImplem = await ethers.getContractFactory('ACL', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
  const proxyAddress = parsedEnv.ACL_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('ACL code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Deploy FHEVMExecutor
////////////////////////////////////////////////////////////////////////////////

task('task:deployFHEVMExecutor').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/FHEVMExecutor.sol:FHEVMExecutor', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
  const proxyAddress = parsedEnv.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem);
  console.info('FHEVMExecutor code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Deploy KMSVerifier
////////////////////////////////////////////////////////////////////////////////

// Returns the integer value of the `name` env variable, or `defaultValue` if it is not set or empty.
function getOptionalIntegerEnvVar(name: string, defaultValue: number): number {
  const value = process.env[name];
  if (value === undefined || value === '') {
    return defaultValue;
  }
  const integer = Number(value);
  if (value.trim() === '' || !Number.isInteger(integer)) {
    throw new Error(`"${name}" env variable must be an integer, got "${value}"`);
  }
  return integer;
}

function buildKmsNodeParams(): Array<{
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
}> {
  const numNodes = +getRequiredEnvVar('NUM_KMS_NODES');
  const nodes: Array<{
    txSenderAddress: string;
    signerAddress: string;
    ipAddress: string;
    storageUrl: string;
    partyId: number;
    mpcIdentity: string;
    caCert: string;
    storagePrefix: string;
  }> = [];
  for (let idx = 0; idx < numNodes; idx++) {
    const txSenderAddress = getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`);
    const signerAddress = getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`);
    const ipAddress = getRequiredEnvVar(`KMS_NODE_IP_${idx}`);
    const storageUrl = getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`);
    const partyId = getOptionalIntegerEnvVar(`KMS_NODE_PARTY_ID_${idx}`, idx);
    const mpcIdentity = getRequiredEnvVar(`KMS_NODE_MPC_IDENTITY_${idx}`);
    const caCert = getRequiredEnvVar(`KMS_NODE_CA_CERT_${idx}`);
    const storagePrefix = getRequiredEnvVar(`KMS_NODE_STORAGE_PREFIX_${idx}`);
    nodes.push({ txSenderAddress, signerAddress, ipAddress, storageUrl, partyId, mpcIdentity, caCert, storagePrefix });
  }
  return nodes;
}

export function buildKmsThresholds(): {
  publicDecryption: number;
  userDecryption: number;
  kmsGen: number;
  mpc: number;
} {
  return {
    publicDecryption: +getRequiredEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
    userDecryption: +getRequiredEnvVar('USER_DECRYPTION_THRESHOLD'),
    kmsGen: +getRequiredEnvVar('KMS_GEN_THRESHOLD'),
    mpc: +getRequiredEnvVar('MPC_THRESHOLD'),
  };
}

task('task:deployKMSVerifier').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
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
// Deploy ProtocolConfig
////////////////////////////////////////////////////////////////////////////////

task('task:deployProtocolConfig').setAction(async function (_taskArguments: TaskArguments, hre) {
  const { ethers, upgrades } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/ProtocolConfig.sol:ProtocolConfig', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
  const proxyAddress = parsedEnv.PROTOCOL_CONFIG_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const softwareVersion = process.env.KMS_SOFTWARE_VERSION ?? '';
  const pcrValues: Array<{ pcr0: string; pcr1: string; pcr2: string }> = [];

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [buildKmsNodeParams(), buildKmsThresholds(), softwareVersion, pcrValues],
    },
  });
  console.info('ProtocolConfig code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Deploy KMSGeneration
////////////////////////////////////////////////////////////////////////////////

task('task:deployKMSGeneration').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/KMSGeneration.sol:KMSGeneration', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
  const proxyAddress = parsedEnv.KMS_GENERATION_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  console.info('KMSGeneration code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Deploy InputVerifier
////////////////////////////////////////////////////////////////////////////////

task('task:deployInputVerifier')
  .addOptionalParam(
    'useAddress',
    'Use addresses instead of private keys env variables for kms signers',
    true,
    types.boolean,
  )
  .setAction(async function ({ useAddress }: { useAddress: boolean }, { ethers, upgrades }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
    const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
    const newImplem = await ethers.getContractFactory('fhevmTemp/contracts/InputVerifier.sol:InputVerifier', deployer);
    const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));

    const proxyAddress = parsedEnv.INPUT_VERIFIER_CONTRACT_ADDRESS;
    const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
    const verifyingContractSource = getRequiredEnvVar('INPUT_VERIFICATION_ADDRESS');
    const chainIDSource = +getRequiredEnvVar('CHAIN_ID_GATEWAY');

    const initialSigners: string[] = [];
    const numSigners = getRequiredEnvVar('NUM_COPROCESSORS');
    for (let idx = 0; idx < +numSigners; idx++) {
      if (!useAddress) {
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
// Deploy HCULimit
////////////////////////////////////////////////////////////////////////////////

task('task:deployHCULimit').setAction(async function (_taskArguments: TaskArguments, { ethers, upgrades }) {
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('HCULimit', deployer);
  const parsedEnv = dotenv.parse(fs.readFileSync(HOST_ADDRESSES_ENV_PATH));
  const proxyAddress = parsedEnv.HCU_LIMIT_CONTRACT_ADDRESS;
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  await upgrades.upgradeProxy(proxy, newImplem, {
    call: { fn: 'initializeFromEmptyProxy', args: [BigInt('281474976710655'), BigInt('5000000'), BigInt('20000000')] },
  });
  console.info('HCULimit code set successfully at address:', proxyAddress);
});

////////////////////////////////////////////////////////////////////////////////
// Deploy PauserSet
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
  .setAction(async function ({ address }: { address: string }, hre) {
    await _assertMatchesZamaConfig(hre, {
      taskName: 'task:setACLAddress',
      contractName: 'ACL',
      zamaConfigField: 'ACLAddress',
      address,
    });

    // ACL is the first contract. Output files are created here.
    await _resetHostContractAddressFiles();
    await _appendHostContractAddress({
      taskName: 'task:setACLAddress',
      contractName: 'ACL',
      envVarName: 'ACL_CONTRACT_ADDRESS',
      solidityConstantName: 'aclAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup FHEVMExecutor Address
////////////////////////////////////////////////////////////////////////////////

task('task:setFHEVMExecutorAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }, hre) {
    await _assertMatchesZamaConfig(hre, {
      taskName: 'task:setFHEVMExecutorAddress',
      contractName: 'FHEVMExecutor',
      zamaConfigField: 'CoprocessorAddress',
      address,
    });

    await _appendHostContractAddress({
      taskName: 'task:setFHEVMExecutorAddress',
      contractName: 'FHEVMExecutor',
      envVarName: 'FHEVM_EXECUTOR_CONTRACT_ADDRESS',
      solidityConstantName: 'fhevmExecutorAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup KMSVerifier Address
////////////////////////////////////////////////////////////////////////////////

task('task:setKMSVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }, hre) {
    await _assertMatchesZamaConfig(hre, {
      taskName: 'task:setKMSVerifierAddress',
      contractName: 'KMSVerifier',
      zamaConfigField: 'KMSVerifierAddress',
      address,
    });

    await _appendHostContractAddress({
      taskName: 'task:setKMSVerifierAddress',
      contractName: 'KMSVerifier',
      envVarName: 'KMS_VERIFIER_CONTRACT_ADDRESS',
      solidityConstantName: 'kmsVerifierAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup InputVerifier Address
////////////////////////////////////////////////////////////////////////////////

task('task:setInputVerifierAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }) {
    await _appendHostContractAddress({
      taskName: 'task:setInputVerifierAddress',
      contractName: 'InputVerifier',
      envVarName: 'INPUT_VERIFIER_CONTRACT_ADDRESS',
      solidityConstantName: 'inputVerifierAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup HCULimit Address
////////////////////////////////////////////////////////////////////////////////

task('task:setHCULimitAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }) {
    await _appendHostContractAddress({
      taskName: 'task:setHCULimitAddress',
      contractName: 'HCULimit',
      envVarName: 'HCU_LIMIT_CONTRACT_ADDRESS',
      solidityConstantName: 'hcuLimitAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup PauserSet Address
////////////////////////////////////////////////////////////////////////////////

task('task:setPauserSetAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }) {
    await _appendHostContractAddress({
      taskName: 'task:setPauserSetAddress',
      contractName: 'PauserSet',
      envVarName: 'PAUSER_SET_CONTRACT_ADDRESS',
      solidityConstantName: 'pauserSetAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup ProtocolConfig Address
////////////////////////////////////////////////////////////////////////////////

task('task:setProtocolConfigAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }) {
    await _appendHostContractAddress({
      taskName: 'task:setProtocolConfigAddress',
      contractName: 'ProtocolConfig',
      envVarName: 'PROTOCOL_CONFIG_CONTRACT_ADDRESS',
      solidityConstantName: 'protocolConfigAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup ConfidentialBridge Address
////////////////////////////////////////////////////////////////////////////////

task('task:setBridgeAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }) {
    await _appendHostContractAddress({
      taskName: 'task:setBridgeAddress',
      contractName: 'ConfidentialBridge',
      envVarName: 'CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS',
      solidityConstantName: 'confidentialBridgeAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Setup KMSGeneration Address
////////////////////////////////////////////////////////////////////////////////

task('task:setKMSGenerationAddress')
  .addParam('address', 'The address of the contract')
  .setAction(async function ({ address }: { address: string }) {
    await _appendHostContractAddress({
      taskName: 'task:setKMSGenerationAddress',
      contractName: 'KMSGeneration',
      envVarName: 'KMS_GENERATION_CONTRACT_ADDRESS',
      solidityConstantName: 'kmsGenerationAdd',
      address,
    });
  });

////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////

// Resets fhevmTemp/addresses/.env.host to empty and fhevmTemp/addresses/FHEVMHostAddresses.sol to its Solidity header,
// before the host contract addresses are appended with appendHostContractAddress.
async function _resetHostContractAddressFiles(): Promise<void> {
  const solidityHeader = `// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;
`;
  await fs.promises.mkdir(HOST_ADDRESSES_DIR, { recursive: true });
  await fs.promises.writeFile(HOST_ADDRESSES_ENV_PATH, '');
  await fs.promises.writeFile(FHEVM_HOST_ADDRESSES_SOL_PATH, solidityHeader, { encoding: 'utf8' });
}

// On a chain supported by config/ZamaConfig.sol, throws unless `address` is the address ZamaConfig.sol hardcodes for
// that chain in `zamaConfigField`: otherwise contracts using ZamaConfig would point to a different contract.
// Does nothing on chains ZamaConfig.sol does not support.
async function _assertMatchesZamaConfig(
  hre: HardhatRuntimeEnvironment,
  {
    taskName,
    contractName,
    zamaConfigField,
    address,
  }: {
    taskName: string;
    contractName: string;
    zamaConfigField: 'ACLAddress' | 'CoprocessorAddress' | 'KMSVerifierAddress';
    address: string;
  },
): Promise<void> {
  const chainId = (await hre.ethers.provider.getNetwork()).chainId;
  const expected = getZamaConfigAddresses(chainId)?.[zamaConfigField];
  if (expected === undefined) {
    return;
  }
  if (!isAddress(address) || getAddress(address) !== expected) {
    throw new Error(
      `${taskName}: ${contractName} address ${address} does not match ZamaConfig.sol's ${zamaConfigField} ` +
        `${expected} for chain ${chainId}`,
    );
  }
}

// Appends a host contract address to fhevmTemp/addresses/.env.host (as `<envVarName>=<address>`) and to
// fhevmTemp/addresses/FHEVMHostAddresses.sol (as `address constant <solidityConstantName> = <address>;`).
// Both files are created by task:setACLAddress, which must run first.
async function _appendHostContractAddress({
  taskName,
  contractName,
  envVarName,
  solidityConstantName,
  address,
}: {
  taskName: string;
  contractName: string;
  envVarName: string;
  solidityConstantName: string;
  address: string;
}): Promise<void> {
  if (!isAddress(address)) {
    throw new Error(`${taskName}: invalid address ${address}`);
  }

  // Both files must already exist (created by task:setACLAddress): appending would otherwise silently
  // create them without the Solidity header and the ACL address.
  for (const filePath of [HOST_ADDRESSES_ENV_PATH, FHEVM_HOST_ADDRESSES_SOL_PATH]) {
    if (!fs.existsSync(filePath)) {
      throw new Error(`${taskName}: ${filePath} does not exist, run task:setACLAddress first`);
    }
  }

  const content = `${envVarName}=${address}\n`;
  try {
    await fs.promises.appendFile(HOST_ADDRESSES_ENV_PATH, content);
    console.info(`${contractName} address ${address} written successfully!`);
  } catch (err) {
    console.error(`Failed to write ${contractName} address:`, err);
  }

  const solidityTemplate = `
address constant ${solidityConstantName} = ${address};\n`;

  try {
    await fs.promises.appendFile(FHEVM_HOST_ADDRESSES_SOL_PATH, solidityTemplate, { encoding: 'utf8' });
    console.info(`${FHEVM_HOST_ADDRESSES_SOL_PATH} file appended with ${solidityConstantName} successfully!`);
  } catch (error) {
    console.error(`Failed to write ${FHEVM_HOST_ADDRESSES_SOL_PATH}`, error);
  }
}
