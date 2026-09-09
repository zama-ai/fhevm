import { resolve } from 'node:path';
import { setImmediate as setImmediateP } from 'node:timers/promises';
import { getEthersTestConfig } from '../fheTest/setup-ethers.js';
import { loadLocalstackChainDefaults } from './support/chainDefaults.js';
import { ensureLocalstackReady } from './support/localstack.js';
import { assertGcIsExposed, MemorySampler } from './support/memorySampler.js';
import { evaluateAllTrends, hasActionableGrowth } from './support/trendDetector.js';
import {
  printHeader,
  printSampleLine,
  printTrendSummary,
  updateMetricBaselines,
  type MetricBaselines,
} from './support/reporter.js';
import { clientReuseScenario } from './scenarios/clientReuse.js';
import { clientChurnScenario } from './scenarios/clientChurn.js';
import { roundtripScenario } from './scenarios/roundtrip.js';
import { providerChurnScenario } from './scenarios/providerChurn.js';
import { valueChurnScenario } from './scenarios/valueChurn.js';
import { permitChurnScenario } from './scenarios/permitChurn.js';
import type { Scenario } from './scenarios/scenario.js';

// This project targets the `localstack` chain only (see plan: no
// multi-version matrix in v1) — forced here rather than left to whatever
// CHAIN happens to be set in the caller's shell.
const CHAIN_NAME = 'localstack';
const MAX_CONSECUTIVE_ITERATION_FAILURES = 10;

const SCENARIOS: Readonly<Record<string, Scenario>> = {
  providerChurn: providerChurnScenario,
  permitChurn: permitChurnScenario,
  valueChurn: valueChurnScenario,
  clientReuse: clientReuseScenario,
  clientChurn: clientChurnScenario,
  roundtrip: roundtripScenario,
};

type CliOptions = {
  readonly scenarioNames: readonly string[];
  readonly iterations: number | undefined;
  readonly durationSeconds: number | undefined;
  readonly sampleIntervalMs: number;
  readonly warmupIterations: number | undefined;
  readonly restartLocalstack: boolean;
  readonly fhevmCliProfile: string | undefined;
  readonly outDir: string;
};

async function main(): Promise<void> {
  assertGcIsExposed();

  const options = parseArgs(process.argv.slice(2));
  process.env.CHAIN = CHAIN_NAME;

  const { rpcUrl } = loadLocalstackChainDefaults(CHAIN_NAME);
  await ensureLocalstackReady({
    restart: options.restartLocalstack,
    rpcUrl,
    chainName: CHAIN_NAME,
    fhevmCliProfile: options.fhevmCliProfile,
  });

  const config = getEthersTestConfig();

  for (const name of options.scenarioNames) {
    const scenario = SCENARIOS[name];
    if (scenario === undefined) {
      throw new Error(`Unknown scenario "${name}". Available: ${Object.keys(SCENARIOS).join(', ')}.`);
    }
    const plateaued = await runScenario(scenario, { config, options });
    if (!plateaued) {
      console.error(`\n❌ ${scenario.name} showed sustained (non-plateauing) memory growth. See trend summary above.`);
      process.exit(1);
    } else {
      console.log(`\n✅ ${scenario.name} plateaued (or had insufficient data to classify). No anomaly detected.`);
    }
  }
}

