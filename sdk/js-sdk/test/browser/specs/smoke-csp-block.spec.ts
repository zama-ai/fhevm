import { test, expect } from '@playwright/test';

test("client.init() throws when page CSP omits 'wasm-unsafe-eval'", async ({ page }) => {
  await page.goto('/test/browser/pages/smoke-csp-block.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke CSP-block logs:\n', logs);

  // "pass" here means the page caught an error that looks like a CSP-blocked
  // WASM compile — i.e. the SDK propagated the failure instead of swallowing it.
  expect(status).toBe('pass');
});
