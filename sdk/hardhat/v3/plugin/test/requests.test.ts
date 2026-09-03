// The onRequest behaviours, first against a fake forwarder (exact arithmetic, pass-through cases),
// then on a live connection: the same transfer estimated WITHOUT the plugin, times 1.2, is what a
// connection with the plugin must answer — proof the handler sits in the request chain.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import type { JsonRpcRequest, JsonRpcResponse } from 'hardhat/types/providers';

import plugin from '#esm/index.js';
import { handleRequest, inflateGasEstimate } from '#esm/internal/requests.js';

const request = (method: string): JsonRpcRequest => ({ jsonrpc: '2.0', id: 1, method, params: [] });
const ok = (result: unknown): JsonRpcResponse => ({ jsonrpc: '2.0', id: 1, result });
const failed: JsonRpcResponse = { jsonrpc: '2.0', id: 1, error: { code: -32000, message: 'reverted' } };

void test('eth_estimateGas results are inflated by 120%', () => {
  assert.deepEqual(inflateGasEstimate(ok('0x5208')), ok('0x6270'));
  assert.deepEqual(inflateGasEstimate(ok('0x0')), ok('0x0'));
});

void test('non-string results and error responses pass through untouched', () => {
  assert.deepEqual(inflateGasEstimate(ok(21000)), ok(21000));
  assert.deepEqual(inflateGasEstimate(failed), failed);
});

void test('only estimate and send requests are handled; everything else is forwarded as is', async () => {
  const seen: string[] = [];
  const forward = (forwarded: JsonRpcRequest): Promise<JsonRpcResponse> => {
    seen.push(forwarded.method);
    return Promise.resolve(ok('0x5208'));
  };
  assert.deepEqual(await handleRequest(request('eth_estimateGas'), forward), ok('0x6270'));
  assert.deepEqual(await handleRequest(request('eth_sendTransaction'), forward), ok('0x5208'));
  assert.deepEqual(await handleRequest(request('eth_blockNumber'), forward), ok('0x5208'));
  assert.deepEqual(seen, ['eth_estimateGas', 'eth_sendTransaction', 'eth_blockNumber']);
});

async function estimateTransfer(plugins: Array<typeof plugin>): Promise<bigint> {
  const hre = await createHardhatRuntimeEnvironment({ plugins });
  const connection = await hre.network.create();
  try {
    const [from, to] = (await connection.provider.request({ method: 'eth_accounts' })) as string[];
    const estimate: unknown = await connection.provider.request({
      method: 'eth_estimateGas',
      params: [{ from, to, value: '0x1' }],
    });
    assert.equal(typeof estimate, 'string');
    return BigInt(estimate as string);
  } finally {
    await connection.close();
  }
}

void test('a live connection answers the bare estimate inflated by 120%', async () => {
  const bare = await estimateTransfer([]);
  const inflated = await estimateTransfer([plugin]);
  assert.equal(inflated, (bare * 120n) / 100n, `bare ${String(bare)}, inflated ${String(inflated)}`);
});
