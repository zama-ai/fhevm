import { expect, test } from '@playwright/test';

test('initializes two TFHE/TKMS versions in one browser realm', async ({ page, browserName }) => {
  test.skip(browserName !== 'chromium', 'Explicit multithreaded coexistence smoke is Chromium-only.');

  await page.goto('/test/browser/pages/smoke-coexistence.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke coexistence logs:\n', logs);

  expect(status).toBe('pass');
});
