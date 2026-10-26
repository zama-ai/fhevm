#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { randomBytes } from 'node:crypto';
import { mkdirSync } from 'node:fs';
import path from 'node:path';
import { parseArgs } from 'node:util';

import { type DiscoveryResult, discoverScenarios } from '../discovery/discovery.js';
import {
  DEFAULT_PROFILES_DIR,
  DEFAULT_REPORTS_DIR,
  DEFAULT_SCENARIOS_DIR,
  DIST_DIR,
  PACKAGE_ROOT,
  readEngineVersion,
} from '../paths.js';
import {
  PlanError,
  type SelectionFilters,
  checkConfirmation,
  hasRunnableItems,
  planExecution,
  selectScenarios,
} from '../planner/planner.js';
import { listProfileNames, listProfiles, loadProfile, secretsOf } from '../profiles/profile.js';
import { BUILTIN_PROFILE, ProfileError } from '../profiles/types.js';
import { buildRunReport, writeRunReport } from '../report/report.js';
import { executePlan } from '../runner/orchestrator.js';
import type { RunMode } from '../runner/types.js';
import { createRedactor } from '../secrets/redactor.js';
import {
  printDiscoveryErrors,
  printPlan,
  printProfileErrors,
  printProfileList,
  printProfileSettings,
  printRunSummary,
  printScenarioHeader,
  printScenarioList,
} from './output.js';

/** CLI exit codes. Precedence when several apply: 130 > 1 > 3 > 0 (2 happens before anything runs). */
const EXIT = {
  success: 0,
  /** At least one scenario failed, timed out or errored (dry-run: invalid). */
  failure: 1,
  /** Usage or configuration error. Nothing was executed and no report is written. */
  usage: 2,
  /** Nothing was executed: everything selected was skipped by the planner. The report is written. */
  nothingExecuted: 3,
  interrupted: 130,
} as const;

const USAGE = `Usage: test-engine <command> [scenario-id...] [options]

Commands:
  list       List discovered scenarios (validates every manifest)
  run        Run the selected scenarios, one isolated process per scenario
  dry-run    Validate the selected scenarios without executing any step
  profiles   List and validate the profiles; with --profile, also resolve that profile
  help       Show this message

Options:
  -p, --profile <name>       Target environment profile (default: none)
      --confirm <name>       Confirm a profile that requires it (required by run; accepted by dry-run;
                             must repeat the profile name; rejected by list and profiles)
  -t, --tags <expr>          Cucumber tag expression on manifest tags, e.g. "@smoke and not @slow"
      --scenarios-dir <dir>  Directory scanned for scenario.yaml files (default: <package>/scenarios)
      --profiles-dir <dir>   Directory holding <profile>.yaml files (default: <package>/profiles)
  -o, --output-dir <dir>     Run output directory (default: <package>/reports/<run-id>)
      --json                 list: print JSON instead of a table
  -v, --version              Print the engine version

Exit codes: 0 success · 1 scenario failure · 2 usage/configuration error · 3 nothing executed · 130 interrupted`;

class UsageError extends Error {}

/** Set once scenarios may have started; exit code 2 is reserved for errors detected before that. */
let executionStarted = false;

interface CommonOptions {
  scenariosDir: string;
  profilesDir: string;
  profile: string;
  confirm: string | undefined;
  filters: SelectionFilters;
}

function createRunId(mode: RunMode, now: Date): string {
  const stamp = now.toISOString().replace(/[-:]/g, '').replace(/\..+$/, '').replace('T', '-');
  return `${stamp}-${mode}-${randomBytes(2).toString('hex')}`;
}

function readGitCommit(): string | undefined {
  try {
    const commit = execFileSync('git', ['rev-parse', 'HEAD'], {
      cwd: PACKAGE_ROOT,
      stdio: ['ignore', 'pipe', 'ignore'],
      timeout: 2_000,
    });
    return commit.toString('utf8').trim() || undefined;
  } catch {
    return undefined;
  }
}

async function discover(options: CommonOptions): Promise<DiscoveryResult> {
  return discoverScenarios({
    scenariosDir: options.scenariosDir,
    sourceRoot: PACKAGE_ROOT,
    distDir: DIST_DIR,
    knownProfiles: listProfileNames(options.profilesDir),
  });
}

