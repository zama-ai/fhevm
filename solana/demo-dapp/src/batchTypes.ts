import type { Address } from '@solana/kit';
import { BatchStatus } from './vault/internal/generated/confidentialBatcher/types/batchStatus.js';

export { BatchStatus };

export type VaultDirection = 'deposit' | 'redeem';

/**
 * The generated `BatchStatus` enum is the source of truth shared by the operator flow, settlement,
 * and the authority reclaim pass. Nothing charges the authority of a batch in one of these states.
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
  | { readonly kind: 'awaiting-dispatch'; readonly remainingSecs: bigint }
  | { readonly kind: 'dispatched' }
  | {
      readonly kind: 'settled';
      readonly totalJoined: bigint;
      readonly payoutReceived: bigint;
      readonly claimed: boolean;
    }
  | { readonly kind: 'canceled' }
  /** `refunded` once the user's own contribution has been returned (their join record is closed). */
  | { readonly kind: 'refunding'; readonly refunded: boolean };

export type OperatorAction = 'dispatch' | 'settle' | 'claim';

export type VaultMetrics = {
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};
