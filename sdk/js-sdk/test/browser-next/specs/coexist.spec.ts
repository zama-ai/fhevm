import { expect, test } from '@playwright/test';

// Coexistence cell (Chunk 2c — per-platform definition of done): a real TFHE runtime
// (current slot) + a cleartext runtime live in one realm and each builds its input
// concurrently. Drives /coexist; lib + thread mode come from NEXT_PUBLIC_* env.
test('coexistence: current slot + cleartext in one realm', async ({ page }) => {
  await page.goto('/coexist');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 180_000 });

  const logs = await page.locator('#log').textContent();
  console.log('coexist logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
