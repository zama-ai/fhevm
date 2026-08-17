import { expect, test } from '@playwright/test';

// module cell (Chunk 2e). NEXT_PUBLIC_FHEVM_TEST_MODULE selects:
//   kms       — a real decrypt client runs the TKMS wasm (generateTransportKeyPair).
//   cleartext — the mock cleartext runtime standalone (encrypt + transport keypair).
test('module: kms keygen / cleartext runtime', async ({ page }) => {
  await page.goto('/module');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const logs = await page.locator('#log').textContent();
  console.log('module logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
