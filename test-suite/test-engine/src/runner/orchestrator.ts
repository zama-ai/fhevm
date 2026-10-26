import { spawn } from 'node:child_process';
import { createWriteStream, mkdirSync } from 'node:fs';
import path from 'node:path';

import type { DiscoveredScenario } from '../discovery/discovery.js';
import { ENGINE_SUPPORT_MODULE, WORKER_ENTRY } from '../paths.js';
import type { ExecutionPlan } from '../planner/planner.js';
import type { ResolvedProfile } from '../profiles/types.js';
import { RedactingLineBuffer, type Redactor } from '../secrets/redactor.js';
import {
  ARTIFACTS,
  ENGINE_ERROR_PREFIX,
  type RunMode,
  type ScenarioRunResult,
  WORKER_EXIT,
  type WorkerJob,
} from './types.js';

/** Extra time given to Cucumber to report a step timeout and flush its output before the hard kill. */
export const KILL_GRACE_MS = 5_000;
const SIGKILL_AFTER_MS = 3_000;

/**
 * Environment variables a worker may inherit. Everything else (in particular the variables
 * profile settings are read from) stays out of the worker: scenarios get values through the
 * profile, never through `process.env`.
 */
const WORKER_ENV_ALLOWLIST = [
  'PATH',
  'HOME',
  'TMPDIR',
  'TEMP',
  'TMP',
  'TZ',
  'LANG',
  'NODE_OPTIONS',
  'NODE_EXTRA_CA_CERTS',
  'NO_COLOR',
  'FORCE_COLOR',
  'SystemRoot',
];

export function buildWorkerEnv(env: NodeJS.ProcessEnv, colorTerminal: boolean): NodeJS.ProcessEnv {
  const workerEnv: NodeJS.ProcessEnv = {};
  for (const [name, value] of Object.entries(env)) {
    if (value !== undefined && (WORKER_ENV_ALLOWLIST.includes(name) || name.startsWith('LC_'))) workerEnv[name] = value;
  }
  if (colorTerminal && workerEnv.NO_COLOR === undefined) workerEnv.FORCE_COLOR ??= '1';
  return workerEnv;
}

export interface OrchestratorOptions {
  mode: RunMode;
  /** Run output directory; scenario artifacts go to `<outputDir>/scenarios/<id>/`. */
  outputDir: string;
  cwd: string;
  profile: ResolvedProfile;
  /** Built from the profile secrets; applied to the worker output before it is printed or stored. */
  redactor: Redactor;
  signal?: AbortSignal;
  onScenarioStart?: (scenario: DiscoveredScenario) => void;
  /** Worker entry point (tests replace it to simulate broken workers). */
  workerEntry?: string;
}

const ANSI_ESCAPE = /\u001b\[[0-9;]*[A-Za-z]/g;

function runWorker(scenario: DiscoveredScenario, options: OrchestratorOptions): Promise<ScenarioRunResult> {
  const { manifest } = scenario;
  const { metadata } = manifest;
  const scenarioId = metadata.id;
  const artifactsDir = path.join(options.outputDir, 'scenarios', scenarioId);
  mkdirSync(artifactsDir, { recursive: true });

  const budgetMs = manifest.spec.timeoutSeconds * 1000;
  const job: WorkerJob = {
    scenarioId,
    mode: options.mode,
    featurePath: scenario.featurePath,
    supportModules: [ENGINE_SUPPORT_MODULE, ...scenario.supportModules],
    artifactsDir,
    cwd: options.cwd,
    stepTimeoutMs: budgetMs,
    worldParameters: {
      scenario: {
        id: metadata.id,
        name: metadata.name,
        owner: metadata.owner,
        tags: metadata.tags,
        ...(metadata.origin !== undefined ? { origin: metadata.origin } : {}),
      },
      profile: options.profile,
    },
  };

  const started = new Date();
  const child = spawn(process.execPath, ['--enable-source-maps', options.workerEntry ?? WORKER_ENTRY], {
    cwd: options.cwd,
    stdio: ['pipe', 'pipe', 'pipe'],
    env: buildWorkerEnv(process.env, process.stdout.isTTY === true),
  });

  // The job goes through stdin: it carries resolved settings, which must never appear in argv.
  // A worker that dies before reading it makes the write fail (EPIPE); that must not crash the engine.
  let stdinError: Error | undefined;
  child.stdin.on('error', (error) => {
    stdinError = error;
  });
  child.stdin.end(JSON.stringify(job));

  // Tee the worker output, redacted line by line: live to the terminal, ANSI-free to console.log.
  const consoleLog = createWriteStream(path.join(artifactsDir, ARTIFACTS.console));
  let workerError: string | undefined;
  const tee = (target: NodeJS.WriteStream) =>
    new RedactingLineBuffer(options.redactor, (text) => {
      target.write(text);
      const plain = text.replace(ANSI_ESCAPE, '');
      consoleLog.write(plain);
      workerError ??= plain
        .split('\n')
        .find((line) => line.startsWith(ENGINE_ERROR_PREFIX))
        ?.slice(ENGINE_ERROR_PREFIX.length);
    });
  const stdoutTee = tee(process.stdout);
  const stderrTee = tee(process.stderr);
  child.stdout.on('data', (chunk: Buffer) => stdoutTee.push(chunk.toString('utf8')));
  child.stderr.on('data', (chunk: Buffer) => stderrTee.push(chunk.toString('utf8')));

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
    let settled = false;
    const finish = (exitCode: number | null, signal: string | null, spawnError?: Error) => {
      if (settled) return;
      settled = true;
      clearTimeout(deadline);
      options.signal?.removeEventListener('abort', onAbort);
      stdoutTee.flush();
      stderrTee.flush();
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
        const detail = stdinError ? ` before reading its job (${stdinError.message})` : '';
        const reason = workerError
          ? `worker could not run the scenario: ${options.redactor.redact(workerError)}`
          : `worker ${how}${detail}; see ${ARTIFACTS.console}`;
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