async function commandList(options: CommonOptions, json: boolean, profileGiven: boolean): Promise<number> {
  const discovery = await discover(options);
  // `list` does not plan, but an unknown --profile is still a mistake worth reporting.
  if (profileGiven && !listProfileNames(options.profilesDir).includes(options.profile)) {
    throw new ProfileError(options.profile, [
      `unknown profile '${options.profile}'. Available: ${listProfileNames(options.profilesDir).join(', ')}`,
    ]);
  }
  const scenarios = selectScenarios(discovery.scenarios, options.filters);
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

function commandProfiles(options: CommonOptions, profileGiven: boolean): number {
  const listing = listProfiles(options.profilesDir);
  printProfileList(listing.profiles);
  printProfileErrors(listing.errors);
  if (listing.errors.length > 0) return EXIT.usage;
  if (profileGiven) printProfileSettings(loadProfile(options.profile, { profilesDir: options.profilesDir }));
  return EXIT.success;
}

async function commandRun(mode: RunMode, options: CommonOptions, outputDirOption: string | undefined): Promise<number> {
  const discovery = await discover(options);
  if (discovery.errors.length > 0) {
    printDiscoveryErrors(discovery.errors);
    return EXIT.usage;
  }
  // Resolved once, before anything runs: a missing variable or env file fails fast.
  const profile = loadProfile(options.profile, { profilesDir: options.profilesDir });
  checkConfirmation(mode, profile, options.confirm);
  const plan = planExecution(discovery.scenarios, options.filters, profile);
  const redactor = createRedactor(secretsOf(profile));

  const startedAt = new Date();
  const runId = createRunId(mode, startedAt);
  const outputDir = path.resolve(outputDirOption ?? path.join(DEFAULT_REPORTS_DIR, runId));
  mkdirSync(outputDir, { recursive: true });
  printPlan(plan, mode);

  const abort = new AbortController();
  const onSignal = () => abort.abort();
  process.once('SIGINT', onSignal);
  process.once('SIGTERM', onSignal);

  // From here on, scenarios may run: an unexpected error is no longer a configuration error (exit 2).
  executionStarted = true;
  // With nothing runnable, executePlan only records the skips: no worker is started.
  const results = await executePlan(plan, {
    mode,
    outputDir,
    cwd: PACKAGE_ROOT,
    profile,
    redactor,
    signal: abort.signal,
    onScenarioStart: (scenario) => printScenarioHeader(scenario, plan, mode),
  });
  process.off('SIGINT', onSignal);
  process.off('SIGTERM', onSignal);

  const gitCommit = readGitCommit();
  const report = buildRunReport({
    runId,
    mode,
    engineVersion: readEngineVersion(),
    startedAt,
    finishedAt: new Date(),
    plan,
    results,
    profile,
    interrupted: abort.signal.aborted,
    ...(gitCommit ? { gitCommit } : {}),
    outputDir,
    scenariosDir: options.scenariosDir,
    packageRoot: PACKAGE_ROOT,
  });
  const files = writeRunReport(report, outputDir, redactor);
  printRunSummary(report, files, hasRunnableItems(plan));

  const { summary } = report;
  if (abort.signal.aborted) return EXIT.interrupted;
  if (summary.failed + summary['timed-out'] + summary.error > 0) return EXIT.failure;
  if (summary.executed === 0) return EXIT.nothingExecuted;
  return EXIT.success;
}

async function main(argv: string[]): Promise<number> {
  const { values, positionals } = parseArgs({
    args: argv,
    allowPositionals: true,
    options: {
      profile: { type: 'string', short: 'p' },
      confirm: { type: 'string' },
      tags: { type: 'string', short: 't' },
      'scenarios-dir': { type: 'string' },
      'profiles-dir': { type: 'string' },
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

  const options: CommonOptions = {
    scenariosDir: path.resolve(values['scenarios-dir'] ?? DEFAULT_SCENARIOS_DIR),
    profilesDir: path.resolve(values['profiles-dir'] ?? DEFAULT_PROFILES_DIR),
    profile: values.profile ?? BUILTIN_PROFILE,
    confirm: values.confirm,
    filters: { ids, ...(values.tags !== undefined ? { tagExpression: values.tags } : {}) },
  };
  if (options.confirm !== undefined && command !== 'run' && command !== 'dry-run') {
    throw new UsageError(`--confirm is only accepted by 'run' and 'dry-run'`);
  }

  switch (command) {
    case 'list':
      return commandList(options, values.json, values.profile !== undefined);
    case 'profiles':
      return commandProfiles(options, values.profile !== undefined);
    case 'run':
    case 'dry-run':
      return commandRun(command, options, values['output-dir']);
    default:
      throw new UsageError(`Unknown command '${command}'.`);
  }
}

main(process.argv.slice(2))
  .then((exitCode) => {
    process.exitCode = exitCode;
  })
  .catch((error: unknown) => {
    if (executionStarted) {
      // An internal error after scenarios started (e.g. the report could not be written).
      console.error(
        `test-engine: internal error after execution started: ${error instanceof Error ? (error.stack ?? error.message) : String(error)}`,
      );
      process.exitCode = EXIT.failure;
      return;
    }
    // Exit code 2: a usage or configuration problem, detected before any scenario ran.
    const isUsage = error instanceof UsageError || (error as { code?: string }).code?.startsWith('ERR_PARSE_ARGS');
    if (error instanceof ProfileError) {
      printProfileErrors([error]);
    } else {
      console.error(`test-engine: ${error instanceof Error ? error.message : String(error)}`);
      if (isUsage) console.error(`Run 'test-engine help' for usage.`);
      if (!isUsage && !(error instanceof PlanError) && error instanceof Error && error.stack)
        console.error(error.stack);
    }
    process.exitCode = EXIT.usage;
  });
