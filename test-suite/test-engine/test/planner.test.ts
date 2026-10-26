import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import {
  PlanError,
  checkConfirmation,
  decide,
  hasRunnableItems,
  planExecution,
  selectScenarios,
} from '../src/planner/planner.js';
import { fakeScenario, planningProfile } from './helpers.js';

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

const NONE = planningProfile('none');

describe('planExecution', () => {
  it('runs enabled scenarios and skips disabled ones with a reason', () => {
    const plan = planExecution(scenarios, { ids: [], tagExpression: '@erc20' }, NONE);
    assert.equal(plan.profile, 'none');
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
    assert.throws(() => planExecution(scenarios, { ids: [], tagExpression: '@nothing' }, NONE), /No scenario matches/);
  });

  it('reports whether anything is runnable', () => {
    assert.equal(hasRunnableItems(planExecution(scenarios, { ids: ['d'] }, NONE)), false);
    assert.equal(hasRunnableItems(planExecution(scenarios, { ids: ['a', 'd'] }, NONE)), true);
  });
});

describe('decide (decision table)', () => {
  const testnet = planningProfile('testnet');
  const mainnet = planningProfile('mainnet', true);
  const reasonOf = (decision: ReturnType<typeof decide>) => (decision.action === 'skip' ? decision.reason : 'run');

  it('rule 1: a disabled scenario is skipped first, whatever else applies', () => {
    const scenario = fakeScenario('x', [], false, { targets: ['local'] });
    assert.equal(reasonOf(decide(scenario, testnet)), 'disabled in manifest (spec.enabled: false)');
  });

  it('rule 2: a profile requiring confirmation must be listed explicitly in targets', () => {
    assert.equal(
      reasonOf(decide(fakeScenario('x'), mainnet)),
      "profile 'mainnet' must be listed explicitly in spec.targets",
    );
    assert.equal(
      reasonOf(decide(fakeScenario('x', [], true, { targets: ['testnet'] }), mainnet)).includes('explicitly'),
      true,
    );
    assert.equal(decide(fakeScenario('x', [], true, { targets: ['mainnet'] }), mainnet).action, 'run');
  });

  it('rule 3: targets present without the profile', () => {
    assert.equal(
      reasonOf(decide(fakeScenario('x', [], true, { targets: ['local', 'devnet'] }), testnet)),
      "profile 'testnet' not in spec.targets [local, devnet]",
    );
  });

  it('rule 4: otherwise the scenario runs (no targets means every profile not requiring confirmation)', () => {
    assert.equal(decide(fakeScenario('x'), testnet).action, 'run');
    assert.equal(decide(fakeScenario('x'), NONE).action, 'run');
    assert.equal(decide(fakeScenario('x', [], true, { targets: ['local', 'testnet'] }), testnet).action, 'run');
    assert.equal(decide(fakeScenario('x', [], true, { targets: ['none'] }), NONE).action, 'run');
  });
});

describe('checkConfirmation', () => {
  const confirmProfile = planningProfile('mainnet', true);
  const plainProfile = planningProfile('testnet');

  it('requires --confirm <name> for run on a profile that requires it', () => {
    assert.throws(
      () => checkConfirmation('run', confirmProfile, undefined),
      /requires confirmation: add --confirm mainnet/,
    );
    assert.doesNotThrow(() => checkConfirmation('run', confirmProfile, 'mainnet'));
  });

  it('does not require it for dry-run', () => {
    assert.doesNotThrow(() => checkConfirmation('dry-run', confirmProfile, undefined));
  });

  it('rejects a wrong name, and a confirmation on a profile that does not require it', () => {
    assert.throws(() => checkConfirmation('run', confirmProfile, 'testnet'), /does not match the selected profile/);
    assert.throws(() => checkConfirmation('run', plainProfile, 'testnet'), /only accepted for profiles that require/);
    assert.throws(() => checkConfirmation('dry-run', plainProfile, 'testnet'), PlanError);
  });
});
