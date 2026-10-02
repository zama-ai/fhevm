import type { EncryptedValueLike } from '../../core/types/encryptedTypes.js';
import type { RelayerPublicDecryptOptions } from '../../core/types/relayer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { bytesToBigInt, bytesToHex, unsafeBytesEquals } from '../../core/base/bytes.js';
import { createKmsExtraDataV1 } from '../../core/kms/kmsExtraData-p.js';
import type { BytesHex, Uint256BigInt } from '../../core/types/primitives.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from '../../core/modules/relayer/module/relayerUrl.js';
import { hexToBytes } from '../../core/base/bytes.js';

export type SolanaPublicDecryptCertificateContext = {
  readonly chain: FhevmSolanaChain;
  readonly runtime: FhevmRuntime;
};

/** A handle made public in an encrypted store. */
export type SolanaPublicHandleEntry = {
  readonly handle: EncryptedValueLike;
  /** The 32-byte EncryptedStore address whose history authorizes public decryption. */
  readonly encryptedStore: Uint8Array;
};

/** Handles certified together: one request, one KMS context, one signature set. */
export type SolanaPublicDecryptBatch = {
  readonly entries: readonly SolanaPublicHandleEntry[];
  /** The 32-byte KMS context id the certificate commits to. */
  readonly contextId: Uint8Array;
  /** Relayer HTTP/polling budget; RPC calls use the supplied signal and the RPC transport policy. */
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

/** The one handle an on-chain consumer has certified. */
export type SolanaPublicDecryptCertificateParameters = SolanaPublicHandleEntry &
  Omit<SolanaPublicDecryptBatch, 'entries'>;

/**
 * An untrusted public-decrypt certificate claim returned by the relayer. Authority exists only
 * after the stateless host `verify_public_decrypt` verifies this certificate on-chain against the
 * certificate's `KmsContext` (directly, or via the token `disclose_secp` wrapper). The consuming
 * program binds it to its own state by comparing the certified handle with one it pinned.
 */
export type SolanaPublicDecryptCertificateClaim = {
  readonly handle: string;
  /** Raw ABI-encoded cleartext returned by the relayer. It is intentionally not interpreted. */
  readonly abiEncodedCleartext: string;
  readonly signatures: readonly string[];
  readonly extraData: string;
};

/** A certificate over every handle of a batch; `abiEncodedCleartext` holds one 32-byte word per handle. */
export type SolanaPublicDecryptBatchClaim = Omit<SolanaPublicDecryptCertificateClaim, 'handle'> & {
  readonly handles: readonly string[];
};

/** Obtains the certificate of a batch of public handles; the relayer unless a cleartext client signs it. */
export type SolanaPublicDecryptCertifier = (batch: SolanaPublicDecryptBatch) => Promise<SolanaPublicDecryptBatchClaim>;

/**
 * The single-handle certificate an on-chain consumer submits: the host verifier checks one handle.
 *
 * @param certify - The batch certifier.
 */
export function singlePublicDecryptCertificate(
  certify: SolanaPublicDecryptCertifier,
): (parameters: SolanaPublicDecryptCertificateParameters) => Promise<SolanaPublicDecryptCertificateClaim> {
  return async ({ handle, encryptedStore, ...shared }) => {
    const requested = toFhevmHandle(handle).bytes32Hex;
    const { handles, ...claim } = await certify({ ...shared, entries: [{ handle, encryptedStore }] });
    if (handles.length !== 1 || handles[0]?.toLowerCase() !== requested.toLowerCase())
      throw new Error('public-decrypt certificate must cover exactly the requested handle');
    return { ...claim, handle: requested };
  };
}

/**
 * The KMS routing a Solana public decrypt carries in `extraData`: version 1, `0x01 ‖ contextId`.
 * The host `KmsContext` holds no epoch, so this is what the EVM flow sends when no epoch is set.
 * The encrypted store travels beside it, in `encryptedStores`.
 *
 * @param contextId - The 32-byte KMS context id.
 */
export function solanaPublicDecryptExtraData(contextId: Uint8Array): BytesHex {
  assertFieldLen('contextId', contextId, 32);
  return createKmsExtraDataV1({ kmsContextId: bytesToBigInt(contextId) as Uint256BigInt }).bytesHex;
}

function assertFieldLen(name: string, bytes: Uint8Array, len: number): void {
  if (bytes.length !== len) {
    throw new Error(`${name} must be ${len} bytes, got ${bytes.length}`);
  }
}

/** Requests one public-decrypt certificate for handles made public in their encrypted stores. */
export async function publicDecryptCertificate(
  context: SolanaPublicDecryptCertificateContext,
  parameters: SolanaPublicDecryptBatch,
): Promise<SolanaPublicDecryptBatchClaim> {
  const handles = parameters.entries.map((entry) => toFhevmHandle(entry.handle).bytes32Hex);

  const requestExtraDataHex = solanaPublicDecryptExtraData(parameters.contextId);
  for (const entry of parameters.entries) assertFieldLen('encryptedStore', entry.encryptedStore, 32);
  const options = { auth: context.runtime.config.auth, ...parameters.options };
  const baseUrl = validateRelayerBaseUrl(context.chain.fhevm.relayerUrl, options.auth !== undefined);
  const request = new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url: buildRelayerUrlString(baseUrl, 'v2/public-decrypt'),
    payload: {
      ciphertextHandles: handles,
      extraData: requestExtraDataHex,
      encryptedStores: parameters.entries.map((entry) => bytesToHex(entry.encryptedStore)),
    },
    options,
    logger: context.runtime.config.logger,
  });
  const result = (await request.run()) as {
    readonly decryptedValue: string;
    readonly signatures: readonly string[];
    readonly extraData?: string | undefined;
  };

  if (
    result.extraData !== undefined &&
    !unsafeBytesEquals(hexToBytes(result.extraData), hexToBytes(requestExtraDataHex))
  ) {
    throw new Error('public-decrypt response extraData does not match the request');
  }
  if (
    result.decryptedValue.length === 0 ||
    result.decryptedValue.length % 2 !== 0 ||
    !/^[0-9a-f]+$/i.test(result.decryptedValue)
  ) {
    throw new Error('public-decrypt response cleartext must be nonempty even-length ABI hex');
  }
  if (result.signatures.length === 0) {
    throw new Error('public-decrypt response must contain at least one signature');
  }
  for (const signature of result.signatures) {
    if (signature.length !== 130 || !/^[0-9a-f]+$/i.test(signature)) {
      throw new Error(`public-decrypt signature must be valid 65-byte hex, got ${signature.length} hex characters`);
    }
  }

  return {
    handles,
    abiEncodedCleartext: result.decryptedValue,
    signatures: result.signatures,
    extraData: result.extraData ?? requestExtraDataHex,
  };
}
