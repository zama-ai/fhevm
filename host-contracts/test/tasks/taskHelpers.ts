import dotenv from 'dotenv';
import { Contract, Signer, Wallet } from 'ethers';
import fs from 'fs';
import { ethers, upgrades } from 'hardhat';
import path from 'path';

import type { KMSGeneration } from '../../types';
import type { KmsNodeParamsStruct, KmsThresholdsStruct } from '../../types/contracts/ProtocolConfig';
import { deployEmptyProxy } from '../utils/deploymentHelpers';
import {
  findEventArg,
  protocolConfigDomain,
  signConfirmation,
  submitContextCreationConfirmations,
  submitEpochActivationConfirmations,
} from '../utils/protocolConfigEip712';

export const HOST_ENV_FILE = path.join(__dirname, '../../addresses/.env.host');

export const HOST_ADDRESSES_SOL_FILE = path.join(__dirname, '../../addresses/FHEVMHostAddresses.sol');

export function readHostAddress(key: string): string {
  const value = dotenv.parse(fs.readFileSync(HOST_ENV_FILE))[key];
  if (!value) {
    throw new Error(`Missing ${key} in ${HOST_ENV_FILE}`);
  }
  return value;
}

export function buildProtocolConfigNodes(): Array<{
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
}> {
  return [
    {
      txSenderAddress: '0x0000000000000000000000000000000000001111',
      signerAddress: '0x0000000000000000000000000000000000002222',
      ipAddress: '127.0.0.1',
      storageUrl: 'https://s0.example.com',
      partyId: 0,
      mpcIdentity: '127.0.0.1',
      caCert: '0x',
      storagePrefix: '',
    },
    {
      txSenderAddress: '0x0000000000000000000000000000000000003333',
      signerAddress: '0x0000000000000000000000000000000000004444',
      ipAddress: '127.0.0.2',
      storageUrl: 'https://s1.example.com',
      partyId: 1,
      mpcIdentity: '127.0.0.2',
      caCert: '0x',
      storagePrefix: '',
    },
    {
      txSenderAddress: '0x0000000000000000000000000000000000005555',
      signerAddress: '0x0000000000000000000000000000000000006666',
      ipAddress: '127.0.0.3',
      storageUrl: 'https://s2.example.com',
      partyId: 2,
      mpcIdentity: '127.0.0.3',
      caCert: '0x',
      storagePrefix: '',
    },
    {
      txSenderAddress: '0x0000000000000000000000000000000000007777',
      signerAddress: '0x0000000000000000000000000000000000008888',
      ipAddress: '127.0.0.4',
      storageUrl: 'https://s3.example.com',
      partyId: 3,
      mpcIdentity: '127.0.0.4',
      caCert: '0x',
      storagePrefix: '',
    },
  ];
}

export function buildProtocolConfigThresholds() {
  return {
    publicDecryption: 1,
    userDecryption: 2,
    kmsGen: 3,
    mpc: 4,
  };
}

async function upgradeEmptyProxy(
  proxyAddress: string,
  deployer: Wallet,
  contractName: string,
  opts?: { call: { fn: string; args?: unknown[] } },
) {
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory(contractName, deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const upgraded = await upgrades.upgradeProxy(proxy, newImplementation, opts);
  await upgraded.waitForDeployment();
  return upgraded;
}

export async function deployFreshKMSGenerationProxy(deployer: Wallet): Promise<KMSGeneration> {
  const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
  await upgradeEmptyProxy(proxyAddress, deployer, 'KMSGeneration', { call: { fn: 'initializeFromEmptyProxy' } });
  return (await ethers.getContractAt('KMSGeneration', proxyAddress, deployer)) as unknown as KMSGeneration;
}

export async function deployFreshEmptyUUPSProxy(deployer: Wallet): Promise<string> {
  const emptyProxyFactory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  return await deployEmptyProxy(emptyProxyFactory);
}

// Upgrades to the ProtocolConfig implementation WITHOUT calling an initializer: getVersion()
// passes the identity check but no KMS context exists (currentKmsContextId=0).
export async function deployFreshUninitializedProtocolConfigProxy(deployer: Wallet): Promise<string> {
  const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
  await upgradeEmptyProxy(proxyAddress, deployer, 'ProtocolConfig');
  return proxyAddress;
}

// Upgrades an existing EmptyUUPSProxy at `proxyAddress` to the ProtocolConfig implementation and runs
// `initializeFromEmptyProxy` with the given KMS committee, returning the initialized contract instance.
export async function initializeProtocolConfigProxy(
  proxyAddress: string,
  deployer: Wallet,
  kmsNodes: Array<{ txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }>,
  thresholds: KmsThresholdsStruct,
): Promise<Contract> {
  const upgraded = await upgradeEmptyProxy(proxyAddress, deployer, 'ProtocolConfig', {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [kmsNodes, thresholds, '', []],
    },
  });
  return upgraded as unknown as Contract;
}

export async function deployFreshProtocolConfigProxy(
  deployer: Wallet,
  kmsNodes: Array<{ txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }>,
  thresholds: KmsThresholdsStruct,
): Promise<string> {
  const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
  await initializeProtocolConfigProxy(proxyAddress, deployer, kmsNodes, thresholds);
  return proxyAddress;
}

export async function deployFreshProtocolConfigReplicaProxy(
  deployer: Wallet,
  contextId: bigint,
  epochId: bigint,
  kmsNodes: Array<{ txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }>,
  thresholds: KmsThresholdsStruct,
): Promise<string> {
  const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
  await upgradeEmptyProxy(proxyAddress, deployer, 'ProtocolConfigReplica', {
    call: { fn: 'initializeFromCanonical', args: [contextId, epochId, kmsNodes, thresholds] },
  });
  return proxyAddress;
}

