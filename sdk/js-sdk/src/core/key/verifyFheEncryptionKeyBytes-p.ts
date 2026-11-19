import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient, ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { EthCallResult, TrustedClient } from '../modules/ethereum/types.js';
import type { BytesHex, ChecksummedAddress } from '../types/primitives.js';
import { keccak_256, shake256 } from '@noble/hashes/sha3.js';
import { addressToChecksummedAddress, asAddress } from '../base/address.js';
import { bytesToHex, bytesToHexNo0x, hexToBytes, unsafeBytesEquals } from '../base/bytes.js';
import { assertIsUint256 } from '../base/uint.js';
import { assertIsNonEmptyString } from '../base/string.js';
import { asCoreClientFhevm } from '../runtime/CoreFhevm-p.js';
import { assertIsHostContractVersionOf, parseVersion } from '../host-contracts/HostContractVersion-p.js';
import {
  getActiveCrsIdAbi,
  getActiveKeyIdAbi,
  getCrsMaterialsAbi,
  getKeyMaterialsAbi,
  getVersionAbi,
} from '../host-contracts/abi-fragments/fragments.js';
import { ensureError } from '../base/errors/utils.js';
import { executeWithBatching } from '../base/promise.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { FheEncryptionKeyVerificationError } from '../errors/FheEncryptionKeyVerificationError.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>;

/** A `KMSGeneration` contract: Ethereum chain id and checksummed address. */
export type KmsGenerationRef = {
  readonly chainId: number;
  readonly address: ChecksummedAddress;
};

type KmsGenerationSource = KmsGenerationRef & {
  readonly trustedClient: TrustedClient;
  /** The `keyVerification` client: its chain id is checked before any of its results is trusted. */
  readonly checkChainId: boolean;
};

type Digests = {
  readonly publicKey: Uint8Array;
  readonly crs: Uint8Array;
};

// IKMSGeneration.KeyType.Public
const KEY_TYPE_PUBLIC = 1;

// Domain separators the KMS prepends before hashing (SHAKE256, 32 bytes).
const PUBLIC_KEY_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_KEY');
const CRS_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_CRS');

// Selectors of the materials getters, to replay a failed read as a raw
// eth_call that tells a revert from a transport error. Derived from the ABI
// fragments the reads use, so the two cannot drift apart.
const GET_KEY_MATERIALS_SELECTOR = _selectorOf(getKeyMaterialsAbi);
const GET_CRS_MATERIALS_SELECTOR = _selectorOf(getCrsMaterialsAbi);

// The digests of each key / CRS bytes object, hashed once: clients sharing a
// downloaded key check it with their own reads, not their own hash.
const digestsByBytes = new WeakMap<FheEncryptionKeyBytes, Digests>();

////////////////////////////////////////////////////////////////////////////////
// requiresKeyVerification
////////////////////////////////////////////////////////////////////////////////

/**
 * `false` only for a relayer module that skips the key check (see
 * `skipsFheEncryptionKeyVerification`): every other key is checked.
 */
export function requiresKeyVerification(runtime: FhevmRuntime): boolean {
  return runtime.relayer.skipsFheEncryptionKeyVerification !== true;
}

////////////////////////////////////////////////////////////////////////////////
// assertKeyVerificationConfig
////////////////////////////////////////////////////////////////////////////////

/**
 * Checks, without any RPC call, that the client can verify key material
 * (see {@link resolveKmsGeneration}).
 *
 * @throws {FhevmConfigError}
 */
export function assertKeyVerificationConfig(context: {
  readonly chain: FhevmChain;
  readonly options: ResolvedFhevmOptions;
}): void {
  resolveKmsGeneration(context);
}

/**
 * The `KMSGeneration` contract the client checks key material against, without
 * any RPC call. It comes from the host chain definition (Ethereum) and/or
 * `options.keyVerification.chain`: one is used as is, two must name the same
 * contract. The address is checksummed, so equal contracts give equal values.
 *
 * @throws {FhevmConfigError} when there is none, when `keyVerification.chain`
 * has none, or when the two disagree.
 */
