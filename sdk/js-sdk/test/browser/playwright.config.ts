import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './specs',
  timeout: 300_000,
  webServer: {
    command: 'npx vite --config test/browser/vite.config.ts',
    port: 3333,
    reuseExistingServer: !process.env.CI,
  },
  use: {
    baseURL: 'http://localhost:3333',
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } },
  ],
});
