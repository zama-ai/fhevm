import type { UintNumber } from '../types/primitives.js';
import { assertIsUintNumber } from './uint.js';

////////////////////////////////////////////////////////////////////////////////

export type Semver = {
  readonly version: string;
  readonly major: UintNumber;
  readonly minor: UintNumber;
  readonly patch: UintNumber;
  readonly prerelease?: readonly string[] | undefined;
  readonly build?: readonly string[] | undefined;
};

export type SemverComparator = 'lt' | 'le' | 'gt' | 'ge' | 'eq';

export type SemverRange = {
  readonly version: string;
  readonly comparator: SemverComparator;
};

export type SemverIntervalLowerBound = {
  readonly version: string;
  readonly comparator: 'gt' | 'ge';
};

export type SemverIntervalUpperBound = {
  readonly version: string;
  readonly comparator: 'lt' | 'le';
};

export type SemverInterval = {
  readonly lowerBound?: SemverIntervalLowerBound | undefined;
  readonly upperBound?: SemverIntervalUpperBound | undefined;
};

const SEMVER_REGEX =
  /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9]\d*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*))*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/;

export function parseSemver(version: string): Semver {
  const match = SEMVER_REGEX.exec(version);
  if (match === null) {
    throw new Error(`Invalid SemVer version: "${version}".`);
  }

  const major = Number(match[1]);
  const minor = Number(match[2]);
  const patch = Number(match[3]);

  assertIsUintNumber(major, {});
  assertIsUintNumber(minor, {});
  assertIsUintNumber(patch, {});

  return Object.freeze({
    version,
    major,
    minor,
    patch,
    prerelease: match[4]?.split('.'),
    build: match[5]?.split('.'),
  });
}

export function isSemverStrictlyBefore(version: string, before: string): boolean {
  return compareSemver(version, before) < 0;
}

export function isSemverInRange(version: string, range: SemverRange): boolean {
  const comparison = compareSemver(version, range.version);

  switch (range.comparator) {
    case 'lt':
      return comparison < 0;
    case 'le':
      return comparison <= 0;
    case 'gt':
      return comparison > 0;
    case 'ge':
      return comparison >= 0;
    case 'eq':
      return comparison === 0;
    default: {
      const exhaustiveCheck: never = range.comparator;
      throw new Error(`Unsupported SemVer comparator "${exhaustiveCheck}".`);
    }
  }
}

export function isSemverInInterval(version: string, interval: SemverInterval): boolean {
  if (interval.lowerBound !== undefined && !isSemverInRange(version, interval.lowerBound)) {
    return false;
  }
  if (interval.upperBound !== undefined && !isSemverInRange(version, interval.upperBound)) {
    return false;
  }
  return true;
}

/**
 * Returns whether a SemVer comparator constraint implies a SemVer range.
 *
 * Mathematically: for every SemVer `x`, if `x comparator version` is true,
 * then `x range.comparator range.version` is also true.
 */
export function semverComparatorImpliesRange(
  version: string,
  comparator: SemverComparator,
  range: SemverRange,
): boolean {
  if (comparator === 'eq') {
    return isSemverInRange(version, range);
  }

  const comparison = compareSemver(version, range.version);

  switch (comparator) {
    case 'lt':
      return range.comparator === 'lt' || range.comparator === 'le' ? comparison <= 0 : false;
    case 'le':
      if (range.comparator === 'lt') {
        return comparison < 0;
      }
      return range.comparator === 'le' ? comparison <= 0 : false;
    case 'gt':
      return range.comparator === 'gt' || range.comparator === 'ge' ? comparison >= 0 : false;
    case 'ge':
      if (range.comparator === 'gt') {
        return comparison > 0;
      }
      return range.comparator === 'ge' ? comparison >= 0 : false;
    default: {
      const exhaustiveCheck: never = comparator;
      throw new Error(`Unsupported SemVer comparator "${exhaustiveCheck}".`);
    }
  }
}

export function compareSemver(a: string, b: string): -1 | 0 | 1 {
  const left = parseSemver(a);
  const right = parseSemver(b);

  const releaseComparison = _compareNumbers(
    [left.major, left.minor, left.patch],
    [right.major, right.minor, right.patch],
  );
  if (releaseComparison !== 0) {
    return releaseComparison;
  }

  return _comparePrerelease(left.prerelease, right.prerelease);
}

function _compareNumbers(a: readonly number[], b: readonly number[]): -1 | 0 | 1 {
  for (let i = 0; i < a.length; ++i) {
    const left = a[i];
    const right = b[i];
    if (left === undefined) {
      return right === undefined ? 0 : -1;
    }
    if (right === undefined) {
      return 1;
    }
    if (left < right) {
      return -1;
    }
    if (left > right) {
      return 1;
    }
  }
  return a.length === b.length ? 0 : -1;
}

function _comparePrerelease(a: readonly string[] | undefined, b: readonly string[] | undefined): -1 | 0 | 1 {
  if (a === undefined && b === undefined) {
    return 0;
  }
  if (a === undefined) {
    return 1;
  }
  if (b === undefined) {
    return -1;
  }

  const length = Math.max(a.length, b.length);
  for (let i = 0; i < length; ++i) {
    const left = a[i];
    const right = b[i];

    if (left === undefined) {
      return -1;
    }
    if (right === undefined) {
      return 1;
    }

    const comparison = _comparePrereleaseIdentifier(left, right);
    if (comparison !== 0) {
      return comparison;
    }
  }

  return 0;
}

function _comparePrereleaseIdentifier(a: string, b: string): -1 | 0 | 1 {
  const aIsNumeric = _isNumericIdentifier(a);
  const bIsNumeric = _isNumericIdentifier(b);

  if (aIsNumeric && bIsNumeric) {
    return _compareNumbers([Number(a)], [Number(b)]);
  }

  if (aIsNumeric) {
    return -1;
  }
  if (bIsNumeric) {
    return 1;
  }

  if (a < b) {
    return -1;
  }
  if (a > b) {
    return 1;
  }
  return 0;
}

function _isNumericIdentifier(value: string): boolean {
  return /^(0|[1-9]\d*)$/.test(value);
}
