// Proves the ordering the pre-deploy relies on: `newConnection` (where the stack is deployed) runs
// before a JSON-RPC server starts listening. `createServer` builds its connection exactly as the
// builtin `node` task does — create the connection, then listen — so the cleartext stack must already
// be there on the FIRST request, from raw HTTP and from a second hardhat environment connecting over
// an `http` network. That second, remote connection must not deploy again: the chain is not its own.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';

import plugin from '#esm/index.js';
import { LOCALHOST_DEPLOYER } from '#esm/internal/constants.js';
import { precomputeLocalhostAddresses } from '#esm/internal/deploy.js';

async function rpc(url: string, method: string, params: unknown[] = []): Promise<unknown> {
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }),
  });
  const body = (await response.json()) as { result?: unknown; error?: unknown };
  assert.equal(body.error, undefined, `${method} failed: ${JSON.stringify(body.error)}`);
  return body.result;
}

void test('the stack is deployed before the node server accepts its first request', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const server = await hre.network.createServer('default', '127.0.0.1', 0);
  const { address, port } = await server.listen();
  const url = `http://${address}:${port}`;
  const { aclAddress } = precomputeLocalhostAddresses().fhevmAddresses;

  try {
    assert.notEqual(await rpc(url, 'eth_getCode', [aclAddress, 'latest']), '0x', 'ACL code on the first request');
    const deployerNonce = await rpc(url, 'eth_getTransactionCount', [LOCALHOST_DEPLOYER.address, 'latest']);
    const blockNumber = await rpc(url, 'eth_blockNumber');

    const remote = await createHardhatRuntimeEnvironment({
      plugins: [plugin],
      networks: { localhost: { type: 'http', url } },
    });
    const connection = await remote.network.create('localhost');
    try {
      assert.equal(connection.networkConfig.type, 'http');
      assert.equal(connection.fhevm.network.kind, 'localhost', 'a remote node on 31337 is a localhost dev node');
      assert.equal(connection.fhevm.isDevelopment, true);
      assert.equal(await rpc(url, 'eth_getTransactionCount', [LOCALHOST_DEPLOYER.address, 'latest']), deployerNonce);
      assert.equal(await rpc(url, 'eth_blockNumber'), blockNumber, 'an http connection must not deploy again');
    } finally {
      await connection.close();
    }
  } finally {
    await server.close();
  }
});
