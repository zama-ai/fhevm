import { address } from '@solana/kit';
import { describe, expect, test } from 'vitest';

import { assertBalanceHandleIsCurrent, redactRedeemPosition } from './redeem';

const nextBatch = address('SysvarRent111111111111111111111111111111111');

describe('redeem recovery', () => {
  test('redacts the confidential amount from persisted and recovered positions', () => {
    expect(redactRedeemPosition({ batchIndex: 8n, batch: nextBatch })).toEqual({
      batchIndex: 8n,
      batch: nextBatch,
      amountBaseUnits: 0n,
    });
  });

  test('rejects a decrypted amount after its encrypted handle changes', () => {
    expect(() => assertBalanceHandleIsCurrent('0xold', '0xnew')).toThrow('private share balance changed');
    expect(() => assertBalanceHandleIsCurrent('0xsame', '0xsame')).not.toThrow();
  });
});
