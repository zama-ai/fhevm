import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import {
  COMPILED_CONTRACTS,
  CONTRACTS_SCENARIOS,
  FHE_PROFILES,
  readReport,
  runCli,
  scenarioOf,
} from '../../helpers.js';

const options = { scenariosDir: CONTRACTS_SCENARIOS };

describe('Compiled contracts through the CLI', () => {
  it('hands --contracts-dir to the scenarios (this.engine.contracts)', () => {
    const result = runCli(['run', 'fixture.compiled-contracts', '--contracts-dir', COMPILED_CONTRACTS], options);
    assert.equal(result.code, 0, result.stdout + result.stderr);
    assert.match(result.stdout, new RegExp(`loaded Counter from ${COMPILED_CONTRACTS}`));
    // On 'none' there is no contracts section to compare the build stamp with.
    assert.match(result.stdout, /contracts\.build +skipped .*profile 'none' has no contracts section/);
    assert.equal(scenarioOf(readReport(result.outputDir), 'fixture.compiled-contracts').outcome, 'passed');
  });

  it('reports a scenario that loads a contract without --contracts-dir as a configuration error', () => {
    const result = runCli(['run', 'fixture.compiled-contracts'], options);
    assert.equal(result.code, 1, result.stdout + result.stderr);
    const scenario = scenarioOf(readReport(result.outputDir), 'fixture.compiled-contracts');
    assert.equal(scenario.outcome, 'error');
    assert.match(
      scenario.reason ?? '',
      /^configuration error: contract 'Counter' needs compiled contracts, but the run has no --contracts-dir/,
    );
  });

  it('accepts --contracts-dir in dry-run only besides run, and checks that it exists', () => {
    assert.equal(runCli(['dry-run', '--contracts-dir', COMPILED_CONTRACTS], options).code, 0);
    const list = runCli(['list', '--contracts-dir', COMPILED_CONTRACTS], options);
    assert.equal(list.code, 2);
    assert.match(list.stderr, /--contracts-dir is only accepted by 'run' and 'dry-run'/);
    const absent = runCli(['run', '--contracts-dir', `${COMPILED_CONTRACTS}/absent`], options);
    assert.equal(absent.code, 2);
    assert.match(absent.stderr, /--contracts-dir .*absent does not exist/);
  });

  it('checks the usage of compile and reports a folder that is not an e2e project', () => {
    const noProfile = runCli(['compile'], options);
    assert.equal(noProfile.code, 2);
    assert.match(noProfile.stderr, /'compile' needs --profile/);
    const projectOnRun = runCli(['run', '--project', '/tmp'], options);
    assert.equal(projectOnRun.code, 2);
    assert.match(projectOnRun.stderr, /--project is only accepted by 'compile'/);
    const selection = runCli(['compile', 'some.scenario', '--profile', 'fx-fhe'], { profilesDir: FHE_PROFILES });
    assert.equal(selection.code, 2);
    assert.match(selection.stderr, /'compile' takes no scenario selection/);

    const notProject = runCli(['compile', '--profile', 'fx-fhe', '--project', COMPILED_CONTRACTS], {
      profilesDir: FHE_PROFILES,
      env: { TE_FHE_ACL: '0x0000000000000000000000000000000000000003' },
    });
    assert.equal(notProject.code, 2, notProject.stdout + notProject.stderr);
    assert.match(
      notProject.stderr,
      /test-engine: compile failed: .* is not an e2e project with the coprocessor config generator/,
    );
    assert.ok(!notProject.stderr.includes('    at '), 'no stack trace for a compile error');
  });
});
