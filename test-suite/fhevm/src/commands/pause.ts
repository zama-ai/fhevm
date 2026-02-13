import type { Command } from "commander";
import { log } from "../log.js";
import { composeFile, PROJECT_NAME } from "../paths.js";
import { spawn } from "../executor.js";

const PAUSABLE = ["gateway", "host"] as const;
type Pausable = (typeof PAUSABLE)[number];

export function registerPauseCommand(program: Command): void {
  program
    .command("pause")
    .argument("<contracts>", `Contracts to pause (${PAUSABLE.join("|")})`)
    .description("Pause specific contracts")
    .action(async (contracts: string) => {
      if (!PAUSABLE.includes(contracts as Pausable)) {
        log.error(`Unknown contracts: ${contracts}`);
        log.error(`Valid options are: ${PAUSABLE.join(", ")}`);
        process.exit(1);
      }

      const pauseCompose = composeFile(`${contracts}-pause`);
      log.info(`Pausing ${contracts}...`);

      // Start the pause service
      const upProc = spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", pauseCompose, "up", "-d"],
        { stdio: ["inherit", "inherit", "inherit"] },
      );
      const upExit = await upProc.exited;
      if (upExit !== 0) {
        log.error(`Failed to start pause service for ${contracts}`);
        process.exit(upExit);
      }

      // Wait for the pause service to complete
      log.info("Waiting for pause operation to complete...");
      const waitProc = spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", pauseCompose, "wait", `${contracts}-sc-pause`],
        { stdio: ["inherit", "inherit", "inherit"] },
      );
      const waitExit = await waitProc.exited;
      if (waitExit !== 0) {
        log.error(`Pause operation failed for ${contracts}`);
        process.exit(waitExit);
      }

      log.success(`${contracts} paused successfully`);
    });
}
