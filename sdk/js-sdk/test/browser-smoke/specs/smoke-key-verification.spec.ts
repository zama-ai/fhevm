import { expect, test } from '../fixtures.js';

test('a real client refuses a key its KMSGeneration does not vouch for', async ({ page }) => {
  await page.goto('/test/browser-smoke/pages/smoke-key-verification.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke key verification logs:\n', logs);

  expect(status).toBe('pass');
});
