import type { ReadContractParameters } from '../modules/ethereum/types.js';
import type { FheEncryptionKeyDigests } from '../types/fheEncryptionKey.js';
import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { ConfiguredFheEncryptionKeyTrust } from '../chains/configuredFheEncryptionKeyTrust-p.js';
import { asUint64BigInt, asUint256BigInt, isUint256 } from '../base/uint.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { normalizeFheEncryptionKeyDigests } from '../key/authenticateFheEncryptionKeyBytes.js';

////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyTrustSnapshot = FheEncryptionKeyDigests & {
  readonly publicKeyId: Uint256BigInt;
  readonly crsId: Uint256BigInt;
};

type ReadContract = (parameters: ReadContractParameters) => Promise<unknown>;

const KMS_GENERATION_READ_BLOCK_TAG = 'finalized';

/** Minimal authenticated host capability needed to acquire a fresh trust snapshot. */
export type FheEncryptionKeyTrustReader = {
  readonly getChainId: () => Promise<unknown>;
  readonly readContract: ReadContract;
};

/**
 * Reads a fresh, internally consistent active-key snapshot from KMSGeneration.
 *
 * This intentionally performs the chain-ID check both before and after the six
 * contract reads, and confirms both active IDs after reading their material. It
 * does not cache snapshots: every caller observes rotation before cache lookup.
 */
export async function readFheEncryptionKeyTrustSnapshot(
  trust: ConfiguredFheEncryptionKeyTrust,
  reader: FheEncryptionKeyTrustReader,
): Promise<FheEncryptionKeyTrustSnapshot> {
  try {
    await assertConnectedChainId(trust, reader);

    const [keyId, crsId] = await Promise.all([
      readActiveId(trust.kmsGenerationAddress, getActiveKeyIdAbi, reader.readContract),
      readActiveId(trust.kmsGenerationAddress, getActiveCrsIdAbi, reader.readContract),
    ]);

    const [keyMaterials, crsMaterials] = await Promise.all([
      reader.readContract({
        address: trust.kmsGenerationAddress,
        abi: getKeyMaterialsAbi,
        args: [keyId],
        blockTag: KMS_GENERATION_READ_BLOCK_TAG,
        functionName: getKeyMaterialsAbi[0].name,
      }),
      reader.readContract({
        address: trust.kmsGenerationAddress,
        abi: getCrsMaterialsAbi,
        args: [crsId],
        blockTag: KMS_GENERATION_READ_BLOCK_TAG,
        functionName: getCrsMaterialsAbi[0].name,
      }),
    ]);

    await assertActiveIds(trust, reader, { publicKeyId: keyId, crsId });
    await assertConnectedChainId(trust, reader);

    const digests = normalizeFheEncryptionKeyDigests({
      publicKeyDigest: readPublicKeyDigest(keyMaterials),
      crsDigest: readCrsDigest(crsMaterials),
    });
    return Object.freeze({
      publicKeyId: keyId,
      crsId,
      publicKeyDigest: digests.publicKeyDigest,
      crsDigest: digests.crsDigest,
    });
  } catch (cause) {
    throw new FhevmConfigError({
      message:
        `Unable to authenticate the active FHE public key and CRS from chain-configured trust material ` +
        `on chain ${trust.chainId.toString()} at KMSGeneration ${trust.kmsGenerationAddress}.`,
      cause: cause instanceof Error ? cause : new Error(String(cause)),
    });
  }
}

export async function assertFheEncryptionKeyTrustSnapshotStillActive(
  trust: ConfiguredFheEncryptionKeyTrust,
  reader: FheEncryptionKeyTrustReader,
  snapshot: FheEncryptionKeyTrustSnapshot,
): Promise<void> {
  try {
    await assertConnectedChainId(trust, reader);
    await assertActiveIds(trust, reader, snapshot);
    await assertConnectedChainId(trust, reader);
  } catch (cause) {
    throw new FhevmConfigError({
      message:
        `Unable to confirm the active FHE public key and CRS remained current before cache admission ` +
        `on chain ${trust.chainId.toString()} at KMSGeneration ${trust.kmsGenerationAddress}.`,
      cause: cause instanceof Error ? cause : new Error(String(cause)),
    });
  }
}

async function assertConnectedChainId(
  trust: ConfiguredFheEncryptionKeyTrust,
  reader: FheEncryptionKeyTrustReader,
): Promise<void> {
  const connectedChainId = asUint64BigInt(await reader.getChainId(), {
    subject: 'FHE encryption-key trust chain ID',
  });
  if (connectedChainId !== trust.chainId) {
    throw new Error(
      `Configured KMSGeneration FHE encryption-key trust must be read on chain ${trust.chainId.toString()}, ` +
        `got ${connectedChainId.toString()}.`,
    );
  }
}

