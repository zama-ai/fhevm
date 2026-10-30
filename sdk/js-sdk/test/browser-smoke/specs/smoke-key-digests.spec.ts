import { test, expect } from '../fixtures.js';

test('hashes the FHE public key and CRS in a blob worker', async ({ page }) => {
  await page.goto('/test/browser-smoke/pages/smoke-key-digests.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 60_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke key digests logs:\n', logs);

  expect(status).toBe('pass');
});
