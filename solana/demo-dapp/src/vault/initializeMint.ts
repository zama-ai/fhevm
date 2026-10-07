import { tokenStoreAddress } from './internal/encryptedStores.js';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';
import {
  getInitializeMintInstructionAsync,
  findTotalSupplyAuthorityPda,
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
} from '@fhevm/confidential-token';

export type SolanaVaultInitializeMintParameters = {
  readonly transientStore: TransientStore;
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
 * and its initial (zero) total-supply handle. The total-supply encrypted store is derived from the
 * mint here, so the seeder supplies only semantic roots. The seeder assembles and sends the
 * returned instruction.
 */
export async function buildInitializeMintInstruction(
  parameters: SolanaVaultInitializeMintParameters,
): Promise<Instruction> {
  const [totalSupplyAuthority] = await findTotalSupplyAuthorityPda({ mint: parameters.mint.address });
  return getInitializeMintInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    authority: parameters.authority,
    mint: parameters.mint,
    underlyingMint: parameters.underlyingMint,
    totalSupplyEncryptedStore: await tokenStoreAddress(parameters.mint.address, totalSupplyAuthority),
    hostConfig: parameters.hostConfig,
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  });
}
