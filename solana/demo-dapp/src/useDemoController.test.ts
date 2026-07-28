import { describe, expect, test } from 'vitest';

import { demoStateReducer, initialDemoState, type DemoState } from './useDemoController';

describe('demoStateReducer', () => {
  test('resets all account-scoped state atomically while selecting the next connection state', () => {
    const dirty = {
      ...initialDemoState,
      deposit: { kind: 'running', stage: 'joining' },
      revealedShares: { value: 12n, handle: 'handle' },
      harvesting: true,
      redeem: { kind: 'running', stage: 'decrypting' },
      revealingUsdc: true,
    } as DemoState;

    expect(demoStateReducer(dirty, { type: 'reset', generation: 1, connection: { kind: 'connecting' } })).toEqual({
      ...initialDemoState,
      generation: 1,
      connection: { kind: 'connecting' },
    });
  });

  test('updates related fields in one reducer event', () => {
    expect(
      demoStateReducer(initialDemoState, {
        type: 'update',
        generation: 0,
        patch: { harvesting: true, harvestError: null, harvestFromPrice: 1 },
      }),
    ).toMatchObject({ harvesting: true, harvestError: null, harvestFromPrice: 1 });
  });

  test('ignores commits from an obsolete generation', () => {
    const current = { ...initialDemoState, generation: 3 };
    expect(
      demoStateReducer(current, {
        type: 'update',
        generation: 2,
        patch: { harvesting: true, harvestError: 'stale' },
      }),
    ).toBe(current);
  });
});
