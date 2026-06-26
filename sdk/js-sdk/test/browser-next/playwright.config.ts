import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './specs',
  timeout: 300_000,
  webServer: {
    command: 'npm run dev',
    url: 'http://127.0.0.1:3334',
    reuseExistingServer: !process.env.CI,
  },
  use: {
    baseURL: 'http://127.0.0.1:3334',
  },
  projects: [{ name: 'chromium', use: { browserName: 'chromium' } }],
});
