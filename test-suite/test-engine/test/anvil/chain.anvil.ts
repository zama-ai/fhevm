/**
 * Chain module tests against the throwaway Anvil node. Run with `npm run test:chain`, which starts
 * the node (test/anvil/compose.yaml), sets TE_ANVIL_RPC_URL and removes the node afterwards.
 */
import type { Contract } from 'ethers';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { after, before, describe, it } from 'node:test';

import type { EvidencePayload } from '../../src/api/evidence.js';
import { loadArtifact } from '../../src/chain/artifacts.js';
import { Chain } from '../../src/chain/chain.js';
import { bufferedGasLimit } from '../../src/chain/gas.js';
import { loadProfile } from '../../src/profiles/profile.js';
import type { ResolvedChainSection } from '../../src/profiles/types.js';
import {
  ANVIL_MNEMONIC,
  CHAIN_PROFILES,
  CHAIN_SCENARIOS,
  COUNTER_ARTIFACT,
  filesBelow,
  readEvidence,
  readReport,
  runCli,
  scenarioOf,
} from '../helpers.js';

const section: ResolvedChainSection = loadProfile('fx-anvil', { profilesDir: CHAIN_PROFILES }).chain!;
const counterArtifact = loadArtifact(COUNTER_ARTIFACT);

function newChain(overrides: Partial<ResolvedChainSection> = {}) {
  const records: Omit<EvidencePayload, 'timestamp'>[] = [];
  const chain = new Chain({
    profileName: 'fx-anvil',
    section: { ...section, ...overrides },
    evidence: {
      record: (kind, data, options = {}) => {
        records.push({ kind, data, ...options });
      },
    },
    pollingIntervalMs: 100,
  });
  return { chain, records };
}

describe('Chain module on Anvil', () => {
  const { chain, records } = newChain();
  let counter: Contract;

  before(async () => {
    await chain.verifyConnection();
  });
  after(() => chain.destroy());

  it('deploys from an artifact, records chain.deploy and returns a working contract', async () => {
    counter = await chain.deploy(counterArtifact, [5], { from: 'alice', label: 'Deploy Counter' });
    const deploy = records.at(-1)!;
    assert.equal(deploy.kind, 'chain.deploy');
    assert.equal(deploy.label, 'Deploy Counter');
    assert.equal(deploy.data.status, 'success');
    assert.equal(deploy.data.chainId, 31337);
    assert.equal(deploy.data.contract, 'Counter');
    assert.equal(deploy.data.address, await counter.getAddress());
    assert.deepEqual(deploy.data.deployer, { role: 'alice', address: chain.wallet('alice').address });
    assert.match(String(deploy.data.hash), /^0x[0-9a-f]{64}$/);
    assert.equal(await counter.getFunction('count')(), 5n);
  });

  it('sends a transaction with the gas buffer and records chain.tx', async () => {
    const alice = chain.wallet('alice');
    const data = counter.interface.encodeFunctionData('increment');
    const estimate = await chain.provider.estimateGas({ from: alice.address, to: await counter.getAddress(), data });

    const receipt = await chain.send(counter.getFunction('increment')(), { label: 'increment' });
    const tx = await chain.provider.getTransaction(receipt.hash);
    assert.equal(tx!.gasLimit, bufferedGasLimit(estimate, section.gas));

    const entry = records.at(-1)!;
    assert.equal(entry.kind, 'chain.tx');
    assert.equal(entry.label, 'increment');
    assert.deepEqual(entry.data, {
      chainId: 31337,
      hash: receipt.hash,
      from: { role: 'alice', address: alice.address },
      to: await counter.getAddress(),
      blockNumber: receipt.blockNumber,
      gasUsed: receipt.gasUsed.toString(),
      status: 'success',
    });
    assert.equal(await counter.getFunction('count')(), 6n);
  });

  it('sends from another role', async () => {
    const bob = chain.wallet('bob');
    await chain.send((counter.connect(bob) as Contract).getFunction('incrementBy')(10));
    assert.deepEqual(records.at(-1)!.data.from, { role: 'bob', address: bob.address });
    assert.equal(await counter.getFunction('count')(), 16n);
  });

  it('records a reverted transaction, then fails', async () => {
    await assert.rejects(
      chain.send(counter.getFunction('fail')({ gasLimit: 100_000 }), { label: 'always fails' }),
      /transaction 'always fails' reverted: 0x[0-9a-f]{64} \(block \d+\)/,
    );
    const entry = records.at(-1)!;
    assert.equal(entry.kind, 'chain.tx');
    assert.equal(entry.data.status, 'reverted');
    assert.match(String(entry.data.hash), /^0x[0-9a-f]{64}$/);
    const receipt = await chain.provider.getTransactionReceipt(String(entry.data.hash));
    assert.equal(receipt!.status, 0);
  });

  it('records a transaction rejected before broadcast (failed gas estimation), then fails', async () => {
    await assert.rejects(
      chain.send(counter.getFunction('fail')(), { label: 'estimate' }),
      /'estimate' was rejected: .*Counter: always fails/,
    );
    const entry = records.at(-1)!;
    assert.equal(entry.data.status, 'rejected');
    assert.match(String(entry.data.error), /Counter: always fails/);
  });

  it('records a reverted deployment, then fails', async () => {
    await assert.rejects(
      chain.deploy(counterArtifact, [1000, { gasLimit: 1_000_000 }], { from: 'carol' }),
      /deployment of Counter reverted: 0x[0-9a-f]{64}/,
    );
    const entry = records.at(-1)!;
    assert.equal(entry.kind, 'chain.deploy');
    assert.equal(entry.data.status, 'reverted');
    assert.equal((entry.data.deployer as { role: string }).role, 'carol');
  });

  it('waits for a block, and times out on a block that never comes', async () => {
    const current = await chain.provider.getBlockNumber();
    assert.ok((await chain.waitForBlock(current)) >= current);
    await assert.rejects(chain.waitForBlock(current + 1_000, 300), /block \d+ not reached within 300 ms/);
  });
});

