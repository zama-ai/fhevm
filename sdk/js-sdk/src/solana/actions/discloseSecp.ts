import { getProgramDerivedAddress, type Address, type Instruction } from '@solana/kit';

import type { SolanaPublicDecryptCertificateClaim } from './publicDecryptCertificate.js';
import { verifyPublicDecryptArgsFromClaim } from './verifyPublicDecrypt.js';
import { getDiscloseSecpInstructionAsync } from '../internal/generated/confidentialToken/instructions/discloseSecp.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';

const EVENT_AUTHORITY_SEED = new TextEncoder().encode('__event_authority');

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
