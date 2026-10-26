import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import type { DiscoveredScenario } from '../discovery/discovery.js';
import type { ExecutionPlan, PlanDecision, SelectionFilters } from '../planner/planner.js';
import { redactedSettings } from '../profiles/profile.js';
import type { ResolvedProfile, ResolvedSetting } from '../profiles/types.js';
import { ARTIFACTS, type RunMode, type ScenarioOutcome, type ScenarioRunResult } from '../runner/types.js';
import type { Redactor } from '../secrets/redactor.js';
import { EVIDENCE_FILE, buildEvidence } from './evidence.js';
import { renderHtmlReport } from './html.js';
import { type CucumberExecution, parseMessages } from './messages.js';

/**
 * Contract of the Report component: the consolidated, machine-readable result of one engine run
 * (`report.json`). `report.html` and `evidence.json` are derived from exactly this object.
 */

export const REPORT_SCHEMA_VERSION = 1;
export const REPORT_FILES = { json: 'report.json', html: 'report.html', evidence: EVIDENCE_FILE } as const;

export interface ScenarioReport {
  id: string;
  name: string;
  owner: string;
  description: string;
  tags: string[];
  origin?: string;
  targets?: string[];
  /** Relative to the package root. */
  manifestPath: string;
  featurePath: string;
  timeoutSeconds: number;
  /** The planner's decision, made before anything ran. */
  decision: PlanDecision;
  outcome: ScenarioOutcome;
  reason?: string;
  startedAt: string;
  finishedAt: string;
  durationMs: number;
  exitCode: number | null;
  signal: string | null;
  /** Number of evidence entries recorded by steps and hooks. */
  evidenceCount: number;
  /** Relative to the run output directory. */
  artifacts?: { messages: string; cucumberHtml: string; console: string };
  cucumber?: CucumberExecution;
}

export interface ProfileSnapshot {
  name: string;
  description: string;
  requiresConfirmation: boolean;
  /** Secret values replaced by `[REDACTED:<key>]`; the source of every value is kept. */
  settings: Record<string, ResolvedSetting>;
}

export interface RunReport {
  schemaVersion: typeof REPORT_SCHEMA_VERSION;
  engine: { name: string; version: string; node: string };
  run: {
    id: string;
    mode: RunMode;
    startedAt: string;
    finishedAt: string;
    durationMs: number;
    filters: SelectionFilters;
    scenariosDir: string;
    profile: ProfileSnapshot;
    gitCommit?: string;
    interrupted: boolean;
  };
  /**
   * `executed` counts scenarios that reached a worker. `success` requires at least one executed
   * scenario, no failed/timed-out/error outcome, and no interruption.
   */
  summary: Record<ScenarioOutcome, number> & { total: number; executed: number; success: boolean };
  scenarios: ScenarioReport[];
}

export interface BuildReportInput {
  runId: string;
  mode: RunMode;
  engineVersion: string;
  startedAt: Date;
  finishedAt: Date;
  plan: ExecutionPlan;
  results: ScenarioRunResult[];
  profile: ResolvedProfile;
  interrupted: boolean;
  gitCommit?: string;
  outputDir: string;
  scenariosDir: string;
  packageRoot: string;
}

function readExecution(artifactsDir: string, mode: RunMode): CucumberExecution | undefined {
  const messagesPath = path.join(artifactsDir, ARTIFACTS.messages);
  return existsSync(messagesPath) ? parseMessages(readFileSync(messagesPath, 'utf8'), mode) : undefined;
}

