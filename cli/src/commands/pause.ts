import { defineCommand } from "citty";

import { getDotFhevmPaths, type DotFhevmPaths } from "../config/dotfhevm";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";
import { green } from "../utils/output";

import { runContractAction, type ContractActionDeps } from "./contract-ops";

export async function runPauseCommand(
  target: string,
  paths: DotFhevmPaths = getDotFhevmPaths(),
  deps?: ContractActionDeps,
): Promise<void> {
  await runContractAction(target, "pause", paths, deps);
}

export function toPauseError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "pause",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "pause",
    description: "Pause gateway or host contracts",
  },
  args: {
    target: { type: "positional", required: true, description: "gateway or host" },
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    const json = args.json ?? false;
    try {
      await runPauseCommand(args.target);
      if (json) {
        console.log(JSON.stringify({ ok: true, command: "pause", target: args.target }));
      } else {
        console.log(green(`Paused ${args.target} contracts.`));
      }
    } catch (error) {
      exitWithError(toPauseError(error), { json });
    }
  },
});