export function resolveKmsGeneration(context: {
  readonly chain: FhevmChain;
  readonly options: Pick<ResolvedFhevmOptions, 'keyVerification'>;
}): KmsGenerationRef {
  const hostKmsGeneration = context.chain.fhevm.contracts.kmsGeneration;
  const host =
    hostKmsGeneration === undefined ? undefined : _kmsGenerationRef(context.chain.id, hostKmsGeneration.address);

  const keyVerification = context.options.keyVerification;
  let option: KmsGenerationRef | undefined;
  if (keyVerification !== undefined) {
    const kmsGeneration = keyVerification.chain.fhevm.contracts.kmsGeneration;
    if (kmsGeneration === undefined) {
      throw new FhevmConfigError({
        message: `'options.keyVerification.chain' (chain ${keyVerification.chain.id}) has no KMSGeneration contract.`,
      });
    }
    option = _kmsGenerationRef(keyVerification.chain.id, kmsGeneration.address);
  }

  const ref = host ?? option;
  if (ref === undefined) {
    throw new FhevmConfigError({
      message:
        `Chain ${context.chain.id} has no KMSGeneration contract to verify the FHE public key and CRS against: ` +
        `set 'options.keyVerification' to { chain, client } with an Ethereum chain definition and a client ` +
        `connected to Ethereum.`,
    });
  }
  if (option !== undefined && (option.chainId !== ref.chainId || option.address !== ref.address)) {
    throw new FhevmConfigError({
      message:
        `'options.keyVerification' names KMSGeneration ${option.address} on chain ${option.chainId}, but chain ` +
        `${ref.chainId} has its own, ${ref.address}: remove 'options.keyVerification' or make it match.`,
    });
  }
  return ref;
}

function _kmsGenerationRef(chainId: number, address: string): KmsGenerationRef {
  return { chainId, address: addressToChecksummedAddress(asAddress(address)) };
}

////////////////////////////////////////////////////////////////////////////////
// verifyFheEncryptionKeyBytes
////////////////////////////////////////////////////////////////////////////////

/**
 * Checks the FHE public key and CRS bytes against the active ones recorded in
 * `KMSGeneration` on Ethereum, before they are used or cached:
 *
 * - id: a 32-byte hex id served by the relayer must equal the active id,
 *   checked as soon as the active ids are read;
 * - digest: SHAKE256(`"PDAT_KEY"` ‖ public key bytes) must equal the active
 *   key's `Public` digest, SHAKE256(`"PDAT_CRS"` ‖ CRS bytes) the active CRS digest.
 *
 * @returns the same bytes, once verified.
 * @throws {FheEncryptionKeyVerificationError} on a digest or id mismatch, when
 * `KMSGeneration` has no active key or CRS (id 0), or when reading the active
 * key or CRS materials reverts (e.g. `KeyNotGenerated`, `CrsNotGenerated`,
 * `CrsAborted`; the read error is the `cause`).
 * @throws {FhevmConfigError} on a misconfigured `keyVerification`.
 * @throws {Error} on a malformed `KMSGeneration` result.
 * @throws the client's own error on a network / RPC failure, unwrapped, as
 * for any other contract read.
 */
