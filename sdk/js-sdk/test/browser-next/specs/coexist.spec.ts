import { expect, test } from '@playwright/test';

// Coexistence cell (Chunk 2c — per-platform definition of done): two TFHE versions
// (v12→1.5.x, v13→1.6.1) + a cleartext runtime live in one realm, each builds a
// proof, and the forward-compat expected-fail (older module + newer key) is
// asserted. Drives /coexist; lib + thread mode come from NEXT_PUBLIC_* env.
test('coexistence: v12 + v13 + cleartext in one realm (+ forward-compat expected-fail)', async ({ page }) => {
  await page.goto('/coexist');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 180_000 });

  const logs = await page.locator('#log').textContent();
  console.log('coexist logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
