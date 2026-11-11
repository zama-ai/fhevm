// Deploys the host contracts on the in-process network, ported from the HH2 tasks (tasks/taskDeploy.ts and
// tasks/addPausers.ts), with the same sequence of transactions.
//
// The host contracts are UUPS proxies, deployed in two phases:
//   1. an empty UUPS proxy per contract (and PauserSet). The addresses are not computed: they are read back from the
//      deployment and written to fhevmTemp/addresses/. They are deterministic (CREATE address = deployer + nonce, on a
//      fresh chain), so they match config/ZamaConfig.sol only if the deployer and the sequence of transactions stay
//      the same as in HH2: do not reorder the deployments.
//   2. once the real contracts are compiled with those address constants, each proxy is upgraded to its real
//      implementation (with its initializer).
//
// All functions take the one network connection the deployment shares (`network.getOrCreate()`).
import { upgrades } from '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import { Wallet, isAddress } from 'ethers';
import fs from 'node:fs';
import path from 'node:path';

import type { HardhatRuntimeEnvironment } from 'hardhat/types/hre';
import type { NetworkConnection } from 'hardhat/types/network';

import { getIntegerEnvVar, getRequiredEnvVar } from './env.js';
import { FHEVM_HOST_ADDRESSES_SOL, FHEVM_TEMP_CONTRACTS_DIR, copyHostContracts } from './hostContracts.js';
import { assertHostAddressesMatchZamaConfig } from './zamaConfigAddresses.js';

const LIBRARY_SOLIDITY_DIR = path.resolve(import.meta.dirname, '..');

/** Host contract addresses as env variables (`<NAME>_CONTRACT_ADDRESS=0x…`), read by the tests. */
export const FHEVM_HOST_ADDRESSES_ENV = path.join(path.dirname(FHEVM_HOST_ADDRESSES_SOL), '.env.host');

/** HCULimit initializer arguments, as in HH2. */
const HCU_LIMIT_INITIAL_VALUES = [BigInt('281474976710655'), BigInt('5000000'), BigInt('20000000')] as const;

/** The deployer of every host contract. Its address is part of the deterministic host contract addresses. */
function getDeployer(connection: NetworkConnection): Wallet {
  return new Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY'), connection.ethers.provider);
}

////////////////////////////////////////////////////////////////////////////////
// Address files (fhevmTemp/addresses/)
////////////////////////////////////////////////////////////////////////////////

function resetHostAddressFiles(): void {
  fs.mkdirSync(path.dirname(FHEVM_HOST_ADDRESSES_SOL), { recursive: true });
  fs.writeFileSync(FHEVM_HOST_ADDRESSES_ENV, '');
  fs.writeFileSync(
    FHEVM_HOST_ADDRESSES_SOL,
    '// SPDX-License-Identifier: BSD-3-Clause-Clear\n\npragma solidity ^0.8.24;\n',
  );
}

function appendHostAddress(envVarName: string, solidityConstantName: string, address: string): void {
  if (!isAddress(address)) {
    throw new Error(`Invalid ${solidityConstantName} address: ${address}`);
  }
  fs.appendFileSync(FHEVM_HOST_ADDRESSES_ENV, `${envVarName}=${address}\n`);
  fs.appendFileSync(FHEVM_HOST_ADDRESSES_SOL, `\naddress constant ${solidityConstantName} = ${address};\n`);
}

/** Reads an address back from fhevmTemp/addresses/.env.host. */
function readHostAddress(envVarName: string): string {
  const parsed = dotenv.parse(fs.readFileSync(FHEVM_HOST_ADDRESSES_ENV));
  const value: string | undefined = Object.hasOwn(parsed, envVarName) ? parsed[envVarName] : undefined;
  if (value === undefined || !isAddress(value)) {
    throw new Error(`${envVarName} is missing from ${FHEVM_HOST_ADDRESSES_ENV}`);
  }
  return value;
}

////////////////////////////////////////////////////////////////////////////////
// Build helpers
////////////////////////////////////////////////////////////////////////////////

