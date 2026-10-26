import path from 'node:path';

import type { DiscoveredScenario } from '../discovery/discovery.js';
import type { ManifestError } from '../manifest/types.js';
import type { ExecutionPlan } from '../planner/planner.js';
import { redactedSettings } from '../profiles/profile.js';
import { BUILTIN_PROFILE, type ProfileDefinition, type ProfileError, type ResolvedProfile } from '../profiles/types.js';
import type { ReportFiles, RunReport, ScenarioReport } from '../report/report.js';
import type { RunMode, ScenarioOutcome } from '../runner/types.js';

/** Short path for display: relative to the working directory when below it, absolute otherwise. */
const relative = (file: string) => {
  const rel = path.relative(process.cwd(), file);
  return rel === '' || rel.startsWith('..') ? file : rel;
};

function printTable(headers: string[], rows: string[][]): void {
  const widths = headers.map((header, index) => Math.max(header.length, ...rows.map((row) => row[index]!.length)));
  const format = (row: string[]) =>
    row
      .map((value, index) => value.padEnd(widths[index]!))
      .join('  ')
      .trimEnd();
  console.log(format(headers));
  console.log(widths.map((width) => '-'.repeat(width)).join('  '));
  rows.forEach((row) => console.log(format(row)));
}

export function printScenarioList(scenarios: DiscoveredScenario[]): void {
  if (scenarios.length === 0) {
    console.log('No scenarios found.');
    return;
  }
  printTable(
    ['ID', 'ENABLED', 'OWNER', 'TAGS', 'TARGETS', 'NAME'],
    scenarios.map(({ manifest: { metadata, spec } }) => [
      metadata.id,
      spec.enabled ? 'yes' : 'no',
      metadata.owner,
      metadata.tags.join(','),
      spec.targets?.join(',') ?? '(any)',
      metadata.name,
    ]),
  );
}

export function printDiscoveryErrors(errors: ManifestError[]): void {
  if (errors.length === 0) return;
  console.error(`\n${errors.length} invalid manifest(s):`);
  for (const error of errors) {
    console.error(`  ${relative(error.manifestPath)}`);
    error.problems.forEach((problem) => console.error(`    - ${problem}`));
  }
}

export function printProfileList(profiles: ProfileDefinition[]): void {
  printTable(
    ['PROFILE', 'CONFIRM', 'SETTINGS', 'DESCRIPTION'],
    [
      [BUILTIN_PROFILE, 'no', '0', 'Built-in profile: no environment, no settings.'],
      ...profiles.map(({ metadata, spec }) => [
        metadata.name,
        spec.requiresConfirmation ? 'yes' : 'no',
        String(Object.keys(spec.settings).length),
        metadata.description,
      ]),
    ],
  );
}

export function printProfileErrors(errors: ProfileError[]): void {
  if (errors.length === 0) return;
  console.error(`\n${errors.length} invalid profile(s):`);
  for (const error of errors) {
    console.error(`  ${path.isAbsolute(error.source) ? relative(error.source) : error.source}`);
    error.problems.forEach((problem) => console.error(`    - ${problem}`));
  }
}

/** Prints the resolved settings of a profile, secrets redacted. */
export function printProfileSettings(profile: ResolvedProfile): void {
  console.log(`\nProfile '${profile.name}' resolved:`);
  const settings = Object.entries(redactedSettings(profile));
  if (settings.length === 0) {
    console.log('No settings.');
    return;
  }
  printTable(
    ['SETTING', 'SECRET', 'SOURCE', 'VALUE'],
    settings.map(([key, setting]) => [
      key,
      setting.secret ? 'yes' : 'no',
      setting.source.startsWith('env-file:')
        ? `env-file:${relative(setting.source.split(':')[1]!)}:${setting.source.split(':').at(-1)}`
        : setting.source,
      String(setting.value),
    ]),
  );
}

export function printPlan(plan: ExecutionPlan, mode: RunMode): void {
  console.log(`\nPlan (${mode}, profile ${plan.profile}):`);
  printTable(
    ['ID', 'DECISION', 'TIMEOUT', 'REASON'],
    plan.items.map(({ scenario, decision }) => [
      scenario.manifest.metadata.id,
      decision.action,
      `${scenario.manifest.spec.timeoutSeconds}s`,
      decision.action === 'skip' ? decision.reason : '',
    ]),
  );
}

export function printScenarioHeader(scenario: DiscoveredScenario, plan: ExecutionPlan, mode: RunMode): void {
  const item = plan.items.find((candidate) => candidate.scenario === scenario);
  if (item?.decision.action !== 'run') return;
  console.log(
    `\n▶ ${mode === 'dry-run' ? 'Validating' : 'Running'} ${scenario.manifest.metadata.id} (${relative(scenario.featurePath)})\n`,
  );
}

/** First failed step of a scenario, as a one-line summary. */
function firstStepFailure(scenario: ScenarioReport): string | undefined {
  for (const testCase of scenario.cucumber?.testCases ?? []) {
    const step = testCase.steps.find((candidate) => candidate.status === 'failed');
    if (step) {
      const error =
        step.error
          ?.split('\n')
          .find((line) => line.trim() !== '')
          ?.trim() ?? 'failed';
      return `${step.keyword} ${step.text}: ${error}`.trim();
    }
  }
  return undefined;
}

const DRY_RUN_LABELS: Partial<Record<ScenarioOutcome, string>> = { passed: 'valid', failed: 'invalid' };

export function printRunSummary(report: RunReport, files: ReportFiles, runnable: boolean): void {
  const dryRun = report.run.mode === 'dry-run';
  console.log(`\nSummary (${report.run.mode}, run ${report.run.id}):`);
  printTable(
    ['ID', 'OUTCOME', 'DURATION', 'DETAILS'],
    report.scenarios.map((scenario) => {
      const details = [scenario.reason, firstStepFailure(scenario), ...new Set(scenario.cucumber?.issues ?? [])]
        .filter(Boolean)
        .join(' | ');
      return [
        scenario.id,
        (dryRun ? DRY_RUN_LABELS[scenario.outcome] : undefined) ?? scenario.outcome,
        `${(scenario.durationMs / 1000).toFixed(2)}s`,
        details,
      ];
    }),
  );
  const { summary } = report;
  console.log(
    `\n${summary.total} scenario(s), ${summary.executed} executed: ${summary.passed} ${dryRun ? 'valid' : 'passed'}, ` +
      `${summary.failed} ${dryRun ? 'invalid' : 'failed'}, ${summary['timed-out']} timed out, ${summary.error} error(s), ` +
      `${summary.skipped} skipped → ${summary.success ? 'SUCCESS' : 'FAILURE'}`,
  );
  if (!runnable) console.log('No scenario was executed: every selected scenario was skipped by the planner.');
  if (report.run.interrupted) console.log('The run was interrupted.');
  console.log(
    `Report:   ${relative(files.html)}\nJSON:     ${relative(files.json)}\nEvidence: ${relative(files.evidence)}`,
  );
}
