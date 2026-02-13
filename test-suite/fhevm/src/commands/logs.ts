import type { Command } from "commander";
import { log } from "../log.js";
import { spawn } from "../executor.js";

export function registerLogsCommand(program: Command): void {
  program
    .command("logs")
    .argument("<service>", "Service/container name to show logs for")
    .description("View logs for a specific service")
    .option("-f, --follow", "Follow log output")
    .action(async (service: string, opts) => {
      log.info(`Showing logs for ${service}...`);

      const args = ["docker", "logs"];
      if (opts.follow) args.push("-f");
      args.push(service);

      const proc = spawn(args, {
        stdio: ["inherit", "inherit", "inherit"],
      });
      const exitCode = await proc.exited;
      if (exitCode !== 0) {
        process.exit(exitCode);
      }
    });
}
