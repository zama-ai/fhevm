import { getProgramDerivedAddress, type Address, type Instruction } from '@solana/kit';

import type { SolanaPublicDecryptCertificateClaim } from '@sdk-src/solana/actions/publicDecryptCertificate.js';
import type { MmrProof } from '@sdk-src/solana/proof.js';
import { verifyPublicDecryptArgsFromClaim } from '@sdk-src/solana/actions/verifyPublicDecrypt.js';
import { getDiscloseSecpInstruction } from '../internal/generated/confidentialToken/instructions/discloseSecp.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';

const EVENT_AUTHORITY_SEED = new TextEncoder().encode('__event_authority');

/** Accounts for the confidential-token `disclose_secp` consume instruction. */
export type SolanaDiscloseSecpAccounts = {
  /** Confidential mint: the scope the disclosed encrypted value account and event carry. */
  readonly mint: Address;
  /** Confidential token account for account-scoped state; omit only for total supply. */
  readonly tokenAccount?: Address | undefined;
  /** The encrypted store whose history authorizes the disclosed handle. */
  readonly encryptedStore: Address;
  /** KMS context PDA for the host's current context id. */
  readonly kmsContext: Address;
  /** ZamaHost config account forwarded to the host verifier. */
  readonly hostConfig: Address;
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
 * Builds the confidential-token `disclose_secp` consume instruction from a certificate claim and
 * the public leaf's inclusion proof (built by the caller from the account's history with
 * `mmrBuildProof`). The instruction CPIs the stateless host verifier, asserts the proven handle
 * equals the pinned handle, and emits a token-scoped `HandleDisclosedEvent`.
 *
 * Disclosure is idempotent by design — there is no on-chain replay marker — so this instruction can
 * be submitted more than once for the same handle without failing; act-once, if needed, is the
 * consuming app's concern.
 */
export async function buildDiscloseSecpInstruction(
  accounts: SolanaDiscloseSecpAccounts,
  claim: SolanaPublicDecryptCertificateClaim,
  inclusionProof: MmrProof,
): Promise<Instruction> {
  const args = verifyPublicDecryptArgsFromClaim(claim, inclusionProof);
  return getDiscloseSecpInstruction({
    mint: accounts.mint,
    ...(accounts.tokenAccount !== undefined ? { tokenAccount: accounts.tokenAccount } : {}),
    encryptedStore: accounts.encryptedStore,
    kmsContext: accounts.kmsContext,
    hostConfig: accounts.hostConfig,
    eventAuthority: await tokenEventAuthority(),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    handle: args.handle,
    cleartext: args.cleartext,
    signatures: [...args.signatures],
    extraData: args.extraData,
    proof: { leafIndex: args.leafIndex, siblings: [...args.siblings] },
  });
}
