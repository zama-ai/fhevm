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

export const runProgram = async (argv: readonly string[]): Promise<void> => {
  const program = createProgram();
  await program.parseAsync(argv as string[]).catch(async (error: unknown) => {
    const { logger } = await import("../shared/logger");
    logger.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  });
};
