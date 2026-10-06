import type { Address, Instruction } from '@solana/kit';

import { hexToBytes } from '../../core/base/bytes.js';
import type { SolanaPublicDecryptCertificateClaim } from './publicDecryptCertificate.js';
import { getVerifyPublicDecryptInstructionAsync } from '@fhevm/solana-zama-host';

/**
 * The certificate payload the stateless host verifier consumes, decoded from a relayer
 * [`SolanaPublicDecryptCertificateClaim`] into the exact wire types the generated
 * `verify_public_decrypt` instruction builder expects.
 */
export type SolanaVerifyPublicDecryptArgs = {
  readonly handle: Uint8Array;
  readonly cleartext: Uint8Array;
  readonly signatures: readonly Uint8Array[];
  readonly extraData: Uint8Array;
};

/**
 * Decodes a relayer public-decrypt certificate claim into the verifier instruction args, validating
 * the fixed-size fields. `handle` and `cleartext` are the 32-byte handle and 32-byte big-endian
 * `uint256` cleartext the KMS signed over; each signature is a 65-byte secp256k1 recoverable
 * signature.
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
    if (bytes.length !== 65)
      throw new Error(`public-decrypt signature[${index}] must be 65 bytes, got ${bytes.length}`);
    return bytes;
  });
  return {
    handle,
    cleartext,
    signatures,
    extraData: hexToBytes(claim.extraData),
  };
}

/** Accounts for the generic, program-agnostic host `verify_public_decrypt` instruction. */
export type SolanaVerifyPublicDecryptAccounts = {
  /** Canonical singleton host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
  /** The zama-host program id of the deployment (`solanaHostProgram(chain)`). */
  readonly programAddress: Address;
  /** KMS context PDA for the id the certificate commits to (any live, non-destroyed context). */
  readonly kmsContext: Address;
};

/**
 * Builds the raw, stateless `zama_host::verify_public_decrypt` instruction from a certificate claim.
 * The verifier checks the KMS certificate, reads no ACL state, and returns
 * `(handle, cleartext, context_id)` via `return_data`; it creates and mutates nothing. A consuming
 * program binds the result to its own state by comparing the handle with one it pinned. Use this
 * when consuming the verifier from a non-token program. The confidential-token wrapper that discloses
 * through the token program is not part of this SDK — it lives with the vault demo dapp. Async
 * because the host config account defaults to its PDA when omitted.
 */
export async function buildVerifyPublicDecryptInstruction(
  accounts: SolanaVerifyPublicDecryptAccounts,
  claim: SolanaPublicDecryptCertificateClaim,
): Promise<Instruction> {
  const args = verifyPublicDecryptArgsFromClaim(claim);
  return getVerifyPublicDecryptInstructionAsync(
    {
      ...(accounts.hostConfig !== undefined ? { hostConfig: accounts.hostConfig } : {}),
      kmsContext: accounts.kmsContext,
      handle: args.handle,
      cleartext: args.cleartext,
      signatures: [...args.signatures],
      extraData: args.extraData,
    },
    { programAddress: accounts.programAddress },
  );
}
