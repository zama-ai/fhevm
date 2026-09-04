import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { expect, it } from 'vitest';

const here = dirname(fileURLToPath(import.meta.url));
const template = readFileSync(resolve(here, '../../../scripts/wasm/tfhe/tfhe-worker.template.mjs'), 'utf8');
const header = template.slice(0, template.indexOf('/* __TFHE_WORKER_BODY__ */'));

it('generated workers use the tfhe-worker template bootstrap', () => {
  for (const version of ['v1.5.3', 'v1.6.0-dev', 'v1.6.2'] as const) {
    const generated = readFileSync(resolve(here, version, 'tfhe-worker.mjs'), 'utf8');
    expect(generated.startsWith(header), version).toBe(true);
  }
});
