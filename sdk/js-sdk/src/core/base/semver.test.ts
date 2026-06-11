import { describe, expect, it } from 'vitest';
import { isSemverInInterval, semverComparatorImpliesRange } from './semver.js';

describe('isSemverInInterval', () => {
  it('handles half-open intervals', () => {
    const interval = {
      lowerBound: { version: '0.4.0', comparator: 'ge' },
      upperBound: { version: '0.5.0', comparator: 'lt' },
    } as const;

    expect(isSemverInInterval('0.3.9', interval)).toBe(false);
    expect(isSemverInInterval('0.4.0', interval)).toBe(true);
    expect(isSemverInInterval('0.4.1', interval)).toBe(true);
    expect(isSemverInInterval('0.5.0', interval)).toBe(false);
  });

  it('handles open-ended intervals', () => {
    expect(isSemverInInterval('0.4.0', { lowerBound: { version: '0.4.0', comparator: 'ge' } })).toBe(true);
    expect(isSemverInInterval('0.3.9', { lowerBound: { version: '0.4.0', comparator: 'ge' } })).toBe(false);

    expect(isSemverInInterval('0.4.0', { upperBound: { version: '0.4.0', comparator: 'le' } })).toBe(true);
    expect(isSemverInInterval('0.4.1', { upperBound: { version: '0.4.0', comparator: 'le' } })).toBe(false);
  });
});

describe('semverComparatorImpliesRange', () => {
  it('handles exact constraints', () => {
    expect(semverComparatorImpliesRange('0.13.0', 'eq', { version: '0.13.0', comparator: 'eq' })).toBe(true);
    expect(semverComparatorImpliesRange('0.13.0', 'eq', { version: '0.12.0', comparator: 'gt' })).toBe(true);
    expect(semverComparatorImpliesRange('0.13.0', 'eq', { version: '0.13.0', comparator: 'gt' })).toBe(false);
  });

  it('handles lower-bound constraints', () => {
    expect(semverComparatorImpliesRange('0.14.0', 'gt', { version: '0.13.0', comparator: 'ge' })).toBe(true);
    expect(semverComparatorImpliesRange('0.13.0', 'gt', { version: '0.13.0', comparator: 'gt' })).toBe(true);
    expect(semverComparatorImpliesRange('0.13.0', 'ge', { version: '0.13.0', comparator: 'gt' })).toBe(false);
    expect(semverComparatorImpliesRange('0.12.0', 'gt', { version: '0.13.0', comparator: 'ge' })).toBe(false);
  });

  it('handles upper-bound constraints', () => {
    expect(semverComparatorImpliesRange('0.11.0', 'lt', { version: '0.13.0', comparator: 'lt' })).toBe(true);
    expect(semverComparatorImpliesRange('0.13.0', 'lt', { version: '0.13.0', comparator: 'lt' })).toBe(true);
    expect(semverComparatorImpliesRange('0.13.0', 'le', { version: '0.13.0', comparator: 'lt' })).toBe(false);
    expect(semverComparatorImpliesRange('0.14.0', 'lt', { version: '0.13.0', comparator: 'lt' })).toBe(false);
  });
});