async function runScenario(
  scenario: Scenario,
  context: { config: ReturnType<typeof getEthersTestConfig>; options: CliOptions },
): Promise<boolean> {
  const { config, options } = context;
  console.log(`\nSetting up scenario "${scenario.name}" — ${scenario.description}`);

  const { iterate, readTfheMemory, readTkmsMemory, teardown } = await scenario.setup({ config });

  let iteration = 0;
  let sampleCount = 0;
  const baselines: MetricBaselines = { rssBytes: undefined, tfheMemoryBytes: undefined, tkmsMemoryBytes: undefined };
  const startedAtMs = Date.now();

  // Computed before the sampler so the onSample closure below can reference
  // them — it fires synchronously from the `sampler.tick()` call right after
  // `sampler.start()`, before any code declared later in this function runs.
  const iterationLimit = options.iterations ?? scenario.defaultIterations;
  const deadlineMs = options.durationSeconds !== undefined ? Date.now() + options.durationSeconds * 1_000 : undefined;
  // Default warmup scales with the run length: a flat default (e.g. 50) would
  // silently swallow every sample on a short manual smoke run (--iterations
  // 50 with a 50-iteration warmup leaves nothing to classify).
  const warmupIterations = options.warmupIterations ?? Math.min(50, Math.max(1, Math.floor(iterationLimit * 0.1)));

  const sampler = new MemorySampler({
    intervalMs: options.sampleIntervalMs,
    getIteration: () => iteration,
    ...(readTfheMemory !== undefined ? { readTfheMemory } : {}),
    ...(readTkmsMemory !== undefined ? { readTkmsMemory } : {}),
    jsonlPath: resolve(options.outDir, `${scenario.name}.jsonl`),
    onSample: (sample) => {
      if (sampleCount === 0) {
        printHeader(scenario.name);
      }
      sampleCount += 1;
      const etaMs = estimateRemainingMs({
        iteration,
        iterationLimit,
        deadlineMs,
        elapsedMs: sample.tMs - startedAtMs,
      });
      // Baselines are tracked per-metric (see updateMetricBaselines) rather
      // than snapshotting one "first sample": RSS is defined from tick 0, but
      // tfhe/tkms WASM memory only becomes defined once a client actually
      // initializes that module, which can be several samples in.
      printSampleLine(sample, baselines, startedAtMs, etaMs);
      updateMetricBaselines(baselines, sample);
    },
  });

  sampler.start();
  await sampler.tick(); // baseline reading at iteration 0, before the loop starts

  let consecutiveFailures = 0;

  try {
    while (iteration < iterationLimit && (deadlineMs === undefined || Date.now() < deadlineMs)) {
      try {
        await iterate();
        consecutiveFailures = 0;
      } catch (error) {
        consecutiveFailures += 1;
        console.error(`[${scenario.name}] iteration ${iteration} failed:`, error);
        if (consecutiveFailures > MAX_CONSECUTIVE_ITERATION_FAILURES) {
          throw new Error(
            `[${scenario.name}] ${consecutiveFailures} consecutive iteration failures — aborting (is localstack still up?).`,
          );
        }
      }
      iteration += 1;
      // Force a real event-loop turn after every iteration. Scenarios whose
      // work involves genuine I/O (a relayer HTTP call) naturally give the
      // timers phase a chance to run, so the sampler's setInterval fires on
      // schedule. Scenarios that are purely local WASM/crypto computation
      // (e.g. permitChurn) never do — every `await` in their chain resolves
      // through the microtask queue alone, and Node drains microtasks before
      // checking timers, so a tight loop of fast, no-I/O iterations can
      // starve the timers phase for the whole run. `setImmediate` forces a
      // genuine macrotask boundary so "print regularly" holds regardless of
      // how a scenario is implemented.
      await setImmediateP();
    }
  } finally {
    await sampler.stop();
    await teardown?.();
  }

  const verdicts = evaluateAllTrends(sampler.samples, { warmupIterations });
  printTrendSummary(scenario.name, verdicts);

  return !hasActionableGrowth(verdicts);
}

/**
 * Estimates remaining wall-clock time from whichever bound the run is closer
 * to hitting. The iteration-based estimate extrapolates the average time per
 * iteration seen so far; the duration-based estimate is just a countdown to
 * the deadline. When both `--iterations` and `--duration-seconds` are set,
 * either can end the run first, so this reports the smaller (sooner) of the
 * two rather than picking one arbitrarily.
 */
function estimateRemainingMs(parameters: {
  readonly iteration: number;
  readonly iterationLimit: number;
  readonly deadlineMs: number | undefined;
  readonly elapsedMs: number;
}): number | undefined {
  const { iteration, iterationLimit, deadlineMs, elapsedMs } = parameters;
  const candidates: number[] = [];

  if (iteration > 0) {
    const averageMsPerIteration = elapsedMs / iteration;
    candidates.push(Math.max(0, averageMsPerIteration * (iterationLimit - iteration)));
  }
  if (deadlineMs !== undefined) {
    candidates.push(Math.max(0, deadlineMs - Date.now()));
  }

  return candidates.length > 0 ? Math.min(...candidates) : undefined;
}

