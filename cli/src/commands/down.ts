import { defineCommand } from "citty";

import { stopAllServices } from "../docker/services";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";
import { green } from "../utils/output";

export async function runDownCommand(): Promise<void> {
  await stopAllServices();
}

export function toDownError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "down",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "down",
    description: "Stop the fhEVM local stack",
  },
  args: {
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    const json = args.json ?? false;
    try {
      await runDownCommand();
      if (json) {
        console.log(JSON.stringify({ ok: true, command: "down" }));
      } else {
        console.log(green("Stack stopped."));
      }
    } catch (error) {
      exitWithError(toDownError(error), { json });
    }
  },
});
