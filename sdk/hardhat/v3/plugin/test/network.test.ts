// Network detection against FAKE connections: only `networkConfig.type`, the live `eth_chainId` and
// the configured chain id feed the classification, so a handful of shaped objects covers every
// branch without a node.

import assert from 'node:assert/strict';
import test from 'node:test';

import { HardhatPluginError } from 'hardhat/plugins';
import type { NetworkConnection } from 'hardhat/types/network';

import { FHEVM_CHAINS } from '#esm/internal/vendored/fhevm-chains.js';
import {
  isCleartextNetwork,
  isDevelopmentNetwork,
  isPublicNetwork,
  resolveFhevmNetwork,
} from '#esm/internal/network.js';

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
  assert.deepEqual(info, { networkName: 'node', chainId: 31337, kind: 'hardhat', url: undefined, publicChains: [] });
  assert.equal(isDevelopmentNetwork(info), true);
  assert.equal(isCleartextNetwork(info), true);
  assert.equal(isPublicNetwork(info), false);
});

void test('a remote node on the development chain id is localhost, with its url', async () => {
  const info = await resolveFhevmNetwork(fakeConnection('anvil', 31337, http('http://localhost:8545')));
  assert.deepEqual(info, {
    networkName: 'anvil',
    chainId: 31337,
    kind: 'localhost',
    url: 'http://localhost:8545',
    publicChains: [],
  });
  assert.equal(isDevelopmentNetwork(info), true);
  assert.equal(isPublicNetwork(info), false);
});

void test('public chains are classified by their live chain id, one entry per group that serves them', async () => {
  const cases: Array<[number, Array<[string, string]>]> = [
    [1, [['mainnet', 'ethereum']]],
    [137, [['mainnet', 'polygon']]],
    // Sepolia and Amoy are served by the testnet AND the devnet gateway, with different addresses.
    [
      11_155_111,
      [
        ['testnet', 'ethereum_sepolia'],
        ['devnet', 'ethereum_sepolia'],
      ],
    ],
    [
      80_002,
      [
        ['testnet', 'polygon_amoy'],
        ['devnet', 'polygon_amoy'],
      ],
    ],
    [97, [['devnet', 'bnb_testnet']]],
    [560_048, [['devnet', 'ethereum_hoodi']]],
  ];
  for (const [chainId, expected] of cases) {
    const info = await resolveFhevmNetwork(fakeConnection('remote', chainId, http('https://rpc.example')));
    assert.equal(info.kind, 'public', `chain ${String(chainId)}`);
    assert.deepEqual(
      info.publicChains.map((chain) => [chain.group, chain.host.name]),
      expected,
      `chain ${String(chainId)}`,
    );
    for (const chain of info.publicChains) assert.equal(chain.host.id, chainId);
    assert.equal(isPublicNetwork(info), true);
    assert.equal(isDevelopmentNetwork(info), false);
    assert.equal(isCleartextNetwork(info), false);
  }
});

void test('an unrecognised chain is unknown, not an error', async () => {
  const info = await resolveFhevmNetwork(fakeConnection('other', 99_999, http('https://rpc.example')));
  assert.equal(info.kind, 'unknown');
  assert.deepEqual(info.publicChains, []);
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
  assert.equal(info.kind, 'public');
});

void test('a public chain carries the addresses of the generated face', async () => {
  const info = await resolveFhevmNetwork(fakeConnection('sepolia', 11_155_111, http('https://rpc.example')));
  const testnet = info.publicChains.find((chain) => chain.group === 'testnet');
  assert.ok(testnet, 'Sepolia is served by the testnet gateway');
  assert.equal(
    testnet.host.fhevm.contracts.acl.address,
    FHEVM_CHAINS.testnet.hosts.ethereum_sepolia.fhevm.contracts.acl.address,
  );
  assert.equal(testnet.host.fhevm.contracts.acl.address, '0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D');
});
