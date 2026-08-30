import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const TEST_ROOT = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  cacheDir: join(TEST_ROOT, '.vitest-cache'),
  root: TEST_ROOT,
  test: {
    environment: 'node',
    include: ['upgrade-e2e.test.ts'],
    testTimeout: 180_000,
  },
});
