import { expect, test } from '@playwright/test';

// SSR cell (Chunk 2b — render: ssr-node). JavaScript is DISABLED for this spec: if
// the page is genuinely server-rendered, #result already carries the final status in
// the HTML the server sent. A CSR page (which computes status in a useEffect) would
// stay 'pending' with JS off and fail here — so this is a real proof the SDK ran on
// the server, not in the browser.
test.use({ javaScriptEnabled: false });

test('ssr-node: server component runs init + encrypt euint64 against the v13 slot', async ({ page }) => {
  await page.goto('/encrypt-ssr');

  const result = page.locator('#result');
  // No client JS to flip it: the attribute is whatever the server rendered.
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const logs = await page.locator('#log').textContent();
  console.log('encrypt-ssr logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
