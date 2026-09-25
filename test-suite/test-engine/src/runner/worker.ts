/**
 * Scenario worker: runs ONE scenario in its own Node process.
 *
 * Cucumber keeps the World constructor, default timeout, parameter types and step definitions
 * in a process-wide registry, and Node caches imported modules. Running each scenario in a fresh
 * process is what keeps those globals, and any resource the support code opens, from leaking
 * into the next scenario.
 */
import { loadConfiguration, runCucumber } from '@cucumber/cucumber/api';
import { type Envelope, TestStepResultStatus } from '@cucumber/messages';
import { mkdirSync, readFileSync } from 'node:fs';
import path from 'node:path';

import { ARTIFACTS, ENGINE_ERROR_PREFIX, WORKER_EXIT, type WorkerJob } from './types.js';

const SIGNAL_FLUSH_MS = 500;

async function main(job: WorkerJob): Promise<number> {
  mkdirSync(job.artifactsDir, { recursive: true });

  const { runConfiguration } = await loadConfiguration(
    {
      file: false,
      provided: {
        paths: [job.featurePath],
        import: job.supportModules,
        format: [
          job.mode === 'dry-run' ? 'summary' : 'pretty',
          ['message', path.join(job.artifactsDir, ARTIFACTS.messages)],
          ['html', path.join(job.artifactsDir, ARTIFACTS.cucumberHtml)],
        ],
        dryRun: job.mode === 'dry-run',
        strict: true,
        parallel: 0,
        worldParameters: { scenarioId: job.scenarioId },
      },
    },
    { cwd: job.cwd },
  );

  const { success } = await runCucumber(runConfiguration, { cwd: job.cwd });
  if (!success) return WORKER_EXIT.scenarioFailed;

  // In dry-run mode Cucumber reports success even with parse errors, undefined or ambiguous
  // steps, so the worker derives its own verdict from the message stream.
  const problems = findBindingProblems(readFileSync(path.join(job.artifactsDir, ARTIFACTS.messages), 'utf8'));
  if (problems.length > 0) {
    console.error(`[test-engine] scenario is invalid: ${problems.join(', ')}`);
    return WORKER_EXIT.scenarioFailed;
  }
  return WORKER_EXIT.success;
}

/** Problems that make a scenario invalid regardless of the run mode. */
function findBindingProblems(ndjson: string): string[] {
  const counts = { 'parse error': 0, 'undefined parameter type': 0, 'undefined step': 0, 'ambiguous step': 0 };
  for (const line of ndjson.split('\n')) {
    if (line.trim() === '') continue;
    const envelope = JSON.parse(line) as Envelope;
    if (envelope.parseError) counts['parse error'] += 1;
    if (envelope.undefinedParameterType) counts['undefined parameter type'] += 1;
    const status = envelope.testStepFinished?.testStepResult.status;
    if (status === TestStepResultStatus.UNDEFINED) counts['undefined step'] += 1;
    if (status === TestStepResultStatus.AMBIGUOUS) counts['ambiguous step'] += 1;
  }
  return Object.entries(counts)
    .filter(([, count]) => count > 0)
    .map(([problem, count]) => `${count} ${problem}(s)`);
}

// On SIGTERM (timeout) or SIGINT (Ctrl+C), give the formatters a moment to flush what has
// already been recorded, so that the partial message stream remains available as evidence.
for (const signal of ['SIGTERM', 'SIGINT'] as const) {
  process.once(signal, () => {
    setTimeout(() => process.exit(WORKER_EXIT.terminated), SIGNAL_FLUSH_MS).unref();
  });
}

const job = JSON.parse(process.argv[2] ?? '{}') as WorkerJob;

main(job)
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
