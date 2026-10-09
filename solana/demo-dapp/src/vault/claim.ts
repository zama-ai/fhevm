import {
  findTokenAccountPda,
  findEventAuthorityPda as findTokenEventAuthorityPda,
} from '@fhevm/confidential-token';
import { findAssociatedTokenPda } from '@solana-program/token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction, TransactionSigner } from '@solana/kit';
import { getClaimInstructionAsync } from './internal/generated/confidentialBatcher/instructions/claim.js';
import { batchApp, tokenApp, withDenyRecords, type HostPolicyParameters } from './internal/hostPolicy.js';
import { findBatchAuthorityPda } from './internal/generated/confidentialBatcher/pdas/index.js';
import { joinStoreAddress, tokenStoreAddress } from './internal/encryptedStores.js';

/**
 * Roots for a permissionless claim. The builder derives the JoinRecord and payout States,
 * and forwards the supplied transient store. The user need not sign: the recipient is fixed by the JoinRecord.
 * The user's payout token account must already exist.
 */
export type SolanaVaultClaimParameters = HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /** Pays State growth. The transient store payer may be a different sponsor. */
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
};

/**
 * Builds the permissionless `claim` instruction: computes the user's exact proportional payout
 * (`encrypted(joined) * payout_received / total_joined`, one MulDiv batch) and transfers it to the
 * user.
 */
export async function buildClaimInstruction(parameters: SolanaVaultClaimParameters): Promise<Instruction> {
  const { user, payoutConfidentialMint } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchPayoutTokenAccount = (await findTokenAccountPda({ mint: payoutConfidentialMint, owner: batchAuthority }))[0];
  const userPayoutTokenAccount = (await findTokenAccountPda({ mint: payoutConfidentialMint, owner: user }))[0];
  const joinStore = await joinStoreAddress(parameters.batch, user);
  const batch = batchApp(parameters.batch);
  const payoutMint = tokenApp(payoutConfidentialMint);
  const [batchHcu, payoutMintHcu] = await Promise.all([
    parameters.host.hcuAccounts(batch),
    parameters.host.hcuAccounts(payoutMint),
  ]);
  const instruction = await getClaimInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: parameters.payer,
    user,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    joinStore,
    payoutConfidentialMint,
    payoutUnderlyingMint: parameters.payoutUnderlyingMint,
    batchAuthorityPayoutAta: (await findAssociatedTokenPda({
      owner: batchAuthority,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.payoutUnderlyingMint,
    }))[0],
    userPayoutAta: (await findAssociatedTokenPda({
      owner: user,
      tokenProgram: parameters.tokenProgram,
      mint: parameters.payoutUnderlyingMint,
    }))[0],
    batchPayoutTokenAccount,
    userPayoutTokenAccount,
    batchPayoutBalanceStore: await tokenStoreAddress(payoutConfidentialMint, batchPayoutTokenAccount),
    userPayoutBalanceStore: await tokenStoreAddress(payoutConfidentialMint, userPayoutTokenAccount),
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    batchHcuBlockMeter: batchHcu.hcuBlockMeter,
    batchHcuTrustedAppRecord: batchHcu.hcuTrustedAppRecord,
    payoutMintHcuBlockMeter: payoutMintHcu.hcuBlockMeter,
    payoutMintHcuTrustedAppRecord: payoutMintHcu.hcuTrustedAppRecord,
  });
  return withDenyRecords(instruction, parameters.host.denyListEnabled, [batch, payoutMint]);
}
