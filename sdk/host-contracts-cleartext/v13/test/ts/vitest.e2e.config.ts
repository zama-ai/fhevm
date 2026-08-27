import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const TEST_ROOT = dirname(fileURLToPath(import.meta.url));

// Only v13 needs a fixture. It is the package being published here, so the e2e consumes it by its
// PUBLISHED name from a real tarball — that is what puts `exports` and `files` on trial.
//
// v12 has no fixture and needs none: it is a workspace member, imported through the link npm created
// (`@fhevm/host-contracts-cleartext-dev-v12/pkg/ts/...`). Nothing to build, pack or install, so there
// is nothing to guard — an unresolvable specifier means a broken install and vitest says so.
const entry = join(TEST_ROOT, 'node_modules', '@fhevm', 'host-contracts-cleartext', 'ts', '_types', 'index.d.ts');
if (!existsSync(entry)) {
  throw new Error(`Missing consumer fixture for host-contracts-cleartext at ${entry}. Run npm run test:upgrade-e2e.`);
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