/**
 * Builds the given files (a partial build keeps every other artifact), or every source folder if `files` is empty
 * (a full build).
 */
async function buildFiles(hre: HardhatRuntimeEnvironment, files: string[]): Promise<void> {
  await hre.tasks.getTask('build').run({ files, quiet: true });
  // The artifact manager caches the artifact list: without this, the artifacts just built are not found
  // (HHE1000) by `ethers.getContractFactory` in this same process. Needed after every build, full or partial.
  await hre.artifacts.clearCache();
}

/** The .sol files of `dir` (absolute paths), including subfolders if `recursive`. */
function solidityFilesIn(dir: string, recursive: boolean): string[] {
  return fs
    .readdirSync(dir, { recursive })
    .map(String)
    .filter((file) => file.endsWith('.sol'))
    .map((file) => path.join(dir, file));
}

/** Builds only the given folder of fhevmTemp/contracts (not its subfolders). */
async function buildHostContractFolder(hre: HardhatRuntimeEnvironment, folder: string): Promise<void> {
  await buildFiles(hre, solidityFilesIn(path.join(FHEVM_TEMP_CONTRACTS_DIR, folder), false));
}

////////////////////////////////////////////////////////////////////////////////
// Phase 1: empty UUPS proxies and PauserSet (generates fhevmTemp/addresses/)
////////////////////////////////////////////////////////////////////////////////

/**
 * Deploys an empty UUPS proxy per host contract, then PauserSet, and writes their addresses to fhevmTemp/addresses/.
 * Same order as HH2's task:deployEmptyUUPSProxies + task:deployPauserSet (the addresses depend on it).
 */
export async function deployEmptyHostProxies(
  hre: HardhatRuntimeEnvironment,
  connection: NetworkConnection,
): Promise<void> {
  const { ethers } = connection;
  const upgradesApi = await upgrades(hre, connection);
  const deployer = getDeployer(connection);

  resetHostAddressFiles();

  // After each build, typechain generates the types of every artifact on disk, and puts them under the common root of
  // their paths. Build lib/ and config/ first (artifacts are only emitted for the files built as roots), so that root
  // is always the project root: otherwise, on a clean checkout, the first partial builds would generate types under
  // typechain-types/emptyProxy/… instead of typechain-types/fhevmTemp/contracts/emptyProxy/…, never cleaned up.
  await buildFiles(hre, [
    ...solidityFilesIn(path.join(LIBRARY_SOLIDITY_DIR, 'lib'), true),
    ...solidityFilesIn(path.join(LIBRARY_SOLIDITY_DIR, 'config'), true),
  ]);

  // ACL first: EmptyUUPSProxyACL does not import the addresses; the other empty proxies import aclAdd (ACLOwnable).
  await buildHostContractFolder(hre, 'emptyProxyACL');
  const emptyAclFactory = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const aclProxy = await upgradesApi.deployProxy(emptyAclFactory, [deployer.address], {
    initializer: 'initialize',
    kind: 'uups',
  });
  await aclProxy.waitForDeployment();
  appendHostAddress('ACL_CONTRACT_ADDRESS', 'aclAdd', await aclProxy.getAddress());

  await buildHostContractFolder(hre, 'emptyProxy');
  const emptyFactory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const emptyProxies: Array<[envVarName: string, solidityConstantName: string]> = [
    ['FHEVM_EXECUTOR_CONTRACT_ADDRESS', 'fhevmExecutorAdd'],
    ['KMS_VERIFIER_CONTRACT_ADDRESS', 'kmsVerifierAdd'],
    ['INPUT_VERIFIER_CONTRACT_ADDRESS', 'inputVerifierAdd'],
    ['HCU_LIMIT_CONTRACT_ADDRESS', 'hcuLimitAdd'],
    ['PROTOCOL_CONFIG_CONTRACT_ADDRESS', 'protocolConfigAdd'],
    // library-solidity is a canonical host: KMSGeneration is always deployed (HH2: --with-kms-generation true).
    ['KMS_GENERATION_CONTRACT_ADDRESS', 'kmsGenerationAdd'],
    ['CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS', 'confidentialBridgeAdd'],
  ];
  for (const [envVarName, solidityConstantName] of emptyProxies) {
    const proxy = await upgradesApi.deployProxy(emptyFactory, { initializer: 'initialize', kind: 'uups' });
    await proxy.waitForDeployment();
    appendHostAddress(envVarName, solidityConstantName, await proxy.getAddress());
  }

  await buildHostContractFolder(hre, 'immutable');
  const pauserSet = await (await ethers.getContractFactory('PauserSet', deployer)).deploy();
  await pauserSet.waitForDeployment();
  appendHostAddress('PAUSER_SET_CONTRACT_ADDRESS', 'pauserSetAdd', await pauserSet.getAddress());
}

