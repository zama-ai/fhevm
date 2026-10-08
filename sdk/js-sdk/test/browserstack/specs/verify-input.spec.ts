import { test, expect } from '../../browser-smoke/fixtures.js';
import { RESULT_SETTLE_DELAY_MS } from '../scripts/common.js';

test('encrypt + on-chain verifyInput via FHETest.addEuint8', async ({ page }) => {
  await page.goto('/test/browserstack/pages/verify-input.html');

  const result = page.locator('#result');
  // Longer timeout: encryption + on-chain tx confirmation
  await result.waitFor({ timeout: 600_000 });
  await page.waitForTimeout(RESULT_SETTLE_DELAY_MS);

  const status = await result.getAttribute('data-status');
  if (status !== 'pass') {
    const logs = await page.locator('#log').textContent();
    console.error('Verify input test logs:\n', logs);
  }

  expect(status).toBe('pass');
});
