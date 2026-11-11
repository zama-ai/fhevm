/**
 * FHE module scenarios through the engine against the local fhEVM stack. Run with
 * `npm run test:integration:fhe`, which copies the package into the e2e container (run.sh next to
 * this file); the engine runs there, and its run directory is copied back for the assertions.
 */
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { before, describe, it } from 'node:test';

import type { EvidenceEntry } from '../../../src/orchestration/reporting/evidence.js';
import { filesBelow, readEvidence, readReport, scenarioOf, tempDir } from '../../helpers.js';

const CONTAINER = process.env.TE_FHE_CONTAINER ?? 'fhevm-test-suite-e2e-debug';
const DIR = process.env.TE_FHE_DIR ?? '/tmp/test-engine-fhe';
const SCENARIOS = [
  'fixture.fhe-input',
  'fixture.fhe-multi-input',
  'fixture.fhe-user-decrypt-types',
  'fixture.fhe-delegation',
];

/** Runs the engine in the container and copies its run directory back. */
function runInContainer(args: string[]): { code: number | null; output: string; outputDir: string } {
  const remoteDir = `${DIR}/reports/integration-fhe`;
  const run = spawnSync(
    'docker',
    ['exec', '-w', DIR, CONTAINER, 'node', 'dist/src/cli/main.js', ...args, '--output-dir', remoteDir],
    { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 },
  );
  const outputDir = tempDir('test-engine-fhe-');
  const copy = spawnSync('docker', ['cp', `${CONTAINER}:${remoteDir}/.`, outputDir], { encoding: 'utf8' });
  assert.equal(copy.status, 0, copy.stderr);
  return { code: run.status, output: `${run.stdout}${run.stderr}`, outputDir };
}

describe('FHE scenarios through the engine on the local stack', () => {
  let result: ReturnType<typeof runInContainer>;
  let entries: EvidenceEntry[];
  const of = (scenarioId: string, kind: string) =>
    entries.filter((entry) => entry.scenarioId === scenarioId && entry.kind === kind);

  before(() => {
    result = runInContainer([
      'run',
      '--profile',
      'local',
      '--scenarios-dir',
      'test/fixtures/fhe-stack-scenarios',
      '--tags',
      '@fhe',
    ]);
    entries = readEvidence(result.outputDir).entries;
  });

  it('passes every FHE scenario', () => {
    assert.equal(result.code, 0, result.output);
    const report = readReport(result.outputDir);
    for (const id of SCENARIOS) assert.equal(scenarioOf(report, id).outcome, 'passed', id);
  });

  it('creates one SDK client per scenario and records it', () => {
    for (const id of SCENARIOS) {
      const clients = of(id, 'fhe.client');
      assert.equal(clients.length, 1, id);
      assert.equal(clients[0]!.data.status, 'success');
      // The relayer directly, or the relayer proxy that test-suite.env points at since main moved it.
      assert.match(String(clients[0]!.data.relayerUrl), /^http:\/\/fhevm-relayer(-proxy)?:\d+\/v2$/);
      assert.equal(clients[0]!.data.threads, 'pool');
    }
  });

  it('records a relayer job ID for every successful FHE operation', () => {
    const operations = entries.filter((entry) => entry.kind.startsWith('fhe.') && entry.kind !== 'fhe.client');
    assert.ok(operations.length >= 10, `only ${operations.length} FHE operations recorded`);
    for (const entry of operations.filter((op) => op.data.status === 'success')) {
      const relayer = entry.data.relayer as { jobId?: string; requestId?: string; status?: string };
      assert.match(relayer.jobId ?? '', /^[0-9a-f-]{36}$/, `${entry.kind} ${entry.label}`);
      assert.match(relayer.requestId ?? '', /^[0-9a-f-]{36}$/);
      assert.equal(relayer.status, 'succeeded');
    }
  });

  it('reuses the cached permit, records the unified permit and the public decryption', () => {
    const decryptions = of('fixture.fhe-input', 'fhe.user-decrypt');
    assert.deepEqual(
      decryptions.map((entry) => [entry.data.permit, entry.data.cache, entry.data.status]),
      [
        ['legacy', 'miss', 'success'],
        ['legacy', 'hit', 'success'],
        ['unified', 'miss', 'success'],
        ['legacy', 'miss', 'failed'],
      ],
    );
    assert.deepEqual((decryptions[0]!.data.values as unknown[])[0], { type: 'uint64', value: '49' });
    assert.equal(of('fixture.fhe-input', 'fhe.public-decrypt')[0]!.data.status, 'success');
  });

  it("records carol's rejection by the SDK before any relayer call", () => {
    const rejected = of('fixture.fhe-input', 'fhe.user-decrypt').find((entry) => entry.data.status === 'failed')!;
    assert.deepEqual((rejected.data.user as { role: string }).role, 'carol');
    assert.match(String(rejected.data.error), /is not authorized to decrypt handle/);
    assert.ok(!('relayer' in rejected.data));
  });

  it('encrypts two types in one proof and decrypts the eight types in one request', () => {
    assert.deepEqual(of('fixture.fhe-multi-input', 'fhe.encrypt')[0]!.data.types, ['uint64', 'uint32']);
    const [batch] = of('fixture.fhe-user-decrypt-types', 'fhe.user-decrypt');
    assert.equal((batch!.data.handles as unknown[]).length, 8);
    assert.equal((batch!.data.values as unknown[]).length, 8);
  });

  it('records the delegated decryption with the delegate and the delegator', () => {
    const [delegated] = of('fixture.fhe-delegation', 'fhe.delegated-decrypt');
    assert.equal((delegated!.data.delegate as { role: string }).role, 'bob');
    assert.equal((delegated!.data.delegator as { role: string }).role, 'alice');
    assert.equal((delegated!.data.relayer as { operation: string }).operation, 'DELEGATED_USER_DECRYPT');
  });

  it('never writes the mnemonic to the console or to any output file', (t) => {
    // The file `fhevm-cli up` writes, which the local profile reads (the container has the same values).
    const envFile = '../../.fhevm/runtime/env/test-suite.env';
    const mnemonic = existsSync(envFile)
      ? /^MNEMONIC="?([^"\n]+)"?$/m.exec(readFileSync(envFile, 'utf8'))?.[1]
      : undefined;
    if (!mnemonic) return t.skip(`no MNEMONIC in ${envFile}`);
    for (const content of [result.output, ...filesBelow(result.outputDir).map((file) => readFileSync(file, 'utf8'))]) {
      assert.ok(!content.includes(mnemonic));
    }
  });
});
