import assert from 'node:assert/strict';
import test from 'node:test';

import {
  compareVersions,
  formatVersion,
  generationRange,
  isCanonicalVersion,
  parseVersion,
  satisfiesGenerationRange,
} from '../base/semver.ts';

const v = (text: string) => {
  const parsed = parseVersion(text);
  if (parsed === undefined) throw new Error(`not canonical: ${text}`);
  return parsed;
};

test('canonical SemVer: three numbers, no leading zeros, optional prerelease, no build metadata', () => {
  for (const ok of ['0.0.0', '0.13.0', '1.2.3', '0.14.0-alpha.0', '2.0.0-rc.1']) assert.ok(isCanonicalVersion(ok), ok);
  for (const bad of ['v1.2.3', '1.2', '01.2.3', '1.2.3+build', '^0.13.0', '1.2.3-', '']) {
    assert.equal(isCanonicalVersion(bad), false, bad);
  }
  assert.deepEqual(v('0.14.0-alpha.0'), { major: 0, minor: 14, patch: 0, prerelease: ['alpha', '0'] });
  assert.equal(formatVersion(v('0.14.0-alpha.0')), '0.14.0-alpha.0');
});

test('ordering follows SemVer §11: core numerically, a release above its prereleases, identifiers in turn', () => {
  const ascending = ['0.13.0', '0.13.1', '0.14.0-alpha.0', '0.14.0-alpha.1', '0.14.0-beta', '0.14.0', '1.0.0'];
  for (let index = 1; index < ascending.length; index += 1) {
    const [older, newer] = [v(ascending[index - 1] ?? ''), v(ascending[index] ?? '')];
    assert.ok(compareVersions(older, newer) < 0, `${ascending[index - 1]} < ${ascending[index]}`);
    assert.ok(compareVersions(newer, older) > 0);
  }
  assert.equal(compareVersions(v('0.13.0'), v('0.13.0')), 0);
  // Numeric identifiers rank below alphanumeric ones (1.0.0-1 < 1.0.0-alpha).
  assert.ok(compareVersions(v('1.0.0-1'), v('1.0.0-alpha')) < 0);
});

test('the rendered range is the generation, whatever the patch; a prerelease pins itself', () => {
  assert.equal(generationRange(v('0.13.0')), '^0.13.0');
  assert.equal(generationRange(v('0.13.4')), '^0.13.0');
  assert.equal(generationRange(v('0.14.0-alpha.0')), '0.14.0-alpha.0');
  assert.equal(generationRange(v('1.2.3')), '^1.0.0');
});

test('a published version satisfies the generation range only inside the generation, never as a prerelease', () => {
  assert.ok(satisfiesGenerationRange(v('0.13.0'), '^0.13.0'));
  assert.ok(satisfiesGenerationRange(v('0.13.9'), '^0.13.0'));
  assert.equal(satisfiesGenerationRange(v('0.14.0'), '^0.13.0'), false);
  assert.equal(satisfiesGenerationRange(v('0.12.9'), '^0.13.0'), false);
  assert.equal(satisfiesGenerationRange(v('0.13.4-0'), '^0.13.0'), false);
  assert.ok(satisfiesGenerationRange(v('1.5.0'), '^1.0.0'));
  assert.equal(satisfiesGenerationRange(v('2.0.0'), '^1.0.0'), false);
  // An exact range, as a prerelease renders to, matches that version alone.
  assert.ok(satisfiesGenerationRange(v('0.14.0-alpha.0'), '0.14.0-alpha.0'));
  assert.equal(satisfiesGenerationRange(v('0.14.0-alpha.1'), '0.14.0-alpha.0'), false);
});
