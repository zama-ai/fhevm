import { expect, test } from '../fixtures.js';

// File-level skips are evaluated before the test-scoped `context` fixture runs,
// so no BrowserStack device session is opened just to be skipped (an in-body
// skip only fires after the session is up, and a slow device makes the test
// time out during fixture setup instead).
test.skip(
  ({ browserName }) => browserName !== 'chromium',
  'Explicit multithreaded coexistence smoke is Chromium-only.',
);
test.skip(
  (process.env.BROWSERSTACK_CONFIG_FILE ?? '').includes('mobile'),
  'Multithreaded WASM worker-pool stress test is desktop-only; mobile devices are not a target for this scenario.',
);

test('runs concurrent chains against a shared TFHE/TKMS module realm', async ({ page }) => {
  await page.goto('/test/browser-smoke/pages/smoke-coexistence.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke coexistence logs:\n', logs);

  expect(status).toBe('pass');
});
