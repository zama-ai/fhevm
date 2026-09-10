import { closeScratchInstruction, INSTRUCTIONS_SYSVAR } from './internal/scratch.js';
import { scratchAddress } from './internal/batcherPdas.js';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getClaimInstructionAsync } from './internal/generated/confidentialBatcher/instructions/claim.js';
import { findBatchAuthorityPda, joinStateAddress, tokenAccountAddress } from './internal/batcherPdas.js';
import {
  associatedTokenAddress,
  tokenStateAddress,
  tokenEventAuthorityAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenAccounts.js';

/**
 * Roots for a permissionless claim. The builder derives the JoinRecord state, temporary scratch,
 * and payout token states. The user need not sign: the recipient is fixed by the JoinRecord.
 * The user's payout token account must already exist.
 */
export type SolanaVaultClaimParameters = {
  /** Pays state growth and temporary scratch rent; scratch rent is refunded at transaction end. */
  readonly payer: TransactionSigner;
  /** The user being claimed for (pins the join record). Not a signer. */
  readonly user: Address;
  /** Batcher config account. */
  readonly batcher: Address;
  /** The settled batch being claimed from. */
  readonly batch: Address;
  /** Confidential mint claims pay out in (`batcher.payout_confidential_mint`). */
  readonly payoutConfidentialMint: Address;
  /** SPL mint wrapped by `payoutConfidentialMint`. Freeze checks the payout owners' ATAs on this mint. */
  readonly payoutUnderlyingMint: Address;
  /** Token program that owns `payoutUnderlyingMint` (`Tokenkeg` or Token-2022). */
  readonly tokenProgram: Address;
  /** ZamaHost config PDA (demo-config `hostConfig`). */
  readonly hostConfig: Address;
};

/**
 * Builds the permissionless `claim` instruction: computes the user's exact proportional payout
 * (`encrypted(joined) * payout_received / total_joined`, one MulDiv batch) and transfers it to the
 * user.
 */
export async function buildClaimInstructions(parameters: SolanaVaultClaimParameters): Promise<readonly Instruction[]> {
  const { user, payoutConfidentialMint } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchPayoutTokenAccount = await tokenAccountAddress(payoutConfidentialMint, batchAuthority);
  const userPayoutTokenAccount = await tokenAccountAddress(payoutConfidentialMint, user);
  const joinState = await joinStateAddress(parameters.batch, user);
  const scratch = await scratchAddress(joinState);
  const instruction = await getClaimInstructionAsync({
    payer: parameters.payer,
    user,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinState,
    scratch,
    instructions: INSTRUCTIONS_SYSVAR,
    payoutConfidentialMint,
    payoutUnderlyingMint: parameters.payoutUnderlyingMint,
    batchAuthorityPayoutAta: await associatedTokenAddress(
      batchAuthority,
      parameters.payoutUnderlyingMint,
      parameters.tokenProgram,
    ),
    userPayoutAta: await associatedTokenAddress(user, parameters.payoutUnderlyingMint, parameters.tokenProgram),
    batchPayoutTokenAccount,
    userPayoutTokenAccount,
    batchPayoutBalanceState: await tokenStateAddress(payoutConfidentialMint, batchPayoutTokenAccount),
    userPayoutBalanceState: await tokenStateAddress(payoutConfidentialMint, userPayoutTokenAccount),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
  });
  return [instruction, closeScratchInstruction(scratch, parameters.payer.address)];
}