/**
 * Prepares fhevmTemp/ and compiles everything: copies the host contracts, deploys the empty proxies on `connection`
 * (which writes fhevmTemp/addresses/), checks the addresses against config/ZamaConfig.sol, then runs one full build
 * (every source folder: lib, config, fhevmTemp/contracts, examples).
 */
export async function prepareAndCompileAll(
  hre: HardhatRuntimeEnvironment,
  connection: NetworkConnection,
): Promise<void> {
  copyHostContracts();
  await deployEmptyHostProxies(hre, connection);

  // Contracts using ZamaConfig must reach the deployed host contracts.
  const { chainId } = await connection.ethers.provider.getNetwork();
  assertHostAddressesMatchZamaConfig(FHEVM_HOST_ADDRESSES_SOL, chainId);

  // Full build. On a clean checkout, the real host contracts (e.g. ACL) are compiled here for the first time:
  // upgradeHostProxies, run next by task:prepareTestContracts, needs them.
  await buildFiles(hre, []);
}

////////////////////////////////////////////////////////////////////////////////
// Phase 2: upgrade each proxy to its real implementation
////////////////////////////////////////////////////////////////////////////////

type KmsNodeParams = {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
};

function buildKmsNodeParams(): KmsNodeParams[] {
  const nodes: KmsNodeParams[] = [];
  for (let idx = 0; idx < getIntegerEnvVar('NUM_KMS_NODES'); idx++) {
    nodes.push({
      txSenderAddress: getRequiredEnvVar(`KMS_TX_SENDER_ADDRESS_${idx}`),
      signerAddress: getRequiredEnvVar(`KMS_SIGNER_ADDRESS_${idx}`),
      ipAddress: getRequiredEnvVar(`KMS_NODE_IP_${idx}`),
      storageUrl: getRequiredEnvVar(`KMS_NODE_STORAGE_URL_${idx}`),
      partyId: getIntegerEnvVar(`KMS_NODE_PARTY_ID_${idx}`, idx),
      mpcIdentity: getRequiredEnvVar(`KMS_NODE_MPC_IDENTITY_${idx}`),
      caCert: getRequiredEnvVar(`KMS_NODE_CA_CERT_${idx}`),
      storagePrefix: getRequiredEnvVar(`KMS_NODE_STORAGE_PREFIX_${idx}`),
    });
  }
  return nodes;
}

function buildKmsThresholds(): { publicDecryption: number; userDecryption: number; kmsGen: number; mpc: number } {
  return {
    publicDecryption: getIntegerEnvVar('PUBLIC_DECRYPTION_THRESHOLD'),
    userDecryption: getIntegerEnvVar('USER_DECRYPTION_THRESHOLD'),
    kmsGen: getIntegerEnvVar('KMS_GEN_THRESHOLD'),
    mpc: getIntegerEnvVar('MPC_THRESHOLD'),
  };
}

function buildCoprocessorSigners(): string[] {
  const signers: string[] = [];
  for (let idx = 0; idx < getIntegerEnvVar('NUM_COPROCESSORS'); idx++) {
    signers.push(getRequiredEnvVar(`COPROCESSOR_SIGNER_ADDRESS_${idx}`));
  }
  return signers;
}

