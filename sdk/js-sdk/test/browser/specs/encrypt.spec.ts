import { test, expect } from '@playwright/test';

test('encryptValues encrypts all FHE types (bool, uint8..uint256, address)', async ({ page }) => {
  await page.goto('/test/browser/pages/encrypt.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('Encrypt test logs:\n', logs);
  }

  expect(status).toBe('pass');
});
