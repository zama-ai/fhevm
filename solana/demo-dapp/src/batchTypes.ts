import type { Address } from '@solana/kit';
import { BatchStatus } from './vault/internal/generated/confidentialBatcher/types/batchStatus.js';

export { BatchStatus };

export type VaultDirection = 'deposit' | 'redeem';

/**
 * The generated `BatchStatus` enum is the source of truth shared by the operator flow, settlement,
 * and the lookup-table crank. A batch in one of these states will never send another transaction
 * against its lookup table.
 */
export const isBatchFinished = (status: BatchStatus): boolean =>
  status === BatchStatus.Settled || status === BatchStatus.Canceled || status === BatchStatus.Refunding;

export type BatchTarget = {
  readonly batchIndex: bigint;
  readonly batch: Address;
};

export type BatchPosition = BatchTarget & {
  readonly amountBaseUnits: bigint;
};

export type BatchLifecycle =
  | { readonly kind: 'awaiting-dispatch'; readonly remainingSlots: bigint }
  | { readonly kind: 'proving'; readonly proofReady: boolean }
  | {
      readonly kind: 'settled';
      readonly totalJoined: bigint;
      readonly payoutReceived: bigint;
      readonly claimed: boolean;
    }
  | { readonly kind: 'canceled' }
  | { readonly kind: 'refunding' };

export type OperatorAction = 'dispatch' | 'settle' | 'claim';

export type VaultMetrics = {
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};
