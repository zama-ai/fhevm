// Generates the ProtocolConfig EIP-712 test vectors (RFC 037 section 2) that KMS Core reproduces.
// It uses ethers only and never reads contract artifacts, so it stays independent of the Solidity under
// test. test/protocolConfig/protocolConfigVectors.t.sol replays the scenario against the contract, and
// test/protocolConfig/vectors.ts checks that the checked-in JSON matches this generator.
//
// Run: npx ts-node scripts/generateProtocolConfigVectors.ts
import {
  AbiCoder,
  TypedDataDomain,
  TypedDataEncoder,
  TypedDataField,
  Wallet,
  id,
  keccak256,
  solidityPacked,
} from 'ethers';
import fs from 'fs';
import path from 'path';

export const VECTORS_PATH = path.join(__dirname, '..', 'test-vectors', 'protocol-config-eip712.json');

const KMS_CONTEXT_COUNTER_BASE = 7n << 248n;
const EPOCH_COUNTER_BASE = 8n << 248n;
const PREP_KEYGEN_COUNTER_BASE = 3n << 248n;
const KEY_COUNTER_BASE = 4n << 248n;
const CRS_COUNTER_BASE = 5n << 248n;
const EXTRA_DATA_V2 = 2;

export const CONFIRMATION_TYPES: Record<string, Record<string, TypedDataField[]>> = {
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
};

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
  publicDecryption: bigint;
  userDecryption: bigint;
  kmsGen: bigint;
  mpc: bigint;
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

const abi = AbiCoder.defaultAbiCoder();

