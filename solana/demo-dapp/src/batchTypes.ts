import type { Address } from '@solana/kit';

export type VaultDirection = 'deposit' | 'redeem';

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

export type OperatorAction = 'dispatch' | 'settle';

export type VaultMetrics = {
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};
