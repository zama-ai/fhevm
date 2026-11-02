import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import {
  ANVIL_MNEMONIC,
  CHAIN_PROFILES,
  CHAIN_SCENARIOS,
  filesBelow,
  readReport,
  runCli,
  scenarioOf,
} from '../../helpers.js';

describe('Chain through the engine (no Anvil)', () => {
  it('fails a step that uses the chain on a profile without a chain section (P-002)', () => {
    const result = runCli(['run', 'fixture.chain-on-none'], {
      scenariosDir: CHAIN_SCENARIOS,
      profilesDir: CHAIN_PROFILES,
    });
    assert.equal(result.code, 1, result.stdout + result.stderr);
    assert.equal(scenarioOf(readReport(result.outputDir), 'fixture.chain-on-none').outcome, 'failed');
    const output = [result.stdout, ...filesBelow(result.outputDir).map((file) => readFileSync(file, 'utf8'))].join(
      '\n',
    );
    assert.match(output, /profile 'none' has no chain section; declare spec\.targets for this scenario/);
  });

  it('plans chain scenarios by target and validates them in a dry-run', () => {
    const result = runCli(['dry-run', '--profile', 'fx-anvil'], {
      scenariosDir: CHAIN_SCENARIOS,
      profilesDir: CHAIN_PROFILES,
    });
    assert.equal(result.code, 0, result.stdout + result.stderr);
    assert.equal(readReport(result.outputDir).summary.executed, 3);
    for (const file of ['report.json', 'evidence.json']) {
      const content = readFileSync(path.join(result.outputDir, file), 'utf8');
      assert.ok(!content.includes(ANVIL_MNEMONIC), `${file} holds the mnemonic`);
      assert.ok(content.includes('[REDACTED:chain.mnemonic]'), `${file} holds the redacted chain section`);
    }
    const onNone = runCli(['dry-run'], { scenariosDir: CHAIN_SCENARIOS, profilesDir: CHAIN_PROFILES });
    const report = readReport(onNone.outputDir);
    assert.match(scenarioOf(report, 'fixture.chain-basic').decision.action, /skip/);
  });

  it('prints the resolved chain section with the mnemonic redacted', () => {
    const result = runCli(['profiles', '--profile', 'fx-anvil'], { profilesDir: CHAIN_PROFILES });
    assert.equal(result.code, 0, result.stderr);
    assert.match(result.stdout, /fx-anvil +no +chain +0/);
    assert.match(result.stdout, /Section chain:/);
    assert.match(
      result.stdout,
      /mnemonic +yes +env-file:.*fx-anvil\.env:TE_ANVIL_MNEMONIC +\[REDACTED:chain\.mnemonic\]/,
    );
    assert.match(result.stdout, /chainId +no +literal +31337/);
    assert.match(result.stdout, /wallets +no +profile +alice=0, bob=1, carol=2, dave=3, eve=4/);
    assert.ok(!result.stdout.includes(ANVIL_MNEMONIC));
  });
});
