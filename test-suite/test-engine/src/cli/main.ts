#!/usr/bin/env node
import { randomBytes } from 'node:crypto';
import { mkdirSync } from 'node:fs';
import path from 'node:path';
import { parseArgs } from 'node:util';

import { type DiscoveryResult, discoverScenarios } from '../discovery/discovery.js';
import { DEFAULT_REPORTS_DIR, DEFAULT_SCENARIOS_DIR, DIST_DIR, PACKAGE_ROOT, readEngineVersion } from '../paths.js';
import { PlanError, type SelectionFilters, planExecution, selectScenarios } from '../planner/planner.js';
import { buildRunReport, writeRunReport } from '../report/report.js';
import { executePlan } from '../runner/orchestrator.js';
import type { RunMode } from '../runner/types.js';
import { printDiscoveryErrors, printPlan, printRunSummary, printScenarioHeader, printScenarioList } from './output.js';

/** CLI exit codes. */
const EXIT = {
  success: 0,
  /** At least one scenario failed, timed out or errored (dry-run: invalid). */
  failure: 1,
  /** Bad usage, invalid manifests, unknown ids, empty selection. Nothing was executed. */
  usage: 2,
  interrupted: 130,
} as const;

const USAGE = `Usage: test-engine <command> [scenario-id...] [options]

Commands:
  list       List discovered scenarios (validates every manifest)
  run        Run the selected scenarios, one isolated process per scenario
  dry-run    Validate the selected scenarios without executing any step
  help       Show this message

Options:
  -t, --tags <expr>          Cucumber tag expression on manifest tags, e.g. "@smoke and not @slow"
      --scenarios-dir <dir>  Directory scanned for scenario.yaml files (default: <package>/scenarios)
  -o, --output-dir <dir>     Run output directory (default: <package>/reports/<run-id>)
      --json                 list: print JSON instead of a table
  -v, --version              Print the engine version

Exit codes: 0 success · 1 scenario failure · 2 usage/configuration error · 130 interrupted`;

class UsageError extends Error {}

function createRunId(mode: RunMode, now: Date): string {
  const stamp = now.toISOString().replace(/[-:]/g, '').replace(/\..+$/, '').replace('T', '-');
  return `${stamp}-${mode}-${randomBytes(2).toString('hex')}`;
}

async function discover(scenariosDir: string): Promise<DiscoveryResult> {
  return discoverScenarios({ scenariosDir, sourceRoot: PACKAGE_ROOT, distDir: DIST_DIR });
}

async function commandList(scenariosDir: string, filters: SelectionFilters, json: boolean): Promise<number> {
  const discovery = await discover(scenariosDir);
  const scenarios = selectScenarios(discovery.scenarios, filters);
  if (json) {
    const output = {
      scenarios: scenarios.map(({ manifest, manifestPath }) => ({
        ...manifest,
        manifestPath: path.relative(PACKAGE_ROOT, manifestPath),
      })),
      errors: discovery.errors.map((error) => ({
        manifestPath: path.relative(PACKAGE_ROOT, error.manifestPath),
        problems: error.problems,
      })),
    };
    console.log(JSON.stringify(output, null, 2));
  } else {
    printScenarioList(scenarios);
    printDiscoveryErrors(discovery.errors);
  }
  return discovery.errors.length > 0 ? EXIT.usage : EXIT.success;
}

async function commandRun(
  mode: RunMode,
  scenariosDir: string,
  filters: SelectionFilters,
  outputDirOption: string | undefined,
): Promise<number> {
  const discovery = await discover(scenariosDir);
  if (discovery.errors.length > 0) {
    printDiscoveryErrors(discovery.errors);
    return EXIT.usage;
  }
  const plan = planExecution(discovery.scenarios, filters);

  const startedAt = new Date();
  const runId = createRunId(mode, startedAt);
  const outputDir = path.resolve(outputDirOption ?? path.join(DEFAULT_REPORTS_DIR, runId));
  mkdirSync(outputDir, { recursive: true });
  printPlan(plan, mode);

  const abort = new AbortController();
  const onSigint = () => abort.abort();
  process.once('SIGINT', onSigint);
  process.once('SIGTERM', onSigint);

  const results = await executePlan(plan, {
    mode,
    outputDir,
    cwd: PACKAGE_ROOT,
    signal: abort.signal,
    onScenarioStart: (scenario) => printScenarioHeader(scenario, mode),
  });
  process.off('SIGINT', onSigint);
  process.off('SIGTERM', onSigint);

  const report = buildRunReport({
    runId,
    mode,
    engineVersion: readEngineVersion(),
    startedAt,
    finishedAt: new Date(),
    plan,
    results,
    outputDir,
    scenariosDir,
    packageRoot: PACKAGE_ROOT,
  });
  const files = writeRunReport(report, outputDir);
  printRunSummary(report, files);

  if (abort.signal.aborted) return EXIT.interrupted;
  return report.summary.success ? EXIT.success : EXIT.failure;
}

async function main(argv: string[]): Promise<number> {
  const { values, positionals } = parseArgs({
    args: argv,
    allowPositionals: true,
    options: {
      tags: { type: 'string', short: 't' },
      'scenarios-dir': { type: 'string' },
      'output-dir': { type: 'string', short: 'o' },
      json: { type: 'boolean', default: false },
      version: { type: 'boolean', short: 'v', default: false },
      help: { type: 'boolean', short: 'h', default: false },
    },
  });

  if (values.version) {
    console.log(readEngineVersion());
    return EXIT.success;
  }
  const [command, ...ids] = positionals;
  if (values.help || command === undefined || command === 'help') {
    console.log(USAGE);
    return command === undefined && !values.help ? EXIT.usage : EXIT.success;
  }

  const scenariosDir = path.resolve(values['scenarios-dir'] ?? DEFAULT_SCENARIOS_DIR);
  const filters: SelectionFilters = { ids, ...(values.tags !== undefined ? { tagExpression: values.tags } : {}) };

  switch (command) {
    case 'list':
      return commandList(scenariosDir, filters, values.json);
    case 'run':
    case 'dry-run':
      return commandRun(command, scenariosDir, filters, values['output-dir']);
    default:
      throw new UsageError(`Unknown command '${command}'.`);
  }
}

main(process.argv.slice(2))
  .then((exitCode) => {
    process.exitCode = exitCode;
  })
  .catch((error: unknown) => {
    // Exit code 1 is reserved for scenario failures; anything thrown here means nothing (more) ran.
    const isUsage = error instanceof UsageError || (error as { code?: string }).code?.startsWith('ERR_PARSE_ARGS');
    const isExpected = isUsage || error instanceof PlanError;
    console.error(`test-engine: ${error instanceof Error ? error.message : String(error)}`);
    if (isUsage) console.error(`Run 'test-engine help' for usage.`);
    if (!isExpected && error instanceof Error && error.stack) console.error(error.stack);
    process.exitCode = EXIT.usage;
  });
