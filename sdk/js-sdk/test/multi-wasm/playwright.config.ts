import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './specs',
  timeout: 600_000,
  workers: 1,
  globalSetup: './global-setup.ts',
  webServer: {
    command: 'npx vite --config test/multi-wasm/vite.config.ts',
    port: 3334,
    reuseExistingServer: !process.env.CI,
  },
  use: {
    baseURL: 'http://localhost:3334',
  },
  projects: [{ name: 'chromium', use: { browserName: 'chromium' } }],
});
