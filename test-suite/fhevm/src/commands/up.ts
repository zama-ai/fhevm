import type { Command } from "commander";
import { log } from "../log.js";
import { composeUp, composeDown, composeDownAll, waitForService, isContainerRunning } from "../docker.js";
import { prepareAllEnvFiles, exportVersions, logVersions } from "../env.js";
import { computeCacheEnvVars } from "../cache.js";
import { STEPS, STEP_IDS, filterSteps, stepsToCleanup, validateDag, getStep, type StepId } from "../dag.js";
import { setDryRun, getRecordedCommands } from "../executor.js";

export function registerUpCommand(program: Command): void {
  program
    .command("up")
    .aliases(["deploy"])
    .description("Deploy the full FHEVM stack")
    .option("--build", "Rebuild images before starting")
    .option("--local", "Enable local BuildKit cache optimizations")
    .option("--dev", "Alias for --local")
    .option("--resume <step>", `Resume from a specific step (${STEP_IDS.join("|")})`)
    .option("--only <step>", `Deploy only a specific step (${STEP_IDS.join("|")})`)
    .option("--dry-run", "Print commands that would be executed without running them")
    .action(async (opts) => {
      const build: boolean = opts.build ?? false;
      const local: boolean = opts.local ?? opts.dev ?? false;
      const dryRun: boolean = opts.dryRun ?? false;
      const resume: StepId | undefined = opts.resume;
      const only: StepId | undefined = opts.only;

      if (dryRun) {
        setDryRun(true);
      }

      // Validate flags
      if (resume && only) {
        log.error("Cannot use --resume and --only together");
        process.exit(1);
      }

      if (resume && !STEP_IDS.includes(resume)) {
        log.error(`Invalid resume step: ${resume}`);
        log.error(`Valid steps are: ${STEP_IDS.join(" ")}`);
        process.exit(1);
      }

      if (only && !STEP_IDS.includes(only)) {
        log.error(`Invalid step: ${only}`);
        log.error(`Valid steps are: ${STEP_IDS.join(" ")}`);
        process.exit(1);
      }

      // Validate DAG integrity
      validateDag();

      // Resolve and export all versions
      const versions = exportVersions();
      const buildTag = build ? " (local build)" : "";
      logVersions(buildTag);

      if (build) {
        log.info("Force build option detected. Services will be rebuilt.");
      }

      // Prepare all env files (skip in dry-run — needs real filesystem)
      if (!dryRun) {
        await prepareAllEnvFiles();
      } else {
        log.info("[dry-run] Skipping env file preparation");
      }

      // Set up local cache vars if --local
      if (local) {
        log.info("Enabling local BuildKit cache and disabling provenance attestations.");
        if (!dryRun) {
          const cacheEnv = computeCacheEnvVars();
          Object.assign(process.env, cacheEnv);
        } else {
          log.info("[dry-run] Would compute and set BuildKit cache env vars");
        }
      }

      // Cleanup phase
      if (only) {
        log.warn(`Only mode: cleaning up '${only}' services...`);
      } else if (resume) {
        log.warn(`Resume mode: cleaning up services from '${resume}' onwards...`);
      } else {
        log.warn("Setup new environment, cleaning up...");
      }

      if (!resume && !only) {
        // Full cleanup
        await composeDownAll({ volumes: true, removeOrphans: true });
      } else {
        // Selective cleanup
        const cleanupSteps = stepsToCleanup({ resume, only });
        for (const step of cleanupSteps) {
          if (step.compose) {
            log.info(`Stopping ${step.compose} services...`);
            await composeDown({
              component: step.compose,
              volumes: true,
              removeOrphans: true,
            });
          }
        }
        if (resume) {
          log.info(`Cleanup complete. Services before '${resume}' preserved.`);
        } else if (only) {
          log.info(`Cleanup complete. Only '${only}' was cleaned.`);
        }
      }

      // Execution phase
      const steps = filterSteps({ resume, only });
      const ctx = { build, local };

      for (const step of steps) {
        log.step(step.id, `Deploying ${step.label}...`);

        // Compose up (if the step has a compose file)
        if (step.compose) {
          await composeUp({
            component: step.compose,
            build: step.supportsBuild && build,
          });
        }

        // Wait for all services in this step
        for (const svc of step.services) {
          await waitForService(svc.container, svc.expect);
        }

        // Run post-deploy hook if present
        if (step.postDeploy) {
          await step.postDeploy(ctx);
        }

        log.success(`${step.label} deployed successfully`);
      }

      // Special case: when --resume skips minio but minio container is running,
      // still run minio's postDeploy to patch the coprocessor env with the current IP
      if (resume && resume !== "minio") {
        const minioRunning = await isContainerRunning("fhevm-minio");
        if (minioRunning) {
          log.info("Skipping step: minio (resuming from " + resume + ")");
          const minioStep = getStep("minio");
          if (minioStep?.postDeploy) {
            await minioStep.postDeploy(ctx);
          }
        }
      }

      log.info("All services started successfully!");
      log.success("FHEVM stack deployment complete!");

      // In dry-run mode, also dump the full trace as structured JSON to stdout
      if (dryRun) {
        const commands = getRecordedCommands();
        console.log("\n--- DRY-RUN COMMAND TRACE (JSON) ---");
        console.log(JSON.stringify(commands, null, 2));
        setDryRun(false);
      }
    });
}