function scenarioReport(
  scenario: DiscoveredScenario,
  decision: PlanDecision,
  result: ScenarioRunResult,
  input: BuildReportInput,
): ScenarioReport {
  const { metadata, spec } = scenario.manifest;
  const relativeToRoot = (file: string) => path.relative(input.packageRoot, file);
  const artifactsDir = result.artifactsDir;
  const cucumber = artifactsDir ? readExecution(artifactsDir, input.mode) : undefined;
  const evidenceCount =
    cucumber?.testCases.reduce(
      (count, testCase) => count + testCase.steps.reduce((sum, step) => sum + (step.evidence?.length ?? 0), 0),
      0,
    ) ?? 0;
  return {
    id: metadata.id,
    name: metadata.name,
    owner: metadata.owner,
    description: metadata.description,
    tags: metadata.tags,
    ...(metadata.origin !== undefined ? { origin: metadata.origin } : {}),
    ...(spec.targets !== undefined ? { targets: spec.targets } : {}),
    manifestPath: relativeToRoot(scenario.manifestPath),
    featurePath: relativeToRoot(scenario.featurePath),
    timeoutSeconds: spec.timeoutSeconds,
    decision,
    outcome: result.outcome,
    ...(result.reason !== undefined ? { reason: result.reason } : {}),
    startedAt: result.startedAt,
    finishedAt: result.finishedAt,
    durationMs: result.durationMs,
    exitCode: result.exitCode,
    signal: result.signal,
    evidenceCount,
    ...(artifactsDir
      ? {
          artifacts: {
            messages: path.relative(input.outputDir, path.join(artifactsDir, ARTIFACTS.messages)),
            cucumberHtml: path.relative(input.outputDir, path.join(artifactsDir, ARTIFACTS.cucumberHtml)),
            console: path.relative(input.outputDir, path.join(artifactsDir, ARTIFACTS.console)),
          },
        }
      : {}),
    ...(cucumber ? { cucumber } : {}),
  };
}

export function buildRunReport(input: BuildReportInput): RunReport {
  const resultsById = new Map(input.results.map((result) => [result.scenarioId, result]));
  const scenarios = input.plan.items.flatMap(({ scenario, decision }) => {
    const result = resultsById.get(scenario.manifest.metadata.id);
    return result ? [scenarioReport(scenario, decision, result, input)] : [];
  });

  const summary = {
    total: scenarios.length,
    executed: scenarios.filter((scenario) => scenario.artifacts !== undefined).length,
    passed: 0,
    failed: 0,
    skipped: 0,
    'timed-out': 0,
    error: 0,
    success: false,
  };
  for (const scenario of scenarios) summary[scenario.outcome] += 1;
  summary.success =
    summary.executed > 0 && summary.failed + summary['timed-out'] + summary.error === 0 && !input.interrupted;

  const { profile } = input;
  return {
    schemaVersion: REPORT_SCHEMA_VERSION,
    engine: { name: '@fhevm/test-engine', version: input.engineVersion, node: process.version },
    run: {
      id: input.runId,
      mode: input.mode,
      startedAt: input.startedAt.toISOString(),
      finishedAt: input.finishedAt.toISOString(),
      durationMs: input.finishedAt.getTime() - input.startedAt.getTime(),
      filters: input.plan.filters,
      scenariosDir: path.relative(input.packageRoot, input.scenariosDir) || '.',
      profile: {
        name: profile.name,
        description: profile.description,
        requiresConfirmation: profile.requiresConfirmation,
        settings: redactedSettings(profile),
      },
      ...(input.gitCommit ? { gitCommit: input.gitCommit } : {}),
      interrupted: input.interrupted,
    },
    summary,
    scenarios,
  };
}

export interface ReportFiles {
  json: string;
  html: string;
  evidence: string;
}

/**
 * Writes `report.json`, `report.html` and `evidence.json` into `outputDir`. The redactor is
 * applied once more to everything written, as a safety net.
 */
export function writeRunReport(report: RunReport, outputDir: string, redactor: Redactor): ReportFiles {
  const safeReport = redactor.redactDeep(report);
  const files = {
    json: path.join(outputDir, REPORT_FILES.json),
    html: path.join(outputDir, REPORT_FILES.html),
    evidence: path.join(outputDir, REPORT_FILES.evidence),
  };
  writeFileSync(files.json, `${JSON.stringify(safeReport, null, 2)}\n`);
  writeFileSync(files.evidence, `${JSON.stringify(redactor.redactDeep(buildEvidence(safeReport)), null, 2)}\n`);
  writeFileSync(files.html, redactor.redact(renderHtmlReport(safeReport)));
  return files;
}
