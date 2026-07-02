import { expect, test } from '@playwright/test';

// Step-1 skeleton: the infra (2 anvils + same-origin gateway serving rpc + keys)
// is reachable from the browser under COOP/COEP. No SDK assertions yet.
test('gateway skeleton: anvil rpc + relayer keys reachable same-origin', async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on('console', (message) => {
    if (message.type() === 'error') {
      consoleErrors.push(message.text());
    }
  });

  await page.goto('/gw-skeleton');

  const result = page.locator('#result');
  await expect(result).toHaveAttribute('data-status', /^(pass|fail)$/, { timeout: 120_000 });

  const logs = await page.locator('#log').textContent();
  console.log('gw-skeleton logs:\n', logs);

  expect(await result.getAttribute('data-status'), logs ?? undefined).toBe('pass');
});
