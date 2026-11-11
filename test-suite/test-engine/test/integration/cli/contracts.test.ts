import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { CONTRACTS_ARTIFACTS_DIR } from '../../../src/infrastructure/paths.js';
import { CONTRACTS_SCENARIOS, FHE_PROFILES, readReport, runCli, scenarioOf } from '../../helpers.js';

const options = { scenariosDir: CONTRACTS_SCENARIOS };

describe('Engine contracts through the CLI', () => {
  it("hands the engine's contracts and the profile's coprocessor config to the scenarios", () => {
    // No chain is needed: the preflight is skipped and nothing is deployed.
    const result = runCli(['run', 'fixture.engine-contracts', '--profile', 'fx-fhe', '--no-preflight'], {
      ...options,
      profilesDir: FHE_PROFILES,
    });
    assert.equal(result.code, 0, result.stdout + result.stderr);
    assert.match(
      result.stdout,
      new RegExp(`loaded TestInput from ${CONTRACTS_ARTIFACTS_DIR}; ACL 0x0000000000000000000000000000000000000003`),
    );
    assert.equal(scenarioOf(readReport(result.outputDir), 'fixture.engine-contracts').outcome, 'passed');
  });

  it('reports the coprocessor config of a profile without contracts section as a configuration error', () => {
    const result = runCli(['run', 'fixture.engine-contracts'], options);
    assert.equal(result.code, 1, result.stdout + result.stderr);
    const scenario = scenarioOf(readReport(result.outputDir), 'fixture.engine-contracts');
    assert.equal(scenario.outcome, 'error');
    assert.match(scenario.reason ?? '', /^configuration error \(P-002\): profile 'none' has no contracts section/);
  });

  it('has no compile step: the command and its options are gone', () => {
    const compile = runCli(['compile', '--profile', 'fx-fhe'], { ...options, profilesDir: FHE_PROFILES });
    assert.equal(compile.code, 2);
    assert.match(compile.stderr, /Unknown command 'compile'/);
    for (const option of ['--contracts-dir', '--project']) {
      const result = runCli(['run', option, '/tmp'], options);
      assert.equal(result.code, 2, option);
      assert.match(result.stderr, new RegExp(`Unknown option '${option}'`), option);
    }
  });
});
