import type { EncryptedValueLike } from '../../core/types/encryptedTypes.js';
import type { RelayerPublicDecryptOptions } from '../../core/types/relayer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { bytesToHex, concatBytes, unsafeBytesEquals } from '../../core/base/bytes.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { removeSuffix } from '../../core/base/string.js';
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
  /** The 32-byte address of the `EncryptedValue` account the handle was made public in. */
  readonly encryptedValueAccount: Uint8Array;
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
 * `extraData` version byte of a Solana public decrypt: `0x03 ‖ contextId(32) ‖ encryptedValueAccount(32)`,
 * exactly 65 bytes. Mirrors `SOLANA_EXTRA_DATA_VERSION_PUBLIC_DECRYPT` in the connector's
 * `solana_extra_data.rs`.
 */
export const SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_VERSION = 0x03;

/**
 * Builds the `extraData` a public-decrypt request carries on the wire.
 *
 * Nothing but the account travels: the Connector reads it and asks the coprocessors for the
 * `PublicDecryptLeaf` proof itself (RFC 035). The Rust half of this hand-mirrored codec is
 * `encode_solana_public_decrypt_extra_data` in the connector's `solana_extra_data.rs`; the two
 * layouts must change together, and `solana/test-fixtures/user-decrypt/extra_data_v1.json` is what
 * pins them to each other.
 *
 * @param contextId - The 32-byte KMS context id.
 * @param encryptedValueAccount - The 32-byte address of the account the handle lives in.
 */
export function buildSolanaPublicDecryptExtraData(
  contextId: Uint8Array,
  encryptedValueAccount: Uint8Array,
): Uint8Array {
  assertExtraDataFieldLen('contextId', contextId, 32);
  assertExtraDataFieldLen('encryptedValueAccount', encryptedValueAccount, 32);
  return concatBytes(new Uint8Array([SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_VERSION]), contextId, encryptedValueAccount);
}

function assertExtraDataFieldLen(name: string, bytes: Uint8Array, len: number): void {
  if (bytes.length !== len) {
    throw new Error(`${name} must be ${len} bytes, got ${bytes.length}`);
  }
}

/** Requests a public-decrypt certificate for a handle made public in an encrypted value account. */
export async function publicDecryptCertificate(
  context: SolanaPublicDecryptCertificateContext,
  parameters: SolanaPublicDecryptCertificateParameters,
): Promise<SolanaPublicDecryptCertificateClaim> {
  const handle = toFhevmHandle(parameters.handle);

  const requestExtraData = buildSolanaPublicDecryptExtraData(parameters.contextId, parameters.encryptedValueAccount);
  const requestExtraDataHex = bytesToHex(requestExtraData);
  const request = new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url: `${removeSuffix(context.chain.fhevm.relayerUrl, '/')}/v2/public-decrypt`,
    retryOnReadinessCheckTimeout: true,
    payload: {
      ciphertextHandles: [handle.bytes32Hex],
      extraData: requestExtraDataHex,
    },
    options: { auth: context.runtime.config.auth, ...parameters.options },
  });
  const result = (await request.run()) as {
    readonly decryptedValue: string;
    readonly signatures: readonly string[];
    readonly extraData?: string | undefined;
  };

  if (result.extraData !== undefined && !unsafeBytesEquals(hexToBytes(result.extraData), requestExtraData)) {
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
