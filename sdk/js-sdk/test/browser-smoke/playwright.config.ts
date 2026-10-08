import { defineConfig } from '@playwright/test';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const serverPath = resolve(__dirname, 'server.ts');

export default defineConfig({
  testDir: './specs',
  timeout: 300_000,
  // BrowserStack real devices are physical units capped by the account's
  // parallel-session plan limit; Playwright's default worker count (half the
  // local CPU cores) fires off far more simultaneous browserType.connect()
  // calls than that, so most queue for a device and their connection idles
  // out ("Socket idle from a long time") before one frees up. One worker
  // keeps requests serialized to match browserstackLocal's parallelsPerPlatform: 1.
  workers: 1,
  // Real devices occasionally fail for infrastructure reasons unrelated to the
  // SDK (a session that takes longer than the test timeout to start, or
  // "page.goto: Failed to execute goto. Internal error." on iOS Safari), so
  // give BrowserStack runs one retry. Local runs stay strict.
  retries: process.env.BROWSERSTACK_CONFIG_FILE ? 1 : 0,
  webServer: {
    command: `npx tsx ${JSON.stringify(serverPath)}`,
    port: 3333,
    reuseExistingServer: !process.env.CI,
  },
  use: {
    // bs-local.com is BrowserStack's dedicated domain that its Local tunnel
    // always routes back to this machine — unlike `localhost` (broken on iOS
    // Safari via BrowserStack Local) or a raw LAN IP (not reliably
    // auto-tunneled on real devices).
    // HTTPS so the page is a secure context (see vite.config.ts); the cert is
    // self-signed, hence ignoreHTTPSErrors.
    baseURL: 'https://bs-local.com:3333',
    ignoreHTTPSErrors: true,
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } },
  ],
});
