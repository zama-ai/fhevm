import { fileURLToPath } from 'node:url';
import { defineConfig } from '@playwright/test';
// Single source of truth for slot ids (test/infra/config.ts). Injected into the
// client bundle as NEXT_PUBLIC_FHEVM_SLOT_* so the browser cells stay version-agnostic.
import { CURRENT_SLOT, LEGACY_SLOT, OLD_MODULE_NEW_KEY_SLOT } from '../infra/config.js';

// Absolute path to test/keys, passed to the Next server so the bundled gateway
// route resolves keys without relying on import.meta.url (Turbopack rewrites it).
const keysDir = fileURLToPath(new URL('../keys', import.meta.url));

export default defineConfig({
  testDir: './specs',
  timeout: 600_000,
  workers: 1,
  globalSetup: './globalSetup.ts',
  globalTeardown: './globalTeardown.ts',
  webServer: {
    command: 'npm run dev',
    url: 'http://127.0.0.1:3334',
    reuseExistingServer: !process.env.CI,
    timeout: 180_000,
    // Merged over process.env by Playwright. NEXT_PUBLIC_* is inlined into the
    // client bundle at `next dev` startup, so switching libs requires a fresh
    // dev server (kill port 3334 between lib runs).
    env: {
      FHEVM_TEST_KEYS_DIR: keysDir,
      FHEVM_TEST_COOP: process.env.FHEVM_TEST_COOP ?? '1',
      NEXT_PUBLIC_FHEVM_TEST_LIB: process.env.FHEVM_TEST_LIB ?? 'viem',
      NEXT_PUBLIC_FHEVM_TEST_THREADS: process.env.FHEVM_TEST_THREADS ?? 'st',
      NEXT_PUBLIC_FHEVM_TEST_WASM_LOAD: process.env.FHEVM_TEST_WASM_LOAD ?? 'embedded-base64',
      NEXT_PUBLIC_FHEVM_TEST_MODULE: process.env.FHEVM_TEST_MODULE ?? 'kms',
      // Slot ids from config.ts → browser bundle (see app/_diag/slots.js).
      NEXT_PUBLIC_FHEVM_SLOT_LEGACY: LEGACY_SLOT,
      NEXT_PUBLIC_FHEVM_SLOT_CURRENT: CURRENT_SLOT,
      NEXT_PUBLIC_FHEVM_SLOT_OLDMOD: OLD_MODULE_NEW_KEY_SLOT,
    },
  },
  use: {
    baseURL: 'http://127.0.0.1:3334',
  },
  // dod.sh selects which project(s) to run via --project (default: chromium only).
  // firefox/webkit require: npx playwright install firefox webkit
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } },
  ],
});