// A KMS committee whose signer addresses are backed by funded Hardhat accounts, so the signed
// lifecycle confirmations can be produced from a test. Anyone may submit them.
export interface ControllableKmsCommittee {
  nodes: ReturnType<typeof buildProtocolConfigNodes>;
  thresholds: KmsThresholdsStruct;
  signerSigners: Signer[];
}

// Builds a two-node committee from distinct funded accounts (skipping account 0, which is typically
// the deployer). Each node uses one account as its tx-sender and another as its signer.
export async function buildControllableKmsCommittee(): Promise<ControllableKmsCommittee> {
  const accounts = await ethers.getSigners();
  const [txSender0, signer0, txSender1, signer1] = accounts.slice(1, 5);
  const node = (txSenderSigner: Signer, signerSigner: Signer, index: number) => ({
    txSenderAddress: (txSenderSigner as unknown as { address: string }).address,
    signerAddress: (signerSigner as unknown as { address: string }).address,
    ipAddress: `127.0.0.${index + 1}`,
    storageUrl: `https://committee-s${index}.example.com`,
    partyId: index,
    mpcIdentity: `127.0.0.${index + 1}`,
    caCert: '0x',
    storagePrefix: '',
  });
  return {
    nodes: [node(txSender0, signer0, 0), node(txSender1, signer1, 1)],
    // Reusing the same committee for the rotated context satisfies both creation-quorum sides
    // (all new signers and n - t previous signers) with the same two confirmations.
    thresholds: { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 },
    signerSigners: [signer0, signer1],
  };
}

// Submits each signer's ContextCreationConfirmation for a pending context. `nodes`/`thresholds` are the
// pending context's definition. Returns the epoch ID of the NewKmsEpoch emitted once the creation quorum
// is reached.
export async function confirmContextCreationBySigners(
  proxyAddress: string,
  signerSigners: Signer[],
  contextId: bigint,
  nodes: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
  extraData = '0x',
): Promise<bigint | undefined> {
  const pc = await ethers.getContractAt('ProtocolConfig', proxyAddress);
  return submitContextCreationConfirmations(pc, signerSigners, contextId, nodes, thresholds, extraData);
}

// Submits each signer's EpochActivationConfirmation over one key result and one CRS result.
// Both arrays must be non-empty, so a key-only payload is rejected. The material is identical across
// signers, so every signer produces the same epochMaterialHash and quorum is reachable.
export async function confirmEpochActivationBySigners(
  proxyAddress: string,
  signerSigners: Signer[],
  contextId: bigint,
  epochId: bigint,
  extraData = '0x',
): Promise<void> {
  const pc = await ethers.getContractAt('ProtocolConfig', proxyAddress);
  const keys = [{ prepKeygenId: 1n, keyId: 1n, keyDigests: [{ keyType: 0, digest: '0x01020304' }] }];
  const crsList = [{ crsId: 1n, maxBitLength: 4096n, crsDigest: '0x01020304' }];
  await submitEpochActivationConfirmations(pc, signerSigners, contextId, epochId, keys, crsList, extraData);
}

// Signs the signer's ContextDestructionConfirmation and submits it from the signer account.
export async function confirmContextDestructionBySigner(
  proxyAddress: string,
  signerSigner: Signer,
  destroyedContextId: bigint,
): Promise<void> {
  const pc = await ethers.getContractAt('ProtocolConfig', proxyAddress, signerSigner);
  const signature = await signConfirmation(
    signerSigner,
    await protocolConfigDomain(pc),
    'ContextDestructionConfirmation',
    {
      destroyedContextId,
      extraData: '0x',
    },
  );
  await (await pc.confirmKmsContextDestruction(destroyedContextId, signature, '0x')).wait();
}

// Signs the signer's EpochDestructionConfirmation and submits it from the signer account.
export async function confirmEpochDestructionBySigner(
  proxyAddress: string,
  signerSigner: Signer,
  destroyedEpochId: bigint,
): Promise<void> {
  const pc = await ethers.getContractAt('ProtocolConfig', proxyAddress, signerSigner);
  const signature = await signConfirmation(
    signerSigner,
    await protocolConfigDomain(pc),
    'EpochDestructionConfirmation',
    {
      destroyedEpochId,
      extraData: '0x',
    },
  );
  await (await pc.confirmKmsEpochDestruction(destroyedEpochId, signature, '0x')).wait();
}

// Rotates the canonical ProtocolConfig to a fresh KMS context that reuses `committee`, driving the full
// epoch lifecycle (define -> confirm creation -> confirm activation) so getCurrentKmsContextId advances.
// Activation requires a non-empty key and CRS array, so each signer submits one result of each.
export async function rotateToNewKmsContext(
  proxyAddress: string,
  ownerSigner: Signer,
  committee: ControllableKmsCommittee,
): Promise<bigint> {
  const asOwner = await ethers.getContractAt('ProtocolConfig', proxyAddress, ownerSigner);
  const defineTx = await asOwner.defineNewKmsContextAndEpoch(committee.nodes, committee.thresholds, '', []);
  const defineReceipt = await defineTx.wait();
  const contextId = findEventArg(asOwner, defineReceipt!.logs, 'NewKmsContext', 'contextId');
  if (contextId === undefined) {
    throw new Error('NewKmsContext not emitted');
  }

  const epochId = await confirmContextCreationBySigners(
    proxyAddress,
    committee.signerSigners,
    contextId,
    committee.nodes,
    committee.thresholds,
  );
  if (epochId === undefined) {
    throw new Error('Context creation quorum did not emit NewKmsEpoch');
  }
  await confirmEpochActivationBySigners(proxyAddress, committee.signerSigners, contextId, epochId);

  return contextId;
}