export async function verifyFheEncryptionKeyBytes(
  context: Context,
  bytes: FheEncryptionKeyBytes,
): Promise<FheEncryptionKeyBytes> {
  const source = _resolveKmsGenerationSource(context);
  const batchRpcCalls = context.options.batchRpcCalls;

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

  // Round 1: chain id (keyVerification client only), version, active ids.
  // The checks are shared: a read failing because of a wrong network or
  // contract waits for them, so that cause is reported, not the read error.
  let chainIdCheck: Promise<void> | undefined;
  const checkChainId = (): Promise<void> => (chainIdCheck ??= _assertChainId(context, source));
  let versionCheck: Promise<void> | undefined;
  const checkVersion = (): Promise<void> =>
    (versionCheck ??= _thenChecked(
      _assertIsKmsGeneration(() => read(getVersionAbi, [])),
      [checkChainId],
    ));
  const readActiveId = (abi: readonly [Record<string, unknown> & { readonly name: string }]): Promise<unknown> =>
    _thenChecked(read(abi, []), [checkChainId, checkVersion]);

  const [, , unknownKeyId, unknownCrsId] = await executeWithBatching<unknown>(
    [checkChainId, checkVersion, () => readActiveId(getActiveKeyIdAbi), () => readActiveId(getActiveCrsIdAbi)],
    batchRpcCalls,
  );
  assertIsUint256(unknownKeyId, {});
  assertIsUint256(unknownCrsId, {});
  const activeKeyId = BigInt(unknownKeyId);
  const activeCrsId = BigInt(unknownCrsId);
  _assertIsActive('FHE public key', activeKeyId);
  _assertIsActive('CRS', activeCrsId);
  // Before the materials reads: a wrong id needs no further RPC call.
  _assertIdMatch('FHE public key', bytes.publicKeyBytes.id, activeKeyId);
  _assertIdMatch('CRS', bytes.crsBytes.id, activeCrsId);

  // Round 2: the active key and CRS materials.
  const [keyMaterials, crsMaterials] = await executeWithBatching<unknown>(
    [
      () =>
        _readMaterials(context, source, {
          subject: 'FHE public key',
          activeId: activeKeyId,
          selector: GET_KEY_MATERIALS_SELECTOR,
          readFn: () => read(getKeyMaterialsAbi, [activeKeyId]),
        }),
      () =>
        _readMaterials(context, source, {
          subject: 'CRS',
          activeId: activeCrsId,
          selector: GET_CRS_MATERIALS_SELECTOR,
          readFn: () => read(getCrsMaterialsAbi, [activeCrsId]),
        }),
    ],
    batchRpcCalls,
  );

  const expectedPublicKeyDigest = _parsePublicKeyDigest(keyMaterials);
  const expectedCrsDigest = _parseCrsDigest(crsMaterials);

  const digests = _digestsOf(bytes);
  _assertDigestMatch('FHE public key', digests.publicKey, expectedPublicKeyDigest);
  _assertDigestMatch('CRS', digests.crs, expectedCrsDigest);

  context.runtime.config.logger?.debug?.(
    `Verified FHE public key and CRS against KMSGeneration ${source.address} (keyId=0x${activeKeyId.toString(16)}, crsId=0x${activeCrsId.toString(16)}).`,
  );

  return bytes;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The KMSGeneration contract (see {@link resolveKmsGeneration}) and the client
 * to read it with: the `keyVerification` client when set, else the host client
 * (the contract is then on the host chain).
 */
function _resolveKmsGenerationSource(context: Context): KmsGenerationSource {
  const ref = resolveKmsGeneration(context);

  const kv = context.options.keyVerification;
  if (kv !== undefined) {
    return { ...ref, trustedClient: kv.trustedClient, checkChainId: true };
  }
  if (context.client === undefined) {
    throw new FhevmConfigError({
      message: 'A host-chain client is required to verify the FHE public key and CRS against KMSGeneration.',
    });
  }
  return { ...ref, trustedClient: asCoreClientFhevm(context).trustedClient, checkChainId: false };
}

/**
 * On failure, runs the checks first: a failing check's error replaces the
 * read error it explains.
 */
async function _thenChecked<T>(promise: Promise<T>, checks: ReadonlyArray<() => Promise<void>>): Promise<T> {
  try {
    return await promise;
  } catch (e) {
    for (const check of checks) {
      await check();
    }
    throw e;
  }
}

/**
 * Checks the `keyVerification` client's chain id (no-op for the host client).
 *
 * @throws {FhevmConfigError} when it is connected to another chain: a wrong
 * network is a config error, not a key mismatch.
 */
async function _assertChainId(context: Context, source: KmsGenerationSource): Promise<void> {
  if (!source.checkChainId) {
    return;
  }
  const chainId = await context.runtime.ethereum.getChainId(source.trustedClient);
  if (chainId !== BigInt(source.chainId)) {
    throw new FhevmConfigError({
      message: `'options.keyVerification' client is connected to chain ${chainId}, but its chain definition is chain ${source.chainId}.`,
    });
  }
}

/**
 * Checks that the contract is `KMSGeneration`. The version is read by each
 * verification, on every chain, and never taken from or stored in the host
 * contract version cache: the read runs alongside the chain id check, so a
 * shared value could come from a client on the wrong network and fail every
 * other client. The cost is one call per verification.
 */
async function _assertIsKmsGeneration(readVersion: () => Promise<unknown>): Promise<void> {
  const res = await readVersion();
  assertIsNonEmptyString(res);
  assertIsHostContractVersionOf(parseVersion(res), 'KMSGeneration');
}

// KMSGeneration returns id 0 when no key / CRS generation has completed.
function _assertIsActive(subject: string, activeId: bigint): void {
  if (activeId === 0n) {
    throw new FheEncryptionKeyVerificationError({
      message: `No active ${subject} in KMSGeneration (active id is 0): the ${subject} cannot be verified.`,
    });
  }
}

/**
 * KMSGeneration reverts (KeyNotGenerated, CrsNotGenerated, CrsAborted) when the
 * active id has no usable materials: the key material cannot be verified.
 *
 * `readContract` errors do not tell a revert from a network / RPC failure, so
 * a failed read is replayed as a raw `eth_call`, whose result does. Only a
 * revert becomes a verification error; anything else rethrows the read error
 * as is, like any other contract read.
 */
async function _readMaterials(
  context: Context,
  source: KmsGenerationSource,
  parameters: {
    readonly subject: string;
    readonly activeId: bigint;
    readonly selector: BytesHex;
    readonly readFn: () => Promise<unknown>;
  },
): Promise<unknown> {
  const { subject, activeId, selector, readFn } = parameters;
  try {
    return await readFn();
  } catch (e) {
    const encodedId = context.runtime.ethereum.encode({ types: ['uint256'], values: [activeId] });
    let result: EthCallResult;
    try {
      result = await context.runtime.ethereum.call(source.trustedClient, {
        to: source.address,
        data: `${selector}${encodedId.slice(2)}` as BytesHex,
      });
    } catch {
      // Transport error: keep the read error.
      throw e;
    }
    if (result.success) {
      // Not a revert (e.g. a transient failure): keep the read error.
      throw e;
    }
    throw new FheEncryptionKeyVerificationError({
      message: `Cannot read the active ${subject} materials (id ${_formatId(activeId)}) from KMSGeneration, the call reverted (${result.reason}): the ${subject} cannot be verified.`,
      cause: ensureError(e),
    });
  }
}

/**
 * keccak256(`name(type,…)`)[0:4] of a single-function ABI fragment. Elementary
 * input types only (as both getters take a `uint256`): a tuple would need its
 * canonical `(…)` form.
 */
function _selectorOf(abi: readonly [Record<string, unknown> & { readonly name: string }]): BytesHex {
  const [fragment] = abi;
  const types = (fragment.inputs as ReadonlyArray<{ readonly type: string }>).map((input) => input.type);
  const signature = `${fragment.name}(${types.join(',')})`;
  return bytesToHex(keccak_256(new TextEncoder().encode(signature)).subarray(0, 4));
}

function _formatId(id: bigint): string {
  return `0x${id.toString(16).padStart(64, '0')}`;
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
      message: `${subject} id mismatch: the relayer served ${relayerId}, but the active id in KMSGeneration is ${_formatId(activeId)}.`,
    });
  }
}

