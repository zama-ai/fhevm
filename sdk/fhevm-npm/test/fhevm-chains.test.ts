import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { type RegistryReader, chainsConfigPath, renderChainsConfig } from '../base/fhevm-chains.ts';
import { checkFhevmChainsOrigin } from '../commands/check-fhevm-chains-origin.ts';

const PIN = 'a'.repeat(40);
const ADDR = (last: string): string => `0x${'1'.repeat(39)}${last}`;

function registry(overrides?: { dropContract?: string; moveChain?: string }): string {
  const contracts: Record<string, { address: string; chain: string }> = {
    ACL_HOST: { address: ADDR('a'), chain: 'ethereum' },
    FHEVM_EXECUTOR: { address: ADDR('b'), chain: 'ethereum' },
    HCU_LIMIT: { address: ADDR('c'), chain: 'ethereum' },
    INPUT_VERIFIER: { address: ADDR('d'), chain: 'ethereum' },
    KMS_GENERATION_HOST: { address: ADDR('e'), chain: 'ethereum' },
    KMS_VERIFIER: { address: ADDR('f'), chain: 'ethereum' },
    PAUSER_SET_HOST: { address: ADDR('0'), chain: 'ethereum' },
    PROTOCOL_CONFIG: { address: ADDR('1'), chain: 'ethereum' },
    // A second host chain, discovered from its prefixed ACL_HOST; no KMS_GENERATION_HOST on purpose.
    POLY_ACL_HOST: { address: ADDR('2'), chain: 'poly' },
    POLY_FHEVM_EXECUTOR: { address: ADDR('3'), chain: 'poly' },
    POLY_HCU_LIMIT: { address: ADDR('4'), chain: 'poly' },
    POLY_INPUT_VERIFIER: { address: ADDR('5'), chain: 'poly' },
    POLY_KMS_VERIFIER: { address: ADDR('6'), chain: 'poly' },
    POLY_PAUSER_SET_HOST: { address: ADDR('7'), chain: 'poly' },
    POLY_PROTOCOL_CONFIG: { address: ADDR('8'), chain: 'poly' },
    CIPHERTEXT_COMMITS: { address: ADDR('9'), chain: 'gw' },
    DECRYPTION: { address: ADDR('aa').slice(0, 42), chain: 'gw' },
    GATEWAY_CONFIG: { address: ADDR('b'), chain: 'gw' },
    INPUT_VERIFICATION: { address: ADDR('c'), chain: 'gw' },
    KMS_GENERATION: { address: ADDR('d'), chain: 'gw' },
    MULTICHAIN_ACL: { address: ADDR('e'), chain: 'gw' },
    PAUSER_SET_GATEWAY: { address: ADDR('f'), chain: 'gw' },
  };
  if (overrides?.dropContract !== undefined) delete contracts[overrides.dropContract];
  if (overrides?.moveChain !== undefined) contracts[overrides.moveChain]!.chain = 'elsewhere';
  return JSON.stringify({
    chains: { ethereum: { chain_id: 1 }, poly: { chain_id: 137 }, gw: { chain_id: 261131 } },
    contracts,
  });
}

function fakeReader(text: string, head: string = PIN): RegistryReader {
  return {
    fetchFile: (path, ref) => {
      assert.match(path, /^dist\/(mainnet|testnet)\.json$/);
      assert.equal(ref, head, 'the check must fetch at the registry HEAD');
      return text;
    },
    resolveHead: () => head,
  };
}

test('renders both networks: discovered hosts, complete gateway set, pinned source header', async () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-chains-'));
  try {
    const text = await renderChainsConfig(workspace, fakeReader(registry()), PIN);
    const parsed = JSON.parse(text) as {
      source: { commit: string };
      networks: Record<string, { relayerUrl: string; gateway: { id: number }; hosts: Record<string, unknown> }>;
    };
    assert.equal(parsed.source.commit, PIN);
    assert.deepEqual(Object.keys(parsed.networks), ['mainnet', 'testnet']);
    assert.equal(parsed.networks['mainnet']?.relayerUrl, 'https://relayer.mainnet.zama.org');
    assert.equal(parsed.networks['testnet']?.relayerUrl, 'https://relayer.testnet.zama.org');
    assert.equal(parsed.networks['mainnet']?.gateway.id, 261131);
    assert.deepEqual(Object.keys(parsed.networks['mainnet']?.hosts ?? {}), ['ethereum', 'poly']);

    const hosts = parsed.networks['mainnet']?.hosts as Record<string, { contracts: Record<string, unknown> }>;
    assert.ok('kmsGeneration' in (hosts['ethereum']?.contracts ?? {}));
    assert.ok(!('kmsGeneration' in (hosts['poly']?.contracts ?? {})), 'optional face is omitted, not failed');
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('refuses a registry missing a required contract, or one on the wrong chain', async () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-chains-'));
  try {
    await assert.rejects(
      renderChainsConfig(workspace, fakeReader(registry({ dropContract: 'POLY_KMS_VERIFIER' })), PIN),
      /registry has no POLY_KMS_VERIFIER/,
    );
    await assert.rejects(
      renderChainsConfig(workspace, fakeReader(registry({ moveChain: 'GATEWAY_CONFIG' })), PIN),
      /GATEWAY_CONFIG is on chain 'elsewhere'/,
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('check-fhevm-chains-origin: missing file, current file, tampered file', async () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-chains-'));
  const reader = fakeReader(registry());
  try {
    const missing = await checkFhevmChainsOrigin({ workspaceRoot: workspace, reader });
    assert.match(missing.violations[0]?.message ?? '', /missing/);

    writeFileSync(chainsConfigPath(workspace), await renderChainsConfig(workspace, reader, PIN));
    assert.deepEqual((await checkFhevmChainsOrigin({ workspaceRoot: workspace, reader })).violations, []);

    const tampered = (await renderChainsConfig(workspace, reader, PIN)).replace(ADDR('a'), ADDR('0'));
    writeFileSync(chainsConfigPath(workspace), tampered);
    const drifted = await checkFhevmChainsOrigin({ workspaceRoot: workspace, reader });
    assert.match(drifted.violations[0]?.message ?? '', /differs from what the registry's main head/);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('check-fhevm-chains-origin follows the registry HEAD: address drift is red, unrelated commits stay green', async () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-chains-'));
  const head = 'b'.repeat(40);
  try {
    // Synced at PIN, registry moved to `head` with the SAME addresses: current, and the pin may stay old.
    writeFileSync(chainsConfigPath(workspace), await renderChainsConfig(workspace, fakeReader(registry()), PIN));
    const unrelated = await checkFhevmChainsOrigin({ workspaceRoot: workspace, reader: fakeReader(registry(), head) });
    assert.deepEqual(unrelated.violations, []);

    // Registry moved AND an address changed: red, telling the dev to catch up with --latest.
    const moved = registry().replace(ADDR('b'), ADDR('0'));
    const drifted = await checkFhevmChainsOrigin({ workspaceRoot: workspace, reader: fakeReader(moved, head) });
    assert.match(drifted.violations[0]?.message ?? '', /main head \(bbbbbbbbbbbb\)/);
    assert.match(drifted.violations[0]?.message ?? '', /sync-fhevm-chains --latest/);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
