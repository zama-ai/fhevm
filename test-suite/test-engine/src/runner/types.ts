/**
 * Contracts of the Runner component: the job handed to a scenario worker process, and the
 * result the orchestrator derives from it.
 */

export type RunMode = 'run' | 'dry-run';

/** Serialised as JSON and passed to the worker process as its only argument. */
export interface WorkerJob {
  scenarioId: string;
  mode: RunMode;
  featurePath: string;
  /** Engine support code first, then the scenario's support code. */
  supportModules: string[];
  /** Directory where the worker writes the Cucumber artifacts of this scenario. */
  artifactsDir: string;
  cwd: string;
}

/** File names the worker writes inside `WorkerJob.artifactsDir`. */
export const ARTIFACTS = {
  messages: 'messages.ndjson',
  cucumberHtml: 'cucumber-report.html',
  console: 'console.log',
} as const;

/** Environment variable carrying the default Cucumber step timeout into the worker. */
export const STEP_TIMEOUT_ENV = 'TEST_ENGINE_STEP_TIMEOUT_MS';

/** Prefix of the single stderr line in which the worker reports why it could not run Cucumber. */
export const ENGINE_ERROR_PREFIX = '[test-engine] worker error: ';

/** Worker process exit codes. */
export const WORKER_EXIT = {
  success: 0,
  scenarioFailed: 1,
  engineError: 2,
  /** Exited after SIGTERM/SIGINT (timeout or interruption), once pending output was flushed. */
  terminated: 143,
} as const;

/**
 * - `passed` — Cucumber reported success (in dry-run: every step is bound and the Gherkin is valid).
 * - `failed` — Cucumber reported a failure (in dry-run: undefined/ambiguous steps or parse errors).
 * - `skipped` — not executed, see `reason`.
 * - `timed-out` — killed after exceeding `spec.timeoutSeconds`.
 * - `error` — the worker could not run Cucumber (crash, support code failing to load, signal).
 */
export type ScenarioOutcome = 'passed' | 'failed' | 'skipped' | 'timed-out' | 'error';

export interface ScenarioRunResult {
  scenarioId: string;
  outcome: ScenarioOutcome;
  reason?: string;
  startedAt: string;
  finishedAt: string;
  durationMs: number;
  exitCode: number | null;
  signal: string | null;
  /** Absolute path of the scenario's artifacts directory; undefined for skipped scenarios. */
  artifactsDir?: string;
}
