import { spawn } from 'node:child_process';
import { createWriteStream, mkdirSync } from 'node:fs';
import path from 'node:path';

import type { DiscoveredScenario } from '../discovery/discovery.js';
import { ENGINE_SUPPORT_MODULE, WORKER_ENTRY } from '../paths.js';
import type { ExecutionPlan } from '../planner/planner.js';
import {
  ARTIFACTS,
  ENGINE_ERROR_PREFIX,
  type RunMode,
  STEP_TIMEOUT_ENV,
  type ScenarioRunResult,
  WORKER_EXIT,
  type WorkerJob,
} from './types.js';

/** Extra time given to Cucumber to report a step timeout and flush its formatters before the hard kill. */
export const KILL_GRACE_MS = 5_000;
const SIGKILL_AFTER_MS = 3_000;

export interface OrchestratorOptions {
  mode: RunMode;
  /** Run output directory; scenario artifacts go to `<outputDir>/scenarios/<id>/`. */
  outputDir: string;
  cwd: string;
  signal?: AbortSignal;
  onScenarioStart?: (scenario: DiscoveredScenario) => void;
}

const ANSI_ESCAPE = /\u001b\[[0-9;]*[A-Za-z]/g;

function runWorker(scenario: DiscoveredScenario, options: OrchestratorOptions): Promise<ScenarioRunResult> {
  const { manifest } = scenario;
  const scenarioId = manifest.metadata.id;
  const artifactsDir = path.join(options.outputDir, 'scenarios', scenarioId);
  mkdirSync(artifactsDir, { recursive: true });

  const job: WorkerJob = {
    scenarioId,
    mode: options.mode,
    featurePath: scenario.featurePath,
    supportModules: [ENGINE_SUPPORT_MODULE, ...scenario.supportModules],
    artifactsDir,
    cwd: options.cwd,
  };
  const budgetMs = manifest.spec.timeoutSeconds * 1000;

  const started = new Date();
  const child = spawn(process.execPath, ['--enable-source-maps', WORKER_ENTRY, JSON.stringify(job)], {
    cwd: options.cwd,
    stdio: ['ignore', 'pipe', 'pipe'],
    env: {
      ...process.env,
      [STEP_TIMEOUT_ENV]: String(budgetMs),
      ...(process.stdout.isTTY && process.env.NO_COLOR === undefined ? { FORCE_COLOR: '1' } : {}),
    },
  });

  // Tee the worker output: live to the terminal, ANSI-free to console.log as evidence.
  const consoleLog = createWriteStream(path.join(artifactsDir, ARTIFACTS.console));
  child.stdout.on('data', (chunk: Buffer) => {
    process.stdout.write(chunk);
    consoleLog.write(chunk.toString('utf8').replace(ANSI_ESCAPE, ''));
  });
  let workerError: string | undefined;
  child.stderr.on('data', (chunk: Buffer) => {
    process.stderr.write(chunk);
    const text = chunk.toString('utf8').replace(ANSI_ESCAPE, '');
    consoleLog.write(text);
    workerError ??= text
      .split('\n')
      .find((line) => line.startsWith(ENGINE_ERROR_PREFIX))
      ?.slice(ENGINE_ERROR_PREFIX.length);
  });

  let timedOut = false;
  let interrupted = false;
  const kill = () => {
    child.kill('SIGTERM');
    setTimeout(() => child.kill('SIGKILL'), SIGKILL_AFTER_MS).unref();
  };
  const deadline = setTimeout(() => {
    timedOut = true;
    kill();
  }, budgetMs + KILL_GRACE_MS);
  const onAbort = () => {
    interrupted = true;
    kill();
  };
  options.signal?.addEventListener('abort', onAbort, { once: true });

  return new Promise((resolve) => {
    const finish = (exitCode: number | null, signal: string | null, spawnError?: Error) => {
      clearTimeout(deadline);
      options.signal?.removeEventListener('abort', onAbort);
      const finished = new Date();
      const base = {
        scenarioId,
        startedAt: started.toISOString(),
        finishedAt: finished.toISOString(),
        durationMs: finished.getTime() - started.getTime(),
        exitCode,
        signal,
        artifactsDir,
      };
      let result: ScenarioRunResult;
      if (spawnError) {
        result = { ...base, outcome: 'error', reason: `worker could not be started: ${spawnError.message}` };
      } else if (timedOut) {
        result = {
          ...base,
          outcome: 'timed-out',
          reason: `exceeded spec.timeoutSeconds (${manifest.spec.timeoutSeconds}s)`,
        };
      } else if (interrupted) {
        result = { ...base, outcome: 'error', reason: 'run interrupted' };
      } else if (exitCode === WORKER_EXIT.success) {
        result = { ...base, outcome: 'passed' };
      } else if (exitCode === WORKER_EXIT.scenarioFailed) {
        result = { ...base, outcome: 'failed' };
      } else {
        const how = signal ? `killed by ${signal}` : `exited with code ${String(exitCode)}`;
        const reason = workerError
          ? `worker could not run the scenario: ${workerError}`
          : `worker ${how}; see ${ARTIFACTS.console}`;
        result = { ...base, outcome: 'error', reason };
      }
      consoleLog.end(() => resolve(result));
    };
    child.once('error', (error) => finish(null, null, error));
    child.once('close', (code, signal) => finish(code, signal));
  });
}

/**
 * Executes the plan sequentially, one worker process per scenario that is planned to run.
 * Never throws for scenario failures: every item of the plan produces exactly one result.
 */
export async function executePlan(plan: ExecutionPlan, options: OrchestratorOptions): Promise<ScenarioRunResult[]> {
  const results: ScenarioRunResult[] = [];
  for (const { scenario, decision } of plan.items) {
    options.onScenarioStart?.(scenario);
    let result: ScenarioRunResult;
    if (decision.action === 'skip' || options.signal?.aborted) {
      const now = new Date().toISOString();
      result = {
        scenarioId: scenario.manifest.metadata.id,
        outcome: 'skipped',
        reason: decision.action === 'skip' ? decision.reason : 'run interrupted',
        startedAt: now,
        finishedAt: now,
        durationMs: 0,
        exitCode: null,
        signal: null,
      };
    } else {
      result = await runWorker(scenario, options);
    }
    results.push(result);
  }
  return results;
}
