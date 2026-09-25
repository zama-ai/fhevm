import assert from 'node:assert/strict';
import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import { discoverScenarios } from '../src/discovery/discovery.js';
import { DIST_DIR } from '../src/paths.js';
import { FIXTURE_SCENARIOS, INVALID_SCENARIOS, discoveryOptions, tempDir } from './helpers.js';

describe('discoverScenarios', () => {
  it('finds, validates and sorts the fixture scenarios', async () => {
    const { scenarios, errors } = await discoverScenarios(discoveryOptions(FIXTURE_SCENARIOS));
    assert.deepEqual(errors, []);
    const ids = scenarios.map((scenario) => scenario.manifest.metadata.id);
    assert.deepEqual(ids, [...ids].sort());
    assert.ok(ids.includes('fixture.passing'));

    const passing = scenarios.find((scenario) => scenario.manifest.metadata.id === 'fixture.passing')!;
    assert.equal(passing.featurePath, path.join(FIXTURE_SCENARIOS, 'passing', 'fixture.feature'));
    assert.deepEqual(passing.supportModules, [
      path.join(DIST_DIR, 'test', 'fixtures', 'scenarios', 'passing', 'steps.js'),
    ]);
  });

  it('reports every invalid manifest and keeps the valid ones', async () => {
    const { scenarios, errors } = await discoverScenarios(discoveryOptions(INVALID_SCENARIOS));
    assert.deepEqual(
      scenarios.map((scenario) => scenario.manifest.metadata.id),
      ['invalid.valid'],
    );
    const byDir = new Map(
      errors.map((error) => [path.basename(path.dirname(error.manifestPath)), error.problems.join('; ')]),
    );
    assert.match(byDir.get('bad-yaml')!, /invalid YAML/);
    assert.match(byDir.get('unknown-field')!, /unknown field 'requires'/);
    assert.match(byDir.get('missing-file')!, /file not found/);
    assert.match(byDir.get('escape')!, /must stay inside the scenario directory/);
    assert.match(byDir.get('dup-1')!, /duplicate metadata.id 'invalid.duplicate'/);
    assert.match(byDir.get('dup-2')!, /duplicate metadata.id 'invalid.duplicate'/);
  });

  it('rejects support code that has not been compiled', async () => {
    const root = tempDir();
    const scenarioDir = path.join(root, 'scenarios', 'uncompiled');
    mkdirSync(scenarioDir, { recursive: true });
    writeFileSync(path.join(scenarioDir, 'x.feature'), 'Feature: X\n');
    writeFileSync(path.join(scenarioDir, 'steps.ts'), '');
    writeFileSync(
      path.join(scenarioDir, 'scenario.yaml'),
      [
        'apiVersion: fhevm.zama.ai/scenario/v1',
        'kind: Scenario',
        'metadata: { id: uncompiled, name: U, owner: qa, description: U, tags: [] }',
        'spec: { runtime: cucumber, entrypoint: ./x.feature, supportCode: [./steps.ts], timeoutSeconds: 5 }',
      ].join('\n'),
    );
    const { errors } = await discoverScenarios({
      scenariosDir: path.join(root, 'scenarios'),
      sourceRoot: root,
      distDir: path.join(root, 'dist'),
    });
    assert.equal(errors.length, 1);
    assert.match(errors[0]!.problems[0]!, /has not been compiled.*npm run build/);
  });

  it('fails when the scenarios directory does not exist', async () => {
    await assert.rejects(
      discoverScenarios(discoveryOptions(path.join(tempDir(), 'missing'))),
      /Scenarios directory not found/,
    );
  });
});
