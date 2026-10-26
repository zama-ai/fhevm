import type { IRunConfiguration } from '@cucumber/cucumber/api';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import { DIST_DIR, PACKAGE_ROOT } from '../src/paths.js';
import { buildRunReport } from '../src/report/report.js';
import { buildWorkerEnv, executePlan } from '../src/runner/orchestrator.js';
import { assertRetryDisabled } from '../src/runner/worker.js';
import { createRedactor } from '../src/secrets/redactor.js';
import { FIXTURE_SECRET, fakeScenario, tempDir } from './helpers.js';

describe('buildWorkerEnv', () => {
  it('keeps only allowlisted variables', () => {
    const env = buildWorkerEnv(
      { PATH: '/bin', HOME: '/home/qa', LC_ALL: 'C', MNEMONIC: 'test test test', RPC_URL: 'http://x', NO_COLOR: '1' },
      false,
    );
    assert.deepEqual(env, { PATH: '/bin', HOME: '/home/qa', LC_ALL: 'C', NO_COLOR: '1' });
  });

  it('forces colours for a colour terminal unless NO_COLOR is set', () => {
    assert.equal(buildWorkerEnv({ PATH: '/bin' }, true).FORCE_COLOR, '1');
    assert.equal(buildWorkerEnv({ PATH: '/bin', NO_COLOR: '1' }, true).FORCE_COLOR, undefined);
  });
});

describe('assertRetryDisabled', () => {
  it('rejects any resolved configuration with retries', () => {
    const configuration = (retry: number) => ({ runtime: { retry } }) as unknown as IRunConfiguration;
    assert.doesNotThrow(() => assertRetryDisabled(configuration(0)));
    assert.throws(() => assertRetryDisabled(configuration(2)), /retries are not allowed/);
  });
});

describe('executePlan with a worker that dies before reading its job', () => {
  it('reports an error outcome instead of crashing (EPIPE on stdin is handled)', async () => {
    // A large job makes the stdin write still pending when the worker exits.
    const profile = {
      name: 'big',
      description: 'big',
      requiresConfirmation: false,
      settings: { big: { value: 'x'.repeat(4 * 1024 * 1024), secret: false, source: 'literal' } },
    };
    const results = await executePlan(
      {
        filters: { ids: [] },
        profile: 'big',
        items: [{ scenario: fakeScenario('crash'), decision: { action: 'run' } }],
      },
      {
        mode: 'run',
        outputDir: tempDir(),
        cwd: PACKAGE_ROOT,
        profile,
        redactor: createRedactor([]),
        workerEntry: path.join(DIST_DIR, 'test', 'fixtures', 'workers', 'exit-immediately.js'),
      },
    );
    assert.equal(results.length, 1);
    assert.equal(results[0]!.outcome, 'error');
    assert.equal(results[0]!.exitCode, 7);
    assert.match(results[0]!.reason!, /exited with code 7/);

    // The report is still built from that result.
    const report = buildRunReport({
      runId: 'r',
      mode: 'run',
      engineVersion: '0',
      startedAt: new Date(),
      finishedAt: new Date(),
      plan: {
        filters: { ids: [] },
        profile: 'big',
        items: [{ scenario: fakeScenario('crash'), decision: { action: 'run' } }],
      },
      results,
      profile,
      interrupted: false,
      outputDir: tempDir(),
      scenariosDir: PACKAGE_ROOT,
      packageRoot: PACKAGE_ROOT,
    });
    assert.equal(report.scenarios[0]!.outcome, 'error');
    assert.equal(report.summary.executed, 1);
    assert.equal(report.summary.success, false);
  });
});

describe('executePlan job transport', () => {
  it('sends the job (profile included) through stdin only: never argv, never the environment', async () => {
    const outputDir = tempDir();
    const profile = {
      name: 'fx',
      description: 'fx',
      requiresConfirmation: false,
      settings: { token: { value: FIXTURE_SECRET, secret: true, source: 'env:TE_FIXTURE_SECRET' } },
    };
    process.env.TE_FIXTURE_SECRET = FIXTURE_SECRET;
    try {
      const results = await executePlan(
        {
          filters: { ids: [] },
          profile: 'fx',
          items: [{ scenario: fakeScenario('probe'), decision: { action: 'run' } }],
        },
        {
          mode: 'run',
          outputDir,
          cwd: PACKAGE_ROOT,
          profile,
          redactor: createRedactor([{ key: 'token', value: FIXTURE_SECRET }]),
          workerEntry: path.join(DIST_DIR, 'test', 'fixtures', 'workers', 'record-invocation.js'),
        },
      );
      assert.equal(results[0]!.outcome, 'passed');
    } finally {
      delete process.env.TE_FIXTURE_SECRET;
    }
    const invocation = JSON.parse(
      readFileSync(path.join(outputDir, 'scenarios', 'probe', 'invocation.json'), 'utf8'),
    ) as {
      argv: string[];
      env: Record<string, string>;
      stdin: string;
    };
    assert.equal(invocation.argv.length, 2, 'argv holds only node and the worker entry');
    assert.ok(!JSON.stringify(invocation.argv).includes(FIXTURE_SECRET));
    assert.equal(invocation.env.TE_FIXTURE_SECRET, undefined);
    assert.ok(!JSON.stringify(invocation.env).includes(FIXTURE_SECRET));
    const job = JSON.parse(invocation.stdin) as { worldParameters: { profile: typeof profile }; stepTimeoutMs: number };
    assert.equal(job.worldParameters.profile.settings.token.value, FIXTURE_SECRET);
    assert.equal(job.stepTimeoutMs, 30_000);
  });
});
