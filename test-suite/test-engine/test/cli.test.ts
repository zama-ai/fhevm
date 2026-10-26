import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { describe, it } from 'node:test';

import { DEFAULT_SCENARIOS_DIR, PACKAGE_ROOT } from '../src/paths.js';
import { parseMessages } from '../src/report/messages.js';
import {
  CLI,
  FIXTURE_PROFILES,
  FIXTURE_SCENARIOS,
  FIXTURE_SECRET,
  INVALID_PROFILES,
  INVALID_SCENARIOS,
  RESERVED_PROFILES,
  readEvidence,
  readReport,
  runCli,
  scenarioOf,
  tempDir,
} from './helpers.js';

const SECRET_ENV = { TE_FIXTURE_SECRET: FIXTURE_SECRET };

/** Every file below `dir`, recursively. */
function filesBelow(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const full = path.join(dir, entry);
    return statSync(full).isDirectory() ? filesBelow(full) : [full];
  });
}

describe('test-engine CLI', () => {
  describe('list', () => {
    it('lists valid scenarios with targets and requirements, and exits 0', () => {
      const result = runCli(['list']);
      assert.equal(result.code, 0, result.stderr);
      assert.match(result.stdout, /TARGETS\s+NAME/);
      assert.match(result.stdout, /fixture\.basic-only .*fx-basic/);
      assert.match(result.stdout, /fixture\.passing .*\(any\)/);
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
      const result = runCli(['list'], { scenariosDir: INVALID_SCENARIOS });
      assert.equal(result.code, 2);
      assert.match(result.stderr, /7 invalid manifest\(s\)/);
    });

    it('exits 2 on an unknown --profile', () => {
      const result = runCli(['list', '--profile', 'bogus']);
      assert.equal(result.code, 2);
      assert.match(result.stderr, /unknown profile 'bogus'/);
    });

    it('exits 2 when the profiles directory contains a none.yaml', () => {
      const result = runCli(['list'], { profilesDir: RESERVED_PROFILES, scenariosDir: INVALID_SCENARIOS });
      assert.equal(result.code, 2);
      assert.match(result.stderr, /'none' is reserved/);
    });

    it('rejects targets that are not profiles of the profiles directory in use', () => {
      // The fixture scenarios target fx-* profiles, unknown to the shipped profiles directory.
      const result = runCli(['list'], { profilesDir: path.join(PACKAGE_ROOT, 'profiles') });
      assert.equal(result.code, 2);
      assert.match(result.stderr, /spec\.targets: unknown profile\(s\): fx-basic/);
    });
  });

  describe('profiles', () => {
    it('lists and validates every profile, including the built-in none', () => {
      const result = runCli(['profiles']);
      assert.equal(result.code, 0, result.stderr);
      for (const name of ['none', 'fx-basic', 'fx-confirm', 'fx-secret']) assert.match(result.stdout, new RegExp(name));
    });

    it('resolves one profile and prints its settings with secrets redacted', () => {
      const result = runCli(['profiles', '--profile', 'fx-secret'], { env: SECRET_ENV });
      assert.equal(result.code, 0, result.stderr);
      assert.match(result.stdout, /fixture\.secret\s+yes\s+env:TE_FIXTURE_SECRET\s+\[REDACTED:fixture\.secret\]/);
      assert.match(result.stdout, /fixture\.literal\s+no\s+literal\s+hello-literal/);
      assert.ok(!result.stdout.includes(FIXTURE_SECRET));
    });

    it('exits 2 on invalid profiles or an unresolvable profile', () => {
      assert.equal(runCli(['profiles'], { profilesDir: INVALID_PROFILES, scenariosDir: INVALID_SCENARIOS }).code, 2);
      assert.equal(runCli(['profiles'], { profilesDir: RESERVED_PROFILES, scenariosDir: INVALID_SCENARIOS }).code, 2);
      const unresolved = runCli(['profiles', '--profile', 'fx-secret']);
      assert.equal(unresolved.code, 2);
      assert.match(unresolved.stderr, /TE_FIXTURE_SECRET' is not set/);
    });
  });

  describe('run', () => {
    it('runs the real smoke scenario with evidence and writes every report', () => {
      const result = runCli(['run', 'smoke.arithmetic'], { scenariosDir: DEFAULT_SCENARIOS_DIR });
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      assert.equal(report.summary.success, true);
      assert.equal(report.summary.executed, 1);
      assert.equal(report.run.profile.name, 'none');
      const smoke = scenarioOf(report, 'smoke.arithmetic');
      assert.equal(smoke.outcome, 'passed');
      assert.equal(smoke.cucumber?.testCases.length, 5);
      assert.ok(smoke.cucumber?.testCases.every((testCase) => testCase.status === 'passed'));
      assert.equal(smoke.evidenceCount, 5);
      for (const artifact of Object.values(smoke.artifacts!)) {
        assert.ok(existsSync(path.join(result.outputDir, artifact)), artifact);
      }
      const evidence = readEvidence(result.outputDir);
      assert.equal(evidence.entries.filter((entry) => entry.kind === 'engine.note').length, 5);
      assert.ok(readFileSync(path.join(result.outputDir, 'report.html'), 'utf8').includes('engine.note'));
      assert.ok(
        readFileSync(path.join(result.outputDir, smoke.artifacts!.cucumberHtml), 'utf8').includes('engine.note'),
      );
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
      // Steps skipped after a failure are not runtime skips.
      assert.ok(!failing.cucumber!.issues.some((issue) => issue.startsWith('Runtime skip')));
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

    it('exits 3 and still writes the report when nothing is executed', () => {
      const result = runCli(['run', 'fixture.disabled']);
      assert.equal(result.code, 3, result.stdout + result.stderr);
      assert.match(result.stdout, /No scenario was executed/);
      const report = readReport(result.outputDir);
      assert.equal(report.summary.executed, 0);
      assert.equal(report.summary.success, false);
      assert.ok(!existsSync(path.join(result.outputDir, 'scenarios')), 'no worker artifacts');
    });

    it('reports undefined steps and Gherkin parse errors as failures', () => {
      const report = readReport(runCli(['run', 'fixture.undefined-step', 'fixture.parse-error']).outputDir);
      const undefinedStep = scenarioOf(report, 'fixture.undefined-step');
      assert.equal(undefinedStep.outcome, 'failed');
      assert.ok(
        undefinedStep.cucumber!.issues.some((issue) =>
          issue.startsWith('Undefined step "a step that nobody implemented"'),
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

  describe('execution guarantees', () => {
    for (const [id, pattern] of [
      ['fixture.runtime-skip-step', /^Runtime skip is not allowed: step "a step that skips at runtime"/],
      ['fixture.runtime-skip-hook', /^Runtime skip is not allowed: hook "skipping hook"/],
    ] as const) {
      it(`fails a runtime skip (${id})`, () => {
        const result = runCli(['run', id]);
        assert.equal(result.code, 1, result.stdout + result.stderr);
        const scenario = scenarioOf(readReport(result.outputDir), id);
        assert.equal(scenario.outcome, 'failed');
        assert.ok(
          scenario.cucumber!.issues.some((issue) => pattern.test(issue)),
          scenario.cucumber!.issues.join('\n'),
        );
      });
    }

    it('flags a runtime skip even when an After hook fails afterwards', () => {
      const scenario = scenarioOf(
        readReport(runCli(['run', 'fixture.runtime-skip-after']).outputDir),
        'fixture.runtime-skip-after',
      );
      assert.equal(scenario.outcome, 'failed');
      assert.ok(scenario.cucumber!.issues.some((issue) => issue.startsWith('Runtime skip is not allowed: step')));
      const hook = scenario.cucumber!.testCases[0]!.steps.find(
        (step) => step.kind === 'hook' && step.status === 'failed',
      );
      assert.match(hook!.error!, /after hook failed/);
    });

    it('does not apply the runtime-skip rule in dry-run', () => {
      const result = runCli(['dry-run', '--tags', '@runtime-skip']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      for (const scenario of readReport(result.outputDir).scenarios) {
        assert.ok(!scenario.cucumber!.issues.some((issue) => issue.startsWith('Runtime skip')));
      }
    });

    it('fails a scenario whose World does not extend EngineWorld', () => {
      const scenario = scenarioOf(
        readReport(runCli(['run', 'fixture.foreign-world']).outputDir),
        'fixture.foreign-world',
      );
      assert.equal(scenario.outcome, 'failed');
      const hook = scenario.cucumber!.testCases[0]!.steps.find((step) => step.kind === 'hook')!;
      assert.match(hook.error!, /the World must extend EngineWorld/);
    });

    it('fails a step reading a setting the profile does not define', () => {
      const scenario = scenarioOf(
        readReport(runCli(['run', 'fixture.missing-setting']).outputDir),
        'fixture.missing-setting',
      );
      assert.equal(scenario.outcome, 'failed');
      assert.match(
        scenario.cucumber!.testCases[0]!.steps.find((step) => step.kind === 'step')!.error!,
        /profile 'none' does not define setting 'does\.not\.exist'/,
      );
    });

    it('exits 130 when interrupted before the first worker finishes', async () => {
      const outputDir = tempDir();
      const child = spawn(
        process.execPath,
        [
          CLI,
          'run',
          'fixture.cumulative-timeout',
          '--scenarios-dir',
          FIXTURE_SCENARIOS,
          '--profiles-dir',
          FIXTURE_PROFILES,
          '--output-dir',
          outputDir,
        ],
        { cwd: PACKAGE_ROOT, stdio: 'ignore', env: { ...process.env, NO_COLOR: '1' } },
      );
      setTimeout(() => child.kill('SIGINT'), 1_500);
      const code = await new Promise<number | null>((resolve) => child.once('close', resolve));
      assert.equal(code, 130);
      const report = readReport(outputDir);
      assert.equal(report.run.interrupted, true);
      assert.equal(report.summary.success, false);
    });
  });

  describe('profiles and planner', () => {
    it('runs a scenario on a profile listed in its targets', () => {
      const result = runCli(['run', 'fixture.basic-only', 'fixture.passing', '--profile', 'fx-basic']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      assert.equal(report.summary.executed, 2);
      assert.equal(report.run.profile.name, 'fx-basic');
      assert.match(result.stdout, /Plan \(run, profile fx-basic\)/);
    });

    it('skips scenarios whose targets exclude the profile, with the reason', () => {
      const result = runCli(['run', 'fixture.basic-only', 'fixture.passing']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      const skipped = scenarioOf(report, 'fixture.basic-only');
      assert.equal(skipped.outcome, 'skipped');
      assert.equal(skipped.reason, "profile 'none' not in spec.targets [fx-basic]");
      assert.equal(scenarioOf(report, 'fixture.passing').outcome, 'passed');
    });

    it('exits 3 when the targets leave nothing to run', () => {
      const result = runCli(['run', 'fixture.basic-only']);
      assert.equal(result.code, 3);
    });

    it('applies the same decisions in dry-run', () => {
      const result = runCli(['dry-run', 'fixture.basic-only', 'fixture.passing']);
      assert.equal(result.code, 0);
      assert.equal(scenarioOf(readReport(result.outputDir), 'fixture.basic-only').outcome, 'skipped');
    });

    describe('confirmation', () => {
      it('refuses to run a profile requiring confirmation without --confirm', () => {
        const result = runCli(['run', 'fixture.confirm-target', '--profile', 'fx-confirm']);
        assert.equal(result.code, 2);
        assert.match(result.stderr, /requires confirmation: add --confirm fx-confirm/);
        assert.ok(!existsSync(path.join(result.outputDir, 'report.json')));
      });

      it('runs with the right confirmation; dry-run needs none', () => {
        assert.equal(
          runCli(['run', 'fixture.confirm-target', '--profile', 'fx-confirm', '--confirm', 'fx-confirm']).code,
          0,
        );
        assert.equal(runCli(['dry-run', 'fixture.confirm-target', '--profile', 'fx-confirm']).code, 0);
      });

      it('rejects a wrong name and a confirmation on a profile that does not need it', () => {
        assert.equal(
          runCli(['run', 'fixture.confirm-target', '--profile', 'fx-confirm', '--confirm', 'fx-basic']).code,
          2,
        );
        assert.equal(runCli(['run', 'fixture.passing', '--profile', 'fx-basic', '--confirm', 'fx-basic']).code, 2);
      });

      it('skips scenarios that do not list the profile explicitly', () => {
        const result = runCli(['run', 'fixture.passing', '--profile', 'fx-confirm', '--confirm', 'fx-confirm']);
        assert.equal(result.code, 3);
        assert.equal(
          scenarioOf(readReport(result.outputDir), 'fixture.passing').reason,
          "profile 'fx-confirm' must be listed explicitly in spec.targets",
        );
      });
    });

    it('exits 2 before any worker starts when a profile variable is missing', () => {
      const result = runCli(['run', 'fixture.secrets', '--profile', 'fx-secret']);
      assert.equal(result.code, 2);
      assert.match(result.stderr, /fixture\.secret: environment variable 'TE_FIXTURE_SECRET' is not set/);
      assert.ok(!existsSync(path.join(result.outputDir, 'report.json')));
    });

    it('exits 2 on an unknown profile', () => {
      const result = runCli(['run', 'fixture.passing', '--profile', 'staging']);
      assert.equal(result.code, 2);
      assert.match(result.stderr, /unknown profile 'staging'/);
    });
  });

  describe('evidence', () => {
    it('collects step and hook evidence with run and scenario evidence into evidence.json', () => {
      const result = runCli(['run', 'fixture.evidence', '--profile', 'fx-basic']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const { entries } = readEvidence(result.outputDir);
      const byKind = new Map(entries.map((entry) => [entry.kind, entry]));
      assert.equal(byKind.get('engine.run')!.scope, 'run');
      assert.equal(byKind.get('engine.profile')!.data.name, 'fx-basic');
      assert.deepEqual(byKind.get('engine.scenario')!.data.decision, { action: 'run' });
      const step = byKind.get('fixture.step')!;
      assert.equal(step.scope, 'step');
      assert.deepEqual(step.data, { scenario: 'fixture.evidence', answer: 42 });
      assert.equal(step.step!.text, 'a step that records evidence');
      const hook = byKind.get('fixture.hook')!;
      assert.equal(hook.scope, 'hook');
      assert.equal(hook.hook!.name, 'recording hook');
      assert.equal(hook.label, 'From a hook');
      assert.match(result.stdout, /profile in use: fx-basic/);

      const report = readReport(result.outputDir);
      assert.equal(scenarioOf(report, 'fixture.evidence').evidenceCount, 2);
      const html = readFileSync(path.join(result.outputDir, 'report.html'), 'utf8');
      assert.ok(html.includes('fixture.step') && html.includes('fixture.hook') && html.includes('recording hook'));

      // The evidence is also in the native Cucumber artifacts, as attachments.
      const artifacts = scenarioOf(report, 'fixture.evidence').artifacts!;
      const messages = readFileSync(path.join(result.outputDir, artifacts.messages), 'utf8');
      assert.equal(messages.split('application/vnd.fhevm.evidence+json').length - 1, 2);
      const cucumberHtml = readFileSync(path.join(result.outputDir, artifacts.cucumberHtml), 'utf8');
      assert.ok(cucumberHtml.includes('fixture.step') && cucumberHtml.includes('fixture.hook'));
    });

    it('fails a scenario recording evidence while no step or hook is running', () => {
      const scenario = scenarioOf(
        readReport(runCli(['run', 'fixture.record-outside-step']).outputDir),
        'fixture.record-outside-step',
      );
      assert.equal(scenario.outcome, 'failed');
      assert.ok(
        scenario.cucumber!.issues.some((issue) =>
          issue.includes('evidence.record() must be called (and awaited) inside a step or hook'),
        ),
        scenario.cucumber!.issues.join('\n'),
      );
    });

    it('records evidence of skipped scenarios (scenario scope only)', () => {
      const result = runCli(['run', 'fixture.disabled', 'fixture.passing']);
      const disabled = readEvidence(result.outputDir).entries.find(
        (entry) => entry.kind === 'engine.scenario' && entry.scenarioId === 'fixture.disabled',
      )!;
      assert.equal((disabled.data.decision as { action: string }).action, 'skip');
      assert.equal(disabled.data.outcome, 'skipped');
    });
  });

  describe('secrets', () => {
    it('keeps the secret out of the worker environment and redacts it from every output', () => {
      const result = runCli(['run', 'fixture.secrets', '--profile', 'fx-secret'], { env: SECRET_ENV });
      assert.equal(result.code, 1, 'the second Gherkin scenario fails by design');
      const report = readReport(result.outputDir);
      const scenario = scenarioOf(report, 'fixture.secrets');
      const [clean, leaking] = scenario.cucumber!.testCases;
      assert.equal(clean!.status, 'passed', 'settings resolved, process environment clean');
      assert.equal(leaking!.status, 'failed');

      const forms = [FIXTURE_SECRET, encodeURIComponent(FIXTURE_SECRET), JSON.stringify(FIXTURE_SECRET).slice(1, -1)];
      const outputs = [
        ['stdout', result.stdout],
        ['stderr', result.stderr],
        ...filesBelow(result.outputDir).map((file) => [
          path.relative(result.outputDir, file),
          readFileSync(file, 'utf8'),
        ]),
      ];
      for (const [name, content] of outputs) {
        for (const form of forms) assert.ok(!content!.includes(form), `${name} contains the secret`);
      }
      const placeholder = '[REDACTED:fixture.secret]';
      for (const file of [
        'scenarios/fixture.secrets/console.log',
        'scenarios/fixture.secrets/messages.ndjson',
        'report.json',
        'evidence.json',
      ]) {
        assert.ok(readFileSync(path.join(result.outputDir, file), 'utf8').includes(placeholder), file);
      }
      assert.ok(result.stdout.includes(placeholder));
      assert.equal(report.run.profile.settings['fixture.secret']!.value, placeholder);
    });
  });

  describe('dry-run', () => {
    it('validates without executing any step', () => {
      // fixture.failing would fail if executed.
      const result = runCli(['dry-run', 'fixture.failing', 'fixture.passing', 'fixture.isolation-a']);
      assert.equal(result.code, 0, result.stdout + result.stderr);
      const report = readReport(result.outputDir);
      assert.equal(report.run.mode, 'dry-run');
      assert.equal(report.summary.executed, 3);
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
      ['--confirm outside run/dry-run', ['list', '--confirm', 'x']],
    ] as const) {
      it(label, () => {
        const result = runCli([...args]);
        assert.equal(result.code, 2, result.stderr);
        assert.ok(!existsSync(path.join(result.outputDir, 'report.json')));
      });
    }

    it('refuses to run when any manifest is invalid', () => {
      const result = runCli(['run', 'invalid.valid'], { scenariosDir: INVALID_SCENARIOS });
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
    const execution = parseMessages(truncated, 'run');
    assert.equal(execution.success, undefined);
    assert.equal(execution.testCases[0]?.status, 'passed');
    assert.ok(execution.issues.some((issue) => issue.includes('truncated')));
  });
});
