import { describe, expect, test } from 'vitest';
import { address } from '@solana/kit';

import { encodeOperatorRequest, encodeVaultMetrics, parseOperatorRequest, parseVaultMetrics } from './demoApi';

const position = {
  batchIndex: 7n,
  batch: address('11111111111111111111111111111111'),
  amountBaseUnits: 100_000_000n,
} as const;

describe('demo API codecs', () => {
  test('round-trips the shared batch position contract', () => {
    expect(parseOperatorRequest(encodeOperatorRequest('settle', 'redeem', position))).toEqual({
      action: 'settle',
      direction: 'redeem',
      position,
    });
  });

  test('round-trips bigint vault metrics and rejects malformed input', () => {
    const metrics = { totalAssets: 125n, totalShares: 100n };
    expect(parseVaultMetrics(encodeVaultMetrics(metrics))).toEqual(metrics);
    expect(() => parseVaultMetrics({ totalAssets: 1, totalShares: '100' })).toThrow('totalAssets');
  });
});
