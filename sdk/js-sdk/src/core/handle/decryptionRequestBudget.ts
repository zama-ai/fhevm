// The decryption-request volume budget, as the Gateway defines it.
//
// A request is bounded by the summed cleartext bit width of the handles it names, not by their
// count: `MAX_DECRYPTION_REQUEST_BITS` in the Gateway's `Decryption.sol`, summed through
// `FHETypeBitSizes.getBitSize`. The Gateway enforces it on an EVM request; a Solana request never
// reaches the Gateway, and the Connector's endpoint enforces the same budget. The rule is the same
// for every host, so it lives here rather than beside any one host's request builder.
//
// What this module is for is the pre-check every layer before enforcement owes its caller: a client
// that learns the limit from a refusal has already spent the attempt.
//
// No width table is defined here. The one the SDK already carries is used as it stands, and the
// parity test reads the Solidity library to check the two still agree; a mirror nobody checks is
// worth less than no mirror at all.

import { isBytes32 } from '../base/bytes.js';
import { bytes32ToHandle } from './FhevmHandle.js';

/** The budget: `MAX_DECRYPTION_REQUEST_BITS` in the Gateway's `Decryption.sol`. */
export const MAX_DECRYPTION_REQUEST_BITS = 2048;

/**
 * The cleartext bit width one handle costs, or `undefined` if it has none.
 *
 * A handle has no width when it is not 32 bytes, or when its FHE type is one the protocol assigns no
 * size to. An unknown type is not a free handle: both enforcement points refuse it, so a pre-check
 * that counted it as zero would wave through a request that is then refused. The two cases are
 * reported the same way because the caller does the same thing with both — refuse before sending —
 * and it is the caller that knows how to name the handle it was given.
 *
 * @param handle - The 32-byte ciphertext handle.
 */
export function decryptionRequestBitsOfHandle(handle: Uint8Array): number | undefined {
  if (!isBytes32(handle)) {
    return undefined;
  }
  try {
    // Goes through the handle type rather than reading byte 30 here: the offset, the version rule
    // and the width table are one piece of knowledge, and this module is not a second copy of it.
    return bytes32ToHandle(handle).encryptionBits;
  } catch {
    return undefined;
  }
}
