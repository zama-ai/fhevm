import type { Command } from "@commander-js/extra-typings";

import { envFromCommand, parsePositiveInt } from "../shared";

export const registerSuiteCommands = (program: Command): void => {
  const suite = program.command("suite").description("Plan and run suites; preparation is explicit");

  suite.command("list").description("List built-in suites").action(async () => {
    const [{ BUILTIN_SUITES, createBuiltinSuite }, { logger }] = await Promise.all([
      import("../../suite/builtin"), import("../../shared/logger"),
    ]);
    for (const name of BUILTIN_SUITES) {
      const definition = createBuiltinSuite(name);
      logger.info(`${name}: ${definition.description}`);
      for (const entry of definition.entries) {
        logger.info(`  - ${entry.scenario}${Object.keys(entry.params).length > 0 ? ` ${JSON.stringify(entry.params)}` : ""}`);
      }
    }
  });

  suite.command("plan <ref>").description("Inspect pool requirements without pool mutation")
    .option("--check", "exit 2 when preparation work is required")
    .option("--out <dir>", "explicit directory for pool-plan.json/.md evidence")
    .action(async (ref, options, command) => {
      const env = await envFromCommand(command);
      const [{ loadSuite }, { resolveSuiteScenarios }, { inspectPoolRequirements, formatPoolPlan }, { logger }] = await Promise.all([
        import("../../suite/load"), import("../../suite/run"),
        import("../../pool/planning"), import("../../shared/logger"),
      ]);
      const definition = await loadSuite(ref);
      const resolved = await resolveSuiteScenarios(definition);
      const plan = await inspectPoolRequirements({
        env,
        scenarios: resolved.map((entry) => entry.scenario),
        pauseSec: definition.pauseSec,
        artifactDir: options.out as string | undefined,
      });
      for (const line of formatPoolPlan(plan)) logger.info(line);
      logger.info(plan.ready
        ? "Pools are ready."
        : "Run `suite prepare <ref>` or `suite run <ref> --prepare` after reviewing the plan.");
      if (options.check && !plan.ready) process.exitCode = 2;
    });

  suite.command("prepare <ref>").description("Prepare pools with local CPU/funded on-chain writes and persist evidence")
    .option("--out <dir>", "output root")
    .option("--skip-readiness", "skip GET /health/readiness before mutation")
    .option("--lanes <n>", "wallet lanes for handle creation", parsePositiveInt)
    .option("--connections <n>", "max sockets for the readiness gate", parsePositiveInt)
    .action(async (ref, options, command) => {
      const env = await envFromCommand(command);
      const [{ loadSuite }, { prepareSuite }, { logger }] = await Promise.all([
        import("../../suite/load"), import("../../suite/run"), import("../../shared/logger"),
      ]);
      const controller = new AbortController();
      const onSignal = (): void => { logger.warn("Interrupt received; stopping pool preparation."); controller.abort(); };
      process.once("SIGINT", onSignal); process.once("SIGTERM", onSignal);
      let result;
      try {
        result = await prepareSuite({
          env, suite: await loadSuite(ref), outputRoot: options.out as string | undefined,
          lanes: options.lanes as number | undefined,
          connections: options.connections as number | undefined,
          skipReadiness: Boolean(options.skipReadiness), signal: controller.signal,
        });
      } finally {
        process.removeListener("SIGINT", onSignal);
        process.removeListener("SIGTERM", onSignal);
      }
      if (result.status === "interrupted") process.exitCode = 130;
      else if (!result.ready) process.exitCode = 1;
      logger.info(`Suite preparation artifacts: ${result.outputRoot}`);
    });

  suite.command("run <ref>").description("Plan and run when ready; never prepare pools implicitly")
    .option("--out <dir>", "output root")
    .option("--baselines-dir <dir>", "baseline reports root", "baselines")
    .option("--prepare", "authorize local CPU and funded on-chain pool writes before execution")
    .option("--skip-readiness", "skip GET /health/readiness")
    .option("--lanes <n>", "wallet lanes for handle creation", parsePositiveInt)
    .option("--connections <n>", "max sockets toward the relayer", parsePositiveInt)
    .action(async (ref, options, command) => {
      if (options.lanes !== undefined && !options.prepare) {
        throw new Error("--lanes is only valid with --prepare for suite run.");
      }
      const env = await envFromCommand(command);
      const [{ loadSuite }, { runSuite }, { logger }] = await Promise.all([
        import("../../suite/load"), import("../../suite/run"), import("../../shared/logger"),
      ]);
      const controller = new AbortController();
      const onSignal = (): void => { logger.warn("Interrupt received; finishing the current run and stopping."); controller.abort(); };
      process.once("SIGINT", onSignal); process.once("SIGTERM", onSignal);
      let result;
      try {
        result = await runSuite({
          env, suite: await loadSuite(ref), outputRoot: options.out as string | undefined,
          baselinesDir: options.baselinesDir as string,
          prepare: Boolean(options.prepare),
          lanes: options.lanes as number | undefined, connections: options.connections as number | undefined,
          skipReadiness: Boolean(options.skipReadiness), signal: controller.signal,
        });
      } finally {
        process.removeListener("SIGINT", onSignal);
        process.removeListener("SIGTERM", onSignal);
      }
      if (result.status === "interrupted") process.exitCode = 130;
      else if (result.status === "blocked") process.exitCode = 2;
      else if (!result.passed) process.exitCode = 1;
      logger.info(`Suite artifacts: ${result.outputRoot}`);
    });
};
