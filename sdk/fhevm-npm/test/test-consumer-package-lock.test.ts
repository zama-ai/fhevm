import assert from 'node:assert/strict';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import {
  consumerInstallArguments,
  consumerInstallMode,
  describeInstallMode,
  regenerateFixturePackageLock,
} from '../base/test-consumer.ts';
import type { NpmManifestEntry } from '../manifest.ts';

test('consumer install arguments select npm install or npm ci', () => {
  assert.deepEqual(consumerInstallArguments(false), ['install', '--install-links', '--no-audit', '--no-fund']);
  assert.deepEqual(consumerInstallArguments(true), ['ci', '--install-links', '--no-audit', '--no-fund']);
});

test('the install mode is derived from membership and the committed lockfile, not from a flag', () => {
  const inventory = (member: boolean): NpmManifestEntry => ({
    kind: member ? 'internal-consumer' : 'standalone',
    type: 'esm',
    browser: false,
    name: 'consumer',
    member,
  });
  const hasLock = (file: string) => file === join('/ws/fixture', 'package-lock.json');

  const isolatedWithLock = consumerInstallMode({ directory: '/ws/fixture', inventory: inventory(false) }, hasLock);
  assert.deepEqual(isolatedWithLock, { mode: 'committed', reason: 'committed lockfile' });
  assert.equal(describeInstallMode(isolatedWithLock), 'committed');

  // A member never replays its own lock, even if one were lying around: the installation root owns it.
  const member = consumerInstallMode({ directory: '/ws/fixture', inventory: inventory(true) }, hasLock);
  assert.deepEqual(member, { mode: 'fresh', reason: 'workspace member' });
  assert.equal(describeInstallMode(member), 'fresh (workspace member)');

  const isolatedWithoutLock = consumerInstallMode({ directory: '/ws/other', inventory: inventory(false) }, hasLock);
  assert.deepEqual(isolatedWithoutLock, { mode: 'fresh', reason: 'no lockfile' });
  assert.equal(describeInstallMode(isolatedWithoutLock), 'fresh (no lockfile)');
});

test('regenerates and validates a consumer lock atomically in a sibling staging directory', () => {
  const root = join(tmpdir(), `fhevm-npm-consumer-lock-${String(process.pid)}-${String(Date.now())}`);
  const consumerRoot = join(root, 'test-consumer');
  const fixture = join(consumerRoot, 'esm');
  const local = join(consumerRoot, 'local');
  mkdirSync(fixture, { recursive: true });
  mkdirSync(local, { recursive: true });
  writeFileSync(join(local, 'package.json'), '{"name":"local","version":"1.0.0"}\n');
  writeFileSync(
    join(fixture, 'package.json'),
    `${JSON.stringify({ name: 'consumer', private: true, type: 'module', dependencies: { local: 'file:../local' } }, null, 2)}\n`,
  );
  writeFileSync(join(fixture, 'package-lock.json'), '{"old":true}\n');

  const calls: string[][] = [];
  try {
    regenerateFixturePackageLock(fixture, (directory, args) => {
      calls.push([...args]);
      assert.notEqual(directory, fixture);
      assert.equal(dirname(directory), dirname(fixture));
      if (args[0] !== 'install') return;
      writeFileSync(
        join(directory, 'package-lock.json'),
        `${JSON.stringify(
          {
            name: 'consumer',
            lockfileVersion: 3,
            packages: {
              '': { dependencies: { local: 'file:../local' } },
              'node_modules/local': { version: '1.0.0', resolved: 'file:../local' },
            },
          },
          null,
          2,
        )}\n`,
      );
    });

    assert.deepEqual(calls, [
      ['install', '--install-links', '--package-lock-only', '--ignore-scripts', '--no-audit', '--no-fund'],
      ['ci', '--install-links', '--ignore-scripts', '--no-audit', '--no-fund'],
      ['ls', '--all', '--install-links'],
    ]);
    const lock = JSON.parse(readFileSync(join(fixture, 'package-lock.json'), 'utf8')) as { lockfileVersion: number };
    assert.equal(lock.lockfileVersion, 3);
    assert.equal(
      readdirSync(consumerRoot).some((entry) => entry.startsWith('.fhevm-npm-lock-')),
      false,
    );
    assert.equal(existsSync(join(fixture, `.package-lock.json.${String(process.pid)}.tmp`)), false);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('keeps the committed lock when regeneration validation fails', () => {
  const root = join(tmpdir(), `fhevm-npm-consumer-lock-failure-${String(process.pid)}-${String(Date.now())}`);
  const fixture = join(root, 'test-consumer', 'esm');
  mkdirSync(fixture, { recursive: true });
  writeFileSync(
    join(fixture, 'package.json'),
    `${JSON.stringify({ name: 'consumer', private: true, type: 'module', dependencies: { local: 'file:../local' } })}\n`,
  );
  const originalLock = '{"preserved":true}\n';
  writeFileSync(join(fixture, 'package-lock.json'), originalLock);

  try {
    assert.throws(
      () =>
        regenerateFixturePackageLock(fixture, (directory, args) => {
          if (args[0] === 'install') {
            writeFileSync(join(directory, 'package-lock.json'), '{"lockfileVersion":2,"packages":{}}\n');
          }
        }),
      /must use lockfileVersion 3/,
    );
    assert.equal(readFileSync(join(fixture, 'package-lock.json'), 'utf8'), originalLock);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
