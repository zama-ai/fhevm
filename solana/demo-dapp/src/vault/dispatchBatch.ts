import { tokenStoreAddress } from './internal/encryptedStores.js';
import {
  findPendingBurnPda,
  findEventAuthorityPda as findTokenEventAuthorityPda,
  findTotalSupplyAuthorityPda,
} from '@fhevm/confidential-token';
import { findAssociatedTokenPda, TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Instruction, TransactionSigner } from '@solana/kit';
import { getDispatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { tokenApp, withDenyRecords, type HostPolicyParameters } from './internal/hostPolicy.js';
import { deriveBatchAddresses, type VaultDemoRoots } from './derive.js';


/**
 * Semantic roots for the batcher `dispatch` instruction. Every account the on-chain handler
 * validates (`dispatch.rs`) — both batches' authorities, token accounts and encrypted stores, the join
 * mint's total-supply authority and pending burn, and both event authorities — is derived internally
 * from the roots and the batch index, so callers never hand-build the account map.
 */
export type SolanaVaultDispatchParameters = HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /**
   * Pays the burn's output rent, the next batch's rent and its authority funding. Anyone — dispatch is
   * permissionless.
   */
  readonly payer: TransactionSigner;
  /** The batcher's immutable topology (from the demo-config projection). */
  readonly roots: VaultDemoRoots;
  /** Index of the pending batch being dispatched; dispatch opens `batchIndex + 1`. */
  readonly batchIndex: bigint;
  /** Lamports the next batch's authority is funded with to pay its owner-charged rent. */
  readonly authorityFundingLamports: bigint;
};

/**
 * Builds the permissionless `dispatch` instruction: once a batch is old enough, it burns the batch
 * account's full encrypted balance, records the created-public burned handle the KMS will certify at
 * settle, and opens the next batch.
 */
export async function buildDispatchBatchInstruction(parameters: SolanaVaultDispatchParameters): Promise<Instruction> {
  const { roots, batchIndex } = parameters;
  const { joinConfidentialMint, payoutConfidentialMint } = roots;
  const [batch, next] = await Promise.all([
    deriveBatchAddresses(roots, batchIndex),
    deriveBatchAddresses(roots, batchIndex + 1n),
  ]);
  const totalSupplyAuthority = (await findTotalSupplyAuthorityPda({ mint: joinConfidentialMint }))[0];
  const joinMint = tokenApp(joinConfidentialMint);
  const payoutMint = tokenApp(payoutConfidentialMint);
  const [joinMintHcu, payoutMintHcu] = await Promise.all([
    parameters.host.hcuAccounts(joinMint),
    parameters.host.hcuAccounts(payoutMint),
  ]);
  const instruction = await getDispatchInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer: parameters.payer,
    batcher: roots.batcher,
    batch: batch.batch,
    batchAuthority: batch.batchAuthority,
    joinConfidentialMint,
    joinUnderlyingMint: roots.joinUnderlyingMint,
    batchAuthorityAta: (await findAssociatedTokenPda({
      owner: batch.batchAuthority,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
      mint: roots.joinUnderlyingMint,
    }))[0],
    totalSupplyAuthority,
    batchJoinTokenAccount: batch.batchJoinTokenAccount,
    // A token account has one encrypted store, holding both its balance and its burned amount.
    batchBalanceStore: batch.batchBurnedAmountStore,
    totalSupplyStore: await tokenStoreAddress(joinConfidentialMint, totalSupplyAuthority),
    pendingBurn: (await findPendingBurnPda({ mint: joinConfidentialMint, tokenAccount: batch.batchJoinTokenAccount }))[0],
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    nextBatch: next.batch,
    nextBatchJoinTokenAccount: next.batchJoinTokenAccount,
    nextBatchJoinBalanceStore: next.batchBurnedAmountStore,
    payoutConfidentialMint,
    payoutUnderlyingMint: roots.payoutUnderlyingMint,
    nextBatchPayoutTokenAccount: next.batchPayoutTokenAccount,
    nextBatchPayoutBalanceStore: next.batchPayoutBalanceStore,
    authorityFundingLamports: parameters.authorityFundingLamports,
    joinMintHcuBlockMeter: joinMintHcu.hcuBlockMeter,
    joinMintHcuTrustedAppRecord: joinMintHcu.hcuTrustedAppRecord,
    payoutMintHcuBlockMeter: payoutMintHcu.hcuBlockMeter,
    payoutMintHcuTrustedAppRecord: payoutMintHcu.hcuTrustedAppRecord,
  });
  // The burn, then the next batch's join and payout token accounts.
  return withDenyRecords(instruction, parameters.host.denyListEnabled, [joinMint, joinMint, payoutMint]);
}
