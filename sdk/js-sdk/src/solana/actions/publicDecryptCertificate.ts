import type { EncryptedValueLike } from '../../core/types/encryptedTypes.js';
import type { RelayerPublicDecryptOptions } from '../../core/types/relayer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { MmrProof } from '../proof.js';
import { bytesToHex } from '../../core/base/bytes.js';
import { buildSolanaUserDecryptMmrProofExtraData } from '../../core/coprocessor/SolanaUserDecrypt-p.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { RelayerAsyncRequest } from '../../core/modules/relayer/module/RelayerAsyncRequest.js';
import { removeSuffix } from '../../core/base/string.js';
import {
  decodeMmrProofTransportBlob,
  hexToBytes,
  MMR_MODE_PUBLIC,
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
 * after a `disclose_*_secp` instruction verifies this certificate on-chain against the
 * witness-pinned `KmsContext`.
 */
export type SolanaPublicDecryptCertificateClaim = {
  readonly handle: string;
  /** Raw ABI-encoded cleartext returned by the relayer. It is intentionally not interpreted. */
  readonly abiEncodedCleartext: string;
  readonly signatures: readonly string[];
  readonly extraData: string;
  readonly inclusionProof: MmrProof;
};

function bytesEqual(left: Uint8Array, right: Uint8Array): boolean {
  if (left.length !== right.length) return false;
  return left.every((byte, index) => byte === right[index]);
}

/** Requests a public-decrypt certificate after verifying its pinned MMR inclusion locally. */
export async function publicDecryptCertificate(
  context: SolanaPublicDecryptCertificateContext,
  parameters: SolanaPublicDecryptCertificateParameters,
): Promise<SolanaPublicDecryptCertificateClaim> {
  const handle = toFhevmHandle(parameters.handle);
  const decoded = decodeMmrProofTransportBlob(parameters.mmrProofBytes);
  if (decoded.mode !== MMR_MODE_PUBLIC) {
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

  const requestExtraData = buildSolanaUserDecryptMmrProofExtraData(
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

  if (result.extraData === undefined) {
    throw new Error('public-decrypt response is missing extraData');
  }
  if (!bytesEqual(hexToBytes(result.extraData), requestExtraData)) {
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
      throw new Error(`public-decrypt signature must be valid 65-byte hex, got ${signature.length / 2} bytes`);
    }
  }

  return {
    handle: handle.bytes32Hex,
    abiEncodedCleartext: result.decryptedValue,
    signatures: result.signatures,
    extraData: result.extraData,
    inclusionProof: decoded.proof,
  };
}
