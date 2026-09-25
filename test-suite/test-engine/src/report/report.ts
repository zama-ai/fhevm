import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import type { DiscoveredScenario } from '../discovery/discovery.js';
import type { ExecutionPlan, SelectionFilters } from '../planner/planner.js';
import { ARTIFACTS, type RunMode, type ScenarioOutcome, type ScenarioRunResult } from '../runner/types.js';
import { renderHtmlReport } from './html.js';
import { type CucumberExecution, parseMessages } from './messages.js';

/**
 * Contract of the Report component: the consolidated, machine-readable result of one engine run
 * (`report.json`). `report.html` is rendered from exactly this object.
 */

export const REPORT_SCHEMA_VERSION = 1;
export const REPORT_FILES = { json: 'report.json', html: 'report.html' } as const;

export interface ScenarioReport {
  id: string;
  name: string;
  owner: string;
  description: string;
  tags: string[];
  origin?: string;
  /** Relative to the package root. */
  manifestPath: string;
  featurePath: string;
  timeoutSeconds: number;
  outcome: ScenarioOutcome;
  reason?: string;
  startedAt: string;
  finishedAt: string;
  durationMs: number;
  exitCode: number | null;
  signal: string | null;
  /** Relative to the run output directory. */
  artifacts?: { messages: string; cucumberHtml: string; console: string };
  cucumber?: CucumberExecution;
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
  };
  summary: Record<ScenarioOutcome, number> & { total: number; success: boolean };
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
  outputDir: string;
  scenariosDir: string;
  packageRoot: string;
}

function readExecution(artifactsDir: string): CucumberExecution | undefined {
  const messagesPath = path.join(artifactsDir, ARTIFACTS.messages);
  return existsSync(messagesPath) ? parseMessages(readFileSync(messagesPath, 'utf8')) : undefined;
}

function scenarioReport(
  scenario: DiscoveredScenario,
  result: ScenarioRunResult,
  input: BuildReportInput,
): ScenarioReport {
  const { metadata, spec } = scenario.manifest;
  const relativeToRoot = (file: string) => path.relative(input.packageRoot, file);
  const artifactsDir = result.artifactsDir;
  return {
    id: metadata.id,
    name: metadata.name,
    owner: metadata.owner,
    description: metadata.description,
    tags: metadata.tags,
    ...(metadata.origin !== undefined ? { origin: metadata.origin } : {}),
    manifestPath: relativeToRoot(scenario.manifestPath),
    featurePath: relativeToRoot(scenario.featurePath),
    timeoutSeconds: spec.timeoutSeconds,
    outcome: result.outcome,
    ...(result.reason !== undefined ? { reason: result.reason } : {}),
    startedAt: result.startedAt,
    finishedAt: result.finishedAt,
    durationMs: result.durationMs,
    exitCode: result.exitCode,
    signal: result.signal,
    ...(artifactsDir
      ? {
          artifacts: {
            messages: path.relative(input.outputDir, path.join(artifactsDir, ARTIFACTS.messages)),
            cucumberHtml: path.relative(input.outputDir, path.join(artifactsDir, ARTIFACTS.cucumberHtml)),
            console: path.relative(input.outputDir, path.join(artifactsDir, ARTIFACTS.console)),
          },
        }
      : {}),
    ...(artifactsDir ? { cucumber: readExecution(artifactsDir) } : {}),
  };
}

export function buildRunReport(input: BuildReportInput): RunReport {
  const resultsById = new Map(input.results.map((result) => [result.scenarioId, result]));
  const scenarios = input.plan.items.flatMap(({ scenario }) => {
    const result = resultsById.get(scenario.manifest.metadata.id);
    return result ? [scenarioReport(scenario, result, input)] : [];
  });

  const summary = {
    total: scenarios.length,
    passed: 0,
    failed: 0,
    skipped: 0,
    'timed-out': 0,
    error: 0,
    success: false,
  };
  for (const scenario of scenarios) summary[scenario.outcome] += 1;
  summary.success = summary.failed + summary['timed-out'] + summary.error === 0;

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
    },
    summary,
    scenarios,
  };
}

/** Writes `report.json` and `report.html` into `outputDir` and returns their paths. */
export function writeRunReport(report: RunReport, outputDir: string): { json: string; html: string } {
  const json = path.join(outputDir, REPORT_FILES.json);
  const html = path.join(outputDir, REPORT_FILES.html);
  writeFileSync(json, `${JSON.stringify(report, null, 2)}\n`);
  writeFileSync(html, renderHtmlReport(report));
  return { json, html };
}