function parseArgs(argv: string[]): CliOptions {
  if (argv.includes('--help') || argv.includes('-h')) {
    printHelp();
    process.exit(0);
  }

  const options: {
    scenarioNames: string[];
    iterations: number | undefined;
    durationSeconds: number | undefined;
    sampleIntervalMs: number;
    warmupIterations: number | undefined;
    restartLocalstack: boolean;
    fhevmCliProfile: string | undefined;
    outDir: string;
  } = {
    scenarioNames: Object.keys(SCENARIOS),
    iterations: undefined,
    durationSeconds: undefined,
    sampleIntervalMs: 2_000,
    warmupIterations: undefined,
    restartLocalstack: false,
    fhevmCliProfile: undefined,
    outDir: resolve(import.meta.dirname, 'reports'),
  };

  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    switch (arg) {
      case '--scenario': {
        const value = requireValue(argv, i);
        i++;
        options.scenarioNames = value === 'all' ? Object.keys(SCENARIOS) : value.split(',');
        break;
      }
      case '--iterations':
        options.iterations = Number(requireValue(argv, i));
        i++;
        break;
      case '--duration-seconds':
        options.durationSeconds = Number(requireValue(argv, i));
        i++;
        break;
      case '--sample-interval-ms':
        options.sampleIntervalMs = Number(requireValue(argv, i));
        i++;
        break;
      case '--warmup':
        options.warmupIterations = Number(requireValue(argv, i));
        i++;
        break;
      case '--restart-localstack':
        options.restartLocalstack = true;
        break;
      case '--fhevm-cli-profile':
        options.fhevmCliProfile = requireValue(argv, i);
        i++;
        break;
      case '--out':
        options.outDir = resolve(requireValue(argv, i));
        i++;
        break;
      default:
        throw new Error(`Unknown option "${arg}". Use --help for usage.`);
    }
  }

  for (const name of options.scenarioNames) {
    if (SCENARIOS[name] === undefined) {
      throw new Error(`Unknown scenario "${name}". Available: ${Object.keys(SCENARIOS).join(', ')}, or "all".`);
    }
  }

  return options;
}

function requireValue(argv: readonly string[], index: number): string {
  const value = argv[index + 1];
  if (value === undefined || value.startsWith('--')) {
    throw new Error(`${argv[index]} requires a value.`);
  }
  return value;
}

function printHelp(): void {
  const descriptions = Object.values(SCENARIOS)
    .map(
      (s) =>
        `  ${s.name.padEnd(16)} ${s.description} (default: ${s.defaultIterations} iterations, ${s.defaultIterationsDuration})`,
    )
    .join('\n');

  console.log(`Usage: node test/memleaks/run.mjs [options]

Runs long stress loops of FHE encrypt/decrypt/serialize operations against a
real localstack Docker stack, sampling process + WASM memory to detect
sustained (non-plateauing) growth. See test/memleaks/README.md.

Options:
  --scenario <name|all>       Comma-separated scenario name(s), or "all" (default: all).
  --iterations <n>            Iteration bound. Defaults to the scenario's own default.
  --duration-seconds <n>      Wall-clock bound instead of (or in addition to) --iterations.
  --sample-interval-ms <n>    Sampling cadence in ms (default: 2000).
  --warmup <n>                Iterations excluded from trend detection as warmup
                              (default: 10% of the iteration bound, capped at 50).
  --restart-localstack        Restart the localstack Docker stack before running.
  --fhevm-cli-profile <name>  Profile filename forwarded to localstack-restart.sh.
  --out <dir>                 Directory for JSONL sample output (default: test/memleaks/reports).
  -h, --help                  Show this help.

Scenarios:
${descriptions}

Examples:
  node test/memleaks/run.mjs --restart-localstack --scenario clientChurn --iterations 500
  node test/memleaks/run.mjs --scenario clientReuse --iterations 5000
  node test/memleaks/run.mjs --scenario all --duration-seconds 1800
`);
}

// Force-exit once our own work is done: the tfhe/tkms WASM modules spawn a
// worker thread pool (`initThreadPool()` in
// src/core/modules/encrypt/module/init-p.ts) that has no teardown path by
// design — the module is a permanent per-process singleton (see this
// project's README). Those workers keep the event loop alive forever, so
// letting the process exit "naturally" would just hang here regardless of
// how cleanly every scenario itself finished.
main()
  .then(() => {
    process.exit(process.exitCode ?? 0);
  })
  .catch((error: unknown) => {
    console.error(error);
    process.exit(1);
  });
