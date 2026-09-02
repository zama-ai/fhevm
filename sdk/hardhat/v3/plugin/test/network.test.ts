// Network detection against FAKE connections: only `networkConfig.type`, the live `eth_chainId` and
// the configured chain id feed the classification, so a handful of shaped objects covers every
// branch without a node.

import assert from 'node:assert/strict';
import test from 'node:test';

import { HardhatPluginError } from 'hardhat/plugins';
import type { NetworkConnection } from 'hardhat/types/network';

import {
  isCleartextNetwork,
  isDevelopmentNetwork,
  isPublicNetwork,
  resolveFhevmNetwork,
} from '../pkg/_esm/internal/network.js';

type FakeConfig =
  { type: 'edr-simulated'; chainId: number } | { type: 'http'; chainId?: number; url: { getUrl(): Promise<string> } };

function fakeConnection(
  networkName: string,
  liveChainId: number,
  networkConfig: FakeConfig,
): NetworkConnection<string> {
  const provider = {
    request: ({ method }: { method: string }): Promise<unknown> => {
      if (method !== 'eth_chainId') throw new Error(`unexpected ${method}`);
      return Promise.resolve(`0x${liveChainId.toString(16)}`);
    },
  };
  return { networkName, networkConfig, provider } as unknown as NetworkConnection<string>;
}

const http = (url: string, chainId?: number): FakeConfig => ({
  type: 'http',
  url: { getUrl: () => Promise.resolve(url) },
  ...(chainId === undefined ? {} : { chainId }),
});

void test('an in-process EDR chain is hardhat, whatever its name', async () => {
  const info = await resolveFhevmNetwork(fakeConnection('node', 31337, { type: 'edr-simulated', chainId: 31337 }));
  assert.deepEqual(info, { networkName: 'node', chainId: 31337, kind: 'hardhat', url: undefined });
  assert.equal(isDevelopmentNetwork(info), true);
  assert.equal(isCleartextNetwork(info), true);
  assert.equal(isPublicNetwork(info), false);
});

void test('a remote node on the development chain id is localhost, with its url', async () => {
  const info = await resolveFhevmNetwork(fakeConnection('anvil', 31337, http('http://localhost:8545')));
  assert.deepEqual(info, { networkName: 'anvil', chainId: 31337, kind: 'localhost', url: 'http://localhost:8545' });
  assert.equal(isDevelopmentNetwork(info), true);
  assert.equal(isPublicNetwork(info), false);
});

void test('public chains are classified by their live chain id', async () => {
  const cases: Array<[number, string]> = [
    [1, 'mainnet'],
    [11_155_111, 'sepolia'],
    [137, 'polygon'],
    [80_002, 'polygon-amoy'],
  ];
  for (const [chainId, kind] of cases) {
    const info = await resolveFhevmNetwork(fakeConnection('remote', chainId, http('https://rpc.example')));
    assert.equal(info.kind, kind, `chain ${String(chainId)}`);
    assert.equal(isPublicNetwork(info), true);
    assert.equal(isDevelopmentNetwork(info), false);
    assert.equal(isCleartextNetwork(info), false);
  }
});

void test('an unrecognised chain is unknown, not an error', async () => {
  const info = await resolveFhevmNetwork(fakeConnection('other', 99_999, http('https://rpc.example')));
  assert.equal(info.kind, 'unknown');
  assert.equal(isPublicNetwork(info), false);
  assert.equal(isDevelopmentNetwork(info), false);
});

void test('a configured chain id that disagrees with the node is a plugin error', async () => {
  await assert.rejects(
    resolveFhevmNetwork(fakeConnection('sepolia', 1, http('https://rpc.example', 11_155_111))),
    (error: unknown) =>
      HardhatPluginError.isHardhatPluginError(error) &&
      error.pluginId === 'fhevm' &&
      error.message.includes('configured with chainId 11155111, but the node reports 1'),
  );
});

void test('a configured chain id that matches the node passes', async () => {
  const info = await resolveFhevmNetwork(
    fakeConnection('sepolia', 11_155_111, http('https://rpc.example', 11_155_111)),
  );
  assert.equal(info.kind, 'sepolia');
});
