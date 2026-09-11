import type { SolanaFheTransactionAccounts } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getClaimInstructionAsync } from './internal/generated/confidentialBatcher/instructions/claim.js';
import { findBatchAuthorityPda, joinStoreAddress, tokenAccountAddress } from './internal/batcherPdas.js';
import {
  associatedTokenAddress,
  tokenStateAddress,
  tokenEventAuthorityAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenAccounts.js';

/**
 * Roots for a permissionless claim. The builder derives the JoinRecord and payout States,
 * and forwards the supplied transaction context. The user need not sign: the recipient is fixed by the JoinRecord.
 * The user's payout token account must already exist.
 */
export type SolanaVaultClaimParameters = {
  readonly fhe: SolanaFheTransactionAccounts;
  /** Pays State growth. The supplied FHE transaction may have a different transientStore sponsor. */
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
export async function buildClaimInstruction(parameters: SolanaVaultClaimParameters): Promise<Instruction> {
  const { user, payoutConfidentialMint } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchPayoutTokenAccount = await tokenAccountAddress(payoutConfidentialMint, batchAuthority);
  const userPayoutTokenAccount = await tokenAccountAddress(payoutConfidentialMint, user);
  const joinStore = await joinStoreAddress(parameters.batch, user);
  const instruction = await getClaimInstructionAsync({
    ...parameters.fhe,
    payer: parameters.payer,
    user,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinStore,
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
    batchPayoutBalanceStore: await tokenStateAddress(payoutConfidentialMint, batchPayoutTokenAccount),
    userPayoutBalanceStore: await tokenStateAddress(payoutConfidentialMint, userPayoutTokenAccount),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
  });
  return instruction;
}
