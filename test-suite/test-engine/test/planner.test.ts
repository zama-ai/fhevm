import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { PlanError, planExecution, selectScenarios } from '../src/planner/planner.js';
import { fakeScenario } from './helpers.js';

const scenarios = [
  fakeScenario('a', ['smoke']),
  fakeScenario('b', ['erc20', 'slow']),
  fakeScenario('c', ['erc20']),
  fakeScenario('d', ['erc20'], false),
];
const ids = (list: { manifest: { metadata: { id: string } } }[]) => list.map((item) => item.manifest.metadata.id);

describe('selectScenarios', () => {
  it('selects everything without filters', () => {
    assert.deepEqual(ids(selectScenarios(scenarios, { ids: [] })), ['a', 'b', 'c', 'd']);
  });

  it('selects by id', () => {
    assert.deepEqual(ids(selectScenarios(scenarios, { ids: ['c', 'a'] })), ['a', 'c']);
  });

  it('selects with a Cucumber tag expression over manifest tags', () => {
    assert.deepEqual(ids(selectScenarios(scenarios, { ids: [], tagExpression: '@erc20 and not @slow' })), ['c', 'd']);
    assert.deepEqual(ids(selectScenarios(scenarios, { ids: [], tagExpression: '@smoke or @slow' })), ['a', 'b']);
  });

  it('combines ids and tags', () => {
    assert.deepEqual(ids(selectScenarios(scenarios, { ids: ['a', 'b'], tagExpression: '@erc20' })), ['b']);
  });

  it('rejects unknown ids and invalid expressions', () => {
    assert.throws(() => selectScenarios(scenarios, { ids: ['zzz'] }), PlanError);
    assert.throws(() => selectScenarios(scenarios, { ids: [], tagExpression: '@a and' }), PlanError);
  });
});

describe('planExecution', () => {
  it('runs enabled scenarios and skips disabled ones with a reason', () => {
    const plan = planExecution(scenarios, { ids: [], tagExpression: '@erc20' });
    assert.deepEqual(
      plan.items.map((item) => [item.scenario.manifest.metadata.id, item.decision.action]),
      [
        ['b', 'run'],
        ['c', 'run'],
        ['d', 'skip'],
      ],
    );
    const skipped = plan.items[2]!.decision;
    assert.ok(skipped.action === 'skip' && skipped.reason.includes('spec.enabled'));
  });

  it('fails when nothing matches', () => {
    assert.throws(() => planExecution(scenarios, { ids: [], tagExpression: '@nothing' }), /No scenario matches/);
  });
});
