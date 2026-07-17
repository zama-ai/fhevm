import { getProgramDerivedAddress, type Address, type Instruction } from '@solana/kit';

import { hexToBytes } from '../../core/base/bytes.js';
import type { SolanaPublicDecryptCertificateClaim } from './publicDecryptCertificate.js';
import { getDiscloseSecpInstructionAsync } from '../internal/generated/confidentialToken/instructions/discloseSecp.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import { getVerifyPublicDecryptInstructionAsync } from '../internal/generated/zamaHost/instructions/verifyPublicDecrypt.js';

const EVENT_AUTHORITY_SEED = new TextEncoder().encode('__event_authority');

/**
 * The certificate + inclusion-proof payload the stateless host verifier consumes, decoded from a
 * relayer [`SolanaPublicDecryptCertificateClaim`] into the exact wire types the generated
 * `verify_public_decrypt` / `disclose_secp` instruction builders expect.
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
 * is [`buildDiscloseSecpInstruction`]). Async because the host config account defaults to its PDA
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

/** Accounts for the confidential-token `disclose_secp` consume instruction. */
export type SolanaDiscloseSecpAccounts = {
  /** Confidential mint whose ACL domain scopes the disclosed lineage and event. */
  readonly mint: Address;
  /** The `EncryptedValue` lineage the disclosed handle belongs to. */
  readonly encryptedValue: Address;
  /** KMS context PDA for the host's current context id. */
  readonly kmsContext: Address;
  /** Host config; defaults to the host config PDA when omitted. */
  readonly hostConfig?: Address | undefined;
};

async function tokenEventAuthority(): Promise<Address> {
  return (
    await getProgramDerivedAddress({
      programAddress: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      seeds: [EVENT_AUTHORITY_SEED],
    })
  )[0];
}

/**
 * Builds the confidential-token `disclose_secp` consume instruction from a certificate claim. The
 * instruction CPIs the stateless host verifier, asserts the proven handle equals the pinned handle,
 * and emits a token-scoped `HandleDisclosedEvent`.
 *
 * Disclosure is idempotent by design — there is no on-chain replay marker — so this instruction can
 * be submitted more than once for the same handle without failing; act-once, if needed, is the
 * consuming app's concern.
 */
export async function buildDiscloseSecpInstruction(
  accounts: SolanaDiscloseSecpAccounts,
  claim: SolanaPublicDecryptCertificateClaim,
): Promise<Instruction> {
  const args = verifyPublicDecryptArgsFromClaim(claim);
  return getDiscloseSecpInstructionAsync({
    mint: accounts.mint,
    encryptedValue: accounts.encryptedValue,
    kmsContext: accounts.kmsContext,
    ...(accounts.hostConfig !== undefined ? { hostConfig: accounts.hostConfig } : {}),
    eventAuthority: await tokenEventAuthority(),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    handle: args.handle,
    cleartext: args.cleartext,
    signatures: [...args.signatures],
    extraData: args.extraData,
    leafIndex: args.leafIndex,
    siblings: [...args.siblings],
  });
}