/**
 * Upgrades each empty proxy to its real host contract, in HH2's order (task:deployAllHostContracts). The real
 * contracts must be compiled with the addresses written by deployEmptyHostProxies (a full build).
 */
export async function upgradeHostProxies(hre: HardhatRuntimeEnvironment, connection: NetworkConnection): Promise<void> {
  const { ethers } = connection;
  const upgradesApi = await upgrades(hre, connection);
  const deployer = getDeployer(connection);
  const emptyAclFactory = await ethers.getContractFactory('EmptyUUPSProxyACL', deployer);
  const emptyFactory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);

  const upgradeToRealImplementation = async (
    envVarName: string,
    contractName: string,
    initializerArgs?: unknown[],
  ): Promise<void> => {
    const proxyAddress = readHostAddress(envVarName);
    const currentFactory = contractName === 'ACL' ? emptyAclFactory : emptyFactory;
    const proxy = await upgradesApi.forceImport(proxyAddress, currentFactory);
    const newFactory = await ethers.getContractFactory(contractName, deployer);
    await upgradesApi.upgradeProxy(
      proxy,
      newFactory,
      initializerArgs === undefined ? {} : { call: { fn: 'initializeFromEmptyProxy', args: initializerArgs } },
    );
  };

  const chainIdGateway = getIntegerEnvVar('CHAIN_ID_GATEWAY');
  await upgradeToRealImplementation('ACL_CONTRACT_ADDRESS', 'ACL');
  await upgradeToRealImplementation(
    'FHEVM_EXECUTOR_CONTRACT_ADDRESS',
    'fhevmTemp/contracts/FHEVMExecutor.sol:FHEVMExecutor',
  );
  await upgradeToRealImplementation(
    'PROTOCOL_CONFIG_CONTRACT_ADDRESS',
    'fhevmTemp/contracts/ProtocolConfig.sol:ProtocolConfig',
    [buildKmsNodeParams(), buildKmsThresholds(), process.env.KMS_SOFTWARE_VERSION ?? '', []],
  );
  await upgradeToRealImplementation(
    'KMS_GENERATION_CONTRACT_ADDRESS',
    'fhevmTemp/contracts/KMSGeneration.sol:KMSGeneration',
    [],
  );
  await upgradeToRealImplementation(
    'KMS_VERIFIER_CONTRACT_ADDRESS',
    'fhevmTemp/contracts/KMSVerifier.sol:KMSVerifier',
    [getRequiredEnvVar('DECRYPTION_ADDRESS'), chainIdGateway],
  );
  await upgradeToRealImplementation(
    'INPUT_VERIFIER_CONTRACT_ADDRESS',
    'fhevmTemp/contracts/InputVerifier.sol:InputVerifier',
    [
      getRequiredEnvVar('INPUT_VERIFICATION_ADDRESS'),
      chainIdGateway,
      buildCoprocessorSigners(),
      getRequiredEnvVar('COPROCESSOR_THRESHOLD'),
    ],
  );
  await upgradeToRealImplementation('HCU_LIMIT_CONTRACT_ADDRESS', 'HCULimit', [...HCU_LIMIT_INITIAL_VALUES]);
}

/** Adds the pausers of .env (NUM_PAUSERS, PAUSER_ADDRESS_<i>) to the deployed PauserSet (HH2: task:addHostPausers). */
export async function addHostPausers(connection: NetworkConnection): Promise<void> {
  const deployer = getDeployer(connection);
  const pauserSet = await connection.ethers.getContractAt(
    'PauserSet',
    readHostAddress('PAUSER_SET_CONTRACT_ADDRESS'),
    deployer,
  );
  for (let idx = 0; idx < getIntegerEnvVar('NUM_PAUSERS'); idx++) {
    await (await pauserSet.addPauser(getRequiredEnvVar(`PAUSER_ADDRESS_${idx}`))).wait();
  }
}
