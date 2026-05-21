import { test, expect } from '@playwright/test';

test('decryptValue performs full user decrypt flow (keypair + permit + TKMS)', async ({ page }) => {
  await page.goto('/test/browser/pages/user-decrypt.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('User decrypt test logs:\n', logs);
  }

  expect(status).toBe('pass');
});
