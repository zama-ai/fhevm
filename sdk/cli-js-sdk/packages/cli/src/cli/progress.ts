import { createConsola } from "consola";

import type { ProgressReporter } from "@cli-fhevm-sdk/toolkit/shared/progress";

const progressLogger = createConsola({
  stdout: process.stderr,
  stderr: process.stderr,
});

export const createProgressReporter = (): ProgressReporter => {
  return (message: string) => {
    progressLogger.start(message);
  };
};
