import { expect, test } from '@playwright/test';

// First real SDK cell: createFhevmClient (public ethers API) against the v13 slot
// via the gateway — init (loads WASM + fetches the relayer key) + a real TFHE
// encryptValue. Proves the public path works against the packed SDK.
test('createFhevmClient: init + encrypt euint64 against the v13 slot', async ({ page }) => {
  await page.goto('/encrypt');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const logs = await page.locator('#log').textContent();
  console.log('encrypt logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
