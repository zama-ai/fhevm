import { Option, type CommandUnknownOpts } from "@commander-js/extra-typings";
import { join } from "node:path";

import { runsDir } from "../../env";
import { FLOWS } from "../../relayer/types";
import type { ScenarioOverrides } from "../../scenario/overrides";
import type { Scenario } from "../../scenario/schema";
import { isoNow } from "../../shared/time";
import {
  emitJson,
  envFromCommand,
  parseBoundedInt,
  parseNonNegativeInt,
  parsePositiveInt,
  parsePositiveNumber,
  readReport,
  useJsonOutput,
  withEnvOptions,
  withFormatOption,
} from "../shared";

const MAX_CONNECTIONS = 1024;
const MAX_LANES = 64;

type ScenarioCommandOptions = Readonly<{
  rps?: number;
  vus?: number;
  thinkTime?: number;
  duration?: number;
  count?: number;
  maxIterations?: number;
  flow?: string;
}>;

type ScenarioPlanOptions = ScenarioCommandOptions & Readonly<{
  out?: string;
  check?: boolean;
}>;

type ScenarioRunOptions = ScenarioCommandOptions & Readonly<{
  out?: string;
  connections?: number;
  baseline?: string;
  skipReadiness?: boolean;
  prepare?: boolean;
  lanes?: number;
}>;

type ScenarioPrepareOptions = ScenarioCommandOptions & Readonly<{
  out?: string;
  connections?: number;
  skipReadiness?: boolean;
  lanes?: number;
}>;

type CliLogger = Readonly<{
  error(message: unknown): void;
  info(message: string): void;
  success(message: string): void;
  warn(message: string): void;
}>;

const scenarioOverrides = (options: ScenarioCommandOptions): ScenarioOverrides => ({
  ...(options.rps !== undefined ? { rps: options.rps } : {}),
  ...(options.vus !== undefined ? { vus: options.vus } : {}),
  ...(options.thinkTime !== undefined ? { thinkTimeMs: options.thinkTime } : {}),
  ...(options.duration !== undefined ? { durationSec: options.duration } : {}),
  ...(options.count !== undefined ? { count: options.count } : {}),
  ...(options.maxIterations !== undefined ? { maxIterations: options.maxIterations } : {}),
  ...(options.flow !== undefined ? { flow: options.flow as ScenarioOverrides["flow"] } : {}),
});

/** Adds the single shared set of model-aware scenario overrides. */
export const addScenarioOverrideOptions = <T extends CommandUnknownOpts>(command: T): T => {
  command
    .option("--rps <n>", "constant/burst rate; scales segmented rates", parsePositiveNumber)
    .option("--vus <n>", "steady VUs; scales staged VUs", parsePositiveInt)
    .option("--think-time <ms>", "closed-model pause after each workflow", parseNonNegativeInt)
    .option("--duration <seconds>", "steady or per-segment/stage duration", parsePositiveNumber)
    .option("--count <n>", "burst request count", parsePositiveInt)
    .option("--max-iterations <n>", "closed-model request cap", parsePositiveInt)
    .addOption(new Option("--flow <flow>", "single-flow override").choices([...FLOWS]));
  return command;
};

const timestampedDirectory = (
  root: string,
  kind: "scenario" | "run",
  scenario: Scenario,
): string =>
  join(
    root,
    `${isoNow().replace(/[:.]/g, "-")}-${kind === "scenario" ? "scenario-" : ""}${scenario.name}`,
  );

const isInterruption = (error: unknown, signal: AbortSignal): boolean =>
  signal.aborted || (error instanceof Error && (
    error.name === "AbortError" || error.name === "RunInterruptedError"
  ));

const withSignals = async <T>(
  logger: CliLogger,
  message: string,
  action: (signal: AbortSignal) => Promise<T>,
): Promise<T | undefined> => {
  const controller = new AbortController();
  const onSignal = (): void => {
    logger.warn(message);
    controller.abort();
  };
  process.once("SIGINT", onSignal);
  process.once("SIGTERM", onSignal);
  try {
    return await action(controller.signal);
  } catch (error) {
    if (isInterruption(error, controller.signal)) {
      process.exitCode = 130;
      logger.warn("Scenario operation interrupted.");
      return undefined;
    }
    throw error;
  } finally {
    process.removeListener("SIGINT", onSignal);
    process.removeListener("SIGTERM", onSignal);
  }
};

