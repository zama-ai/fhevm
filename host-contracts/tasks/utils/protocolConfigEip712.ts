// ProtocolConfig EIP-712 types, digest-input hashes (RFC 037 section 2) and confirmation loops. The
// host-contracts and library-solidity tests use them to sign and submit lifecycle confirmations.
import {
  AbiCoder,
  Provider,
  Signer,
  TypedDataDomain,
  TypedDataEncoder,
  TypedDataField,
  keccak256,
  solidityPacked,
} from 'ethers';

import type { ProtocolConfig } from '../../types';
import type {
  IProtocolConfig,
  KmsNodeParamsStruct,
  KmsThresholdsStruct,
  ProtocolConfigInterface,
} from '../../types/contracts/ProtocolConfig';

export const CONFIRMATION_TYPES = {
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

export type ConfirmationType = keyof typeof CONFIRMATION_TYPES;

export type ConfirmationValue = {
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

function protocolConfigDomain(chainId: bigint, verifyingContract: string): TypedDataDomain {
  return { name: 'ProtocolConfig', version: '1', chainId, verifyingContract };
}

export async function protocolConfigDomainFor(provider: Provider, verifyingContract: string): Promise<TypedDataDomain> {
  return protocolConfigDomain((await provider.getNetwork()).chainId, verifyingContract);
}

const abi = AbiCoder.defaultAbiCoder();

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). The encode types come off the
// ProtocolConfig ABI, so the tuple layout cannot drift from the contract. KmsNode is the projection of
// KmsNodeParams that ProtocolConfig stores, and the encoder drops the fields KmsNode does not have.
export function nodeConfigHash(
  iface: ProtocolConfigInterface,
  params: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
): string {
  const [kmsNodesType] = iface.getFunction('getKmsNodesForContext').outputs;
  const [, thresholdsType] = iface.getFunction('defineNewKmsContextAndEpoch').inputs;
  return keccak256(abi.encode([kmsNodesType, thresholdsType], [params, thresholds]));
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

export function epochMaterialHash(
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

// Signs each signer's ContextCreationConfirmation for the pending context `contextId`, defined by `nodes`
// and `thresholds`, and submits it from the signer account. The previous context is the active one.
// Returns the epoch ID of the NewKmsEpoch emitted once the creation quorum is reached.
export async function submitContextCreationConfirmations(
  pc: ProtocolConfig,
  signers: Signer[],
  contextId: bigint,
  nodes: KmsNodeParamsStruct[],
  thresholds: KmsThresholdsStruct,
  extraData = '0x',
): Promise<bigint | undefined> {
  const domain = await protocolConfigDomainFor(pc.runner!.provider!, await pc.getAddress());
  const previousContextId = await pc.getCurrentKmsContextId();
  let epochId: bigint | undefined;
  for (const signer of signers) {
    const signature = await signConfirmation(signer, domain, 'ContextCreationConfirmation', {
      previousContextId,
      newContextId: contextId,
      nodeConfigHash: nodeConfigHash(pc.interface, nodes, thresholds),
      extraData,
    });
    const receipt = await (await pc.connect(signer).confirmKmsContextCreation(contextId, signature, extraData)).wait();
    epochId ??= findEventArg(pc, receipt!.logs, 'NewKmsEpoch', 'epochId');
  }
  return epochId;
}

// Signs each signer's EpochActivationConfirmation over `keys` and `crsList` and submits it from the signer
// account. `contextId`/`epochId` are the pair being activated. The previous epoch is the active one.
export async function submitEpochActivationConfirmations(
  pc: ProtocolConfig,
  signers: Signer[],
  contextId: bigint,
  epochId: bigint,
  keys: IProtocolConfig.EpochKeyResultStruct[],
  crsList: IProtocolConfig.EpochCrsResultStruct[],
  extraData = '0x',
): Promise<void> {
  const domain = await protocolConfigDomainFor(pc.runner!.provider!, await pc.getAddress());
  const [, previousEpochId] = await pc.getCurrentKmsContextAndEpoch();
  for (const signer of signers) {
    const signature = await signConfirmation(signer, domain, 'EpochActivationConfirmation', {
      contextId,
      previousEpochId,
      epochId,
      epochMaterialHash: epochMaterialHash(keys, crsList),
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
  return undefined;
}
