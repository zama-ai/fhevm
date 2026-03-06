import { defineCommand } from "citty";

import { cleanDotFhevm, getDotFhevmPaths, type DotFhevmPaths } from "../config/dotfhevm";
import { stopAllServices } from "../docker/services";
import { ExitCode, FhevmCliError, exitWithError, type CliError } from "../errors";
import { dim, green, yellow } from "../utils/output";

export interface CleanOptions {
  all: boolean;
  dryRun: boolean;
}

export interface CleanFailure {
  phase: "docker" | "filesystem";
  error: Error;
}

export interface CleanCommandResult {
  removed: string[];
  failures: CleanFailure[];
}

export async function runCleanCommand(
  options: CleanOptions,
  paths: DotFhevmPaths = getDotFhevmPaths(),
): Promise<CleanCommandResult> {
  const failures: CleanFailure[] = [];

  if (!options.dryRun) {
    try {
      await stopAllServices({ volumes: true });
    } catch (error) {
      failures.push({ phase: "docker", error: toError(error) });
    }
  }

  let removed: string[] = [];
  try {
    removed = await cleanDotFhevm(paths, options);
  } catch (error) {
    failures.push({ phase: "filesystem", error: toError(error) });
  }

  return { removed, failures };
}

export function toCleanError(failures: CleanFailure[]): CliError {
  const firstTyped = failures
    .map((failure) => failure.error)
    .find((error): error is FhevmCliError => error instanceof FhevmCliError);

  if (firstTyped) {
    return firstTyped;
  }

  const hasDockerFailure = failures.some((failure) => failure.phase === "docker");
  const message = failures.map((failure) => `${failure.phase}: ${failure.error.message}`).join("; ");

  return {
    exitCode: hasDockerFailure ? ExitCode.DOCKER : ExitCode.GENERAL,
    step: "clean",
    message,
    cause: failures.map((failure) => failure.error),
  };
}

export default defineCommand({
  meta: {
    name: "clean",
    description: "Remove local stack artifacts",
  },
  args: {
    all: { type: "boolean", required: false, description: "Also remove cached keys" },
    "dry-run": { type: "boolean", required: false, description: "Preview what would be removed" },
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    const json = args.json ?? false;
    const options: CleanOptions = {
      all: args.all ?? false,
      dryRun: args["dry-run"] ?? false,
    };

    if (options.dryRun && !json) {
      console.log(dim("Would remove Docker containers and volumes for project 'fhevm'"));
    }

    const { removed, failures } = await runCleanCommand(options);

    if (!json && failures.some((failure) => failure.phase === "docker")) {
      const dockerFailure = failures.find((failure) => failure.phase === "docker");
      if (dockerFailure) {
        console.error(yellow(`Docker cleanup failed: ${dockerFailure.error.message}`));
        console.error(dim("Continuing with filesystem cleanup..."));
      }
    }

    if (failures.length > 0) {
      exitWithError(toCleanError(failures), { json });
    }

    if (json) {
      console.log(
        JSON.stringify({
          ok: true,
          command: "clean",
          dryRun: options.dryRun,
          all: options.all,
          removed,
        }),
      );
      return;
    }

    if (options.dryRun) {
      if (removed.length === 0) {
        console.log(dim("Nothing to remove."));
      } else {
        console.log(dim("Would remove:"));
        for (const path of removed) {
          console.log(dim(`  ${path}`));
        }
      }
      return;
    }

    console.log(green("Clean complete."));
  },
});

function toError(error: unknown): Error {
  return error instanceof Error ? error : new Error(String(error));
}
