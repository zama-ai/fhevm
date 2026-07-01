import dotenv from 'dotenv';
import { Contract, Signer, Wallet } from 'ethers';
import fs from 'fs';
import { ethers, upgrades } from 'hardhat';
import path from 'path';

import type { KMSGeneration, ProtocolConfig } from '../../types';
import { deployEmptyProxy } from '../utils/deploymentHelpers';

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

export async function deployFreshKMSGenerationProxy(deployer: Wallet): Promise<KMSGeneration> {
  const emptyProxyFactory = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const proxyAddress = await deployEmptyProxy(emptyProxyFactory);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory('KMSGeneration', deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);

  const upgraded = await upgrades.upgradeProxy(proxy, newImplementation, {
    call: { fn: 'initializeFromEmptyProxy' },
  });
  await upgraded.waitForDeployment();

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
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory('ProtocolConfig', deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const upgraded = await upgrades.upgradeProxy(proxy, newImplementation);
  await upgraded.waitForDeployment();
  return proxyAddress;
}

// Upgrades an existing EmptyUUPSProxy at `proxyAddress` to the ProtocolConfig implementation and runs
// `initializeFromEmptyProxy` with the given KMS committee, returning the initialized contract instance.
export async function initializeProtocolConfigProxy(
  proxyAddress: string,
  deployer: Wallet,
  kmsNodes: Array<{ txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }>,
  thresholds: { publicDecryption: number; userDecryption: number; kmsGen: number; mpc: number },
): Promise<Contract> {
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplementation = await ethers.getContractFactory('ProtocolConfig', deployer);
  const proxy = await upgrades.forceImport(proxyAddress, currentImplementation);
  const upgraded = await upgrades.upgradeProxy(proxy, newImplementation, {
    call: {
      fn: 'initializeFromEmptyProxy',
      args: [kmsNodes, thresholds, '', []],
    },
  });
  await upgraded.waitForDeployment();
  return upgraded as unknown as Contract;
}

export async function deployFreshProtocolConfigProxy(
  deployer: Wallet,
  kmsNodes: Array<{ txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }>,
  thresholds: { publicDecryption: number; userDecryption: number; kmsGen: number; mpc: number },
): Promise<string> {
  const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
  await initializeProtocolConfigProxy(proxyAddress, deployer, kmsNodes, thresholds);
  return proxyAddress;
}

// A KMS committee whose tx-sender and signer addresses are backed by funded Hardhat accounts, so the
// epoch-lifecycle confirmation steps (which are sent by those addresses) can be driven from a test.
export interface ControllableKmsCommittee {
  nodes: Array<{
    txSenderAddress: string;
    signerAddress: string;
    ipAddress: string;
    storageUrl: string;
    partyId: number;
    mpcIdentity: string;
    caCert: string;
    storagePrefix: string;
  }>;
  thresholds: { publicDecryption: number; userDecryption: number; kmsGen: number; mpc: number };
  signerSigners: Signer[];
  txSenderSigners: Signer[];
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
    // mpc threshold 1 keeps the previous-signer creation quorum at the full set, satisfied by reusing
    // the same committee for the rotated context.
    thresholds: { publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1 },
    signerSigners: [signer0, signer1],
    txSenderSigners: [txSender0, txSender1],
  };
}

function findEventArg(
  contract: ProtocolConfig,
  logs: readonly { topics: string[]; data: string }[],
  eventName: string,
  argName: string,
): bigint {
  for (const log of logs) {
    let parsed;
    try {
      parsed = contract.interface.parseLog({ topics: [...log.topics], data: log.data });
    } catch {
      continue;
    }
    if (parsed?.name === eventName) {
      return parsed.args[argName] as bigint;
    }
  }
  throw new Error(`Event ${eventName} not found in transaction logs`);
}

// Rotates the canonical ProtocolConfig to a fresh KMS context that reuses `committee`, driving the full
// epoch lifecycle (define -> confirm creation -> confirm activation) so getCurrentKmsContextId advances.
// Empty key/CRS material is sufficient for a context switch; only same-set resharing needs attestations.
export async function rotateToNewKmsContext(
  proxyAddress: string,
  ownerSigner: Signer,
  committee: ControllableKmsCommittee,
): Promise<bigint> {
  const asOwner = (await ethers.getContractAt(
    'ProtocolConfig',
    proxyAddress,
    ownerSigner,
  )) as unknown as ProtocolConfig;
  const defineTx = await asOwner.defineNewKmsContextAndEpoch(committee.nodes, committee.thresholds, '', []);
  const defineReceipt = await defineTx.wait();
  const contextId = findEventArg(asOwner, defineReceipt!.logs, 'NewKmsContext', 'contextId');

  let epochId: bigint | undefined;
  for (const signerSigner of committee.signerSigners) {
    const asSigner = (await ethers.getContractAt(
      'ProtocolConfig',
      proxyAddress,
      signerSigner,
    )) as unknown as ProtocolConfig;
    const receipt = await (await asSigner.confirmKmsContextCreation(contextId)).wait();
    try {
      epochId = findEventArg(asSigner, receipt!.logs, 'NewKmsEpoch', 'epochId');
    } catch {
      // NewKmsEpoch is only emitted once the creation quorum is reached.
    }
  }
  if (epochId === undefined) {
    throw new Error('Context creation quorum did not emit NewKmsEpoch');
  }

  for (const txSenderSigner of committee.txSenderSigners) {
    const asTxSender = (await ethers.getContractAt(
      'ProtocolConfig',
      proxyAddress,
      txSenderSigner,
    )) as unknown as ProtocolConfig;
    await (await asTxSender.confirmEpochActivation(epochId, [], [])).wait();
  }

  return contextId;
}
