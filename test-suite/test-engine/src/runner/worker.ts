/**
 * Scenario worker: runs ONE scenario in its own Node process.
 *
 * Cucumber keeps the World constructor, default timeout, parameter types and step definitions
 * in a process-wide registry, and Node caches imported modules. Running each scenario in a fresh
 * process is what keeps those globals, and any resource the support code opens, from leaking
 * into the next scenario.
 *
 * The job (resolved profile included) arrives on stdin. The worker writes `messages.ndjson` and
 * `cucumber-report.html` itself, from redacted envelopes, so that nothing unredacted ever
 * reaches the disk.
 */
import { type IConfiguration, type IRunConfiguration, loadConfiguration, runCucumber } from '@cucumber/cucumber/api';
import { CucumberHtmlStream } from '@cucumber/html-formatter';
import type { Envelope } from '@cucumber/messages';
import { createWriteStream, mkdirSync } from 'node:fs';
import path from 'node:path';
import { finished } from 'node:stream/promises';
import { fileURLToPath } from 'node:url';

import { secretsOf } from '../profiles/profile.js';
import { findExecutionProblems } from '../report/execution-problems.js';
import { createRedactor } from '../secrets/redactor.js';
import { setCurrentJob } from './job-context.js';
import { ARTIFACTS, ENGINE_ERROR_PREFIX, WORKER_EXIT, type WorkerJob } from './types.js';

const SIGNAL_FLUSH_MS = 500;

/** Retries could turn a failure into a pass; the engine never allows them. */
export function assertRetryDisabled(configuration: IRunConfiguration): void {
  if (configuration.runtime.retry !== 0) {
    throw new Error(`retries are not allowed (resolved retry: ${configuration.runtime.retry})`);
  }
}

async function readJob(): Promise<WorkerJob> {
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) chunks.push(chunk as Buffer);
  const text = Buffer.concat(chunks).toString('utf8');
  if (text.trim() === '') throw new Error('no job received on stdin');
  return JSON.parse(text) as WorkerJob;
}

async function runJob(job: WorkerJob): Promise<number> {
  setCurrentJob(job);
  mkdirSync(job.artifactsDir, { recursive: true });
  const redactor = createRedactor(secretsOf(job.worldParameters.profile));

  const messagesFile = createWriteStream(path.join(job.artifactsDir, ARTIFACTS.messages));
  const htmlFile = createWriteStream(path.join(job.artifactsDir, ARTIFACTS.cucumberHtml));
  const html = new CucumberHtmlStream();
  html.pipe(htmlFile);
  const envelopes: Envelope[] = [];
  const onMessage = (envelope: Envelope) => {
    const safe = redactor.redactDeep(envelope);
    envelopes.push(safe);
    messagesFile.write(`${JSON.stringify(safe)}\n`);
    html.write(safe);
  };

  const { runConfiguration } = await loadConfiguration(
    {
      file: false,
      provided: {
        paths: [job.featurePath],
        import: job.supportModules,
        // Only the live terminal output is a Cucumber formatter; the orchestrator redacts it.
        format: [job.mode === 'dry-run' ? 'summary' : 'pretty'],
        dryRun: job.mode === 'dry-run',
        strict: true,
        retry: 0,
        parallel: 0,
        // Plain JSON data (see EngineWorldParameters); Cucumber types it as a generic JSON object.
        worldParameters: job.worldParameters as unknown as IConfiguration['worldParameters'],
      },
    },
    { cwd: job.cwd },
  );
  assertRetryDisabled(runConfiguration);

  const { success } = await runCucumber(runConfiguration, { cwd: job.cwd }, onMessage);
  messagesFile.end();
  html.end();
  await Promise.all([finished(messagesFile), finished(htmlFile)]);

  // Cucumber reports success in dry-run mode even with parse errors, undefined or ambiguous
  // steps, and in both modes for runtime skips, so the worker derives its own verdict.
  const problems = findExecutionProblems(envelopes, job.mode);
  if (problems.length > 0) {
    console.error(`[test-engine] scenario is invalid: ${problems.map((problem) => problem.message).join('; ')}`);
    return WORKER_EXIT.scenarioFailed;
  }
  return success ? WORKER_EXIT.success : WORKER_EXIT.scenarioFailed;
}

function main(): void {
  // On SIGTERM (timeout) or SIGINT (Ctrl+C), give pending writes a moment to flush, so that the
  // partial message stream remains available as evidence.
  for (const signal of ['SIGTERM', 'SIGINT'] as const) {
    process.once(signal, () => {
      setTimeout(() => process.exit(WORKER_EXIT.terminated), SIGNAL_FLUSH_MS).unref();
    });
  }

  readJob()
    .then(runJob)
    .then((exitCode) => {
      // Explicit exit: anything the support code left open (sockets, worker pools) dies with the
      // process instead of keeping it alive.
      process.exit(exitCode);
    })
    .catch((error: unknown) => {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`${ENGINE_ERROR_PREFIX}${message.split('\n')[0]}`);
      if (error instanceof Error && error.stack) console.error(error.stack);
      process.exit(WORKER_EXIT.engineError);
    });
}

// Run only when executed as the worker entry point (tests import assertRetryDisabled).
if (process.argv[1] && fileURLToPath(import.meta.url) === path.resolve(process.argv[1])) main();
