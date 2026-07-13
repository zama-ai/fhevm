import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const TEST_ROOT = dirname(fileURLToPath(import.meta.url));
const consumerTsEntry = join(
  TEST_ROOT,
  'node_modules',
  '@fhevm',
  'host-contracts-cleartext',
  'ts',
  '_types',
  'index.d.ts',
);

if (!existsSync(consumerTsEntry)) {
  throw new Error(`Missing tarball consumer fixture at ${consumerTsEntry}. Run npm run test:tarball.`);
}

export default defineConfig({
  cacheDir: join(TEST_ROOT, '.vitest-cache'),
  root: TEST_ROOT,
  test: {
    environment: 'node',
    include: ['tarball-consumer.test.ts', 'acl-owner-upgrade.test.ts', 'deploy-v13.test.ts'],
    testTimeout: 60_000,
  },
});
