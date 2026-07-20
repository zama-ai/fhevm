import type { Address, Instruction } from '@solana/kit';

import { hexToBytes } from '../../core/base/bytes.js';
import type { SolanaPublicDecryptCertificateClaim } from './publicDecryptCertificate.js';
import { getVerifyPublicDecryptInstructionAsync } from '../internal/generated/zamaHost/instructions/verifyPublicDecrypt.js';

/**
 * The certificate + inclusion-proof payload the stateless host verifier consumes, decoded from a
 * relayer [`SolanaPublicDecryptCertificateClaim`] into the exact wire types the generated
 * `verify_public_decrypt` instruction builder expects.
 */
export type SolanaVerifyPublicDecryptArgs = {
  readonly handle: Uint8Array;
  readonly cleartext: Uint8Array;
  readonly signatures: readonly Uint8Array[];
  readonly extraData: Uint8Array;
  readonly leafIndex: bigint;
  readonly siblings: readonly Uint8Array[];
};

/**
 * Decodes a relayer public-decrypt certificate claim into the verifier instruction args, validating
 * the fixed-size fields. `handle` and `cleartext` are the 32-byte handle and 32-byte big-endian
 * `uint256` cleartext the KMS signed over; each signature is a 65-byte secp256k1 recoverable
 * signature; the proof is the MMR public-leaf inclusion path for `handle`.
 */
export function verifyPublicDecryptArgsFromClaim(
  claim: SolanaPublicDecryptCertificateClaim,
): SolanaVerifyPublicDecryptArgs {
  const handle = hexToBytes(claim.handle);
  if (handle.length !== 32) throw new Error(`public-decrypt handle must be 32 bytes, got ${handle.length}`);
  const cleartext = hexToBytes(claim.abiEncodedCleartext);
  if (cleartext.length !== 32) {
    throw new Error(`public-decrypt cleartext must be a 32-byte uint256, got ${cleartext.length} bytes`);
  }
  const signatures = claim.signatures.map((signature, index) => {
    const bytes = hexToBytes(signature);
    if (bytes.length !== 65) throw new Error(`public-decrypt signature[${index}] must be 65 bytes, got ${bytes.length}`);
    return bytes;
  });
  return {
    handle,
    cleartext,
    signatures,
    extraData: hexToBytes(claim.extraData),
    leafIndex: claim.inclusionProof.leafIndex,
    siblings: [...claim.inclusionProof.siblings],
  };
}

/** Accounts for the generic, program-agnostic host `verify_public_decrypt` instruction. */
export type SolanaVerifyPublicDecryptAccounts = {
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
  /** KMS context PDA for the host's current context id. */
  readonly kmsContext: Address;
  /** The `EncryptedValue` lineage the inclusion proof is checked against. */
  readonly encryptedValue: Address;
};

/**
 * Builds the raw, stateless `zama_host::verify_public_decrypt` instruction from a certificate claim.
 * The verifier reads state and returns `(handle, cleartext)` via `return_data`; it creates and
 * mutates nothing. Use this when consuming the verifier from a non-token program (the token wrapper
 * is `buildDiscloseSecpInstruction`). Async because the host config account defaults to its PDA
 * when omitted.
 */
export async function buildVerifyPublicDecryptInstruction(
  accounts: SolanaVerifyPublicDecryptAccounts,
  claim: SolanaPublicDecryptCertificateClaim,
): Promise<Instruction> {
  const args = verifyPublicDecryptArgsFromClaim(claim);
  return getVerifyPublicDecryptInstructionAsync({
    ...(accounts.hostConfig !== undefined ? { hostConfig: accounts.hostConfig } : {}),
    kmsContext: accounts.kmsContext,
    encryptedValue: accounts.encryptedValue,
    handle: args.handle,
    cleartext: args.cleartext,
    signatures: [...args.signatures],
    extraData: args.extraData,
    leafIndex: args.leafIndex,
    siblings: [...args.siblings],
  });
}
