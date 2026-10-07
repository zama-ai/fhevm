import { type Address, type Instruction } from '@solana/kit';

import type { SolanaPublicDecryptCertificateClaim } from '@fhevm/sdk/solana';
import { verifyPublicDecryptArgsFromClaim } from '@fhevm/sdk/solana';
import { getDiscloseSecpInstructionAsync, findEventAuthorityPda, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';

/** Accounts for the confidential-token `disclose_secp` consume instruction. */
export type SolanaDiscloseSecpAccounts = {
  /** KMS context PDA for the context id the certificate commits to. */
  readonly kmsContext: Address;
};

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
  return getDiscloseSecpInstructionAsync({
    kmsContext: accounts.kmsContext,
    eventAuthority: (await findEventAuthorityPda())[0],
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    handle: args.handle,
    cleartext: args.cleartext,
    signatures: [...args.signatures],
    extraData: args.extraData,
  });
}
