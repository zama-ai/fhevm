/**
 * The official end-to-end scenarios against the local fhEVM stack, through the one command a user
 * runs: `run --profile local`. Run with `npm run test:e2e:local`, which copies the package into the
 * e2e container (run.sh next to this file); the engine runs there, and its run directory is copied
 * back for the assertions.
 */
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { before, describe, it } from 'node:test';

import type { EvidenceEntry } from '../../../src/orchestration/reporting/evidence.js';
import { readEvidence, readReport, scenarioOf, tempDir } from '../../helpers.js';

const CONTAINER = process.env.TE_E2E_CONTAINER ?? 'fhevm-test-suite-e2e-debug';
const DIR = process.env.TE_E2E_DIR ?? '/tmp/test-engine-e2e';

describe('Official end-to-end scenarios on the local stack', () => {
  let run: { code: number | null; output: string };
  let outputDir: string;
  let entries: EvidenceEntry[];

  before(() => {
    const remoteDir = `${DIR}/reports/e2e-local`;
    const engine = spawnSync(
      'docker',
      [
        'exec',
        '-w',
        DIR,
        CONTAINER,
        'node',
        'dist/src/cli/main.js',
        'run',
        '--profile',
        'local',
        '--tags',
        '@e2e',
        '--output-dir',
        remoteDir,
      ],
      { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 },
    );
    run = { code: engine.status, output: `${engine.stdout}${engine.stderr}` };
    outputDir = tempDir('test-engine-e2e-');
    const copy = spawnSync('docker', ['cp', `${CONTAINER}:${remoteDir}/.`, outputDir], { encoding: 'utf8' });
    assert.equal(copy.status, 0, copy.stderr);
    entries = readEvidence(outputDir).entries;
  });

  it('passes the preflight, protocol contracts included', () => {
    const { preflight } = readReport(outputDir).run;
    assert.equal(preflight?.status, 'passed', run.output);
    assert.deepEqual(
      preflight?.checks.map((check) => [check.name, check.status]),
      [
        ['chain.rpc', 'passed'],
        ['contracts.code', 'passed'],
        ['relayer.keyurl', 'passed'],
        ['relayer.fhe-key', 'passed'],
      ],
    );
  });

  it('passes fhe.input-decrypt with the single run command', () => {
    assert.equal(run.code, 0, run.output);
    assert.equal(scenarioOf(readReport(outputDir), 'fhe.input-decrypt').outcome, 'passed');
  });

  it('records the deployment, the transaction and every FHE operation with its relayer job', () => {
    const kinds = (kind: string) =>
      entries.filter((entry) => entry.scenarioId === 'fhe.input-decrypt' && entry.kind === kind);
    assert.equal(kinds('chain.deploy').length, 1);
    assert.equal(kinds('chain.tx').length, 1);
    for (const kind of ['fhe.encrypt', 'fhe.user-decrypt', 'fhe.public-decrypt']) {
      const [entry] = kinds(kind);
      assert.ok(entry, `${kind} recorded`);
      assert.equal(entry.data.status, 'success', kind);
      const relayer = entry.data.relayer as { jobId?: string } | undefined;
      assert.match(relayer?.jobId ?? '', /^[0-9a-f-]{36}$/, `${kind} has a relayer job ID`);
    }
  });
});
