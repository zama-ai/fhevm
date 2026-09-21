// Run through the checked-out SDK's async-request implementation.
// This exercises transport/result handling, not cryptographic SDK validation.
import { strict as assert } from 'node:assert';
import { readFileSync } from 'node:fs';
import { RelayerAsyncRequest } from '../../../sdk/js-sdk/src/core/modules/relayer/module/RelayerAsyncRequest.ts';

const [url, payloadPath] = process.argv.slice(2);
const calls: Array<{ method: string; status: number; elapsedMs: number }> = [];
const originalFetch = globalThis.fetch;
const started = performance.now();
globalThis.fetch = async (input, init) => {
  const response = await originalFetch(input, init);
  calls.push({ method: init?.method ?? 'GET', status: response.status, elapsedMs: performance.now() - started });
  return response;
};
try {
  await new RelayerAsyncRequest({
    relayerOperation: 'USER_DECRYPT',
    url,
    payload: JSON.parse(readFileSync(payloadPath, 'utf8')),
  }).run();
  assert.fail('Expected terminal readiness error');
} catch (error: any) {
  assert.equal(error.status, 503);
  assert.equal(error.relayerApiError?.label, 'readiness_check_timed_out');
}
assert.equal(calls.filter((c) => c.method === 'POST').length, 1);
assert.equal(calls.at(-1)?.status, 503);
const terminalCount = calls.length;
await Bun.sleep(3000);
assert.equal(calls.length, terminalCount, 'SDK must not silently resubmit after terminal 503');
console.log(JSON.stringify({ calls, elapsedMs: performance.now() - started }));
