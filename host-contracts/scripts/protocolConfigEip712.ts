// ProtocolConfig EIP-712 types and digest-input hashes (RFC 037 section 2), shared by the vectors generator
// and the tests that sign confirmations. It uses ethers only and never reads contract artifacts, so the
// vectors stay independent of the Solidity under test.
import {
  AbiCoder,
  BigNumberish,
  Provider,
  Signer,
  TypedDataDomain,
  TypedDataEncoder,
  TypedDataField,
  keccak256,
  solidityPacked,
} from 'ethers';

const EXTRA_DATA_V2 = 2;

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
      { name: 'destroyedEpochIds', type: 'uint256[]' },
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

export const KEYGEN_TYPES: Record<string, TypedDataField[]> = {
  KeygenVerification: [
    { name: 'prepKeygenId', type: 'uint256' },
    { name: 'keyId', type: 'uint256' },
    { name: 'keyDigests', type: 'KeyDigest[]' },
    { name: 'extraData', type: 'bytes' },
  ],
  KeyDigest: [
    { name: 'keyType', type: 'uint8' },
    { name: 'digest', type: 'bytes' },
  ],
};

export const CRSGEN_TYPES: Record<string, TypedDataField[]> = {
  CrsgenVerification: [
    { name: 'crsId', type: 'uint256' },
    { name: 'maxBitLength', type: 'uint256' },
    { name: 'crsDigest', type: 'bytes' },
    { name: 'extraData', type: 'bytes' },
  ],
};

export interface KmsNodeParams {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
}

export interface KmsThresholds {
  publicDecryption: BigNumberish;
  userDecryption: BigNumberish;
  kmsGen: BigNumberish;
  mpc: BigNumberish;
}

export interface KeyResult {
  prepKeygenId: bigint;
  keyId: bigint;
  keyDigests: { keyType: number; digest: string }[];
}

export interface CrsResult {
  crsId: bigint;
  maxBitLength: bigint;
  crsDigest: string;
}

export function protocolConfigDomain(chainId: bigint, verifyingContract: string): TypedDataDomain {
  return { name: 'ProtocolConfig', version: '1', chainId, verifyingContract };
}

export async function protocolConfigDomainFor(provider: Provider, verifyingContract: string): Promise<TypedDataDomain> {
  return protocolConfigDomain((await provider.getNetwork()).chainId, verifyingContract);
}

const abi = AbiCoder.defaultAbiCoder();

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). KmsNode is the projection of
// KmsNodeParams that ProtocolConfig stores: (txSenderAddress, signerAddress, ipAddress, storageUrl).
export function encodeNodeConfig(params: KmsNodeParams[], thresholds: KmsThresholds): string {
  return abi.encode(
    ['tuple(address,address,string,string)[]', 'tuple(uint256,uint256,uint256,uint256)'],
    [
      params.map((p) => [p.txSenderAddress, p.signerAddress, p.ipAddress, p.storageUrl]),
      [thresholds.publicDecryption, thresholds.userDecryption, thresholds.kmsGen, thresholds.mpc],
    ],
  );
}

export function nodeConfigHash(params: KmsNodeParams[], thresholds: KmsThresholds): string {
  return keccak256(encodeNodeConfig(params, thresholds));
}

// keccak256(abi.encode(prepKeygenId, keyId, keyDigestsHash)), where keyDigestsHash is the EIP-712 hash of
// the KeyDigest[] array.
export function keyHash(key: KeyResult): string {
  const keyDigestHashes = key.keyDigests.map((d) => TypedDataEncoder.hashStruct('KeyDigest', KEYGEN_TYPES, d));
  const keyDigestsHash = keccak256(solidityPacked(['bytes32[]'], [keyDigestHashes]));
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes32'], [key.prepKeygenId, key.keyId, keyDigestsHash]));
}

// keccak256(abi.encode(crsId, maxBitLength, crsDigest)). Unlike keyHash, crsDigest is ABI-encoded as raw
// dynamic bytes, not hashed first.
export function crsHash(crs: CrsResult): string {
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes'], [crs.crsId, crs.maxBitLength, crs.crsDigest]));
}

export function epochMaterialHash(keys: KeyResult[], crsList: CrsResult[]): string {
  return keccak256(abi.encode(['bytes32[]', 'bytes32[]'], [keys.map(keyHash), crsList.map(crsHash)]));
}

// Per-result extraData: abi.encodePacked(EXTRA_DATA_V2, contextId, epochId).
export function resultExtraData(contextId: bigint, epochId: bigint): string {
  return solidityPacked(['uint8', 'uint256', 'uint256'], [EXTRA_DATA_V2, contextId, epochId]);
}

// Signs each key and CRS result with the per-result extraData of (contextId, epochId), then signs the
// EpochActivationConfirmation over their epochMaterialHash. Returns the payload for confirmEpochActivation.
export async function signEpochActivation(
  signer: Signer,
  domain: TypedDataDomain,
  a: {
    contextId: bigint;
    previousEpochId: bigint;
    epochId: bigint;
    keys: KeyResult[];
    crsList: CrsResult[];
    extraData: string;
  },
) {
  const perResultExtraData = resultExtraData(a.contextId, a.epochId);
  const keys = await Promise.all(
    a.keys.map(async (k) => ({
      ...k,
      signature: await signer.signTypedData(domain, KEYGEN_TYPES, { ...k, extraData: perResultExtraData }),
    })),
  );
  const crsList = await Promise.all(
    a.crsList.map(async (c) => ({
      ...c,
      signature: await signer.signTypedData(domain, CRSGEN_TYPES, { ...c, extraData: perResultExtraData }),
    })),
  );
  const signature = await signer.signTypedData(domain, CONFIRMATION_TYPES.EpochActivationConfirmation, {
    contextId: a.contextId,
    previousEpochId: a.previousEpochId,
    epochId: a.epochId,
    epochMaterialHash: epochMaterialHash(a.keys, a.crsList),
    extraData: a.extraData,
  });
  return { keys, crsList, signature };
}
