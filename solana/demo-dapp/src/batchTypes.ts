import type { Address } from '@solana/kit';

export type VaultDirection = 'deposit' | 'redeem';

/**
 * `BatchStatus` as the batcher program encodes it. Declared once here because three modules read
 * it: the operator flow, the settlement path, and the lookup-table crank, which has to tell a
 * batch that is still settling from one that is finished with its table.
 */
export const BATCH_PENDING = 0;
export const BATCH_DISPATCHED = 1;
export const BATCH_SETTLED = 2;
export const BATCH_CANCELED = 3;

/** A batch in one of these states will never send another transaction against its lookup table. */
export const isBatchFinished = (status: number): boolean =>
  status === BATCH_SETTLED || status === BATCH_CANCELED;

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
  | { readonly kind: 'canceled' };

export type OperatorAction = 'dispatch' | 'settle' | 'claim';

export type VaultMetrics = {
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};
