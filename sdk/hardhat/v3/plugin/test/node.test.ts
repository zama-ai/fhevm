// Proves the ordering the pre-deploy relies on: `newConnection` (where the chain is prepared) runs
// before a JSON-RPC server starts listening. `createServer` builds its connection exactly as the
// builtin `node` task does — create the connection, then listen — so the marker the hook leaves
// (one mined block) must already be visible on the FIRST request, from raw HTTP and from a second
// hardhat environment connecting over an `http` network. That second, remote connection must not
// prepare anything itself: the chain is not its own.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';

import plugin from '../pkg/_esm/index.js';

async function rpc(url: string, method: string): Promise<unknown> {
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params: [] }),
  });
  const body = (await response.json()) as { result?: unknown; error?: unknown };
  assert.equal(body.error, undefined, `${method} failed: ${JSON.stringify(body.error)}`);
  return body.result;
}

void test('the chain is prepared before the node server accepts its first request', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const server = await hre.network.createServer('default', '127.0.0.1', 0);
  const { address, port } = await server.listen();
  const url = `http://${address}:${port}`;

  try {
    assert.equal(await rpc(url, 'eth_blockNumber'), '0x1', 'the first request must already see the marker block');

    const remote = await createHardhatRuntimeEnvironment({
      plugins: [plugin],
      networks: { localhost: { type: 'http', url } },
    });
    const connection = await remote.network.create('localhost');
    try {
      assert.equal(connection.networkConfig.type, 'http');
      assert.equal(connection.fhevm.network.kind, 'localhost', 'a remote node on 31337 is a localhost dev node');
      assert.equal(connection.fhevm.isDevelopment, true);
      assert.equal(await connection.provider.request({ method: 'eth_blockNumber' }), '0x1');
      assert.equal(await rpc(url, 'eth_blockNumber'), '0x1', 'an http connection must not prepare the chain again');
    } finally {
      await connection.close();
    }
  } finally {
    await server.close();
  }
});
