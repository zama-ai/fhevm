import type { Command } from "commander";
import { log } from "../log.js";
import { composeDownAll } from "../docker.js";
import { PROJECT_NAME } from "../paths.js";
import { spawn } from "../executor.js";

export function registerCleanCommand(program: Command): void {
  program
    .command("clean")
    .description("Remove FHEVM stack containers and volumes")
    .option("--soft", "Remove containers only (keep volumes)")
    .option("--hard", "Remove containers, volumes, orphans, and sweep leftovers")
    .action(async (opts) => {
      if (opts.hard) {
        log.info("Cleaning up FHEVM stack (hard)...");

        // Take down compose project with volumes
        await composeDownAll({ volumes: true, removeOrphans: true });

        // Sweep orphan containers
        log.info("Sweeping orphan containers...");
        const psProc = spawn(
          ["docker", "ps", "-a", "--filter", "name=fhevm", "-q"],
          { stdout: "pipe", stderr: "pipe" },
        );
        const containerIds = (await new Response(psProc.stdout!).text()).trim();
        await psProc.exited;

        if (containerIds) {
          const ids = containerIds.split("\n").filter(Boolean);
          const rmProc = spawn(
            ["docker", "rm", "-f", ...ids],
            { stdio: ["inherit", "inherit", "inherit"] },
          );
          await rmProc.exited;
        }

        // Sweep orphan networks
        log.info("Sweeping orphan networks...");
        const netProc = spawn(
          ["docker", "network", "ls", "--filter", `label=com.docker.compose.project=${PROJECT_NAME}`, "-q"],
          { stdout: "pipe", stderr: "pipe" },
        );
        const networkIds = (await new Response(netProc.stdout!).text()).trim();
        await netProc.exited;

        if (networkIds) {
          const ids = networkIds.split("\n").filter(Boolean);
          const rmNetProc = spawn(
            ["docker", "network", "rm", ...ids],
            { stdio: ["inherit", "inherit", "inherit"] },
          );
          await rmNetProc.exited;
        }

        log.success("FHEVM stack cleaned (hard) successfully");
      } else if (opts.soft) {
        log.info("Cleaning up FHEVM stack (soft)...");
        await composeDownAll({ removeOrphans: true });
        log.success("FHEVM stack cleaned (soft) successfully");
      } else {
        // Default: same as the original clean — volumes + orphans
        log.info("Cleaning up FHEVM stack...");
        await composeDownAll({ volumes: true, removeOrphans: true });
        log.success("FHEVM stack cleaned successfully");
      }
    });
}
