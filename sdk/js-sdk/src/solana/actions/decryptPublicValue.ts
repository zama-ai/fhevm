import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from '../../core/modules/relayer/module/relayerUrl.js';
import { assertKmsDecryptionBitLimit } from '../../core/kms/utils.js';
import {
  fetchEncodedAccount,
  fetchEncodedAccounts,
  type MaybeEncodedAccount,
  type ReadonlyUint8Array,
} from '@solana/kit';
import type { SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import type { RelayerPublicDecryptOptions } from '../../core/types/relayer.js';
import type { SolanaPublicDecryptCertifier, SolanaPublicHandleEntry } from './publicDecryptCertificate.js';
import { MAX_SOLANA_DECRYPT_HANDLES } from '../userDecrypt/request.js';
import { solanaPublicDecryptExtraData } from './publicDecryptCertificate.js';
import {
  findHostConfigPda,
  findKmsContextPda,
  getHostConfigDecoder,
  getKmsContextDecoder,
  HOST_CONFIG_DISCRIMINATOR,
  KMS_CONTEXT_DISCRIMINATOR,
} from '@fhevm/solana-zama-host';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { bytesToHex, concatBytes, hexToBytes, unsafeBytesEquals } from '../../core/base/bytes.js';
import { recoverAddress } from '../../core/base/sign.js';
import { createKmsPublicDecryptEip712 } from '../../core/kms/createKmsPublicDecryptEip712.js';
import type { KmsPublicDecryptEip712 } from '../../core/types/kms.js';
import type { Bytes65Hex, TypedValue } from '../../core/types/primitives.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { bytesToClearValueType } from '../../core/handle/FheType.js';
import { createClearValue, clearValueToTypedValue } from '../../core/handle/ClearValue.js';
import { eip712Digest, keccakUtf8 } from '../internal/eip712.js';

const PUBLIC_DECRYPT_TOKEN = Symbol('fhevm.solana.public-decrypt');

/**
 * The `PublicDecryptVerification` digest the host verifies a KMS certificate against. The schema is
 * shared with EVM, so its type string derives from the canonical field list.
 */
export function publicDecryptDigest(eip712: KmsPublicDecryptEip712): Uint8Array {
  const fields = eip712.types.PublicDecryptVerification;
  const struct = keccak_256(
    concatBytes(
      keccakUtf8(`PublicDecryptVerification(${fields.map(({ name, type }) => `${type} ${name}`).join(',')})`),
      keccak_256(concatBytes(...eip712.message.ctHandles.map(hexToBytes))),
      keccak_256(hexToBytes(eip712.message.decryptedResult)),
      keccak_256(hexToBytes(eip712.message.extraData)),
    ),
  );
  return eip712Digest(eip712.domain, struct);
}

/** Counts distinct registered signers using the host's recoverable-signature rules. */
export function verifyPublicDecryptSignatures(
  hash: Uint8Array,
  signatures: readonly string[],
  signers: readonly ReadonlyUint8Array[],
  threshold: number,
): void {
  const allowed = new Set(signers.map((signer) => bytesToHex(new Uint8Array(signer)).toLowerCase()));
  if (threshold < 1 || threshold > allowed.size) throw new Error('Invalid public decryption threshold');
  const valid = new Set<string>();
  for (const signature of signatures) {
    try {
      const bytes = hexToBytes(signature);
      if (bytes.length !== 65 || (bytes[64] !== 27 && bytes[64] !== 28)) continue;
      const s = BigInt(bytesToHex(bytes.subarray(32, 64)));
      if (s > 0x7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0n) continue;
      const signer = recoverAddress({
        hash: bytesToHex(hash),
        signature: bytesToHex(bytes) as Bytes65Hex,
      }).toLowerCase();
      if (allowed.has(signer)) valid.add(signer);
    } catch {
      // Like the host, malformed or unrelated signatures do not contribute to the threshold.
    }
  }
  if (valid.size < threshold) throw new Error('Public decryption signature threshold not met');
}

export type SolanaDecryptPublicValueParameters = SolanaPublicHandleEntry & {
  /** The KMS context the certificate commits to; the host's current context when omitted. */
  readonly contextId?: Uint8Array | undefined;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

export type SolanaDecryptPublicValuesParameters = Omit<
  SolanaDecryptPublicValueParameters,
  'handle' | 'encryptedStore'
> & {
  readonly entries: readonly SolanaPublicHandleEntry[];
};

/**
 * Authenticates and decodes plaintext. An on-chain consumer verifies the certificate itself and
 * compares the certified handle with one it pinned.
 */
export async function decryptPublicValue(
  client: SolanaClientParameters,
  parameters: SolanaDecryptPublicValueParameters,
  certify: SolanaPublicDecryptCertifier,
): Promise<TypedValue> {
  const { handle, encryptedStore, ...shared } = parameters;
  const [value] = await decryptPublicValues(client, { ...shared, entries: [{ handle, encryptedStore }] }, certify);
  if (value === undefined) throw new Error('Public decrypt returned no value');
  return value;
}

/**
 * Authenticates and decodes the plaintexts of several public handles, in order, from one
 * certificate: one relayer request and one host read for the whole batch.
 */
export async function decryptPublicValues(
  client: SolanaClientParameters,
  parameters: SolanaDecryptPublicValuesParameters,
  certify: SolanaPublicDecryptCertifier,
): Promise<TypedValue[]> {
  const signal = parameters.options?.signal;
  const checkAbort = (): void => {
    if (signal?.aborted === true)
      throw new RelayerAbortError({
        operation: 'PUBLIC_DECRYPT',
        url: buildRelayerUrlString(validateRelayerBaseUrl(client.chain.fhevm.relayerUrl, false), 'v2/public-decrypt'),
      });
  };
  checkAbort();
  const handles = parameters.entries.map((entry) => toFhevmHandle(entry.handle));
  if (handles.length === 0) throw new Error('Public decrypt requires at least one handle');
  if (handles.length > MAX_SOLANA_DECRYPT_HANDLES)
    throw new Error(`Public decrypt takes at most ${MAX_SOLANA_DECRYPT_HANDLES} handles per request`);
  assertKmsDecryptionBitLimit(handles);
  if (handles.some((handle) => BigInt(handle.chainId) !== client.chain.id))
    throw new Error('Public decrypt handle belongs to another chain');
  const programAddress = solanaHostProgram(client.chain);
  const read = (account: MaybeEncodedAccount | undefined, discriminator: ReadonlyUint8Array): Uint8Array => {
    if (
      account === undefined ||
      !account.exists ||
      account.programAddress !== programAddress ||
      account.executable ||
      !discriminator.every((byte, index) => account.data[index] === byte)
    ) {
      throw new Error(`Invalid host account ${account?.address ?? 'missing'}`);
    }
    return new Uint8Array(account.data);
  };
  const [configAddress, configBump] = await findHostConfigPda({ programAddress });
  const initial = getHostConfigDecoder().decode(
    read(
      await fetchEncodedAccount(client.rpc, configAddress, {
        commitment: 'finalized',
        ...(signal === undefined ? {} : { abortSignal: signal }),
      }).catch((error: unknown) => {
        checkAbort();
        throw error;
      }),
      HOST_CONFIG_DISCRIMINATOR,
    ),
  );
  checkAbort();
  const contextId = new Uint8Array(parameters.contextId ?? initial.currentKmsContextId);
  if (contextId.length !== 32 || contextId.every((byte) => byte === 0))
    throw new Error('KMS context is not configured');
  const claim = await certify({ entries: parameters.entries, contextId, options: parameters.options });
  // Read the requested context after the response. A rotation preserves an old live context;
  // destruction invalidates it. Do not substitute the new current context for the signed one.
  const [contextAddress, contextBump] = await findKmsContextPda({ contextId }, { programAddress });
  const [configAccount, contextAccount] = await fetchEncodedAccounts(client.rpc, [configAddress, contextAddress], {
    commitment: 'finalized',
    ...(signal === undefined ? {} : { abortSignal: signal }),
  }).catch((error: unknown) => {
    checkAbort();
    throw error;
  });
  checkAbort();
  const config = getHostConfigDecoder().decode(read(configAccount, HOST_CONFIG_DISCRIMINATOR));
  if (config.bump !== configBump || config.chainId !== client.chain.id)
    throw new Error('Host configuration does not match the client');
  if (config.decryptionContract.every((byte) => byte === 0))
    throw new Error('Host decryption contract is not configured');
  const kms = getKmsContextDecoder().decode(read(contextAccount, KMS_CONTEXT_DISCRIMINATOR));
  if (kms.bump !== contextBump || kms.destroyed || !unsafeBytesEquals(new Uint8Array(kms.contextId), contextId))
    throw new Error('Invalid or destroyed KMS context');
  // One 32-byte ABI word per handle, in request order.
  const cleartext = hexToBytes(claim.abiEncodedCleartext);
  if (cleartext.length !== 32 * handles.length) throw new Error('Public decrypt cleartext must be 32 bytes per handle');
  const eip712 = createKmsPublicDecryptEip712({
    verifyingContractAddressDecryption: bytesToHex(new Uint8Array(config.decryptionContract)),
    chainId: config.gatewayChainId,
    handles,
    decryptedResult: bytesToHex(cleartext),
    extraData: solanaPublicDecryptExtraData(contextId),
  });
  verifyPublicDecryptSignatures(
    publicDecryptDigest(eip712),
    claim.signatures,
    kms.signers,
    kms.thresholds.publicDecryption,
  );
  return handles.map((handle, index) =>
    clearValueToTypedValue(
      createClearValue({
        handle,
        value: bytesToClearValueType(handle.fheType, cleartext.subarray(32 * index, 32 * (index + 1))),
        originToken: PUBLIC_DECRYPT_TOKEN,
      }),
      PUBLIC_DECRYPT_TOKEN,
    ),
  );
}
