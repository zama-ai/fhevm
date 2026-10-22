// ProtocolConfig EIP-712 types, digest-input hashes (RFC 037 section 2) and confirmation loops. The
// host-contracts and library-solidity tests use them to sign and submit lifecycle confirmations.
import { AbiCoder, Signer, TypedDataDomain, TypedDataEncoder, TypedDataField, keccak256, solidityPacked } from 'ethers';

import type { ProtocolConfig } from '../../types';
import type {
  IProtocolConfig,
  KmsNodeParamsStruct,
  KmsThresholdsStruct,
  ProtocolConfigInterface,
} from '../../types/contracts/ProtocolConfig';

const CONFIRMATION_TYPES = {
  ContextCreationConfirmation: {
    ContextCreationConfirmation: [
      { name: 'previousContextId', type: 'uint256' },
      { name: 'newContextId', type: 'uint256' },
      { name: 'nodeConfigHash', type: 'bytes32' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
  EpochActivationConfirmation: {
    EpochActivationConfirmation: [
      { name: 'contextId', type: 'uint256' },
      { name: 'previousEpochId', type: 'uint256' },
      { name: 'epochId', type: 'uint256' },
      { name: 'epochMaterialHash', type: 'bytes32' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
  ContextDestructionConfirmation: {
    ContextDestructionConfirmation: [
      { name: 'destroyedContextId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
  EpochDestructionConfirmation: {
    EpochDestructionConfirmation: [
      { name: 'destroyedEpochId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  },
} satisfies Record<string, Record<string, TypedDataField[]>>;

type ConfirmationType = keyof typeof CONFIRMATION_TYPES;

type ConfirmationValue = {
  ContextCreationConfirmation: {
    previousContextId: bigint;
    newContextId: bigint;
    nodeConfigHash: string;
    extraData: string;
  };
  EpochActivationConfirmation: {
    contextId: bigint;
    previousEpochId: bigint;
    epochId: bigint;
    epochMaterialHash: string;
    extraData: string;
  };
  ContextDestructionConfirmation: { destroyedContextId: bigint; extraData: string };
  EpochDestructionConfirmation: { destroyedEpochId: bigint; extraData: string };
};

const KEY_DIGEST_TYPES: Record<string, TypedDataField[]> = {
  KeyDigest: [
    { name: 'keyType', type: 'uint8' },
    { name: 'digest', type: 'bytes' },
  ],
};

export async function protocolConfigDomain(pc: ProtocolConfig): Promise<TypedDataDomain> {
  const { chainId } = await pc.runner!.provider!.getNetwork();
  return { name: 'ProtocolConfig', version: '1', chainId, verifyingContract: await pc.getAddress() };
}

const abi = AbiCoder.defaultAbiCoder();

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). The encode types come from the
// ProtocolConfig ABI, so the tuple layout cannot drift from the contract. KmsNode is the projection of
// KmsNodeParams that ProtocolConfig stores, then `connectorUrls[i]` as node i's connectorUrl. The encoder
// drops the fields KmsNode does not have.
function nodeConfigHash(
  iface: ProtocolConfigInterface,
  params: KmsNodeParamsStruct[],
  connectorUrls: string[],
  thresholds: KmsThresholdsStruct,
): string {
  const [kmsNodesType] = iface.getFunction('getKmsNodesForContext').outputs;
  const [, , thresholdsType] = iface.getFunction('defineNewKmsContextAndEpoch').inputs;
  const nodes = params.map((p, i) => ({ ...p, connectorUrl: connectorUrls[i] }));
  return keccak256(abi.encode([kmsNodesType, thresholdsType], [nodes, thresholds]));
}

// keccak256(abi.encode(prepKeygenId, keyId, keyDigestsHash)), where keyDigestsHash is the EIP-712 hash of
// the KeyDigest[] array.
function keyHash(key: IProtocolConfig.EpochKeyResultStruct): string {
  const keyDigestHashes = key.keyDigests.map((d) => TypedDataEncoder.hashStruct('KeyDigest', KEY_DIGEST_TYPES, d));
  const keyDigestsHash = keccak256(solidityPacked(['bytes32[]'], [keyDigestHashes]));
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes32'], [key.prepKeygenId, key.keyId, keyDigestsHash]));
}

// keccak256(abi.encode(crsId, maxBitLength, crsDigest)). Unlike keyHash, crsDigest is ABI-encoded as raw
// dynamic bytes, not hashed first.
function crsHash(crs: IProtocolConfig.EpochCrsResultStruct): string {
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes'], [crs.crsId, crs.maxBitLength, crs.crsDigest]));
}

function epochMaterialHash(
  keys: IProtocolConfig.EpochKeyResultStruct[],
  crsList: IProtocolConfig.EpochCrsResultStruct[],
): string {
  return keccak256(abi.encode(['bytes32[]', 'bytes32[]'], [keys.map(keyHash), crsList.map(crsHash)]));
}

export function signConfirmation<T extends ConfirmationType>(
  signer: Signer,
  domain: TypedDataDomain,
  primaryType: T,
  value: ConfirmationValue[T],
): Promise<string> {
  return signer.signTypedData(domain, CONFIRMATION_TYPES[primaryType], value);
}

// Signs each signer's ContextCreationConfirmation for the pending context `contextId`, defined by `nodes`,
// `connectorUrls` and `thresholds`, and submits it from the signer account. The previous context is the active one.
// Returns the epoch ID of the NewKmsEpoch emitted once the creation quorum is reached.
export async function submitContextCreationConfirmations(
  pc: ProtocolConfig,
  signers: Signer[],
  contextId: bigint,
  nodes: KmsNodeParamsStruct[],
  connectorUrls: string[],
  thresholds: KmsThresholdsStruct,
  extraData = '0x',
): Promise<bigint | undefined> {
  const domain = await protocolConfigDomain(pc);
  const previousContextId = await pc.getCurrentKmsContextId();
  const configHash = nodeConfigHash(pc.interface, nodes, connectorUrls, thresholds);
  let epochId: bigint | undefined;
  for (const signer of signers) {
    const signature = await signConfirmation(signer, domain, 'ContextCreationConfirmation', {
      previousContextId,
      newContextId: contextId,
      nodeConfigHash: configHash,
      extraData,
    });
    const receipt = await (await pc.connect(signer).confirmKmsContextCreation(contextId, signature, extraData)).wait();
    epochId ??= findEventArg(pc, receipt!.logs, 'NewKmsEpoch', 'epochId');
  }
  return epochId;
}

// Signs each signer's EpochActivationConfirmation over `keys` and `crsList` and submits it from the signer
// account. `contextId` and `epochId` identify the epoch to activate. The previous epoch is the active one.
export async function submitEpochActivationConfirmations(
  pc: ProtocolConfig,
  signers: Signer[],
  contextId: bigint,
  epochId: bigint,
  keys: IProtocolConfig.EpochKeyResultStruct[],
  crsList: IProtocolConfig.EpochCrsResultStruct[],
  extraData = '0x',
): Promise<void> {
  const domain = await protocolConfigDomain(pc);
  const [, previousEpochId] = await pc.getCurrentKmsContextAndEpoch();
  const materialHash = epochMaterialHash(keys, crsList);
  for (const signer of signers) {
    const signature = await signConfirmation(signer, domain, 'EpochActivationConfirmation', {
      contextId,
      previousEpochId,
      epochId,
      epochMaterialHash: materialHash,
      extraData,
    });
    await (await pc.connect(signer).confirmEpochActivation(epochId, keys, crsList, signature, extraData)).wait();
  }
}

export function findEventArg(
  contract: ProtocolConfig,
  logs: readonly { topics: readonly string[]; data: string }[],
  eventName: string,
  argName: string,
): bigint | undefined {
  for (const log of logs) {
    const parsed = contract.interface.parseLog(log);
    if (parsed?.name === eventName) {
      return parsed.args[argName] as bigint;
    }
  }
  return undefined;
}