describe('Chain ID check on Anvil', () => {
  it('fails before any transaction is sent when the RPC reports another chain ID', async () => {
    const { chain, records } = newChain({ chainId: { value: 1, secret: false, source: 'literal' } });
    const blockBefore = await chain.provider.getBlockNumber();
    const alice = chain.wallet('alice');
    const nonceBefore = await chain.provider.getTransactionCount(alice.address);

    await assert.rejects(
      chain.send(alice.sendTransaction({ to: chain.wallet('bob').address, value: 1n })),
      /profile 'fx-anvil' expects chain ID 1, but the RPC at http:\/\/127\.0\.0\.1:\d+ reports 31337; no transaction was sent/,
    );
    await assert.rejects(chain.deploy(counterArtifact, [0], { from: 'alice' }), /expects chain ID 1/);
    assert.equal(await chain.provider.getBlockNumber(), blockBefore);
    assert.equal(await chain.provider.getTransactionCount(alice.address), nonceBefore);
    assert.deepEqual(
      records.map((record) => [record.kind, record.data.status]),
      [
        ['chain.tx', 'rejected'],
        ['chain.deploy', 'rejected'],
      ],
    );
    chain.destroy();
  });

  it('fails with a clear message when the RPC is down (wrong port)', async () => {
    const { chain } = newChain({ rpcUrl: { value: 'http://127.0.0.1:9', secret: false, source: 'literal' } });
    await assert.rejects(
      chain.verifyConnection(),
      /profile 'fx-anvil': cannot reach the chain RPC at http:\/\/127\.0\.0\.1:9/,
    );
    chain.destroy();
  });
});

describe('Chain scenarios through the engine on Anvil', () => {
  const options = { scenariosDir: CHAIN_SCENARIOS, profilesDir: CHAIN_PROFILES };

  it('runs the fixture scenario on fx-anvil, records chain evidence and exits 0', () => {
    const result = runCli(['run', 'fixture.chain-basic', '--profile', 'fx-anvil'], options);
    assert.equal(result.code, 0, result.stdout + result.stderr);
    assert.equal(scenarioOf(readReport(result.outputDir), 'fixture.chain-basic').outcome, 'passed');

    const { entries } = readEvidence(result.outputDir);
    const chainEntries = entries.filter((entry) => entry.kind.startsWith('chain.'));
    assert.deepEqual(
      chainEntries.map((entry) => [entry.kind, entry.label, entry.data.status, entry.scope]),
      [
        ['chain.deploy', 'Deploy Counter', 'success', 'step'],
        ['chain.tx', 'alice increments', 'success', 'step'],
        ['chain.tx', 'bob increments', 'success', 'step'],
      ],
    );
    assert.equal(chainEntries[0]!.step!.text, 'alice deploys a Counter starting at 5');

    const profile = entries.find((entry) => entry.kind === 'engine.profile')!;
    assert.deepEqual((profile.data.chain as ResolvedChainSection).mnemonic.value, '[REDACTED:chain.mnemonic]');
    assert.ok(readFileSync(`${result.outputDir}/report.html`, 'utf8').includes('chain.tx'));
  });

  it('fails the scenario on a reverted transaction and keeps its evidence', () => {
    const result = runCli(['run', 'fixture.chain-revert', '--profile', 'fx-anvil'], options);
    assert.equal(result.code, 1, result.stdout + result.stderr);
    assert.equal(scenarioOf(readReport(result.outputDir), 'fixture.chain-revert').outcome, 'failed');
    const reverted = readEvidence(result.outputDir).entries.filter(
      (entry) => entry.kind === 'chain.tx' && entry.data.status === 'reverted',
    );
    assert.equal(reverted.length, 1);
    assert.equal(reverted[0]!.label, 'always fails');
  });

  it('never writes the mnemonic to the console or to any output file', () => {
    const result = runCli(['run', '--profile', 'fx-anvil'], options);
    assert.equal(result.code, 1, result.stdout + result.stderr);
    for (const content of [
      result.stdout,
      result.stderr,
      ...filesBelow(result.outputDir).map((file) => readFileSync(file, 'utf8')),
    ]) {
      assert.ok(!content.includes(ANVIL_MNEMONIC));
    }
  });
});