function _assertDigestMatch(subject: string, actual: Uint8Array, expected: Uint8Array): void {
  if (!unsafeBytesEquals(actual, expected)) {
    throw new FheEncryptionKeyVerificationError({
      message: `${subject} digest mismatch: expected ${bytesToHexNo0x(expected)}, got ${bytesToHexNo0x(actual)}.`,
    });
  }
}

/** The key and CRS digests of `bytes`, memoized per bytes object. */
function _digestsOf(bytes: FheEncryptionKeyBytes): Digests {
  let digests = digestsByBytes.get(bytes);
  if (digests === undefined) {
    digests = {
      publicKey: _digest(PUBLIC_KEY_DIGEST_DOMAIN, bytes.publicKeyBytes.bytes),
      crs: _digest(CRS_DIGEST_DOMAIN, bytes.crsBytes.bytes),
    };
    digestsByBytes.set(bytes, digests);
  }
  return digests;
}

/**
 * SHAKE256(domain ‖ bytes), 32 bytes. One blocking call: ~140ms for the
 * ~4.6MB CRS on desktop (~33ms/MiB with @noble/hashes), once per bytes object.
 */
function _digest(domain: Uint8Array, bytes: Uint8Array): Uint8Array {
  return shake256.create({ dkLen: 32 }).update(domain).update(bytes).digest();
}
