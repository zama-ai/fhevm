import {
  AttachmentContentEncoding,
  type Envelope,
  type GherkinDocument,
  type Hook,
  HookType,
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

import { EVIDENCE_MEDIA_TYPE, type EvidencePayload } from '../api/evidence.js';
import type { RunMode } from '../runner/types.js';
import { findExecutionProblems, parseEnvelopes } from './execution-problems.js';

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
  /** Hooks only: where the hook is defined. */
  source?: string;
  status: StepStatus;
  durationMs: number;
  error?: string;
  /** Evidence recorded while this step or hook was running. */
  evidence?: EvidencePayload[];
}

export interface TestCaseReport {
  name: string;
  uri: string;
  line?: number;
  tags: string[];
  /** `unknown` when the test case never started or never finished (e.g. the worker was killed). */
  status: StepStatus;
  durationMs: number;
  attempt: number;
  steps: StepReport[];
}

export interface CucumberExecution {
  /** From `testRunFinished`; undefined when the stream ended before the run finished. */
  success?: boolean;
  testCases: TestCaseReport[];
  /** Problems not tied to a single step result: parse errors, undefined steps, runtime skips, etc. */
  issues: string[];
  stepDefinitionCount: number;
}

const toStatus = (status: TestStepResultStatus): StepStatus => status.toLowerCase() as StepStatus;
const toMs = (result: TestStepResult): number => TimeConversion.durationToMilliseconds(result.duration);

const HOOK_KEYWORDS: Partial<Record<HookType, string>> = {
  [HookType.BEFORE_TEST_CASE]: 'Before',
  [HookType.AFTER_TEST_CASE]: 'After',
  [HookType.BEFORE_TEST_STEP]: 'BeforeStep',
  [HookType.AFTER_TEST_STEP]: 'AfterStep',
};

type GherkinScenario = NonNullable<NonNullable<GherkinDocument['feature']>['children'][number]['scenario']>;

function indexGherkin(document: GherkinDocument, steps: Map<string, Step>, lines: Map<string, number>): void {
  const visitSteps = (list: readonly Step[]) => list.forEach((step) => steps.set(step.id, step));
  const visit = (child: { background?: { steps: readonly Step[] }; scenario?: GherkinScenario }) => {
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
    visit(child);
    child.rule?.children.forEach(visit);
  }
}

function readEvidence(body: string, encoding: AttachmentContentEncoding): EvidencePayload | undefined {
  try {
    const text = encoding === AttachmentContentEncoding.BASE64 ? Buffer.from(body, 'base64').toString('utf8') : body;
    const payload = JSON.parse(text) as EvidencePayload;
    return typeof payload.kind === 'string' && typeof payload.data === 'object' ? payload : undefined;
  } catch {
    return undefined;
  }
}

function hookSource(hook: Hook | undefined): string | undefined {
  const reference = hook?.sourceReference;
  return reference?.uri ? `${reference.uri}:${reference.location?.line ?? '?'}` : undefined;
}

export function parseMessages(ndjson: string, mode: RunMode): CucumberExecution {
  const { envelopes, unreadable } = parseEnvelopes(ndjson);
  return buildExecution(envelopes, mode, unreadable);
}

export function buildExecution(envelopes: readonly Envelope[], mode: RunMode, unreadable = 0): CucumberExecution {
  const pickles = new Map<string, Pickle>();
  const testCases: TestCase[] = [];
  const gherkinSteps = new Map<string, Step>();
  const astLines = new Map<string, number>();
  const hooks = new Map<string, Hook>();
  /** Last attempt per test case. */
  const startedByTestCase = new Map<string, TestCaseStarted>();
  const finishedAt = new Map<string, number>();
  const stepResults = new Map<string, Map<string, TestStepFinished>>();
  /** testCaseStartedId → testStepId → evidence. */
  const evidence = new Map<string, Map<string, EvidencePayload[]>>();
  const issues: string[] = findExecutionProblems(envelopes, mode).map((problem) => problem.message);
  let success: boolean | undefined;
  let stepDefinitionCount = 0;
  let unreadableEvidence = 0;

  for (const envelope of envelopes) {
    if (envelope.gherkinDocument) indexGherkin(envelope.gherkinDocument, gherkinSteps, astLines);
    else if (envelope.pickle) pickles.set(envelope.pickle.id, envelope.pickle);
    else if (envelope.stepDefinition) stepDefinitionCount += 1;
    else if (envelope.hook) hooks.set(envelope.hook.id, envelope.hook);
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
    } else if (envelope.attachment?.mediaType === EVIDENCE_MEDIA_TYPE) {
      const { testCaseStartedId, testStepId, body, contentEncoding } = envelope.attachment;
      const payload = readEvidence(body, contentEncoding);
      if (!payload || !testCaseStartedId || !testStepId) {
        unreadableEvidence += 1;
        continue;
      }
      const byStep = evidence.get(testCaseStartedId) ?? new Map<string, EvidencePayload[]>();
      byStep.set(testStepId, [...(byStep.get(testStepId) ?? []), payload]);
      evidence.set(testCaseStartedId, byStep);
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
  if (unreadableEvidence > 0) issues.push(`${unreadableEvidence} unreadable evidence attachment(s)`);

  const reports = testCases.flatMap((testCase): TestCaseReport[] => {
    const pickle = pickles.get(testCase.pickleId);
    if (!pickle) return [];
    const pickleSteps = new Map(pickle.steps.map((step) => [step.id, step]));
    const started = startedByTestCase.get(testCase.id);
    const results = started ? stepResults.get(started.id) : undefined;
    const stepEvidence = started ? evidence.get(started.id) : undefined;

    const steps = testCase.testSteps.flatMap((testStep): StepReport[] => {
      const finished = results?.get(testStep.id);
      const status: StepStatus = finished ? toStatus(finished.testStepResult.status) : 'unknown';
      const recorded = stepEvidence?.get(testStep.id);
      const common = {
        status,
        durationMs: finished ? toMs(finished.testStepResult) : 0,
        ...(finished?.testStepResult.message ? { error: finished.testStepResult.message } : {}),
        ...(recorded ? { evidence: recorded } : {}),
      };
      if (testStep.hookId !== undefined) {
        // Passing hooks are plumbing; only surface hooks that did not pass or that recorded evidence.
        if (!recorded && (status === 'passed' || (status === 'unknown' && !started))) return [];
        const hook = hooks.get(testStep.hookId);
        const source = hookSource(hook);
        return [
          {
            kind: 'hook',
            keyword: (hook?.type && HOOK_KEYWORDS[hook.type]) ?? 'Hook',
            text: hook?.name ?? 'hook',
            ...(source ? { source } : {}),
            ...common,
          },
        ];
      }
      const pickleStep = testStep.pickleStepId ? pickleSteps.get(testStep.pickleStepId) : undefined;
      const gherkinStep = pickleStep ? gherkinSteps.get(pickleStep.astNodeIds[0] ?? '') : undefined;
      const line = gherkinStep?.location.line;
      return [
        {
          kind: 'step',
          keyword: gherkinStep?.keyword.trim() ?? '',
          text: pickleStep?.text ?? '',
          ...(line !== undefined ? { line } : {}),
          ...common,
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
