import type { Envelope } from '@cucumber/messages';
import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { findExecutionProblems, parseEnvelopes } from '../src/report/execution-problems.js';

type Status = 'PASSED' | 'FAILED' | 'SKIPPED' | 'UNDEFINED' | 'AMBIGUOUS' | 'PENDING';

/**
 * Synthetic message stream: one Gherkin scenario whose test steps are given in order, each one
 * either a hook (`hook:<name>`) or a Gherkin step (`step:<text>`), with its result status.
 */
function stream(steps: [string, Status][], extra: Envelope[] = []): Envelope[] {
  const gherkinSteps = steps
    .filter(([ref]) => ref.startsWith('step:'))
    .map(([ref], index) => ({
      id: `g${index}`,
      text: ref.slice(5),
      keyword: 'Given ',
      location: { line: 10 + index },
    }));
  const pickleSteps = gherkinSteps.map((step, index) => ({ id: `ps${index}`, text: step.text, astNodeIds: [step.id] }));
  let stepIndex = 0;
  const testSteps = steps.map(([ref], index) =>
    ref.startsWith('hook:')
      ? { id: `ts${index}`, hookId: ref.slice(5) }
      : { id: `ts${index}`, pickleStepId: `ps${stepIndex++}` },
  );
  const hooks = steps
    .filter(([ref]) => ref.startsWith('hook:'))
    .map(([ref]) => ({
      hook: { id: ref.slice(5), name: ref.slice(5), sourceReference: { uri: 'steps.ts', location: { line: 3 } } },
    }));
  const timestamp = { seconds: 0, nanos: 0 };
  return [
    {
      gherkinDocument: {
        uri: 'x.feature',
        comments: [],
        feature: { children: [{ scenario: { id: 'sc', steps: gherkinSteps, examples: [], location: { line: 2 } } }] },
      },
    },
    {
      pickle: {
        id: 'p',
        uri: 'x.feature',
        name: 'S',
        language: 'en',
        steps: pickleSteps,
        tags: [],
        astNodeIds: ['sc'],
      },
    },
    ...hooks,
    { testCase: { id: 'tc', pickleId: 'p', testSteps } },
    { testCaseStarted: { id: 'tcs', testCaseId: 'tc', attempt: 0, timestamp } },
    ...steps.map(([, status], index) => ({
      testStepFinished: {
        testCaseStartedId: 'tcs',
        testStepId: `ts${index}`,
        testStepResult: { status, duration: timestamp },
        timestamp,
      },
    })),
    ...extra,
  ] as unknown as Envelope[];
}

const types = (envelopes: Envelope[], mode: 'run' | 'dry-run' = 'run') =>
  findExecutionProblems(envelopes, mode).map((problem) => problem.type);

describe('findExecutionProblems', () => {
  it('reports nothing for a passing scenario', () => {
    assert.deepEqual(
      types(
        stream([
          ['hook:before', 'PASSED'],
          ['step:a', 'PASSED'],
        ]),
      ),
      [],
    );
  });

  it('reports a runtime skip from a step, once, with the step and its location', () => {
    const problems = findExecutionProblems(
      stream([
        ['step:a', 'PASSED'],
        ['step:b', 'SKIPPED'],
        ['step:c', 'SKIPPED'],
      ]),
      'run',
    );
    assert.deepEqual(problems, [
      { type: 'runtime-skip', message: 'Runtime skip is not allowed: step "b" (x.feature:11)' },
    ]);
  });

  it('reports a runtime skip from a hook, with the hook and its source', () => {
    const problems = findExecutionProblems(
      stream([
        ['hook:skipper', 'SKIPPED'],
        ['step:a', 'SKIPPED'],
      ]),
      'run',
    );
    assert.deepEqual(problems, [
      { type: 'runtime-skip', message: 'Runtime skip is not allowed: hook "skipper" (steps.ts:3)' },
    ]);
  });

  it('allows steps skipped after a failed, undefined, ambiguous or pending result', () => {
    for (const status of ['FAILED', 'PENDING'] as const) {
      assert.deepEqual(
        types(
          stream([
            ['step:a', status],
            ['step:b', 'SKIPPED'],
          ]),
        ),
        [],
        status,
      );
    }
    assert.deepEqual(
      types(
        stream([
          ['step:a', 'UNDEFINED'],
          ['step:b', 'SKIPPED'],
        ]),
      ),
      ['undefined-step'],
    );
    assert.deepEqual(
      types(
        stream([
          ['step:a', 'AMBIGUOUS'],
          ['step:b', 'SKIPPED'],
        ]),
      ),
      ['ambiguous-step'],
    );
  });

  it('flags a runtime skip followed by a failing After hook', () => {
    assert.deepEqual(
      types(
        stream([
          ['step:a', 'SKIPPED'],
          ['hook:after', 'FAILED'],
        ]),
      ),
      ['runtime-skip'],
    );
  });

  it('does not apply the runtime-skip rule in dry-run, but keeps the other problems', () => {
    assert.deepEqual(
      types(
        stream([
          ['step:a', 'SKIPPED'],
          ['step:b', 'UNDEFINED'],
        ]),
        'dry-run',
      ),
      ['undefined-step'],
    );
  });

  it('reports parse errors and undefined parameter types', () => {
    const problems = findExecutionProblems(
      stream([], [
        { parseError: { source: { uri: 'x.feature' }, message: '(4:5): unexpected' } },
        { undefinedParameterType: { name: 'amount', expression: 'Alice sends {amount}' } },
      ] as unknown as Envelope[]),
      'run',
    );
    assert.deepEqual(problems, [
      { type: 'parse-error', message: 'Parse error in x.feature: (4:5): unexpected' },
      { type: 'undefined-parameter-type', message: "Undefined parameter type {amount} used in 'Alice sends {amount}'" },
    ]);
  });
});

describe('parseEnvelopes', () => {
  it('skips unreadable lines and counts them', () => {
    const { envelopes, unreadable } = parseEnvelopes('{"meta":{}}\n\n{"truncated":');
    assert.equal(envelopes.length, 1);
    assert.equal(unreadable, 1);
  });
});
