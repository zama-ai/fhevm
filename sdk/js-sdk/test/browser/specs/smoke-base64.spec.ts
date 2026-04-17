import { test, expect } from '@playwright/test';

test('createFhevmClient initializes with base64 WASM', async ({ page }) => {
  await page.goto('/test/browser/pages/smoke-base64.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('Smoke base64 logs:\n', logs);
  }

  expect(status).toBe('pass');
});
