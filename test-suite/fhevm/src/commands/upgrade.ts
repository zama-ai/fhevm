import type { Command } from "commander";
import { log } from "../log.js";
import { localEnvFile, composeFile, PROJECT_NAME } from "../paths.js";
import { spawn } from "../executor.js";

const UPGRADEABLE_SERVICES = [
  "minio", "core", "gateway-node", "gateway-sc", "gateway-mocked-payment",
  "host-node", "host-sc", "kms-connector", "coprocessor", "relayer", "test-suite",
];

export function registerUpgradeCommand(program: Command): void {
  program
    .command("upgrade")
    .argument("<service>", `Service to upgrade (${UPGRADEABLE_SERVICES.join("|")})`)
    .description("Upgrade a specific service")
    .action(async (service: string) => {
      if (!UPGRADEABLE_SERVICES.includes(service)) {
        log.error(`Unknown service: ${service}`);
        log.error(`Valid services are: ${UPGRADEABLE_SERVICES.join(", ")}`);
        process.exit(1);
      }

      log.info(`Upgrading ${service}...`);

      const proc = spawn(
        [
          "docker", "compose",
          "-p", PROJECT_NAME,
          "--env-file", localEnvFile(service),
          "-f", composeFile(service),
          "up", "-d",
        ],
        { stdio: ["inherit", "inherit", "inherit"] },
      );

      const exitCode = await proc.exited;
      if (exitCode !== 0) {
        log.error(`Failed to upgrade ${service}`);
        process.exit(exitCode);
      }

      log.success(`${service} upgraded successfully`);
    });
}
