import { getProgramDerivedAddress, type Address, type Instruction } from '@solana/kit';

import type { SolanaPublicDecryptCertificateClaim } from '@fhevm/sdk/solana';
import { verifyPublicDecryptArgsFromClaim } from '@fhevm/sdk/solana';
import { getDiscloseSecpInstruction, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';

const EVENT_AUTHORITY_SEED = new TextEncoder().encode('__event_authority');

/** Accounts for the confidential-token `disclose_secp` consume instruction. */
export type SolanaDiscloseSecpAccounts = {
  /** KMS context PDA for the context id the certificate commits to. */
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
 * Builds the confidential-token `disclose_secp` instruction from a certificate claim. The
 * instruction CPIs the stateless host verifier and emits `HandleDisclosedEvent { handle,
 * cleartext_amount }`, as ERC-7984 `discloseEncryptedAmount` does. A reader links the handle to a
 * token account through the token's handle history.
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
  return getDiscloseSecpInstruction({
    kmsContext: accounts.kmsContext,
    hostConfig: accounts.hostConfig,
    eventAuthority: await tokenEventAuthority(),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    handle: args.handle,
    cleartext: args.cleartext,
    signatures: [...args.signatures],
    extraData: args.extraData,
  });
}
