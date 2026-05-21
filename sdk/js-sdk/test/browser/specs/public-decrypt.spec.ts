import { test, expect } from '@playwright/test';

test('readPublicValue decrypts known Sepolia public handles', async ({ page }) => {
  await page.goto('/test/browser/pages/public-decrypt.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('Public decrypt test logs:\n', logs);
  }

  expect(status).toBe('pass');
});
