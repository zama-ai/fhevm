import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { TrustedClient } from '../modules/ethereum/types.js';
import type { Bytes, ChecksummedAddress } from '../types/primitives.js';
import { shake256 } from '@noble/hashes/sha3.js';
import { addressToChecksummedAddress, asAddress } from '../base/address.js';
import { bytesToHexNo0x, hexToBytes } from '../base/bytes.js';
import { assertIsUint256 } from '../base/uint.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { assertIsHostContractVersionOf, parseVersion } from '../host-contracts/HostContractVersion-p.js';
import {
  getActiveCrsIdAbi,
  getActiveKeyIdAbi,
  getCrsMaterialsAbi,
  getKeyMaterialsAbi,
  getVersionAbi,
} from '../host-contracts/abi-fragments/fragments.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { FheEncryptionKeyVerificationError } from '../errors/FheEncryptionKeyVerificationError.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly options: ResolvedFhevmOptions;
  readonly client: NonNullable<object> | undefined;
};

type KmsGenerationSource = {
  readonly address: ChecksummedAddress;
  readonly trustedClient: TrustedClient;
};

// IKMSGeneration.KeyType.Public
const KEY_TYPE_PUBLIC = 1;

// Domain separators the KMS prepends before hashing (SHAKE256, 32 bytes).
const PUBLIC_KEY_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_KEY');
const CRS_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_CRS');

////////////////////////////////////////////////////////////////////////////////
// assertKeyVerificationConfig
////////////////////////////////////////////////////////////////////////////////

/**
 * Checks, without any RPC call, that the client can verify key material:
 * either the host chain is Ethereum (its definition has `kmsGeneration`), or
 * `keyVerification` names an Ethereum chain definition.
 *
 * @throws {FhevmConfigError}
 */
