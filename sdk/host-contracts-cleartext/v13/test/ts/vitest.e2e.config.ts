import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const TEST_ROOT = dirname(fileURLToPath(import.meta.url));

// The e2e test consumes BOTH published packages: v12 (to deploy the "before" stack) and v13 (to upgrade).
for (const pkg of ['host-contracts-cleartext', 'host-contracts-cleartext-v12']) {
  const entry = join(TEST_ROOT, 'node_modules', '@fhevm', pkg, 'ts', '_types', 'index.d.ts');
  if (!existsSync(entry)) {
    throw new Error(`Missing consumer fixture for ${pkg} at ${entry}. Run npm run test:upgrade-e2e.`);
  }
}

export default defineConfig({
  cacheDir: join(TEST_ROOT, '.vitest-cache'),
  root: TEST_ROOT,
  test: {
    environment: 'node',
    include: ['upgrade-e2e.test.ts'],
    testTimeout: 180_000,
  },
});
