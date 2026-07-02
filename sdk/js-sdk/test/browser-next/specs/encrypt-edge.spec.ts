import { expect, test } from '@playwright/test';

// SSR-edge cell (Chunk 2b — render: ssr-edge). The page runs in the Next EDGE
// runtime as a server component (JS disabled, so the result is server-rendered).
// Edge has no workers/SAB → TFHE is always single-threaded; this asserts the SDK
// loads + runs the WASM there in ST (or surfaces an edge limitation in the logs).
test.use({ javaScriptEnabled: false });

test('ssr-edge: edge server component runs init + encrypt euint64 (single-threaded)', async ({ page }) => {
  await page.goto('/encrypt-edge');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const logs = await page.locator('#log').textContent();
  console.log('encrypt-edge logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
