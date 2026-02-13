import type { Command } from "commander";
import { log } from "../log.js";
import { composeUp } from "../docker.js";
import { exportVersions } from "../env.js";
import { STEPS, STEP_IDS, type StepId } from "../dag.js";

export function registerRestartCommand(program: Command): void {
  program
    .command("restart")
    .argument("<service>", `Service to restart (${STEP_IDS.filter((s) => s !== "kms-signer").join("|")})`)
    .description("Restart a service with version-env resolution")
    .option("--build", "Rebuild images before starting")
    .action(async (service: string, opts) => {
      const step = STEPS.find((s) => s.id === service);
      if (!step || !step.compose) {
        log.error(`Unknown or non-restartable service: ${service}`);
        log.error(`Valid services are: ${STEP_IDS.filter((s) => s !== "kms-signer").join(", ")}`);
        process.exit(1);
      }

      exportVersions();

      log.info(`Restarting ${step.label}...`);
      await composeUp({
        component: step.compose,
        build: step.supportsBuild && opts.build,
      });
      log.success(`${step.label} restarted successfully`);
    });
}
