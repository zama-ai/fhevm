import { describe, expect, test } from 'vitest';
import { address } from '@solana/kit';

import {
  encodeBatchTarget,
  encodeOperatorRequest,
  encodeVaultMetrics,
  parseBatchTarget,
  parseOperatorRequest,
  parseVaultMetrics,
} from './demoApi';

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

  test('round-trips a prepared batch target and rejects negative indices', () => {
    const target = { batchIndex: position.batchIndex, batch: position.batch };
    expect(parseBatchTarget(encodeBatchTarget(target))).toEqual(target);
    expect(() => parseBatchTarget({ batchIndex: '-1', batch: position.batch })).toThrow(
      'prepared batch index cannot be negative',
    );
  });

  test('round-trips bigint vault metrics and rejects malformed input', () => {
    const metrics = { totalAssets: 125n, totalShares: 100n };
    expect(parseVaultMetrics(encodeVaultMetrics(metrics))).toEqual(metrics);
    expect(() => parseVaultMetrics({ totalAssets: 1, totalShares: '100' })).toThrow('totalAssets');
  });
});
