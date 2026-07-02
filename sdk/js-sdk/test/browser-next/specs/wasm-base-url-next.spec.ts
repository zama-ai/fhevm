import { expect, test } from '@playwright/test';

const consoleErrorNeedles = ['require is not defined', 'node:url', 'Module not found', "Can't resolve"];

test('packed SDK initializes in a Next.js Turbopack client bundle', async ({ page }) => {
  const pageErrors: string[] = [];
  const consoleErrors: string[] = [];

  page.on('pageerror', (err) => {
    pageErrors.push(err.message);
  });
  page.on('console', async (message) => {
    const values = await Promise.all(
      message.args().map(async (arg) => {
        try {
          return await arg.jsonValue();
        } catch {
          return arg.toString();
        }
      }),
    );
    console.log(`[browser:${message.type()}]`, ...values);

    if (message.type() === 'error') {
      consoleErrors.push(message.text());
    }
  });

  await page.goto('/');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  // #log is now a readable, one-entry-per-line trace (see app/_diag/diagnostics.jsx),
  // not a JSON blob — assert on the log lines the smoke test emits.
  const logs = (await page.locator('#log').textContent()) ?? '';
  const relevantConsoleErrors = consoleErrors.filter((message) =>
    consoleErrorNeedles.some((needle) => message.includes(needle)),
  );

  console.log('Next.js packed SDK logs:\n', logs);

  expect(pageErrors).toEqual([]);
  expect(relevantConsoleErrors).toEqual([]);
  expect(logs).toContain('[PASS] Imported @fhevm/sdk/ethers');
  expect(logs).toContain('[PASS] Encrypt runtime initialized');
  expect(status, logs).toBe('pass');
});
