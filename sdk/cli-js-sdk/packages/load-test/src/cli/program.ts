import "./env-file";

import { Command } from "@commander-js/extra-typings";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { registerPoolCommands } from "./commands/pool";
import { registerScenarioCommands } from "./commands/scenarios";
import { registerSuiteCommands } from "./commands/suite";
import { registerReportCommands } from "./commands/report";
import { registerBaselineCommands } from "./commands/baseline";

const packageJsonPath = fileURLToPath(new URL("../../package.json", import.meta.url));
const packageVersion = (
  JSON.parse(readFileSync(packageJsonPath, "utf8")) as { version?: string }
).version ?? "0.0.0";

export const createProgram = (): Command => {
  const program = new Command()
    .name("load-test")
    .description("FHEVM relayer load-test tool for legacy and v2 implementations")
    .version(packageVersion);

  const command = program as unknown as Command;
  registerPoolCommands(command);
  registerScenarioCommands(command);
  registerSuiteCommands(command);
  registerReportCommands(command);
  registerBaselineCommands(command);
  return command;
};

/**
 * Flattens an error into printable lines, following AggregateError.errors and
 * Error.cause chains so compound failures surface every underlying message.
 */
export const describeError = (error: unknown, depth = 0): string[] => {
  const indent = "  ".repeat(depth);
  if (!(error instanceof Error)) return [`${indent}${String(error)}`];
  const lines = [`${indent}${error.message}`];
  if (error instanceof AggregateError) {
    for (const inner of error.errors) lines.push(...describeError(inner, depth + 1));
  }
  if (error.cause !== undefined) {
    lines.push(`${indent}  caused by:`);
    lines.push(...describeError(error.cause, depth + 2));
  }
  return lines;
};

export const runProgram = async (argv: readonly string[]): Promise<void> => {
  const program = createProgram();
  await program.parseAsync(argv as string[]).catch(async (error: unknown) => {
    const { logger } = await import("../shared/logger");
    logger.error(describeError(error).join("\n"));
    process.exitCode = 1;
  });
};
