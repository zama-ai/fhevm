// The essential job, per CHAIN: every in-process connection is a fresh chain and gets its own stack;
// a cached connection is one chain, so one deploy; a public network is never touched — its provider
// sees nothing but the detection probe.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import type { NetworkConnection } from 'hardhat/types/network';

import plugin from '#esm/index.js';
import { LOCALHOST_DEPLOYER } from '#esm/internal/constants.js';
import { precomputeLocalhostAddresses } from '#esm/internal/deploy.js';
import { prepareDevelopmentChain } from '#esm/internal/prepare.js';

const { aclAddress } = precomputeLocalhostAddresses().fhevmAddresses;

async function code(connection: NetworkConnection, address: string): Promise<unknown> {
  return connection.provider.request({ method: 'eth_getCode', params: [address, 'latest'] });
}

async function deployerNonce(connection: NetworkConnection): Promise<unknown> {
  return connection.provider.request({
    method: 'eth_getTransactionCount',
    params: [LOCALHOST_DEPLOYER.address, 'latest'],
  });
}

void test('two created connections are two chains, each with its own stack', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const first = await hre.network.create();
  const second = await hre.network.create();
  try {
    assert.notEqual(await code(first, aclAddress), '0x');
    assert.notEqual(await code(second, aclAddress), '0x');
    assert.equal(await deployerNonce(first), await deployerNonce(second), 'same sequence on both chains');
  } finally {
    await first.close();
    await second.close();
  }
});

void test('a cached connection is one chain, so one deploy', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const first = await hre.network.getOrCreate();
  const second = await hre.network.getOrCreate();
  try {
    assert.equal(first, second, 'getOrCreate returns the same connection');
    assert.notEqual(await code(first, aclAddress), '0x');
  } finally {
    await first.close();
  }
});

void test('a public network is left untouched', async () => {
  const requests: string[] = [];
  const connection = {
    networkName: 'sepolia',
    networkConfig: { type: 'http', url: { getUrl: () => Promise.resolve('https://rpc.example') } },
    provider: {
      request: ({ method }: { method: string }): Promise<unknown> => {
        requests.push(method);
        return Promise.resolve('0xaa36a7');
      },
    },
  } as unknown as NetworkConnection<string>;

  const network = {
    networkName: 'sepolia',
    chainId: 11_155_111,
    kind: 'public',
    url: 'https://rpc.example',
    publicChains: [],
  } as const;
  assert.equal(await prepareDevelopmentChain(connection, network), undefined);
  assert.deepEqual(requests, [], 'no request at all reaches a public network');
});
