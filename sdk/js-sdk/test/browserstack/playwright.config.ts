import { defineConfig } from '@playwright/test';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const viteConfigPath = resolve(__dirname, 'vite.config.ts');

export default defineConfig({
  testDir: './specs',
  timeout: 300_000,
  // Same device constraints as test/browser-smoke/playwright.config.ts: one
  // worker keeps simultaneous connects within parallelsPerPlatform: 1. It also
  // keeps verify-input and user-decrypt from sending transactions from the
  // same MNEMONIC wallet at the same time, which would collide on nonces.
  workers: 1,
  // Real devices occasionally fail for infrastructure reasons unrelated to the
  // SDK, so give BrowserStack runs one retry. Local runs stay strict.
  retries: process.env.BROWSERSTACK_CONFIG_FILE ? 1 : 0,
  webServer: {
    command: `npx vite --config ${JSON.stringify(viteConfigPath)}`,
    port: 3333,
    reuseExistingServer: !process.env.CI,
  },
  use: {
    // bs-local.com is BrowserStack's dedicated domain that its Local tunnel
    // always routes back to this machine — unlike `localhost` (broken on iOS
    // Safari via BrowserStack Local) or a raw LAN IP (not reliably
    // auto-tunneled on real devices).
    baseURL: 'http://bs-local.com:3333',
  },
  projects: [{ name: 'chromium', use: { browserName: 'chromium' } }],
});
