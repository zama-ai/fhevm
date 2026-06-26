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
  const logs = await page.locator('#log').textContent();
  const diagnostics = JSON.parse(logs ?? '{}') as { initialized?: boolean; packageSpecifier?: string };
  const relevantConsoleErrors = consoleErrors.filter((message) =>
    consoleErrorNeedles.some((needle) => message.includes(needle)),
  );

  console.log('Next.js packed SDK logs:\n', logs);

  expect(pageErrors).toEqual([]);
  expect(relevantConsoleErrors).toEqual([]);
  expect(diagnostics.packageSpecifier).toBe('@fhevm/sdk/ethers');
  expect(diagnostics.initialized).toBe(true);
  expect(status, logs ?? undefined).toBe('pass');
});
