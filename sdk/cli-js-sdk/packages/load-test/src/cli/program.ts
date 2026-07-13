import "./env-file";

import { DEFAULT_NETWORK, NETWORKS } from "@cli-fhevm-sdk/toolkit/types";
import { Command, Option } from "@commander-js/extra-typings";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { registerPoolCommands } from "./commands/pool";
import { registerRunCommand } from "./commands/run";
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
    .version(packageVersion)
    .addOption(
      new Option("-n, --network <network>", `network to target (default: ${DEFAULT_NETWORK})`).choices(NETWORKS),
    )
    .option("--relayer-url <url>", "relayer base URL override")
    .option("--relayer-api-prefix <prefix>", "primary relayer API route prefix (raw flows only)")
    .option("--relayer-b <url>", "candidate relayer base URL for paired dispatch")
    .option("--relayer-b-api-prefix <prefix>", "candidate API route prefix (raw flows only)")
    .option("--rpc-url <url>", "host chain RPC URL override")
    .option("--data-dir <dir>", "pools and run artifacts root (default .load-test)")
    .option("--relayer-config <path>", "primary relayer config file to snapshot")
    .option("--relayer-b-config <path>", "candidate relayer config file to snapshot");

  const command = program as unknown as Command;
  registerPoolCommands(command);
  registerScenarioCommands(command);
  registerSuiteCommands(command);
  registerRunCommand(command);
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
