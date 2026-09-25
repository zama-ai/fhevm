import {
  type Envelope,
  type GherkinDocument,
  type Pickle,
  type Step,
  type TestCase,
  type TestCaseStarted,
  type TestStepFinished,
  type TestStepResult,
  TestStepResultStatus,
  TimeConversion,
  getWorstTestStepResult,
} from '@cucumber/messages';

/**
 * Turns the canonical Cucumber message stream (NDJSON) of one scenario into a compact,
 * engine-owned model. Tolerates truncated streams (e.g. a worker killed on timeout).
 */

export type StepStatus = Lowercase<keyof typeof TestStepResultStatus>;

export interface StepReport {
  kind: 'step' | 'hook';
  keyword: string;
  text: string;
  line?: number;
  status: StepStatus;
  durationMs: number;
  error?: string;
}

export interface TestCaseReport {
  name: string;
  uri: string;
  line?: number;
  tags: string[];
  /** `unknown` when the test case never started (e.g. the worker was killed earlier). */
  status: StepStatus;
  durationMs: number;
  attempt: number;
  steps: StepReport[];
}

export interface CucumberExecution {
  /** From `testRunFinished`; undefined when the stream ended before the run finished. */
  success?: boolean;
  testCases: TestCaseReport[];
  /** Problems that are not tied to a single step result: parse errors, undefined steps, etc. */
  issues: string[];
  stepDefinitionCount: number;
}

const toStatus = (status: TestStepResultStatus): StepStatus => status.toLowerCase() as StepStatus;
const toMs = (result: TestStepResult): number => TimeConversion.durationToMilliseconds(result.duration);

function indexGherkinSteps(document: GherkinDocument, steps: Map<string, Step>, lines: Map<string, number>): void {
  const visitSteps = (list: readonly Step[]) => list.forEach((step) => steps.set(step.id, step));
  const visitScenarioLike = (child: { background?: { steps: readonly Step[] }; scenario?: GherkinScenario }) => {
    if (child.background) visitSteps(child.background.steps);
    if (child.scenario) {
      lines.set(child.scenario.id, child.scenario.location.line);
      visitSteps(child.scenario.steps);
      child.scenario.examples.forEach((examples) =>
        examples.tableBody.forEach((row) => lines.set(row.id, row.location.line)),
      );
    }
  };
  for (const child of document.feature?.children ?? []) {
    visitScenarioLike(child);
    child.rule?.children.forEach(visitScenarioLike);
  }
}

type GherkinScenario = NonNullable<NonNullable<GherkinDocument['feature']>['children'][number]['scenario']>;

