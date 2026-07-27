import { describe, expect, test } from 'vitest';

import { classifyProofReadiness } from './settlement';

describe('classifyProofReadiness', () => {
  test('retries only a typed lagging response', () => {
    expect(classifyProofReadiness(503, { status: 'lagging' })).toBe(false);
  });

  test('surfaces a terminal missing leaf instead of polling forever', () => {
    expect(() => classifyProofReadiness(404, { status: 'leaf_not_found' })).toThrow('leaf_not_found');
  });

  test('surfaces non-lagging service failures', () => {
    expect(() => classifyProofReadiness(503, { code: 'overloaded' })).toThrow('overloaded');
  });

  test('accepts only a typed successful readiness result', () => {
    expect(classifyProofReadiness(200, { verified: true })).toBe(true);
    expect(classifyProofReadiness(200, { verified: false })).toBe(false);
    expect(() => classifyProofReadiness(200, {})).toThrow('malformed');
  });
});
