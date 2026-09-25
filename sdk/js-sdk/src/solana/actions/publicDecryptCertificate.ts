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
import { hexToBytes } from '../proof.js';

export type SolanaPublicDecryptCertificateContext = {
  readonly chain: FhevmSolanaChain;
  readonly runtime: FhevmRuntime;
};

export type SolanaPublicDecryptCertificateParameters = {
  /** The single ciphertext handle covered by the public-decrypt certificate. */
  readonly handle: EncryptedValueLike;
  /** The 32-byte KMS context id the certificate commits to. */
  readonly contextId: Uint8Array;
  /** The 32-byte EncryptedStore address whose history authorizes public decryption. */
  readonly encryptedStore: Uint8Array;
  /** Relayer HTTP/polling budget; RPC calls use the supplied signal and the RPC transport policy. */
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

/**
 * An untrusted public-decrypt certificate claim returned by the relayer. Authority exists only
 * after the stateless host `verify_public_decrypt` verifies this certificate on-chain against the
 * current `KmsContext` (directly, or via the token `disclose_secp` wrapper), together with the
 * public-leaf inclusion proof the caller builds from the account's history.
 */
export type SolanaPublicDecryptCertificateClaim = {
  readonly handle: string;
  /** Raw ABI-encoded cleartext returned by the relayer. It is intentionally not interpreted. */
  readonly abiEncodedCleartext: string;
  readonly signatures: readonly string[];
  readonly extraData: string;
};

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

/** Requests a public-decrypt certificate for a handle made public in an encrypted store. */
export async function publicDecryptCertificate(
  context: SolanaPublicDecryptCertificateContext,
  parameters: SolanaPublicDecryptCertificateParameters,
): Promise<SolanaPublicDecryptCertificateClaim> {
  const handle = toFhevmHandle(parameters.handle);

  const requestExtraDataHex = solanaPublicDecryptExtraData(parameters.contextId);
  assertFieldLen('encryptedStore', parameters.encryptedStore, 32);
  const options = { auth: context.runtime.config.auth, ...parameters.options };
  const baseUrl = validateRelayerBaseUrl(context.chain.fhevm.relayerUrl, options.auth !== undefined);
  const request = new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url: buildRelayerUrlString(baseUrl, 'v2/public-decrypt'),
    retryOnReadinessCheckTimeout: true,
    payload: {
      ciphertextHandles: [handle.bytes32Hex],
      extraData: requestExtraDataHex,
      encryptedStores: [bytesToHex(parameters.encryptedStore)],
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
    handle: handle.bytes32Hex,
    abiEncodedCleartext: result.decryptedValue,
    signatures: result.signatures,
    extraData: result.extraData ?? requestExtraDataHex,
  };
}
