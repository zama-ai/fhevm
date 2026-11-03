import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { after, describe, it } from 'node:test';

import { FHE_PROFILES, FHE_SCENARIOS, readEvidence, readReport, runCli, runCliAsync } from '../../helpers.js';
import { type FakeEnvironment, startFakeEnvironment } from '../../unit/boot/fake-environment.js';

const options = { scenariosDir: FHE_SCENARIOS, profilesDir: FHE_PROFILES };

describe('Preflight through the CLI', () => {
  let env: FakeEnvironment | undefined;
  after(() => env?.close());

  it('checks the environment before running, then records the checks', async () => {
    env = await startFakeEnvironment();
    const result = await runCliAsync(['run', 'fixture.profile-sections', '--profile', 'fx-preflight'], {
      ...options,
      env: { TE_PF_RPC_URL: env.url, TE_PF_RELAYER_URL: `${env.url}/v2` },
    });
    assert.equal(result.code, 0, result.stdout + result.stderr);
    assert.match(result.stdout, /Preflight \(passed\):/);
    assert.ok(result.stdout.indexOf('Preflight') < result.stdout.indexOf('Running fixture.profile-sections'));
    const { preflight } = readReport(result.outputDir).run;
    assert.equal(preflight?.status, 'passed');
    assert.deepEqual(
      preflight?.checks.map((check) => check.name),
      ['chain.rpc', 'relayer.keyurl', 'relayer.fhe-key'],
    );
    const entry = readEvidence(result.outputDir).entries.find((candidate) => candidate.kind === 'engine.preflight');
    assert.equal(entry?.scope, 'run');
    assert.equal(entry?.data.status, 'passed');
  });

  it('stops before any scenario with exit 2 when the environment does not answer', () => {
    // fx-fhe points at the local stack's container names, which do not resolve here.
    const result = runCli(['run', 'fixture.profile-sections', '--profile', 'fx-fhe'], options);
    assert.equal(result.code, 2, result.stdout + result.stderr);
    assert.match(result.stdout, /Preflight \(failed\):/);
    assert.match(result.stdout, /chain\.rpc +failed/);
    assert.match(result.stdout, /relayer\.keyurl +failed .*relayer unreachable/);
    assert.match(result.stdout, /relayer\.fhe-key +skipped/);
    assert.match(result.stderr, /preflight failed; no scenario was run \(--no-preflight skips the checks\)/);
    assert.ok(!result.stdout.includes('Running fixture.profile-sections'), 'no worker started');
    assert.ok(!existsSync(path.join(result.outputDir, 'report.json')), 'no report on exit 2');
  });

  it('skips the checks with --no-preflight, on a profile without sections, and in dry-run', () => {
    const skipped = runCli(['run', 'fixture.profile-sections', '--profile', 'fx-fhe', '--no-preflight'], options);
    assert.equal(skipped.code, 0, skipped.stdout + skipped.stderr);
    assert.match(skipped.stdout, /Preflight skipped: --no-preflight\./);
    assert.deepEqual(readReport(skipped.outputDir).run.preflight, {
      status: 'skipped',
      reason: '--no-preflight',
      checks: [],
    });

    const none = runCli(['run', 'fixture.profile-sections'], options);
    assert.equal(none.code, 0, none.stdout + none.stderr);
    assert.equal(readReport(none.outputDir).run.preflight?.reason, "profile 'none' has no chain or relayer section");

    const dryRun = runCli(['dry-run', '--profile', 'fx-fhe'], options);
    assert.equal(dryRun.code, 0, dryRun.stdout + dryRun.stderr);
    assert.ok(!dryRun.stdout.includes('Preflight'));
    assert.equal(readReport(dryRun.outputDir).run.preflight, undefined);
  });

  it('accepts --no-preflight only with run', () => {
    for (const command of ['dry-run', 'list', 'profiles']) {
      const result = runCli([command, '--no-preflight'], options);
      assert.equal(result.code, 2, command);
      assert.match(result.stderr, /--no-preflight is only accepted by 'run'/);
    }
  });
});
