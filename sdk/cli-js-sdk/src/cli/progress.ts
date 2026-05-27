import type { ProgressReporter } from "../shared/progress";

export const createProgressReporter = (): ProgressReporter => {
  const startedAt = performance.now();

  return (message: string) => {
    const elapsedSeconds = ((performance.now() - startedAt) / 1000).toFixed(1);
    process.stderr.write(`[${elapsedSeconds}s] ${message}\n`);
  };
};
