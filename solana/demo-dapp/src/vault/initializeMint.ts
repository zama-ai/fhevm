import type { SolanaFheTransactionAccounts } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getInitializeMintInstructionAsync } from './internal/generated/confidentialToken/instructions/initializeMint.js';
import { findTotalSupplyAuthorityPda } from './internal/generated/confidentialToken/pdas/totalSupplyAuthority.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './internal/generated/confidentialToken/programAddress.js';
import { tokenStateAddress, tokenEventAuthorityAddress, zamaEventAuthorityAddress } from './internal/tokenAccounts.js';

export type SolanaVaultInitializeMintParameters = {
  readonly fhe: SolanaFheTransactionAccounts;
  /** Mint authority and rent payer. */
  readonly authority: TransactionSigner;
  /** The confidential mint account created here (a fresh keypair signs its own creation). */
  readonly mint: TransactionSigner;
  /** The underlying SPL mint this confidential mint wraps. */
  readonly underlyingMint: Address;
  /** zama-host config PDA used for handle derivation. */
  readonly hostConfig: Address;
};

/**
 * Builds `confidential_token::initialize_mint`: creates a confidential mint wrapping `underlyingMint`
 * and its initial (zero) total-supply handle. The total-supply encrypted State and the two Anchor
 * event authorities are derived from the mint here, so the seeder supplies only semantic roots. The
 * seeder assembles and sends the returned instruction.
 */
export async function buildInitializeMintInstruction(
  parameters: SolanaVaultInitializeMintParameters,
): Promise<Instruction> {
  const [totalSupplyAuthority] = await findTotalSupplyAuthorityPda({ mint: parameters.mint.address });
  return getInitializeMintInstructionAsync({
    ...parameters.fhe,
    authority: parameters.authority,
    mint: parameters.mint,
    underlyingMint: parameters.underlyingMint,
    totalSupplyEncryptedState: await tokenStateAddress(parameters.mint.address, totalSupplyAuthority),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    eventAuthority: await tokenEventAuthorityAddress(),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  });
}
