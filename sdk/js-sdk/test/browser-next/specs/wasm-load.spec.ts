import { expect, test } from '@playwright/test';

// wasm-load cell (Chunk 2d). Drives /wasm-load with NEXT_PUBLIC_FHEVM_TEST_WASM_LOAD
// selecting the URL-based load mode (verified-blob / *-direct-url / auto). The page
// asserts the SDK resolved its assets via locateFile (resolution 'user') in the
// requested mode and still produced a valid proof.
test('wasm-load: URL-based WASM loading against the gateway asset route', async ({ page }) => {
  await page.goto('/wasm-load');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const logs = await page.locator('#log').textContent();
  console.log('wasm-load logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