async function assertActiveIds(
  trust: ConfiguredFheEncryptionKeyTrust,
  reader: FheEncryptionKeyTrustReader,
  snapshot: Pick<FheEncryptionKeyTrustSnapshot, 'publicKeyId' | 'crsId'>,
): Promise<void> {
  const [confirmedKeyId, confirmedCrsId] = await Promise.all([
    readActiveId(trust.kmsGenerationAddress, getActiveKeyIdAbi, reader.readContract),
    readActiveId(trust.kmsGenerationAddress, getActiveCrsIdAbi, reader.readContract),
  ]);
  if (confirmedKeyId !== snapshot.publicKeyId || confirmedCrsId !== snapshot.crsId) {
    throw new Error('The active FHE key or CRS rotated while its trust material was being read.');
  }
}

async function readActiveId(
  address: ChecksummedAddress,
  abi: typeof getActiveKeyIdAbi | typeof getActiveCrsIdAbi,
  readContract: ReadContract,
): Promise<Uint256BigInt> {
  const value = await readContract({
    address,
    abi,
    args: [],
    blockTag: KMS_GENERATION_READ_BLOCK_TAG,
    functionName: abi[0].name,
  });
  if (!isUint256(value)) {
    throw new Error(`Invalid ${abi[0].name} result.`);
  }
  return asUint256BigInt(BigInt(value), { subject: `${abi[0].name} result` });
}

function readPublicKeyDigest(materials: unknown): string {
  const keyDigests = tupleValue(materials, 1, 'keyDigests');
  if (!Array.isArray(keyDigests)) {
    throw new Error('Invalid getKeyMaterials result.');
  }

  const publicKeyDigests = keyDigests
    .filter((keyDigest) => asUint(tupleValue(keyDigest, 0, 'keyType')) === 1n)
    .map((keyDigest) => asHex(tupleValue(keyDigest, 1, 'digest')));

  const publicKeyDigest = publicKeyDigests[0];
  if (publicKeyDigest === undefined || publicKeyDigests.length !== 1) {
    throw new Error('Expected exactly one active FHE public-key digest.');
  }
  return publicKeyDigest;
}

function readCrsDigest(materials: unknown): string {
  return asHex(tupleValue(materials, 1, 'crsDigest'));
}

function tupleValue(value: unknown, index: number, name: string): unknown {
  if (Array.isArray(value)) {
    return value[index];
  }
  if (value !== null && typeof value === 'object' && name in value) {
    return (value as Record<string, unknown>)[name];
  }
  return undefined;
}

function asUint(value: unknown): bigint | undefined {
  return isUint256(value) ? BigInt(value) : undefined;
}

function asHex(value: unknown): string {
  if (typeof value !== 'string') {
    throw new Error('Invalid on-chain FHE key digest.');
  }
  return value;
}

////////////////////////////////////////////////////////////////////////////////

const getActiveKeyIdAbi = [
  {
    inputs: [],
    name: 'getActiveKeyId',
    outputs: [{ internalType: 'uint256', name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

const getActiveCrsIdAbi = [
  {
    inputs: [],
    name: 'getActiveCrsId',
    outputs: [{ internalType: 'uint256', name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

const keyDigestComponents = [
  { internalType: 'enum IKMSGeneration.KeyType', name: 'keyType', type: 'uint8' },
  { internalType: 'bytes', name: 'digest', type: 'bytes' },
] as const;

const getKeyMaterialsAbi = [
  {
    inputs: [{ internalType: 'uint256', name: 'keyId', type: 'uint256' }],
    name: 'getKeyMaterials',
    outputs: [
      { internalType: 'string[]', name: 'kmsNodeStorageUrls', type: 'string[]' },
      {
        components: keyDigestComponents,
        internalType: 'struct IKMSGeneration.KeyDigest[]',
        name: 'keyDigests',
        type: 'tuple[]',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

const getCrsMaterialsAbi = [
  {
    inputs: [{ internalType: 'uint256', name: 'crsId', type: 'uint256' }],
    name: 'getCrsMaterials',
    outputs: [
      { internalType: 'string[]', name: 'kmsNodeStorageUrls', type: 'string[]' },
      { internalType: 'bytes', name: 'crsDigest', type: 'bytes' },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;
