import { tokenStoreAddress } from './internal/encryptedStores.js';
import { findBatchPda } from './internal/generated/confidentialBatcher/pdas/index.js';
import {
  findEventAuthorityPda as findTokenEventAuthorityPda,
} from '@fhevm/confidential-token';
import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Instruction, TransactionSigner } from '@solana/kit';
import { tokenApp, withDenyRecords, type HostPolicyParameters } from './internal/hostPolicy.js';
import { getOpenBatchInstructionAsync } from './internal/generated/confidentialBatcher/instructions/openBatch.js';
import { deriveBatchAddresses, type VaultDemoRoots } from './derive.js';

export type SolanaVaultOpenBatchForBatcherParameters = HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /** The batcher's immutable topology (from the demo-config projection). */
  readonly roots: VaultDemoRoots;
  /** Zero-based index of the batch to open. The first `open_batch` on a fresh batcher opens index 0. */
  readonly batchIndex: bigint;
  /** Pays batch-account rent and the batch authority funding. */
  readonly payer: TransactionSigner;
  /** Lamports the batch authority is funded with to pay its owner-charged rent during the batch. */
  readonly authorityFundingLamports: number | bigint;
};

/**
 * Opens one batch on a batcher from its {@link VaultDemoRoots} — the single call the demo seeder makes
 * per batcher. It derives every one of `open_batch`'s accounts (batch, the batch's join/payout
 * token accounts and their balance encrypted stores, and the two
 * Anchor event authorities) from the roots and the batch index, and appends the deny records of both
 * of the batch's mints.
 * The seeder never hand-rolls these accounts; the risky derivation stays here on the tested SDK surface.
 */
export async function openBatchForBatcher(parameters: SolanaVaultOpenBatchForBatcherParameters): Promise<Instruction> {
  const { roots, batchIndex, payer } = parameters;
  const batch = await deriveBatchAddresses(roots, batchIndex);
  // The first batch has no predecessor; a later batch must name the immediately preceding one.
  const previousBatch = batchIndex === 0n ? undefined : (await findBatchPda({ batcher: roots.batcher, index: batchIndex - 1n }))[0];
  const joinMint = tokenApp(roots.joinConfidentialMint);
  const payoutMint = tokenApp(roots.payoutConfidentialMint);
  const [joinMintHcu, payoutMintHcu] = await Promise.all([
    parameters.host.hcuAccounts(joinMint),
    parameters.host.hcuAccounts(payoutMint),
  ]);
  const instruction = await getOpenBatchInstructionAsync({
    transientStore: parameters.transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    payer,
    batcher: roots.batcher,
    index: batchIndex,
    ...(previousBatch === undefined ? {} : { previousBatch }),
    batch: batch.batch,
    joinConfidentialMint: roots.joinConfidentialMint,
    batchJoinTokenAccount: batch.batchJoinTokenAccount,
    batchJoinBalanceStore: await tokenStoreAddress(roots.joinConfidentialMint, batch.batchJoinTokenAccount),
    payoutConfidentialMint: roots.payoutConfidentialMint,
    batchPayoutTokenAccount: batch.batchPayoutTokenAccount,
    batchPayoutBalanceStore: batch.batchPayoutBalanceStore,
    joinUnderlyingMint: roots.joinUnderlyingMint,
    payoutUnderlyingMint: roots.payoutUnderlyingMint,
    confidentialTokenEventAuthority: (await findTokenEventAuthorityPda())[0],
    authorityFundingLamports: parameters.authorityFundingLamports,
    joinMintHcuBlockMeter: joinMintHcu.hcuBlockMeter,
    joinMintHcuTrustedAppRecord: joinMintHcu.hcuTrustedAppRecord,
    payoutMintHcuBlockMeter: payoutMintHcu.hcuBlockMeter,
    payoutMintHcuTrustedAppRecord: payoutMintHcu.hcuTrustedAppRecord,
  });
  return withDenyRecords(instruction, parameters.host.denyListEnabled, [joinMint, payoutMint]);
}
