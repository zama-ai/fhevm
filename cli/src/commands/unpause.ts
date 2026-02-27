import { defineCommand } from "citty";

import { getDotFhevmPaths, type DotFhevmPaths } from "../config/dotfhevm";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";
import { green } from "../utils/output";

import { runContractAction, type ContractActionDeps } from "./contract-ops";

export async function runUnpauseCommand(
  target: string,
  paths: DotFhevmPaths = getDotFhevmPaths(),
  deps?: ContractActionDeps,
): Promise<void> {
  await runContractAction(target, "unpause", paths, deps);
}

export function toUnpauseError(error: unknown): CliError {
  if (error instanceof FhevmCliError) {
    return error;
  }

  return {
    exitCode: ExitCode.DOCKER,
    step: "unpause",
    message: error instanceof Error ? error.message : String(error),
    cause: error,
  };
}

export default defineCommand({
  meta: {
    name: "unpause",
    description: "Unpause gateway or host contracts",
  },
  args: {
    target: { type: "positional", required: true, description: "gateway or host" },
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    const json = args.json ?? false;
    try {
      await runUnpauseCommand(args.target);
      if (json) {
        console.log(JSON.stringify({ ok: true, command: "unpause", target: args.target }));
      } else {
        console.log(green(`Unpaused ${args.target} contracts.`));
      }
    } catch (error) {
      exitWithError(toUnpauseError(error), { json });
    }
  },
});
