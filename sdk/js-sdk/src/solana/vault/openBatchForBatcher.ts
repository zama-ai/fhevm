import type { TransactionSigner } from '@solana/kit';

import { findComputeSignerPda } from '../internal/generated/confidentialToken/pdas/computeSigner.js';
import { openBatch, type SolanaVaultOpenBatchResult } from './openBatch.js';
import { deriveBatchAddresses, deriveSettleLookupTableAddresses, type VaultDemoRoots } from './derive.js';
import { balanceValueAddress, tokenEventAuthorityAddress, zamaEventAuthorityAddress } from './internal/tokenLineage.js';
import { batchAddress } from './internal/batcherPdas.js';

export type SolanaVaultOpenBatchForBatcherParameters = {
  /** The batcher's immutable topology (from the demo-config projection). */
  readonly roots: VaultDemoRoots;
  /** Zero-based index of the batch to open. The first `open_batch` on a fresh batcher opens index 0. */
  readonly batchIndex: bigint;
  /** Pays batch-account rent and the batch authority funding; doubles as the lookup-table authority. */
  readonly payer: TransactionSigner;
  /** Recent finalized slot used to derive the settle lookup table (see {@link openBatch}). */
  readonly recentSlot: bigint;
  /** Lamports the batch authority is funded with to pay its owner-charged rent during the batch. */
  readonly authorityFundingLamports: number | bigint;
};

/**
 * Opens one batch on a batcher from its {@link VaultDemoRoots} — the single call the demo seeder makes
 * per batcher. It derives every one of `open_batch`'s ~24 accounts (batch, both confidential mints'
 * compute signers, the batch's join/payout token accounts and their balance lineages, and the two
 * Anchor event authorities) from the roots and the batch index, assembles the settle lookup-table
 * address set, and delegates to {@link openBatch} for the create/extend instructions. The seeder never
 * hand-rolls these accounts; the risky derivation stays here on the tested SDK surface.
 */
export async function openBatchForBatcher(
  parameters: SolanaVaultOpenBatchForBatcherParameters,
): Promise<SolanaVaultOpenBatchResult> {
  const { roots, batchIndex, payer } = parameters;
  const batch = await deriveBatchAddresses(roots, batchIndex);
  const [joinComputeSigner] = await findComputeSignerPda({ mint: roots.joinConfidentialMint });
  const [payoutComputeSigner] = await findComputeSignerPda({ mint: roots.payoutConfidentialMint });
  // The first batch has no predecessor; a later batch must name the immediately preceding one.
  const previousBatch = batchIndex === 0n ? undefined : await batchAddress(roots.batcher, batchIndex - 1n);
  return openBatch({
    openBatch: {
      payer,
      batcher: roots.batcher,
      ...(previousBatch === undefined ? {} : { previousBatch }),
      batch: batch.batch,
      joinConfidentialMint: roots.joinConfidentialMint,
      joinComputeSigner,
      batchJoinTokenAccount: batch.batchJoinTokenAccount,
      batchJoinBalanceValue: await balanceValueAddress(roots.joinConfidentialMint, batch.batchJoinTokenAccount),
      payoutConfidentialMint: roots.payoutConfidentialMint,
      payoutComputeSigner,
      batchPayoutTokenAccount: batch.batchPayoutTokenAccount,
      batchPayoutBalanceValue: batch.batchPayoutBalanceValue,
      joinUnderlyingMint: roots.joinUnderlyingMint,
      payoutUnderlyingMint: roots.payoutUnderlyingMint,
      zamaEventAuthority: await zamaEventAuthorityAddress(),
      hostConfig: roots.hostConfig,
      confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
      authorityFundingLamports: parameters.authorityFundingLamports,
    },
    recentSlot: parameters.recentSlot,
    settleLookupTableAddresses: await deriveSettleLookupTableAddresses(roots, batch),
  });
}
