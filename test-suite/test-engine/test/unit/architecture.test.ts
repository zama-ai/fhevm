import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import { PACKAGE_ROOT } from '../../src/infrastructure/paths.js';
import { filesBelow } from '../helpers.js';

const SRC = path.join(PACKAGE_ROOT, 'src');

/**
 * First-level folders of `src/` and the folders each one may import from (see "Architecture" in
 * the README). `orchestration` (main process) and `runtime` (scenario worker) never import each
 * other: what they share lives in `model`.
 */
const ALLOWED: Record<string, string[]> = {
  infrastructure: [],
  config: ['infrastructure'],
  model: ['config', 'infrastructure'],
  orchestration: ['model', 'config', 'infrastructure'],
  runtime: ['model', 'config', 'infrastructure'],
  api: ['runtime', 'model', 'config'],
  boot: ['runtime', 'model', 'config', 'infrastructure'],
  cli: ['boot', 'orchestration', 'model', 'config', 'infrastructure'],
};

const IMPORT = /(?:from|import)\s+'(\.{1,2}\/[^']+)'/g;

const layerOf = (file: string): string => path.relative(SRC, file).split(path.sep)[0]!;

describe('source layout', () => {
  it('has a dependency rule for every first-level folder of src/', () => {
    const folders = readdirSync(SRC, { withFileTypes: true });
    assert.deepEqual(
      folders.filter((entry) => !entry.isDirectory()).map((entry) => entry.name),
      [],
      'src/ holds folders only',
    );
    assert.deepEqual(folders.map((entry) => entry.name).sort(), Object.keys(ALLOWED).sort());
  });

  it('imports only from the folders allowed for each layer', () => {
    const violations: string[] = [];
    for (const file of filesBelow(SRC).filter((name) => name.endsWith('.ts'))) {
      const from = layerOf(file);
      for (const [, specifier] of readFileSync(file, 'utf8').matchAll(IMPORT)) {
        const to = layerOf(path.resolve(path.dirname(file), specifier!));
        if (to !== from && !ALLOWED[from]!.includes(to)) {
          violations.push(`${path.relative(SRC, file)} → ${specifier} (${from} may not import ${to})`);
        }
      }
    }
    assert.deepEqual(violations, []);
  });
});