export function assertKeyVerificationConfig(context: {
  readonly chain: FhevmChain;
  readonly options: ResolvedFhevmOptions;
}): void {
  if (context.chain.fhevm.contracts.kmsGeneration !== undefined) {
    return;
  }

  const keyVerification = context.options.keyVerification;
  if (keyVerification === undefined) {
    throw new FhevmConfigError({
      message:
        `Chain ${context.chain.id} has no KMSGeneration contract: set 'options.keyVerification' ` +
        `to an Ethereum chain definition and a client for it, so the FHE public key and CRS can be verified.`,
    });
  }

  if (keyVerification.chain.fhevm.contracts.kmsGeneration === undefined) {
    throw new FhevmConfigError({
      message: `'options.keyVerification.chain' (chain ${keyVerification.chain.id}) has no KMSGeneration contract.`,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////
// verifyFheEncryptionKeyBytes
////////////////////////////////////////////////////////////////////////////////

/**
 * Checks the FHE public key and CRS bytes against the active ones recorded in
 * `KMSGeneration` on Ethereum, before they are used or cached:
 *
 * - digest: SHAKE256(`"PDAT_KEY"` ‖ public key bytes) must equal the active
 *   key's `Public` digest, SHAKE256(`"PDAT_CRS"` ‖ CRS bytes) the active CRS digest;
 * - id: a 32-byte hex id served by the relayer must equal the active id.
 *
 * @returns the same bytes, once verified.
 * @throws {FheEncryptionKeyVerificationError} on a digest or id mismatch.
 * @throws {FhevmConfigError} on a misconfigured `keyVerification`.
 */
export async function verifyFheEncryptionKeyBytes(
  context: Context,
  bytes: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyBytes> {
  const source = await _resolveKmsGenerationSource(context);

  const read = (
    abi: readonly [Record<string, unknown> & { readonly name: string }],
    args: readonly unknown[],
  ): Promise<unknown> =>
    context.runtime.ethereum.readContract(source.trustedClient, {
      address: source.address,
      abi,
      args,
      functionName: abi[0].name,
    });

  const [unknownKeyId, unknownCrsId] = await Promise.all([read(getActiveKeyIdAbi, []), read(getActiveCrsIdAbi, [])]);
  assertIsUint256(unknownKeyId, {});
  assertIsUint256(unknownCrsId, {});
  const activeKeyId = BigInt(unknownKeyId);
  const activeCrsId = BigInt(unknownCrsId);

  const [keyMaterials, crsMaterials] = await Promise.all([
    read(getKeyMaterialsAbi, [activeKeyId]),
    read(getCrsMaterialsAbi, [activeCrsId]),
  ]);

  const expectedPublicKeyDigest = _parsePublicKeyDigest(keyMaterials);
  const expectedCrsDigest = _parseCrsDigest(crsMaterials);

  _assertIdMatch('FHE public key', bytes.publicKeyBytes.id, activeKeyId);
  _assertIdMatch('CRS', bytes.crsBytes.id, activeCrsId);

  _assertDigestMatch('FHE public key', PUBLIC_KEY_DIGEST_DOMAIN, bytes.publicKeyBytes.bytes, expectedPublicKeyDigest);
  _assertDigestMatch('CRS', CRS_DIGEST_DOMAIN, bytes.crsBytes.bytes, expectedCrsDigest);

  context.runtime.config.logger?.debug?.(
    `Verified FHE public key and CRS against KMSGeneration ${source.address} (keyId=0x${activeKeyId.toString(16)}, crsId=0x${activeCrsId.toString(16)}).`,
  );

  return bytes;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Picks the KMSGeneration contract and the client to read it with: the host
 * chain's when it is Ethereum, else the `keyVerification` one, after checking
 * that client's chain id and the contract's name.
 */
async function _resolveKmsGenerationSource(context: Context): Promise<KmsGenerationSource> {
  assertKeyVerificationConfig(context);

  const hostKmsGeneration = context.chain.fhevm.contracts.kmsGeneration;
  const keyVerification = context.options.keyVerification;

  let source: KmsGenerationSource;
  if (hostKmsGeneration !== undefined) {
    if (context.client === undefined) {
      throw new FhevmConfigError({
        message: 'A host-chain client is required to verify the FHE public key and CRS against KMSGeneration.',
      });
    }
    source = {
      address: addressToChecksummedAddress(asAddress(hostKmsGeneration.address)),
      trustedClient: getTrustedClient(context as Context & { readonly client: NonNullable<object> }),
    };
  } else {
    const kv = keyVerification;
    const kmsGeneration = kv?.chain.fhevm.contracts.kmsGeneration;
    if (kv === undefined || kmsGeneration === undefined) {
      // Unreachable: checked by assertKeyVerificationConfig
      throw new FhevmConfigError({ message: "Missing 'options.keyVerification'." });
    }

    // A wrong network is a config error, not a key mismatch.
    const chainId = await context.runtime.ethereum.getChainId(kv.trustedClient);
    if (chainId !== BigInt(kv.chain.id)) {
      throw new FhevmConfigError({
        message: `'options.keyVerification' client is connected to chain ${chainId}, but its chain definition is chain ${kv.chain.id}.`,
      });
    }

    source = {
      address: addressToChecksummedAddress(asAddress(kmsGeneration.address)),
      trustedClient: kv.trustedClient,
    };
  }

  // Read directly (not through the host-contract version cache), as the
  // keyVerification client is not a host-chain client.
  const version = await context.runtime.ethereum.readContract(source.trustedClient, {
    address: source.address,
    abi: getVersionAbi,
    args: [],
    functionName: getVersionAbi[0].name,
  });
  if (typeof version !== 'string') {
    throw new Error('Invalid KMSGeneration version.');
  }
  assertIsHostContractVersionOf(parseVersion(version), 'KMSGeneration');

  return source;
}

function _parsePublicKeyDigest(keyMaterials: unknown): Uint8Array {
  if (!Array.isArray(keyMaterials) || keyMaterials.length < 2 || !Array.isArray(keyMaterials[1])) {
    throw new Error('Invalid KMSGeneration.getKeyMaterials result.');
  }
  for (const keyDigest of keyMaterials[1] as unknown[]) {
    // viem decodes tuples as objects, ethers as array-like Results with named keys.
    const { keyType, digest } = keyDigest as { readonly keyType: unknown; readonly digest: unknown };
    if (Number(keyType) === KEY_TYPE_PUBLIC) {
      if (typeof digest !== 'string') {
        throw new Error('Invalid KMSGeneration public key digest.');
      }
      return hexToBytes(digest);
    }
  }
  throw new FheEncryptionKeyVerificationError({
    message: 'The active key in KMSGeneration has no public key digest.',
  });
}

function _parseCrsDigest(crsMaterials: unknown): Uint8Array {
  if (!Array.isArray(crsMaterials) || crsMaterials.length < 2 || typeof crsMaterials[1] !== 'string') {
    throw new Error('Invalid KMSGeneration.getCrsMaterials result.');
  }
  return hexToBytes(crsMaterials[1]);
}

// A label id (e.g. "fhe-public-key-data-id", served by relayers today) is not compared, as the digest check already binds the bytes to the active key; to be made strict with the unified caching scheme.
function _assertIdMatch(subject: string, relayerId: string, activeId: bigint): void {
  if (!/^0x[0-9a-fA-F]{64}$/.test(relayerId)) {
    return;
  }
  if (BigInt(relayerId) !== activeId) {
    throw new FheEncryptionKeyVerificationError({
      message: `${subject} id mismatch: the relayer served ${relayerId}, but the active id in KMSGeneration is 0x${activeId.toString(16).padStart(64, '0')}.`,
    });
  }
}

function _assertDigestMatch(subject: string, domain: Uint8Array, bytes: Bytes, expected: Uint8Array): void {
  const hasher = shake256.create({ dkLen: 32 });
  hasher.update(domain);
  hasher.update(bytes);
  const actual = hasher.digest();

  if (actual.length !== expected.length || !actual.every((b, i) => b === expected[i])) {
    throw new FheEncryptionKeyVerificationError({
      message: `${subject} digest mismatch: expected ${bytesToHexNo0x(expected)}, got ${bytesToHexNo0x(actual)}.`,
    });
  }
}
