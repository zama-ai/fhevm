// ProtocolConfig EIP-712 types and digest-input hashes (RFC 037 section 2), shared by the tests that sign
// confirmations.
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

function protocolConfigDomain(chainId: bigint, verifyingContract: string): TypedDataDomain {
  return { name: 'ProtocolConfig', version: '1', chainId, verifyingContract };
}

export async function protocolConfigDomainFor(provider: Provider, verifyingContract: string): Promise<TypedDataDomain> {
  return protocolConfigDomain((await provider.getNetwork()).chainId, verifyingContract);
}

const abi = AbiCoder.defaultAbiCoder();

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). KmsNode is the projection of
// KmsNodeParams that ProtocolConfig stores: (txSenderAddress, signerAddress, ipAddress, storageUrl).
export function nodeConfigHash(params: KmsNodeParams[], thresholds: KmsThresholds): string {
  return keccak256(
    abi.encode(
      ['tuple(address,address,string,string)[]', 'tuple(uint256,uint256,uint256,uint256)'],
      [
        params.map((p) => [p.txSenderAddress, p.signerAddress, p.ipAddress, p.storageUrl]),
        [thresholds.publicDecryption, thresholds.userDecryption, thresholds.kmsGen, thresholds.mpc],
      ],
    ),
  );
}

// keccak256(abi.encode(prepKeygenId, keyId, keyDigestsHash)), where keyDigestsHash is the EIP-712 hash of
// the KeyDigest[] array.
function keyHash(key: KeyResult): string {
  const keyDigestHashes = key.keyDigests.map((d) => TypedDataEncoder.hashStruct('KeyDigest', KEY_DIGEST_TYPES, d));
  const keyDigestsHash = keccak256(solidityPacked(['bytes32[]'], [keyDigestHashes]));
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes32'], [key.prepKeygenId, key.keyId, keyDigestsHash]));
}

// keccak256(abi.encode(crsId, maxBitLength, crsDigest)). Unlike keyHash, crsDigest is ABI-encoded as raw
// dynamic bytes, not hashed first.
function crsHash(crs: CrsResult): string {
  return keccak256(abi.encode(['uint256', 'uint256', 'bytes'], [crs.crsId, crs.maxBitLength, crs.crsDigest]));
}

export function epochMaterialHash(keys: KeyResult[], crsList: CrsResult[]): string {
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