// keccak256(abi.encode(KmsNode[] nodes, KmsThresholds thresholds)). KmsNode is the projection of
// KmsNodeParams that ProtocolConfig stores: (txSenderAddress, signerAddress, ipAddress, storageUrl).
function encodeNodeConfig(params: KmsNodeParams[], thresholds: KmsThresholds): string {
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

// Deterministic (RFC 6979) 65-byte r || s || v signature, the same layout as foundry's vm.sign packed.
function sign(wallet: Wallet, digest: string): string {
  return wallet.signingKey.sign(digest).serialized;
}

// JSON-safe copy: bigints become decimal strings.
function json<T>(value: T): unknown {
  return JSON.parse(JSON.stringify(value, (_, v) => (typeof v === 'bigint' ? v.toString() : v)));
}

export function generateVectors(): unknown {
  const domain: TypedDataDomain = {
    name: 'ProtocolConfig',
    version: '1',
    chainId: 12345n,
    verifyingContract: '0x00000000000000000000000000000000C0FFEE01',
  };
  const wallets = [1, 2, 3, 4, 5, 6, 7].map(
    (i) => new Wallet('0x' + (BigInt(i) * 0x100n).toString(16).padStart(64, '0')),
  );
  const node = (wallet: Wallet, i: number): KmsNodeParams => ({
    txSenderAddress: '0x' + (0xa1 + i).toString(16).padStart(40, '0'),
    signerAddress: wallet.address,
    ipAddress: `127.0.0.${i + 1}`,
    storageUrl: `https://s${i}.example.com`,
    partyId: i,
    mpcIdentity: `127.0.0.${i + 1}`,
    caCert: '0x',
    storagePrefix: `kms/node${i}`,
  });
  // n = 4, t = 1 satisfies n >= 3t + 1. The previous-committee creation quorum is n - t = 3.
  const thresholds: KmsThresholds = { publicDecryption: 1n, userDecryption: 1n, kmsGen: 1n, mpc: 1n };
  const genesisNodes = wallets.slice(0, 4).map(node);
  // Switch A is defined then destroyed, so switch B leaves a gap in the context ids.
  const switchANodes = wallets.slice(4, 7).map((w, i) => node(w, i + 4));
  // Switch B shares wallets[3] with the genesis committee: one signature counts for both committees.
  const switchBNodes = wallets.slice(3, 7).map((w, i) => node(w, i + 3));

  const genesisContextId = KMS_CONTEXT_COUNTER_BASE + 1n;
  const genesisEpochId = EPOCH_COUNTER_BASE + 1n;
  const switchAContextId = KMS_CONTEXT_COUNTER_BASE + 2n;
  const switchBContextId = KMS_CONTEXT_COUNTER_BASE + 3n;
  const switchBEpochId = EPOCH_COUNTER_BASE + 2n;
  // Resharing epoch destroyed while Pending, then the resharing that activates (gap in the epoch ids).
  const abortedResharingEpochId = EPOCH_COUNTER_BASE + 3n;
  const resharingEpochId = EPOCH_COUNTER_BASE + 4n;

  const nonEmptyExtraData = '0x01abcdef';

  const keys: KeyResult[] = [
    {
      prepKeygenId: PREP_KEYGEN_COUNTER_BASE + 1n,
      keyId: KEY_COUNTER_BASE + 1n,
      keyDigests: [
        { keyType: 0, digest: '0xaabbccdd' },
        { keyType: 1, digest: '0x11223344' },
      ],
    },
  ];
  const crsList: CrsResult[] = [{ crsId: CRS_COUNTER_BASE + 1n, maxBitLength: 4096n, crsDigest: '0xdeadbeef' }];

  const message = (primaryType: string, wallet: Wallet, fields: Record<string, unknown>) => {
    const types = CONFIRMATION_TYPES[primaryType];
    const digest = TypedDataEncoder.hash(domain, types, fields);
    const typeString = TypedDataEncoder.from(types).encodeType(primaryType);
    return {
      typeString,
      typehash: id(typeString),
      fields,
      structHash: TypedDataEncoder.hashStruct(primaryType, types, fields),
      digest,
      signer: wallet.address,
      signature: sign(wallet, digest),
    };
  };

  const perResult: unknown[] = [];
  const epochActivation = (wallet: Wallet, contextId: bigint, previousEpochId: bigint, epochId: bigint) => {
    const extraData = resultExtraData(contextId, epochId);
    const keySignatures = keys.map((key) => {
      const fields = { ...key, extraData };
      const digest = TypedDataEncoder.hash(domain, KEYGEN_TYPES, fields);
      const signature = sign(wallet, digest);
      perResult.push({
        type: 'KeygenVerification',
        fields,
        structHash: TypedDataEncoder.hashStruct('KeygenVerification', KEYGEN_TYPES, fields),
        digest,
        signer: wallet.address,
        signature,
      });
      return signature;
    });
    const crsSignatures = crsList.map((crs) => {
      const fields = { ...crs, extraData };
      const digest = TypedDataEncoder.hash(domain, CRSGEN_TYPES, fields);
      const signature = sign(wallet, digest);
      perResult.push({
        type: 'CrsgenVerification',
        fields,
        structHash: TypedDataEncoder.hashStruct('CrsgenVerification', CRSGEN_TYPES, fields),
        digest,
        signer: wallet.address,
        signature,
      });
      return signature;
    });
    return {
      ...message('EpochActivationConfirmation', wallet, {
        contextId,
        previousEpochId,
        epochId,
        epochMaterialHash: epochMaterialHash(keys, crsList),
        extraData: '0x',
      }),
      keySignatures,
      crsSignatures,
    };
  };

  const switchBNodeConfigHash = nodeConfigHash(switchBNodes, thresholds);

  return json({
    domain: { ...domain, separator: TypedDataEncoder.hashDomain(domain) },
    signers: wallets.map((w) => ({ privateKey: w.privateKey, address: w.address })),
    scenario: {
      thresholds,
      genesis: { contextId: genesisContextId, epochId: genesisEpochId, kmsNodeParams: genesisNodes },
      // 1. define switch A, 2. destroyKmsContext(A) and contextDestruction confirmations,
      // 3. define switch B and contextCreation confirmations (NewKmsEpoch for switchB.epochId),
      // 4. epochActivation of switchB.epochId, 5. defineNewEpochForCurrentKmsContext then
      // destroyKmsEpoch(abortedResharingEpochId), 6. defineNewEpochForCurrentKmsContext and epochActivation
      // of resharingEpochId, 7. destroyKmsEpoch(switchB.epochId), 8. epochDestruction confirmations.
      switchA: { contextId: switchAContextId, kmsNodeParams: switchANodes },
      switchB: { contextId: switchBContextId, epochId: switchBEpochId, kmsNodeParams: switchBNodes },
      abortedResharingEpochId,
      resharingEpochId,
    },
    nodeConfigHash: [
      {
        kmsNodeParams: switchBNodes,
        nodes: switchBNodes.map((p) => ({
          txSenderAddress: p.txSenderAddress,
          signerAddress: p.signerAddress,
          ipAddress: p.ipAddress,
          storageUrl: p.storageUrl,
        })),
        thresholds,
        encoded: encodeNodeConfig(switchBNodes, thresholds),
        hash: switchBNodeConfigHash,
      },
    ],
    epochMaterialHash: [
      {
        keys,
        crs: crsList,
        keyHashes: keys.map(keyHash),
        crsHashes: crsList.map(crsHash),
        hash: epochMaterialHash(keys, crsList),
      },
    ],
    messages: {
      // Signed by the genesis committee, active when switch A is destroyed. The canonical does not check
      // destroyedEpochIds, so the two-id list is valid digest material.
      contextDestruction: [
        message('ContextDestructionConfirmation', wallets[0], {
          destroyedContextId: switchAContextId,
          destroyedEpochIds: [],
          extraData: '0x',
        }),
        message('ContextDestructionConfirmation', wallets[1], {
          destroyedContextId: switchAContextId,
          destroyedEpochIds: [EPOCH_COUNTER_BASE + 10n, EPOCH_COUNTER_BASE + 11n],
          extraData: nonEmptyExtraData,
        }),
      ],
      // Previous side: wallets[0], wallets[1], wallets[3] (3 = n - t). New side: wallets[3..6] (all 4).
      // wallets[3] is in both committees and confirms last, completing both sides at once.
      contextCreation: [0, 1, 4, 5, 6, 3].map((i) =>
        message('ContextCreationConfirmation', wallets[i], {
          previousContextId: genesisContextId,
          newContextId: switchBContextId,
          nodeConfigHash: switchBNodeConfigHash,
          extraData: '0x',
        }),
      ),
      epochActivation: [
        ...[3, 4, 5, 6].map((i) => epochActivation(wallets[i], switchBContextId, genesisEpochId, switchBEpochId)),
        ...[3, 4, 5, 6].map((i) => epochActivation(wallets[i], switchBContextId, switchBEpochId, resharingEpochId)),
      ],
      // Signed by the switch B committee, active after step 4.
      epochDestruction: [
        message('EpochDestructionConfirmation', wallets[3], {
          destroyedEpochId: abortedResharingEpochId,
          extraData: '0x',
        }),
        message('EpochDestructionConfirmation', wallets[4], {
          destroyedEpochId: switchBEpochId,
          extraData: nonEmptyExtraData,
        }),
      ],
    },
    perResult,
  });
}

if (require.main === module) {
  fs.mkdirSync(path.dirname(VECTORS_PATH), { recursive: true });
  fs.writeFileSync(VECTORS_PATH, JSON.stringify(generateVectors(), null, 2) + '\n');
  console.log(`Wrote ${VECTORS_PATH}`);
}
