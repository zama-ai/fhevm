import { test, expect } from '../../browser-smoke/fixtures.js';
import { RESULT_SETTLE_DELAY_MS } from '../scripts/common.js';

test('encryptValues encrypts all FHE types (bool, uint8..uint256, address)', async ({ page }) => {
  await page.goto('/test/browserstack/pages/encrypt.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });
  await page.waitForTimeout(RESULT_SETTLE_DELAY_MS);

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('Encrypt test logs:\n', logs);
  }

  expect(status).toBe('pass');
});
