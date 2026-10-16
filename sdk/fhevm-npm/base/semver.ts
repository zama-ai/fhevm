// The four SemVer questions the release tooling asks, answered here so fhevm-npm needs no dependency for
// them (RELEASE_PLAN.md, decision 9): is a value canonical, which of two is newer, is one a prerelease,
// and what range does a generation render to.

export type ParsedVersion = {
  readonly major: number;
  readonly minor: number;
  readonly patch: number;
  /** Dot-separated identifiers after `-`, empty for a release. */
  readonly prerelease: readonly string[];
};

// Canonical: three numbers without leading zeros, an optional prerelease, no build metadata.
const CANONICAL = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/;

/** The parsed version, or undefined when the text is not canonical SemVer. */
export function parseVersion(value: string): ParsedVersion | undefined {
  const match = CANONICAL.exec(value);
  if (match === null) return undefined;
  const [, major, minor, patch, prerelease] = match;
  return {
    major: Number(major),
    minor: Number(minor),
    patch: Number(patch),
    prerelease: prerelease === undefined ? [] : prerelease.split('.'),
  };
}

export function isCanonicalVersion(value: string): boolean {
  return parseVersion(value) !== undefined;
}

export function isPrerelease(version: ParsedVersion): boolean {
  return version.prerelease.length > 0;
}

/** Negative, zero or positive as `left` is older than, equal to or newer than `right` (SemVer §11). */
export function compareVersions(left: ParsedVersion, right: ParsedVersion): number {
  const core = left.major - right.major || left.minor - right.minor || left.patch - right.patch;
  if (core !== 0) return core;
  // A release outranks any prerelease of the same core.
  if (isPrerelease(left) !== isPrerelease(right)) return isPrerelease(left) ? -1 : 1;
  return comparePrerelease(left.prerelease, right.prerelease);
}

// Identifier by identifier: numeric before alphanumeric, numerics by value, the shorter list first.
function comparePrerelease(left: readonly string[], right: readonly string[]): number {
  for (let index = 0; index < Math.max(left.length, right.length); index += 1) {
    const [a, b] = [left[index], right[index]];
    if (a === undefined) return -1;
    if (b === undefined) return 1;
    const [numericA, numericB] = [/^\d+$/.test(a), /^\d+$/.test(b)];
    if (numericA && numericB && Number(a) !== Number(b)) return Number(a) - Number(b);
    if (numericA !== numericB) return numericA ? -1 : 1;
    if (a !== b) return a < b ? -1 : 1;
  }
  return 0;
}

/**
 * The range a published tarball carries for a dependency at this central version (decision 1): the
 * generation with any patch, `^0.13.0` for `0.13.4`; a prerelease pins itself exactly.
 */
export function generationRange(version: ParsedVersion): string {
  if (isPrerelease(version)) return formatVersion(version);
  return version.major === 0 ? `^0.${version.minor}.0` : `^${version.major}.0.0`;
}

/** Does a published release satisfy a range produced by `generationRange`? Prereleases never do. */
export function satisfiesGenerationRange(candidate: ParsedVersion, range: string): boolean {
  if (!range.startsWith('^')) return formatVersion(candidate) === range;
  const floor = parseVersion(range.slice(1));
  if (floor === undefined || isPrerelease(candidate)) return false;
  const sameGeneration = floor.major === 0 ? candidate.minor === floor.minor : candidate.major === floor.major;
  return candidate.major === floor.major && sameGeneration && compareVersions(candidate, floor) >= 0;
}

export function formatVersion(version: ParsedVersion): string {
  const core = `${version.major}.${version.minor}.${version.patch}`;
  return isPrerelease(version) ? `${core}-${version.prerelease.join('.')}` : core;
}
