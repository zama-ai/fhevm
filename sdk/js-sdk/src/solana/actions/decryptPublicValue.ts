import { RelayerAbortError } from '../../core/errors/RelayerAbortError.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from '../../core/modules/relayer/module/relayerUrl.js';
import { executeWithBatching } from '../../core/base/promise.js';
import { assertKmsDecryptionBitLimit } from '../../core/kms/utils.js';
import {
  getEncodedSize,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  type MaybeEncodedAccount,
  type ReadonlyUint8Array,
} from '@solana/kit';
import type { SolanaClientParameters } from '../clients/createFhevmBaseClient.js';
import { solanaHostProgram } from '../clients/createFhevmBaseClient.js';
import type { SolanaPublicDecryptCertificateParameters } from './publicDecryptCertificate.js';
import { publicDecryptCertificate, buildSolanaPublicDecryptExtraData } from './publicDecryptCertificate.js';
import { getSolanaRuntime } from '../internal/runtime.js';
import { findHostConfigPda } from '../internal/generated/zamaHost/pdas/hostConfig.js';
import { findKmsContextPda } from '../internal/generated/zamaHost/pdas/kmsContext.js';
import { getHostConfigDecoder, HOST_CONFIG_DISCRIMINATOR } from '../internal/generated/zamaHost/accounts/hostConfig.js';
import {
  getKmsContextDecoder,
  getKmsContextEncoder,
  KMS_CONTEXT_DISCRIMINATOR,
} from '../internal/generated/zamaHost/accounts/kmsContext.js';
import { bytesToHex, hexToBytes, unsafeBytesEquals } from '../../core/base/bytes.js';
import { recoverAddress } from '../../core/base/sign.js';
import { createKmsPublicDecryptEip712, publicDecryptDigest } from '../../core/kms/createKmsPublicDecryptEip712.js';
import type { Bytes65Hex, TypedValue } from '../../core/types/primitives.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { bytesToClearValueType } from '../../core/handle/FheType.js';
import { createClearValue, clearValueToTypedValue } from '../../core/handle/ClearValue.js';

const PUBLIC_DECRYPT_TOKEN = Symbol('fhevm.solana.public-decrypt');
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

export type SolanaDecryptPublicValueParameters = Omit<SolanaPublicDecryptCertificateParameters, 'contextId'> & {
  readonly contextId?: Uint8Array | undefined;
};

/** Authenticates and decodes plaintext. On-chain consumers still need the public MMR proof. */
export async function decryptPublicValue(
  client: SolanaClientParameters,
  parameters: SolanaDecryptPublicValueParameters,
): Promise<TypedValue> {
  const signal = parameters.options?.signal;
  const checkAbort = (): void => {
    if (signal?.aborted === true)
      throw new RelayerAbortError({
        operation: 'PUBLIC_DECRYPT',
        url: buildRelayerUrlString(validateRelayerBaseUrl(client.chain.fhevm.relayerUrl, false), 'v2/public-decrypt'),
      });
  };
  checkAbort();
  const handle = toFhevmHandle(parameters.handle);
  if (BigInt(handle.chainId) !== client.chain.id) throw new Error('Public decrypt handle belongs to another chain');
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
      await fetchEncodedAccount(client.rpc, configAddress, signal === undefined ? {} : { abortSignal: signal }).catch(
        (error: unknown) => {
          checkAbort();
          throw error;
        },
      ),
      HOST_CONFIG_DISCRIMINATOR,
    ),
  );
  checkAbort();
  const contextId = new Uint8Array(parameters.contextId ?? initial.currentKmsContextId);
  if (contextId.length !== 32 || contextId.every((byte) => byte === 0))
    throw new Error('KMS context is not configured');
  const claim = await publicDecryptCertificate(
    { chain: client.chain, runtime: getSolanaRuntime() },
    { ...parameters, contextId },
  );
  // Read the requested context after the response. A rotation preserves an old live context;
  // destruction invalidates it. Do not substitute the new current context for the signed one.
  const [contextAddress, contextBump] = await findKmsContextPda({ contextId }, { programAddress });
  const [configAccount, contextAccount] = await fetchEncodedAccounts(
    client.rpc,
    [configAddress, contextAddress],
    signal === undefined ? {} : { abortSignal: signal },
  ).catch((error: unknown) => {
    checkAbort();
    throw error;
  });
  checkAbort();
  const config = getHostConfigDecoder().decode(read(configAccount, HOST_CONFIG_DISCRIMINATOR));
  if (config.bump !== configBump || config.chainId !== client.chain.id)
    throw new Error('Host configuration does not match the client');
  if (config.decryptionContract.every((byte) => byte === 0))
    throw new Error('Host decryption contract is not configured');
  const contextBytes = read(contextAccount, KMS_CONTEXT_DISCRIMINATOR);
  const kms = getKmsContextDecoder().decode(contextBytes);
  if (kms.bump !== contextBump || kms.destroyed || !unsafeBytesEquals(new Uint8Array(kms.contextId), contextId))
    throw new Error('Invalid or destroyed KMS context');
  // Kit decodes only 1 as true, so reject other nonzero encodings explicitly.
  // Use the schema size, not the allocated account length: trailing bytes may exist.
  const destroyedOffset = getEncodedSize(kms, getKmsContextEncoder()) - 2;
  if (contextBytes[destroyedOffset] !== 0) throw new Error('Invalid or destroyed KMS context');
  const cleartext = hexToBytes(claim.abiEncodedCleartext);
  if (cleartext.length !== 32) throw new Error('Public decrypt cleartext must be 32 bytes');
  const eip712 = createKmsPublicDecryptEip712({
    verifyingContractAddressDecryption: bytesToHex(new Uint8Array(config.decryptionContract)),
    chainId: config.gatewayChainId,
    handles: [handle],
    decryptedResult: bytesToHex(cleartext),
    extraData: bytesToHex(buildSolanaPublicDecryptExtraData(contextId, parameters.encryptedStore)),
  });
  verifyPublicDecryptSignatures(
    publicDecryptDigest(eip712),
    claim.signatures,
    kms.signers,
    kms.thresholds.publicDecryption,
  );
  return clearValueToTypedValue(
    createClearValue({
      handle,
      value: bytesToClearValueType(handle.fheType, cleartext),
      originToken: PUBLIC_DECRYPT_TOKEN,
    }),
    PUBLIC_DECRYPT_TOKEN,
  );
}

/** Ordered single-handle certificates; the current Solana verifier does not accept a batch certificate. */
export async function decryptPublicValues(
  client: SolanaClientParameters,
  parameters: {
    readonly entries: readonly SolanaDecryptPublicValueParameters[];
  },
): Promise<TypedValue[]> {
  const handles = parameters.entries.map((entry) => toFhevmHandle(entry.handle));
  if (handles.length === 0) throw new Error('Public decrypt requires at least one handle');
  assertKmsDecryptionBitLimit(handles);
  if (handles.some((handle) => BigInt(handle.chainId) !== client.chain.id))
    throw new Error('Public decrypt handle belongs to another chain');
  return executeWithBatching(parameters.entries.map((entry) => () => decryptPublicValue(client, entry)));
}
