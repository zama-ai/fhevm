import type { EncryptedValueLike } from '../../core/types/encryptedTypes.js';
import type { RelayerPublicDecryptOptions } from '../../core/types/relayer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { MmrProof } from '../proof.js';
import { bytesToHex, concatBytes, unsafeBytesEquals } from '../../core/base/bytes.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { removeSuffix } from '../../core/base/string.js';
import {
  decodeMmrProofTransportBlob,
  hexToBytes,
  MMR_PROOF_MODE_PUBLIC,
  u64BE,
  verifyPublicDecryptProof,
} from '../proof.js';

export type SolanaPublicDecryptCertificateContext = {
  readonly chain: FhevmSolanaChain;
  readonly runtime: FhevmRuntime;
};

export type SolanaPublicDecryptCertificateParameters = {
  /** The single ciphertext handle covered by the public-decrypt certificate. */
  readonly handle: EncryptedValueLike;
  readonly contextId: Uint8Array;
  readonly aclValueKey: Uint8Array;
  readonly proofSlot: bigint;
  readonly encryptedValueAccount: Uint8Array;
  readonly peaks: readonly Uint8Array[];
  readonly leafCount: bigint;
  /** Canonical `0x02 || Borsh(MmrProof)` bytes; no separately decoded proof is accepted. */
  readonly mmrProofBytes: Uint8Array;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

/**
 * An untrusted public-decrypt certificate claim returned by the relayer. Authority exists only
 * after the stateless host `verify_public_decrypt` verifies this certificate on-chain against the
 * current `KmsContext` (directly, or via the token `disclose_secp` wrapper).
 */
export type SolanaPublicDecryptCertificateClaim = {
  readonly handle: string;
  /** Raw ABI-encoded cleartext returned by the relayer. It is intentionally not interpreted. */
  readonly abiEncodedCleartext: string;
  readonly signatures: readonly string[];
  readonly extraData: string;
  readonly inclusionProof: MmrProof;
};

/**
 * `extraData` version byte carrying the context id plus the MMR-proof tail (RFC-024): version ‖
 * contextId(32) ‖ aclValueKey(32) ‖ proofSlot(8 BE) ‖ mmrProofLen(4 BE) ‖ mmrProofBytes. Mirrors
 * `SOLANA_EXTRA_DATA_VERSION_MMR_PROOF` in the connector's `solana_extra_data.rs`.
 */
export const SOLANA_MMR_PROOF_EXTRA_DATA_VERSION = 0x03;

/** Encodes a u32 as 4 big-endian bytes (`u32::to_be_bytes`). */
function u32BE(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, false);
  return out;
}

/**
 * Builds the MMR-proof-tail `extraData` a public-decrypt request carries on the wire.
 *
 * Owned by public decrypt: user decrypt stopped carrying this blob when its requests moved to the
 * permit envelope — its proofs travel as bare borsh beside the handle, and its `extraData` is the
 * version-`0x02` KMS routing. The Rust half of this hand-mirrored codec is
 * `encode_solana_extra_data_mmr_proof` in the connector's `solana_extra_data.rs`; the two layouts
 * must change together, and `solana/test-fixtures/user-decrypt/extra_data_v1.json` is what pins
 * them to each other.
 *
 * @param contextId - The 32-byte KMS context id.
 * @param aclValueKey - The 32-byte ACL value key naming the encrypted-value account.
 * @param proofSlot - The slot the proof was built at.
 * @param mmrProofBytes - The canonical transport proof blob, verbatim.
 */
export function buildSolanaPublicDecryptMmrProofExtraData(
  contextId: Uint8Array,
  aclValueKey: Uint8Array,
  proofSlot: bigint,
  mmrProofBytes: Uint8Array,
): Uint8Array {
  assertExtraDataFieldLen('contextId', contextId, 32);
  assertExtraDataFieldLen('aclValueKey', aclValueKey, 32);
  return concatBytes(
    new Uint8Array([SOLANA_MMR_PROOF_EXTRA_DATA_VERSION]),
    contextId,
    aclValueKey,
    u64BE(proofSlot),
    u32BE(mmrProofBytes.length),
    mmrProofBytes,
  );
}

function assertExtraDataFieldLen(name: string, bytes: Uint8Array, len: number): void {
  if (bytes.length !== len) {
    throw new Error(`${name} must be ${len} bytes, got ${bytes.length}`);
  }
}

/** Requests a public-decrypt certificate after verifying its pinned MMR inclusion locally. */
export async function publicDecryptCertificate(
  context: SolanaPublicDecryptCertificateContext,
  parameters: SolanaPublicDecryptCertificateParameters,
): Promise<SolanaPublicDecryptCertificateClaim> {
  const handle = toFhevmHandle(parameters.handle);
  const decoded = decodeMmrProofTransportBlob(parameters.mmrProofBytes);
  if (decoded.mode !== MMR_PROOF_MODE_PUBLIC) {
    throw new Error(`public-decrypt MMR proof must use mode 0x02, got 0x${decoded.mode.toString(16).padStart(2, '0')}`);
  }
  if (parameters.proofSlot !== parameters.leafCount) {
    throw new Error(
      `public-decrypt proof slot must equal the pinned leaf count: ${parameters.proofSlot} != ${parameters.leafCount}`,
    );
  }
  if (
    !verifyPublicDecryptProof(
      parameters.encryptedValueAccount,
      parameters.peaks,
      parameters.leafCount,
      handle.bytes32,
      decoded.proof,
    )
  ) {
    throw new Error('public-decrypt MMR proof failed client-side verification');
  }

  const requestExtraData = buildSolanaPublicDecryptMmrProofExtraData(
    parameters.contextId,
    parameters.aclValueKey,
    parameters.proofSlot,
    parameters.mmrProofBytes,
  );
  const requestExtraDataHex = bytesToHex(requestExtraData);
  const request = new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url: `${removeSuffix(context.chain.fhevm.relayerUrl, '/')}/v2/public-decrypt`,
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
    inclusionProof: decoded.proof,
  };
}
