import { expect, test } from '../fixtures.js';

test('every client type shares a single TFHE and a single TKMS module', async ({ page, context }) => {
  // Every fetch of a WASM binary, from the page or any of its workers.
  const wasmRequests: string[] = [];
  context.on('request', (request) => {
    const { pathname } = new URL(request.url());
    if (pathname.endsWith('.wasm')) {
      wasmRequests.push(pathname);
    }
  });

  await page.goto('/test/browser-smoke/pages/smoke-clients.html');

  const result = page.locator('#result');
  await result.waitFor({ timeout: 300_000 });

  const status = await result.getAttribute('data-status');
  const logs = await page.locator('#log').textContent();
  console.log('Smoke clients logs:\n', logs);

  expect(status).toBe('pass');
  expect(wasmRequests.filter((p) => p.endsWith('/tfhe_bg.wasm'))).toHaveLength(1);
  expect(wasmRequests.filter((p) => p.endsWith('/kms_lib_bg.wasm'))).toHaveLength(1);
  expect(wasmRequests).toHaveLength(2);
});
