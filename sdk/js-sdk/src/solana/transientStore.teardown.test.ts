// The wrap/accounts send path is gone from the SDK, the demo, the test-suite and the docs.
//
// A surviving `createSolanaFheTransaction`, `fhe.wrap` or `fhe.accounts` keeps the rejected
// transaction object alive silently. The gate scans those trees and fails while any of the
// names exist. Mentions in this file are exempt because its job is to name them.

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const FORBIDDEN: ReadonlyArray<readonly [string, string]> = [
  ['createSolanaFheTransaction', 'the wrap/accounts factory; prepareTransientStore replaced it'],
  ['SolanaFheTransactionAccounts', 'the bag callers spread into Codama; pass the TransientStore instead'],
  ['SolanaFheTransaction', 'the wrap object; appendTransientStoreInstructions replaced wrap()'],
  ['fhe.wrap', 'the body sandwich on the transaction object'],
  ['fhe.accounts', 'the shared account bag on the transaction object'],
];

const EXEMPT_FILES = new Set(['transientStore.teardown.test.ts']);

const REPO_ROOT = fileURLToPath(new URL('../../../../', import.meta.url));

const TREES: readonly string[] = [
  'sdk/js-sdk/src/solana',
  'sdk/js-sdk/README.md',
  'sdk/js-sdk/docs',
  'solana/demo-dapp/src',
  'solana/README.md',
  'test-suite/fhevm/src',
  'test-suite/fhevm/e2e',
  'test-suite/fhevm/demo',
];

const SKIP_DIR_NAMES = new Set(['node_modules', '_cjs', '_esm', '_types', 'wasm', 'generated', 'dist']);

function sources(relativePath: string): readonly string[] {
  const absolute = join(REPO_ROOT, relativePath);
  const info = statSync(absolute);
  if (info.isFile()) return [absolute];
  const found: string[] = [];
  for (const entry of readdirSync(absolute, { withFileTypes: true })) {
    if (entry.name.startsWith('.')) continue;
    if (entry.isDirectory()) {
      if (SKIP_DIR_NAMES.has(entry.name)) continue;
      found.push(...sources(join(relativePath, entry.name)));
      continue;
    }
    if (EXEMPT_FILES.has(entry.name)) continue;
    if (/\.(ts|tsx|mjs|md)$/.test(entry.name)) found.push(join(REPO_ROOT, relativePath, entry.name));
  }
  return found;
}

describe('the wrap/accounts FHE send surface', () => {
  it('is gone from the SDK, demo, test-suite and docs', () => {
    const files = TREES.flatMap(sources);
    expect(files.length, 'the gate found suspiciously few sources — did the layout move?').toBeGreaterThan(40);

    const survivors: string[] = [];
    for (const file of files) {
      const text = readFileSync(file, 'utf8');
      const repoPath = relative(REPO_ROOT, file);
      for (const [token, reason] of FORBIDDEN) {
        text.split('\n').forEach((line, index) => {
          if (line.includes(token)) survivors.push(`${repoPath}:${index + 1} — ${reason}`);
        });
      }
    }

    expect(survivors, `the wrap/accounts FHE send surface is still alive:\n${survivors.join('\n')}`).toEqual([]);
  });
});