const loadResolvedScenario = async (
  ref: string,
  options: ScenarioCommandOptions,
): Promise<Scenario> => {
  const [{ loadScenario }, { ceilingWarnings }, { logger }] = await Promise.all([
    import("../../scenario/load"),
    import("../../scenario/limits"),
    import("../../shared/logger"),
  ]);
  const scenario = await loadScenario(ref, scenarioOverrides(options));
  for (const warning of ceilingWarnings(scenario)) logger.warn(warning);
  return scenario;
};

const runScenarioAction = async (
  scenarioRef: string,
  options: ScenarioRunOptions,
  command: CommandUnknownOpts,
): Promise<void> => {
  if (options.lanes !== undefined && !options.prepare) {
    throw new Error("--lanes is only valid with --prepare for scenario runs.");
  }
  const env = await envFromCommand(command);
  const [{ formatPoolPlan, inspectPoolRequirements, preparePoolRequirements }, { logger }] = await Promise.all([
    import("../../pool/planning"),
    import("../../shared/logger"),
  ]);
  const scenario = await loadResolvedScenario(scenarioRef, options);

  await withSignals(
    logger,
    "Interrupt received; stopping preparation or submission and draining.",
    async (signal) => {
      const outputDir = options.out ?? timestampedDirectory(runsDir(env), "run", scenario);
      const initialPlan = await inspectPoolRequirements({
        env,
        scenarios: [scenario],
        artifactDir: outputDir,
      });
      for (const line of formatPoolPlan(initialPlan)) logger.info(line);
      signal.throwIfAborted();

      if (!initialPlan.ready && !options.prepare) {
        logger.error(
          "Pool preparation is required. Re-run with --prepare to authorize the listed " +
            "local/on-chain work, or use `scenario prepare` first.",
        );
        process.exitCode = 2;
        return;
      }

      if (initialPlan.ready && options.lanes !== undefined) {
        logger.warn(
          "--lanes has no effect: pools are already ready, so no preparation is needed.",
        );
      }

      if (!initialPlan.ready && options.prepare) {
        // preparePoolRequirements throws when the re-inspected live plan is
        // not ready, so a returning call is always a ready plan.
        await preparePoolRequirements({
          env,
          scenarios: [scenario],
          artifactDir: outputDir,
          lanes: options.lanes,
          signal,
          onProgress: (message) => logger.info(message),
          beforeActions: async () => {
            const { assertRelayerReadiness } = await import("../../runner/readiness");
            await assertRelayerReadiness({
              env,
              connections: options.connections,
              skipReadiness: Boolean(options.skipReadiness),
              signal,
            });
          },
        });
      }

      signal.throwIfAborted();
      const [{ executeRun }, { diffReports }] = await Promise.all([
        import("../../runner/run"),
        import("../../report/diff"),
      ]);
      const runResult = await executeRun({
        scenario,
        env,
        outputDir,
        connections: options.connections,
        skipReadiness: Boolean(options.skipReadiness),
        signal,
      });
      const { report } = runResult;
      if (runResult.status === "interrupted" || signal.aborted) {
        process.exitCode = 130;
        logger.warn(`Run interrupted; partial artifacts: ${runResult.outputDir}`);
        return;
      }
      if (options.baseline) {
        const diff = diffReports(await readReport(options.baseline), report);
        for (const note of diff.notes) logger.warn(note);
        for (const regression of diff.regressions) {
          logger.error(
            `Regression: ${regression.flow} ${regression.metric} ` +
              `${regression.baseline.toString()} -> ${regression.current.toString()}`,
          );
        }
        if (!diff.passed) process.exitCode = 1;
      }
      if (!report.thresholds.passed) process.exitCode = 1;
      logger.info(`Artifacts: ${runResult.outputDir}`);
    },
  );
};

/** Registers the canonical `scenario run` action. */
export const registerScenarioRunCommand = (parent: CommandUnknownOpts): void => {
  const command = withEnvOptions(addScenarioOverrideOptions(
    parent.command("run <scenario>").description("Run a scenario (built-in name or scenario JSON path)"),
  ))
    .option("--out <dir>", "output directory override")
    .option("--connections <n>", "max sockets toward the relayer", parseBoundedInt("--connections", MAX_CONNECTIONS))
    .option("--baseline <path>", "baseline report.json")
    .option("--skip-readiness", "skip GET /health/readiness")
    .option(
      "--prepare",
      "explicitly create missing pools first (may use local CPU and send funded on-chain transactions)",
    )
    .option("--lanes <n>", "funded wallet lanes for on-chain handle creation", parseBoundedInt("--lanes", MAX_LANES));
  command.action((scenarioRef, options, actionCommand) =>
    runScenarioAction(scenarioRef, options as ScenarioRunOptions, actionCommand),
  );
};

