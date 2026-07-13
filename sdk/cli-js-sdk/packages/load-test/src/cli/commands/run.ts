import type { Command } from "@commander-js/extra-typings";

import {
  envFromCommand,
  parseNonNegativeInt,
  parsePositiveInt,
  parsePositiveNumber,
  readReport,
} from "../shared";

export const registerRunCommand = (program: Command): void => {
  program.command("run <scenario>").description("Run a scenario (built-in name or scenario JSON path)")
    .option("--rps <n>", "rate override", parsePositiveNumber)
    .option("--vus <n>", "active client override", parsePositiveInt)
    .option("--think-time <ms>", "pause between closed-model iterations", parseNonNegativeInt)
    .option("--duration <seconds>", "duration override", parsePositiveInt)
    .option("--count <n>", "request count override", parsePositiveInt)
    .option("--max-iterations <n>", "closed-model request cap", parsePositiveInt)
    .option("--flow <flow>", "flow override")
    .option("--out <dir>", "output directory override")
    .option("--connections <n>", "max sockets toward the relayer", parsePositiveInt)
    .option("--baseline <path>", "baseline report.json")
    .option("--skip-readiness", "skip GET /health/readiness")
    .action(async (scenarioRef, options, command) => {
      const env = await envFromCommand(command);
      const [{ loadScenario }, { executeRun, RunInterruptedError }, { diffReports }, { logger }] = await Promise.all([
        import("../../scenario/load"), import("../../runner/run"),
        import("../../report/diff"), import("../../shared/logger"),
      ]);
      const scenario = await loadScenario(scenarioRef, {
        rps: options.rps as number | undefined, vus: options.vus as number | undefined,
        thinkTimeMs: options.thinkTime as number | undefined, durationSec: options.duration as number | undefined,
        count: options.count as number | undefined, maxIterations: options.maxIterations as number | undefined,
        flow: options.flow as never,
      });
      const controller = new AbortController();
      const onSignal = (): void => { logger.warn("Interrupt received; stopping submission and draining."); controller.abort(); };
      process.once("SIGINT", onSignal); process.once("SIGTERM", onSignal);
      let runResult;
      try {
        runResult = await executeRun({
          scenario, env, outputDir: options.out as string | undefined,
          connections: options.connections as number | undefined,
          skipReadiness: Boolean(options.skipReadiness), signal: controller.signal,
        });
      } catch (error) {
        if (error instanceof RunInterruptedError) {
          process.exitCode = 130;
          logger.warn("Run interrupted before a report could be completed.");
          return;
        }
        throw error;
      } finally {
        process.removeListener("SIGINT", onSignal);
        process.removeListener("SIGTERM", onSignal);
      }
      const { report, outputDir } = runResult;
      if (runResult.status === "interrupted" || controller.signal.aborted) {
        process.exitCode = 130;
        logger.warn(`Run interrupted; partial artifacts: ${outputDir}`);
        return;
      }
      if (options.baseline) {
        const diff = diffReports(await readReport(options.baseline as string), report);
        for (const note of diff.notes) logger.warn(note);
        for (const regression of diff.regressions) logger.error(`Regression: ${regression.flow} ${regression.metric} ${regression.baseline.toString()} -> ${regression.current.toString()}`);
        if (!diff.passed) process.exitCode = 1;
      }
      if (!report.thresholds.passed) process.exitCode = 1;
      logger.info(`Artifacts: ${outputDir}`);
    });
};
