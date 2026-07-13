import type { Command } from "@commander-js/extra-typings";

import { envFromCommand, parsePositiveInt } from "../shared";

export const registerSuiteCommands = (program: Command): void => {
  const suite = program.command("suite").description("Prepare pools and run grouped scenarios with one command");

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

  suite.command("plan <ref>").description("Show pool requirements and deficits without running")
    .action(async (ref, _options, command) => {
      const env = await envFromCommand(command);
      const [{ loadSuite }, { loadScenario }, { planPools }, { logger }] = await Promise.all([
        import("../../suite/load"), import("../../scenario/load"),
        import("../../suite/requirements"), import("../../shared/logger"),
      ]);
      const definition = await loadSuite(ref);
      const scenarios = await Promise.all(definition.entries.map((entry) => loadScenario(entry.scenario, entry.params)));
      const plan = await planPools(env, scenarios, { pauseSec: definition.pauseSec });
      let totalDeficit = 0;
      let refreshRequired = false;
      for (const item of plan) {
        totalDeficit += item.deficit;
        refreshRequired ||= item.refreshRequired ?? false;
        const status = item.deficit > 0
          ? `❌ needs ${item.deficit.toString()} more`
          : item.refreshRequired
            ? "❌ needs ACL refresh"
            : "✅ ready";
        logger.info(`${item.pool}: ${item.current.toString()} item(s) for ${item.needed.toString()} request(s) — ${status}`);
        logger.info(`  ${item.detail}`);
      }
      logger.info(totalDeficit === 0 && !refreshRequired
        ? "Pools are ready; `suite run` will skip preparation."
        : "Run `suite run <ref>` to prepare deficits/ACL refreshes and execute, or use `--prepare-only`.");
    });

  suite.command("run <ref>").description("Prepare pools, run every scenario, and summarize")
    .option("--out <dir>", "output root")
    .option("--baselines-dir <dir>", "baseline reports root", "baselines")
    .option("--prepare-only", "prepare pools and exit")
    .option("--skip-prepare", "fail instead of creating missing pool items")
    .option("--skip-readiness", "skip GET /health/readiness")
    .option("--lanes <n>", "wallet lanes for handle creation", parsePositiveInt)
    .option("--connections <n>", "max sockets toward the relayer", parsePositiveInt)
    .action(async (ref, options, command) => {
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
          prepareOnly: Boolean(options.prepareOnly), skipPrepare: Boolean(options.skipPrepare),
          lanes: options.lanes as number | undefined, connections: options.connections as number | undefined,
          skipReadiness: Boolean(options.skipReadiness), signal: controller.signal,
        });
      } finally {
        process.removeListener("SIGINT", onSignal);
        process.removeListener("SIGTERM", onSignal);
      }
      if (result.status === "interrupted") process.exitCode = 130;
      else if (!result.passed) process.exitCode = 1;
      logger.info(`Suite artifacts: ${result.outputRoot}`);
    });
};
