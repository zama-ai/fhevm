import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const TEST_ROOT = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  cacheDir: join(TEST_ROOT, '.vitest-cache'),
  root: TEST_ROOT,
  test: {
    environment: 'node',
    include: [
      'adapter-nonce-diagnostics.test.ts',
      'precompute-addresses.test.ts',
      'ethers-adapter.test.ts',
      'acl-owner-upgrade.test.ts',
      'deploy-v12.test.ts',
      'create2-precompute.test.ts',
      'fhe-rand.test.ts',
    ],
    testTimeout: 60_000,
  },
});
