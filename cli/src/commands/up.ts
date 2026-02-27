import { defineCommand } from "citty";

import { LOCAL_COMPONENT_MAP } from "../config/service-map";
import { validateLocalComponents } from "../docker/local";
import { ExitCode, FhevmCliError, exitWithError } from "../errors";
import { runBootPipeline, type BootOptions } from "../pipeline/boot-pipeline";

function parseIntegerArg(value: string | undefined, flagName: string): number | undefined {
  if (value === undefined) {
    return undefined;
  }

  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed)) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "up",
      message: `invalid integer for --${flagName}: ${value}`,
    });
  }

  return parsed;
}

function parseLocalComponents(value: string | undefined): string[] | undefined {
  if (!value) {
    return undefined;
  }

  return value
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean);
}

function hasLocalCoprocessorComponent(components: readonly string[]): boolean {
  return components.some((component) =>
    (LOCAL_COMPONENT_MAP[component] ?? []).some((serviceName) => serviceName.startsWith("coprocessor-")),
  );
}

export function validateUpOptions(options: BootOptions): void {
  if (options.resume && options.from) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "up",
      message: "--resume and --from are mutually exclusive",
    });
  }

  if (options.local?.length) {
    validateLocalComponents(options.local);
  }

  const numCopro = options.numCoprocessors ?? 1;
  if (typeof options.threshold === "number" && options.threshold > numCopro) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "up",
      message: `--threshold (${options.threshold}) cannot exceed --coprocessors (${numCopro})`,
    });
  }

  if (numCopro <= 1 || !options.local?.length) {
    return;
  }

  if (hasLocalCoprocessorComponent(options.local)) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "up",
      message: "--local for coprocessor services is not supported with --coprocessors > 1",
    });
  }
}

export default defineCommand({
  meta: {
    name: "up",
    description: "Start the fhEVM local stack",
  },
  args: {
    local: { type: "string", required: false, description: "Run component(s) natively" },
    coprocessors: { type: "string", required: false, description: "Number of coprocessors" },
    threshold: { type: "string", required: false, description: "Threshold for N/T mode" },
    "no-cache": { type: "boolean", required: false, description: "Disable BuildKit cache" },
    json: { type: "boolean", required: false, description: "JSON output" },
    resume: { type: "boolean", required: false, description: "Resume from last failed step" },
    from: { type: "string", required: false, description: "Start from a specific step" },
  },
  async run({ args }) {
    const json = args.json ?? false;
    try {
      const options: BootOptions = {
        local: parseLocalComponents(args.local),
        numCoprocessors: parseIntegerArg(args.coprocessors, "coprocessors"),
        threshold: parseIntegerArg(args.threshold, "threshold"),
        noCache: args["no-cache"] ?? false,
        json,
        resume: args.resume ?? false,
        from: args.from,
      };

      validateUpOptions(options);

      await runBootPipeline(options);
    } catch (error) {
      if (error instanceof FhevmCliError) {
        exitWithError(error, { json });
      }

      exitWithError({
        exitCode: ExitCode.GENERAL,
        step: "up",
        message: error instanceof Error ? error.message : String(error),
        cause: error,
      }, { json });
    }
  },
});