export const registerScenarioCommands = (program: CommandUnknownOpts): void => {
  const scenarios = program.command("scenario").description("Inspect, plan, prepare, and run scenarios");

  withFormatOption(scenarios.command("list").description("List built-in scenarios"))
    .action(async (options) => {
      const json = await useJsonOutput(options);
      const { BUILTIN_SCENARIOS, createBuiltinScenario } = await import("../../scenario/builtin");
      const entries = BUILTIN_SCENARIOS.map((name) => ({
        name,
        description: createBuiltinScenario(name).description,
      }));
      if (json) {
        emitJson(entries);
        return;
      }
      const { logger } = await import("../../shared/logger");
      for (const entry of entries) logger.info(`${entry.name}: ${entry.description}`);
    });

  const show = addScenarioOverrideOptions(
    scenarios.command("show <ref>")
      .description("Print resolved scenario JSON (built-in name or file path)"),
  );
  show.action(async (ref, options) => {
    console.log(JSON.stringify(await loadResolvedScenario(ref, options), null, 2));
  });

  const plan = withFormatOption(withEnvOptions(addScenarioOverrideOptions(
    scenarios.command("plan <ref>")
      .description("Inspect pool requirements without creating payloads, handles, or ACL grants"),
  )))
    .option("--check", "exit 2 when pool preparation is required")
    .option("--out <dir>", "explicit directory for pool-plan.json/.md evidence");
  plan.action(async (ref, options, command) => {
    const json = await useJsonOutput(options);
    const env = await envFromCommand(command);
    const [{ formatPoolPlan, inspectPoolRequirements }, { logger }] = await Promise.all([
      import("../../pool/planning"), import("../../shared/logger"),
    ]);
    const planOptions = options as ScenarioPlanOptions;
    const resolved = await loadResolvedScenario(ref, planOptions);
    const result = await inspectPoolRequirements({
      env,
      scenarios: [resolved],
      artifactDir: planOptions.out,
    });
    if (json) emitJson(result);
    else for (const line of formatPoolPlan(result)) logger.info(line);
    if (options.check && !result.ready) process.exitCode = 2;
  });

  const prepare = withEnvOptions(addScenarioOverrideOptions(
    scenarios.command("prepare <ref>")
      .description("Create missing pools and ACL grants; may send funded on-chain transactions"),
  ))
    .option("--out <dir>", "preparation artifact directory override")
    .option("--lanes <n>", "funded wallet lanes for on-chain handle creation", parseBoundedInt("--lanes", MAX_LANES))
    .option("--connections <n>", "max sockets used for relayer readiness", parseBoundedInt("--connections", MAX_CONNECTIONS))
    .option("--skip-readiness", "skip GET /health/readiness before preparation");
  prepare.action(async (ref, options, command) => {
    const env = await envFromCommand(command);
    const [{ formatPoolPlan, inspectPoolRequirements, preparePoolRequirements }, { logger }] = await Promise.all([
      import("../../pool/planning"), import("../../shared/logger"),
    ]);
    const resolved = await loadResolvedScenario(ref, options as ScenarioPrepareOptions);
    await withSignals(
      logger,
      "Interrupt received; stopping pool preparation.",
      async (signal) => {
        const artifactDir = options.out ?? timestampedDirectory(
          join(env.dataDir, "preparations"),
          "scenario",
          resolved,
        );
        const initial = await inspectPoolRequirements({
          env,
          scenarios: [resolved],
          artifactDir,
        });
        for (const line of formatPoolPlan(initial)) logger.info(line);
        signal.throwIfAborted();
        // preparePoolRequirements throws when the re-inspected live plan is
        // not ready, so a returning call is always a ready plan.
        await preparePoolRequirements({
          env,
          scenarios: [resolved],
          artifactDir,
          lanes: options.lanes,
          signal,
          onProgress: (message) => logger.info(message),
          beforeActions: async () => {
            const { assertRelayerReadiness } = await import("../../runner/readiness");
            await assertRelayerReadiness({
              env,
              connections: options.connections,
              skipReadiness: Boolean(options.skipReadiness),
              signal,
            });
          },
        });
        logger.success(`Pool preparation complete; artifacts: ${artifactDir}`);
      },
    );
  });

  registerScenarioRunCommand(scenarios);
};
