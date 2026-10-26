import {
  type Envelope,
  type Hook,
  type Pickle,
  type Step,
  type TestCase,
  type TestStepFinished,
  TestStepResultStatus,
} from '@cucumber/messages';

import type { RunMode } from '../runner/types.js';

/**
 * Problems that make a scenario fail or invalid regardless of what Cucumber reports. One shared
 * implementation, used by the worker for its verdict and by the report for its issues.
 *
 * - parse errors, undefined parameter types, undefined and ambiguous steps: always (Cucumber
 *   reports them as success in dry-run mode);
 * - runtime skips (`run` mode only): a step or hook result `SKIPPED` with no earlier
 *   failed/undefined/ambiguous/pending result in the same test case. Partial execution is never
 *   allowed; steps skipped after a failure are fine, since the scenario has already failed.
 */

export type ExecutionProblemType =
  'parse-error' | 'undefined-parameter-type' | 'undefined-step' | 'ambiguous-step' | 'runtime-skip';

export interface ExecutionProblem {
  type: ExecutionProblemType;
  message: string;
}

const BLOCKING_STATUSES = new Set<TestStepResultStatus>([
  TestStepResultStatus.FAILED,
  TestStepResultStatus.UNDEFINED,
  TestStepResultStatus.AMBIGUOUS,
  TestStepResultStatus.PENDING,
]);

function indexGherkinSteps(envelopes: readonly Envelope[]): Map<string, Step> {
  const steps = new Map<string, Step>();
  const visit = (list: readonly Step[] | undefined) => list?.forEach((step) => steps.set(step.id, step));
  for (const { gherkinDocument } of envelopes) {
    for (const child of gherkinDocument?.feature?.children ?? []) {
      visit(child.background?.steps);
      visit(child.scenario?.steps);
      for (const ruleChild of child.rule?.children ?? []) {
        visit(ruleChild.background?.steps);
        visit(ruleChild.scenario?.steps);
      }
    }
  }
  return steps;
}

export function findExecutionProblems(envelopes: readonly Envelope[], mode: RunMode): ExecutionProblem[] {
  const gherkinSteps = indexGherkinSteps(envelopes);
  const pickles = new Map<string, Pickle>();
  const testCases = new Map<string, TestCase>();
  const hooks = new Map<string, Hook>();
  const results = new Map<string, Map<string, TestStepFinished>>();
  const started: { id: string; testCaseId: string }[] = [];
  const problems: ExecutionProblem[] = [];

  for (const envelope of envelopes) {
    if (envelope.pickle) pickles.set(envelope.pickle.id, envelope.pickle);
    else if (envelope.testCase) testCases.set(envelope.testCase.id, envelope.testCase);
    else if (envelope.hook) hooks.set(envelope.hook.id, envelope.hook);
    else if (envelope.testCaseStarted) started.push(envelope.testCaseStarted);
    else if (envelope.testStepFinished) {
      const { testCaseStartedId, testStepId } = envelope.testStepFinished;
      const byStep = results.get(testCaseStartedId) ?? new Map<string, TestStepFinished>();
      byStep.set(testStepId, envelope.testStepFinished);
      results.set(testCaseStartedId, byStep);
    } else if (envelope.parseError) {
      const { source, message } = envelope.parseError;
      problems.push({ type: 'parse-error', message: `Parse error in ${source.uri ?? 'unknown source'}: ${message}` });
    } else if (envelope.undefinedParameterType) {
      const { name, expression } = envelope.undefinedParameterType;
      problems.push({
        type: 'undefined-parameter-type',
        message: `Undefined parameter type {${name}} used in '${expression}'`,
      });
    }
  }

  for (const { id: startedId, testCaseId } of started) {
    const testCase = testCases.get(testCaseId);
    const pickle = testCase ? pickles.get(testCase.pickleId) : undefined;
    if (!testCase || !pickle) continue;
    const byStep = results.get(startedId);
    let blocked = false;
    let skipReported = false;

    for (const testStep of testCase.testSteps) {
      const status = byStep?.get(testStep.id)?.testStepResult.status;
      if (status === undefined) continue;

      let describe: () => string;
      if (testStep.hookId !== undefined) {
        const hook = hooks.get(testStep.hookId);
        const location = hook?.sourceReference.uri
          ? ` (${hook.sourceReference.uri}:${hook.sourceReference.location?.line ?? '?'})`
          : '';
        describe = () => `hook "${hook?.name ?? hook?.type ?? 'hook'}"${location}`;
      } else {
        const pickleStep = pickle.steps.find((step) => step.id === testStep.pickleStepId);
        const line = gherkinSteps.get(pickleStep?.astNodeIds[0] ?? '')?.location.line ?? '?';
        describe = () => `step "${pickleStep?.text ?? ''}" (${pickle.uri}:${line})`;
      }

      if (status === TestStepResultStatus.UNDEFINED) {
        problems.push({ type: 'undefined-step', message: `Undefined ${describe()}` });
      } else if (status === TestStepResultStatus.AMBIGUOUS) {
        problems.push({ type: 'ambiguous-step', message: `Ambiguous ${describe()}` });
      } else if (status === TestStepResultStatus.SKIPPED && mode === 'run' && !blocked && !skipReported) {
        problems.push({ type: 'runtime-skip', message: `Runtime skip is not allowed: ${describe()}` });
        skipReported = true;
      }
      if (BLOCKING_STATUSES.has(status)) blocked = true;
    }
  }

  const seen = new Set<string>();
  return problems.filter((problem) => !seen.has(problem.message) && seen.add(problem.message));
}

/** Parses an NDJSON message stream, skipping unreadable lines (e.g. a truncated last line). */
export function parseEnvelopes(ndjson: string): { envelopes: Envelope[]; unreadable: number } {
  const envelopes: Envelope[] = [];
  let unreadable = 0;
  for (const line of ndjson.split('\n')) {
    if (line.trim() === '') continue;
    try {
      envelopes.push(JSON.parse(line) as Envelope);
    } catch {
      unreadable += 1;
    }
  }
  return { envelopes, unreadable };
}
