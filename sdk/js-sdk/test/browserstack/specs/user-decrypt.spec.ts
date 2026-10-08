import { test, expect } from '../../browser-smoke/fixtures.js';
import { RESULT_SETTLE_DELAY_MS } from '../scripts/common.js';

test('decryptValue performs full user decrypt flow (keypair + permit + TKMS)', async ({ page }) => {
  await page.goto('/test/browserstack/pages/user-decrypt.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });
  await page.waitForTimeout(RESULT_SETTLE_DELAY_MS);

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('User decrypt test logs:\n', logs);
  }

  expect(status).toBe('pass');
});
