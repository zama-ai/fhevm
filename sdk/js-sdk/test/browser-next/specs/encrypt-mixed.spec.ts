import { expect, test } from '@playwright/test';

// Mixed cell (Chunk 2b — render: mixed). JS is ENABLED (the CSR leg needs it). The
// SSR leg's #ssr-result is baked into the server-rendered HTML; the CSR leg's
// #csr-result is computed in the browser after hydration. Asserting BOTH pass proves
// the SDK works in the server render AND the client render of one page.
test('mixed: SSR leg + CSR leg both run the SDK in one page', async ({ page }) => {
  await page.goto('/encrypt-mixed');

  const ssr = page.locator('#ssr-result');
  const csr = page.locator('#csr-result');
  await expect(ssr).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });
  await expect(csr).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const ssrLog = await page.locator('#ssr-log').textContent();
  const csrLog = await page.locator('#csr-log').textContent();
  console.log('mixed SSR leg:\n', ssrLog, '\nmixed CSR leg:\n', csrLog);

  expect(await ssr.getAttribute('data-status'), ssrLog ?? undefined).toBe('pass');
  expect(await csr.getAttribute('data-status'), csrLog ?? undefined).toBe('pass');
});
