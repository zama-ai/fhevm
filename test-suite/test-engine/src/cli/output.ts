import path from 'node:path';

import type { DiscoveredScenario } from '../discovery/discovery.js';
import type { ManifestError } from '../manifest/types.js';
import type { ExecutionPlan } from '../planner/planner.js';
import type { RunReport, ScenarioReport } from '../report/report.js';
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
    ['ID', 'ENABLED', 'OWNER', 'TAGS', 'NAME'],
    scenarios.map(({ manifest: { metadata, spec } }) => [
      metadata.id,
      spec.enabled ? 'yes' : 'no',
      metadata.owner,
      metadata.tags.join(','),
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

export function printPlan(plan: ExecutionPlan, mode: RunMode): void {
  console.log(`\nPlan (${mode}):`);
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

export function printScenarioHeader(scenario: DiscoveredScenario, mode: RunMode): void {
  if (!scenario.manifest.spec.enabled) return;
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

export function printRunSummary(report: RunReport, files: { json: string; html: string }): void {
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
    `\n${summary.total} scenario(s): ${summary.passed} ${dryRun ? 'valid' : 'passed'}, ${summary.failed} ${dryRun ? 'invalid' : 'failed'}, ` +
      `${summary['timed-out']} timed out, ${summary.error} error(s), ${summary.skipped} skipped → ${summary.success ? 'SUCCESS' : 'FAILURE'}`,
  );
  console.log(`Report: ${relative(files.html)}\nJSON:   ${relative(files.json)}`);
}
