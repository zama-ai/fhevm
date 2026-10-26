import type { WorkerJob } from './types.js';

/**
 * Holds the job of the current worker process. The worker stores it before Cucumber loads any
 * support code, so that engine support code (imported by Cucumber at load time, before any World
 * exists) can read job-level settings such as the default step timeout. Cucumber `import()`s the
 * same file URL, so both sides share this module instance.
 */
let currentJob: WorkerJob | undefined;

export function setCurrentJob(job: WorkerJob): void {
  currentJob = job;
}

export function getCurrentJob(): WorkerJob | undefined {
  return currentJob;
}
