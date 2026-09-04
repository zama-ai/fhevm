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
    baseURL: 'http://localhost:3333',
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } },
  ],
});
