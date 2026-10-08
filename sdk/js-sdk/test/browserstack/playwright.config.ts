import { defineConfig } from '@playwright/test';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const viteConfigPath = resolve(__dirname, 'vite.config.ts');

export default defineConfig({
  testDir: './specs',
  timeout: 300_000,
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