export function parseMessages(ndjson: string): CucumberExecution {
  const pickles = new Map<string, Pickle>();
  const testCases: TestCase[] = [];
  const gherkinSteps = new Map<string, Step>();
  const astLines = new Map<string, number>();
  const hookNames = new Map<string, string>();
  /** Last attempt per test case. */
  const startedByTestCase = new Map<string, TestCaseStarted>();
  const finishedAt = new Map<string, number>();
  const stepResults = new Map<string, Map<string, TestStepFinished>>();
  const issues: string[] = [];
  let success: boolean | undefined;
  let stepDefinitionCount = 0;
  let unreadable = 0;

  for (const line of ndjson.split('\n')) {
    if (line.trim() === '') continue;
    let envelope: Envelope;
    try {
      envelope = JSON.parse(line) as Envelope;
    } catch {
      unreadable += 1;
      continue;
    }
    if (envelope.gherkinDocument) indexGherkinSteps(envelope.gherkinDocument, gherkinSteps, astLines);
    else if (envelope.pickle) pickles.set(envelope.pickle.id, envelope.pickle);
    else if (envelope.stepDefinition) stepDefinitionCount += 1;
    else if (envelope.hook) hookNames.set(envelope.hook.id, envelope.hook.name ?? `${envelope.hook.type ?? 'hook'}`);
    else if (envelope.testCase) testCases.push(envelope.testCase);
    else if (envelope.testCaseStarted)
      startedByTestCase.set(envelope.testCaseStarted.testCaseId, envelope.testCaseStarted);
    else if (envelope.testStepFinished) {
      const { testCaseStartedId } = envelope.testStepFinished;
      const results = stepResults.get(testCaseStartedId) ?? new Map<string, TestStepFinished>();
      results.set(envelope.testStepFinished.testStepId, envelope.testStepFinished);
      stepResults.set(testCaseStartedId, results);
    } else if (envelope.testCaseFinished) {
      finishedAt.set(
        envelope.testCaseFinished.testCaseStartedId,
        TimeConversion.timestampToMillisecondsSinceEpoch(envelope.testCaseFinished.timestamp),
      );
    } else if (envelope.parseError) {
      const { source, message } = envelope.parseError;
      issues.push(`Parse error in ${source.uri ?? 'unknown source'}: ${message}`);
    } else if (envelope.undefinedParameterType) {
      const { name, expression } = envelope.undefinedParameterType;
      issues.push(`Undefined parameter type {${name}} used in '${expression}'`);
    } else if (envelope.testRunHookFinished) {
      const { result } = envelope.testRunHookFinished;
      if (result.status !== TestStepResultStatus.PASSED) {
        issues.push(`Test run hook ${toStatus(result.status)}: ${result.message ?? ''}`.trim());
      }
    } else if (envelope.testRunFinished) {
      success = envelope.testRunFinished.success;
      const { message, exception } = envelope.testRunFinished;
      if (message) issues.push(message);
      if (exception) issues.push(`${exception.type}: ${exception.message ?? ''}`.trim());
    }
  }
  if (unreadable > 0) issues.push(`${unreadable} unreadable message line(s); the stream is probably truncated`);

  const reports = testCases.flatMap((testCase): TestCaseReport[] => {
    const pickle = pickles.get(testCase.pickleId);
    if (!pickle) return [];
    const pickleSteps = new Map(pickle.steps.map((step) => [step.id, step]));
    const started = startedByTestCase.get(testCase.id);
    const results = started ? stepResults.get(started.id) : undefined;

    const steps = testCase.testSteps.flatMap((testStep): StepReport[] => {
      const finished = results?.get(testStep.id);
      const status: StepStatus = finished ? toStatus(finished.testStepResult.status) : 'unknown';
      const durationMs = finished ? toMs(finished.testStepResult) : 0;
      const error = finished?.testStepResult.message;
      if (testStep.hookId !== undefined) {
        // Passing hooks are plumbing; only surface hooks that did not pass.
        if (status === 'passed' || (status === 'unknown' && !started)) return [];
        return [
          {
            kind: 'hook',
            keyword: 'Hook',
            text: hookNames.get(testStep.hookId) ?? 'hook',
            status,
            durationMs,
            ...(error ? { error } : {}),
          },
        ];
      }
      const pickleStep = testStep.pickleStepId ? pickleSteps.get(testStep.pickleStepId) : undefined;
      const gherkinStep = pickleStep ? gherkinSteps.get(pickleStep.astNodeIds[0] ?? '') : undefined;
      const text = pickleStep?.text ?? '';
      const line = gherkinStep?.location.line;
      if (status === 'undefined') issues.push(`Undefined step: "${text}" (${pickle.uri}:${line ?? '?'})`);
      if (status === 'ambiguous') issues.push(`Ambiguous step: "${text}" (${pickle.uri}:${line ?? '?'})`);
      return [
        {
          kind: 'step',
          keyword: gherkinStep?.keyword.trim() ?? '',
          text,
          ...(line !== undefined ? { line } : {}),
          status,
          durationMs,
          ...(error ? { error } : {}),
        },
      ];
    });

    const finishedResults = [...(results?.values() ?? [])].map((finished) => finished.testStepResult);
    const startMs = started ? TimeConversion.timestampToMillisecondsSinceEpoch(started.timestamp) : undefined;
    const endMs = started ? finishedAt.get(started.id) : undefined;
    const worst = finishedResults.length > 0 ? toStatus(getWorstTestStepResult(finishedResults).status) : 'unknown';
    // A test case that never finished cannot be reported as passed, whatever its completed steps say.
    const status: StepStatus = endMs === undefined && worst === 'passed' ? 'unknown' : worst;
    // Examples rows carry their own line; otherwise use the scenario line.
    const line = astLines.get(pickle.astNodeIds.at(-1) ?? '') ?? astLines.get(pickle.astNodeIds[0] ?? '');

    return [
      {
        name: pickle.name,
        uri: pickle.uri,
        ...(line !== undefined ? { line } : {}),
        tags: pickle.tags.map((tag) => tag.name),
        status,
        durationMs: startMs !== undefined && endMs !== undefined ? endMs - startMs : 0,
        attempt: started?.attempt ?? 0,
        steps,
      },
    ];
  });

  return { ...(success !== undefined ? { success } : {}), testCases: reports, issues, stepDefinitionCount };
}
