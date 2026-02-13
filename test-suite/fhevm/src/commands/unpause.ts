import type { Command } from "commander";
import { log } from "../log.js";
import { composeFile, PROJECT_NAME } from "../paths.js";
import { spawn } from "../executor.js";

const PAUSABLE = ["gateway", "host"] as const;
type Pausable = (typeof PAUSABLE)[number];

export function registerUnpauseCommand(program: Command): void {
  program
    .command("unpause")
    .argument("<contracts>", `Contracts to unpause (${PAUSABLE.join("|")})`)
    .description("Unpause specific contracts")
    .action(async (contracts: string) => {
      if (!PAUSABLE.includes(contracts as Pausable)) {
        log.error(`Unknown contracts: ${contracts}`);
        log.error(`Valid options are: ${PAUSABLE.join(", ")}`);
        process.exit(1);
      }

      const unpauseCompose = composeFile(`${contracts}-unpause`);
      log.info(`Unpausing ${contracts}...`);

      // Start the unpause service
      const upProc = spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", unpauseCompose, "up", "-d"],
        { stdio: ["inherit", "inherit", "inherit"] },
      );
      const upExit = await upProc.exited;
      if (upExit !== 0) {
        log.error(`Failed to start unpause service for ${contracts}`);
        process.exit(upExit);
      }

      // Wait for the unpause service to complete
      log.info("Waiting for unpause operation to complete...");
      const waitProc = spawn(
        ["docker", "compose", "-p", PROJECT_NAME, "-f", unpauseCompose, "wait", `${contracts}-sc-unpause`],
        { stdio: ["inherit", "inherit", "inherit"] },
      );
      const waitExit = await waitProc.exited;
      if (waitExit !== 0) {
        log.error(`Unpause operation failed for ${contracts}`);
        process.exit(waitExit);
      }

      log.success(`${contracts} unpaused successfully`);
    });
}
