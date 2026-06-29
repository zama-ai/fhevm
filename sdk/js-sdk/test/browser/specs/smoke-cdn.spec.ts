import { test, expect } from '@playwright/test';

test('createFhevmClient initializes with jsdelivr CDN WASM', async ({ page }) => {
  await page.goto('/test/browser/pages/smoke-cdn.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke CDN logs:\n', logs);

  expect(status).toBe('pass');
});
