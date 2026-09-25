import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import { DEFAULT_SCENARIOS_DIR } from '../src/paths.js';
import { parseMessages } from '../src/report/messages.js';
import { INVALID_SCENARIOS, readReport, runCli, scenarioOf } from './helpers.js';

describe('test-engine CLI', () => {
  describe('list', () => {
    it('lists valid scenarios and exits 0', () => {
      const result = runCli(['list']);
      assert.equal(result.code, 0);
      assert.match(result.stdout, /fixture\.passing/);
    });

    it('prints machine-readable JSON', () => {
      const output = JSON.parse(runCli(['list', '--json', '--tags', '@isolation']).stdout) as {
        scenarios: { metadata: { id: string } }[];
        errors: unknown[];
      };
      assert.deepEqual(
        output.scenarios.map((scenario) => scenario.metadata.id),
        ['fixture.isolation-a', 'fixture.isolation-b'],
      );
      assert.deepEqual(output.errors, []);
    });

    it('exits 2 and lists problems when a manifest is invalid', () => {
      const result = runCli(['list'], INVALID_SCENARIOS);
      assert.equal(result.code, 2);
      assert.match(result.stderr, /6 invalid manifest\(s\)/);
    });
  });

  describe('run', () => {
    it('runs the real smoke scenario and writes the report', () => {
      const result = runCli(['run', 'smoke.arithmetic'], DEFAULT_SCENARIOS_DIR);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      assert.equal(report.summary.success, true);
      const smoke = scenarioOf(report, 'smoke.arithmetic');
      assert.equal(smoke.outcome, 'passed');
      assert.equal(smoke.cucumber?.testCases.length, 5);
      assert.ok(smoke.cucumber?.testCases.every((testCase) => testCase.status === 'passed'));
      for (const artifact of Object.values(smoke.artifacts!)) {
        assert.ok(existsSync(path.join(result.outputDir, artifact)), artifact);
      }
      assert.ok(existsSync(path.join(result.outputDir, 'report.html')));
    });

    it('isolates scenarios: World, default timeout and step definitions do not leak', () => {
      const result = runCli(['run', '--tags', '@isolation']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      assert.equal(scenarioOf(report, 'fixture.isolation-a').outcome, 'passed');
      assert.equal(scenarioOf(report, 'fixture.isolation-b').outcome, 'passed');
    });

    it('reports failures with the failing step and exits 1', () => {
      const result = runCli(['run', 'fixture.failing', 'fixture.passing']);
      assert.equal(result.code, 1);
      const report = readReport(result.outputDir);
      assert.equal(scenarioOf(report, 'fixture.passing').outcome, 'passed');
      const failing = scenarioOf(report, 'fixture.failing');
      assert.equal(failing.outcome, 'failed');
      const steps = failing.cucumber!.testCases[0]!.steps;
      assert.deepEqual(
        steps.map((step) => step.status),
        ['passed', 'failed', 'skipped'],
      );
      assert.match(steps[1]!.error!, /expected 2 but got 3/);
      assert.match(result.stdout, /When a step fails with "expected 2 but got 3": Error: expected 2 but got 3/);
    });

    it('skips disabled scenarios with a reason', () => {
      const result = runCli(['run', 'fixture.disabled', 'fixture.passing']);
      assert.equal(result.code, 0);
      const disabled = scenarioOf(readReport(result.outputDir), 'fixture.disabled');
      assert.equal(disabled.outcome, 'skipped');
      assert.match(disabled.reason!, /spec\.enabled: false/);
      assert.equal(disabled.artifacts, undefined);
    });

    it('reports undefined steps and Gherkin parse errors as failures', () => {
      const report = readReport(runCli(['run', 'fixture.undefined-step', 'fixture.parse-error']).outputDir);
      const undefinedStep = scenarioOf(report, 'fixture.undefined-step');
      assert.equal(undefinedStep.outcome, 'failed');
      assert.ok(
        undefinedStep.cucumber!.issues.some((issue) =>
          issue.startsWith('Undefined step: "a step that nobody implemented"'),
        ),
      );
      const parseError = scenarioOf(report, 'fixture.parse-error');
      assert.equal(parseError.outcome, 'failed');
      assert.ok(parseError.cucumber!.issues.some((issue) => issue.startsWith('Parse error')));
    });

    it('reports support code that fails to load as an error', () => {
      const result = runCli(['run', 'fixture.import-crash']);
      assert.equal(result.code, 1);
      const crash = scenarioOf(readReport(result.outputDir), 'fixture.import-crash');
      assert.equal(crash.outcome, 'error');
      assert.equal(crash.exitCode, 2);
      assert.match(crash.reason!, /support code failed to load/);
    });

    it('applies spec.timeoutSeconds as the default step timeout', () => {
      const stepTimeout = scenarioOf(
        readReport(runCli(['run', 'fixture.step-timeout']).outputDir),
        'fixture.step-timeout',
      );
      assert.equal(stepTimeout.outcome, 'failed');
      assert.match(stepTimeout.cucumber!.testCases[0]!.steps[0]!.error!, /timed out.*1000 milliseconds/);
    });

    it('stops a scenario whose steps together exceed the budget and keeps the partial evidence', () => {
      const result = runCli(['run', 'fixture.cumulative-timeout']);
      assert.equal(result.code, 1);
      const timedOut = scenarioOf(readReport(result.outputDir), 'fixture.cumulative-timeout');
      assert.equal(timedOut.outcome, 'timed-out');
      assert.equal(timedOut.exitCode, 143);
      const testCase = timedOut.cucumber!.testCases[0]!;
      assert.equal(testCase.status, 'unknown');
      const statuses = testCase.steps.map((step) => step.status);
      assert.ok(statuses.includes('passed') && statuses.includes('unknown'), statuses.join(','));
    });

    it('kills a scenario that blocks the event loop', () => {
      const result = runCli(['run', 'fixture.hard-timeout']);
      assert.equal(result.code, 1);
      const hardTimeout = scenarioOf(readReport(result.outputDir), 'fixture.hard-timeout');
      assert.equal(hardTimeout.outcome, 'timed-out');
      assert.equal(hardTimeout.signal, 'SIGKILL');
    });
  });

  describe('dry-run', () => {
    it('validates without executing any step', () => {
      // fixture.failing would fail if executed; fixture.disabled throws if executed.
      const result = runCli(['dry-run', 'fixture.failing', 'fixture.passing', 'fixture.isolation-a']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      assert.equal(report.run.mode, 'dry-run');
      for (const scenario of report.scenarios) {
        assert.equal(scenario.outcome, 'passed');
        const statuses = scenario.cucumber!.testCases.flatMap((testCase) => testCase.steps.map((step) => step.status));
        assert.ok(
          statuses.every((status) => status === 'skipped'),
          `${scenario.id}: ${statuses.join(',')}`,
        );
      }
    });

    it('fails on undefined steps, ambiguous steps and parse errors', () => {
      const result = runCli([
        'dry-run',
        'fixture.undefined-step',
        'fixture.ambiguous-step',
        'fixture.parse-error',
        'fixture.passing',
      ]);
      assert.equal(result.code, 1);
      const report = readReport(result.outputDir);
      assert.equal(scenarioOf(report, 'fixture.undefined-step').outcome, 'failed');
      const ambiguous = scenarioOf(report, 'fixture.ambiguous-step');
      assert.equal(ambiguous.outcome, 'failed');
      assert.ok(ambiguous.cucumber!.issues.some((issue) => issue.startsWith('Ambiguous step')));
      assert.equal(scenarioOf(report, 'fixture.parse-error').outcome, 'failed');
      assert.equal(scenarioOf(report, 'fixture.passing').outcome, 'passed');
      assert.match(result.stdout, /invalid/);
    });
  });

  describe('usage errors exit 2 without running anything', () => {
    for (const [label, args] of [
      ['unknown id', ['run', 'does.not.exist']],
      ['invalid tag expression', ['run', '--tags', '@a and']],
      ['empty selection', ['run', '--tags', '@nothing']],
      ['unknown command', ['frobnicate']],
      ['unknown option', ['run', '--bogus']],
    ] as const) {
      it(label, () => {
        const result = runCli([...args]);
        assert.equal(result.code, 2, result.stderr);
        assert.ok(!existsSync(path.join(result.outputDir, 'report.json')));
      });
    }

    it('refuses to run when any manifest is invalid', () => {
      const result = runCli(['run', 'invalid.valid'], INVALID_SCENARIOS);
      assert.equal(result.code, 2);
    });
  });
});

describe('parseMessages', () => {
  it('tolerates a truncated message stream', () => {
    const result = runCli(['run', 'fixture.passing']);
    const report = readReport(result.outputDir);
    const messages = readFileSync(
      path.join(result.outputDir, scenarioOf(report, 'fixture.passing').artifacts!.messages),
      'utf8',
    );
    const truncated = messages.slice(0, messages.lastIndexOf('{"testRunFinished"') + 10);
    const execution = parseMessages(truncated);
    assert.equal(execution.success, undefined);
    assert.equal(execution.testCases[0]?.status, 'passed');
    assert.ok(execution.issues.some((issue) => issue.includes('truncated')));
  });
});
