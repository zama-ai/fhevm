import { test, expect } from '@playwright/test';

test('createFhevmClient initializes with URL-based WASM', async ({ page }) => {
  await page.goto('/test/browser/pages/smoke-wasm.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('Smoke wasm logs:\n', logs);
  }

  expect(status).toBe('pass');
});
