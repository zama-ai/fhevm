import { test, expect } from '../fixtures.js';

test('createFhevmClient initializes with URL-based WASM', async ({ page }) => {
  page.on('console', (msg) => console.log(`[console.${msg.type()}] ${msg.text()}`));
  page.on('pageerror', (err) => console.log(`[pageerror] ${err.message}`));
  // A failed module request leaves the page script unexecuted without any
  // pageerror, and the console message doesn't name the URL.
  page.on('requestfailed', (req) => console.log(`[requestfailed] ${req.url()} ${req.failure()?.errorText ?? ''}`));
  page.on('response', (res) => {
    if (res.status() >= 400) {
      console.log(`[response ${res.status()}] ${res.url()}`);
    }
  });

  await page.goto('/test/browser-smoke/pages/smoke-wasm.html');

  const result = page.locator('#result');
  let logs: string | null = null;
  try {
    await result.waitFor({ timeout: 240_000 });
  } finally {
    // Print the page log even when #result never appears, so a hang shows
    // which step it got stuck on.
    logs = await page
      .locator('#log')
      .textContent()
      .catch(() => '<unavailable>');
    console.log('Smoke wasm logs:\n', logs);
  }

  const status = await result.getAttribute('data-status');

  // The page log as the message puts the [FAIL] line in the failure summary
  // and error-context.md, not only in the interleaved stdout.
  expect(status, `page log:\n${logs}`).toBe('pass');
});
