import { defineCommand, runCommand } from "citty";

import { formatCliError, PreflightError } from "./lean/errors";
import { listScenarios, logs, pause, showResumeHint, status, unpause, up, upDryRun, clean, down, upgrade } from "./lean/stack";
import { test } from "./lean/test";
import { resolveServiceOverrides } from "./layout";
import { OVERRIDE_GROUPS, STEP_NAMES, TARGETS, type LocalOverride, type OverrideGroup, type StepName, type VersionTarget } from "./types";

const ALL_OVERRIDES: LocalOverride[] = OVERRIDE_GROUPS.map((group) => ({ group }));
const BUILD_OVERRIDES: LocalOverride[] = ALL_OVERRIDES.filter(({ group }) => group !== "coprocessor");

const expandBuildOverrides = (scenarioPath?: string) => (scenarioPath ? BUILD_OVERRIDES : ALL_OVERRIDES);

const parseLocalOverride = (value: string): LocalOverride[] => {
  if (value === "all") {
    return ALL_OVERRIDES;
  }
  const colonIdx = value.indexOf(":");
  if (colonIdx < 0) {
    if (!OVERRIDE_GROUPS.includes(value as OverrideGroup)) {
      throw new Error(`Unsupported override ${value}`);
    }
    return [{ group: value as OverrideGroup }];
  }
  const group = value.slice(0, colonIdx);
  const rest = value.slice(colonIdx + 1);
  if (!OVERRIDE_GROUPS.includes(group as OverrideGroup)) {
    throw new Error(`Unsupported override group "${group}"`);
  }
  const parts = rest.split(",").map((part) => part.trim()).filter(Boolean);
  if (!parts.length) {
    throw new Error(`Expected at least one service name in override "${value}"`);
  }
  return [{ group: group as OverrideGroup, services: resolveServiceOverrides(group as OverrideGroup, parts) }];
};

const asString = (value: unknown) => (typeof value === "string" && value.length ? value : undefined);
const asBool = (value: unknown) => value === true;
const asStringList = (value: unknown) =>
  Array.isArray(value) ? value.map(String) : typeof value === "string" && value.length ? [value] : [];

const parseUpInput = (args: Record<string, unknown>) => {
  const target = asString(args.target);
  const sha = asString(args.sha);
  const fromStepRaw = asString(args.fromStep);
  const lockFile = asString(args.lockFile);
  const scenarioPath = asString(args.scenario);
  const resume = asBool(args.resume);
  const dryRun = asBool(args.dryRun);
  const reset = asBool(args.reset);
  const allowSchemaMismatch = asBool(args.allowSchemaMismatch);
  const build = asBool(args.build);

  if (target && !TARGETS.includes(target as VersionTarget)) {
    throw new PreflightError(`Unsupported target ${target}`);
  }
  const validTarget = (target ?? "latest-main") as VersionTarget;

  let fromStep: StepName | undefined;
  if (fromStepRaw) {
    if (!STEP_NAMES.includes(fromStepRaw as StepName)) {
      throw new PreflightError(`Unknown step ${fromStepRaw}`);
    }
    fromStep = fromStepRaw as StepName;
  }

  if (validTarget === "sha" && !sha) {
    throw new PreflightError("--target sha requires --sha");
  }
  if (validTarget !== "sha" && sha) {
    throw new PreflightError("--sha requires --target sha");
  }
  if (sha && lockFile) {
    throw new PreflightError("--sha cannot be used with --lock-file");
  }
  if (fromStep && !resume && !dryRun) {
    throw new PreflightError("--from-step requires --resume or --dry-run");
  }

  const overrideValues = asStringList(args.override);
  if (build && overrideValues.length) {
    throw new PreflightError("--build cannot be combined with --override");
  }
  const overrides = [
    ...(build ? expandBuildOverrides(scenarioPath) : []),
    ...overrideValues.flatMap(parseLocalOverride),
  ];
  if (scenarioPath && overrides.some((item) => item.group === "coprocessor")) {
    throw new PreflightError("--scenario cannot be combined with --override coprocessor");
  }

  return {
    target: validTarget,
    requestedTarget: target as VersionTarget | undefined,
    sha,
    overrides,
    scenarioPath,
    fromStep,
    lockFile,
    allowSchemaMismatch,
    resume,
    dryRun,
    reset,
  };
};

const root = defineCommand({
  meta: {
    name: "fhevm-cli",
    version: "0.1.0",
    description: "Local orchestration entrypoint for the fhEVM test stack.",
  },
  subCommands: {
    up: defineCommand({
      meta: { description: "Boot the fhevm stack from a target, lock file, or persisted state." },
      args: {
        target: { type: "string", description: "Bundle source to boot." },
        sha: { type: "string", description: "Commit SHA to resolve when --target sha is used." },
        override: { type: "string", description: "Build selected workspace groups locally.", alias: "o" },
        fromStep: { type: "string", description: "Start from a specific pipeline step when resuming or previewing." },
        lockFile: { type: "string", description: "Use an existing lock snapshot instead of resolving versions live." },
        scenario: { type: "string", description: "Scenario preset name or path." },
        resume: { type: "boolean", description: "Resume from persisted state." },
        dryRun: { type: "boolean", description: "Print the resolved plan and stop before mutating state." },
        reset: { type: "boolean", description: "Discard cached resolution and regenerate from scratch." },
        allowSchemaMismatch: { type: "boolean", description: "Bypass schema-coupled local override safety checks." },
        build: { type: "boolean", description: "Build every workspace-owned group locally." },
      },
      async run({ args }) {
        const parsed = parseUpInput(args);
        if (parsed.dryRun) {
          const { dryRun: _dryRun, ...rest } = parsed;
          await upDryRun(rest);
          return;
        }
        await up(parsed);
      },
    }),
    deploy: defineCommand({
      meta: { description: "Alias of `up` kept for deployment-oriented workflows." },
      args: {
        target: { type: "string" },
        sha: { type: "string" },
        override: { type: "string", alias: "o" },
        fromStep: { type: "string" },
        lockFile: { type: "string" },
        scenario: { type: "string" },
        resume: { type: "boolean" },
        dryRun: { type: "boolean" },
        reset: { type: "boolean" },
        allowSchemaMismatch: { type: "boolean" },
        build: { type: "boolean" },
      },
      async run({ args }) {
        const parsed = parseUpInput(args);
        if (parsed.dryRun) {
          const { dryRun: _dryRun, ...rest } = parsed;
          await upDryRun(rest);
          return;
        }
        await up(parsed);
      },
    }),
    down: defineCommand({
      meta: { description: "Stop all stack containers in reverse order." },
      async run() {
        await down();
      },
    }),
    clean: defineCommand({
      meta: { description: "Stop containers, optionally remove CLI-owned images, and delete .fhevm." },
      args: {
        images: { type: "boolean", description: "Also remove locally built images." },
      },
      async run({ args }) {
        await clean({ images: asBool(args.images) });
      },
    }),
    status: defineCommand({
      meta: { description: "Print persisted state and running containers." },
      async run() {
        await status();
      },
    }),
    logs: defineCommand({
      meta: { description: "Follow container logs for a specified or first container." },
      args: {
        service: { type: "positional", description: "Container alias or service name to target." },
        noFollow: { type: "boolean", description: "Print recent logs and exit instead of following." },
      },
      async run({ args }) {
        await logs(asString(args.service), { follow: !asBool(args.noFollow) });
      },
    }),
    upgrade: defineCommand({
      meta: { description: "Rebuild and restart an active local runtime override group." },
      args: {
        group: { type: "positional", description: "Local override group to rebuild in-place." },
      },
      async run({ args }) {
        await upgrade(asString(args.group));
      },
    }),
    test: defineCommand({
      meta: { description: "Run e2e tests inside the fhevm test-suite container." },
      args: {
        testName: { type: "positional", description: "Named test profile to run." },
        grep: { type: "string", description: "Custom grep pattern passed through to the e2e runner." },
        network: { type: "string", description: "Hardhat network passed to the test suite.", default: "staging" },
        verbose: { type: "boolean", description: "Enable verbose output from the test command." },
        parallel: { type: "boolean", description: "Run supported test suites in parallel." },
      },
      async run({ args }) {
        await test(asString(args.testName), {
          grep: asString(args.grep),
          network: asString(args.network) ?? "staging",
          verbose: asBool(args.verbose),
          parallel: args.parallel === undefined ? undefined : asBool(args.parallel),
        });
      },
    }),
    pause: defineCommand({
      meta: { description: "Pause host or gateway contracts." },
      args: {
        scope: { type: "positional", description: "Pause target: host or gateway." },
      },
      async run({ args }) {
        await pause(asString(args.scope));
      },
    }),
    unpause: defineCommand({
      meta: { description: "Unpause host or gateway contracts." },
      args: {
        scope: { type: "positional", description: "Pause target: host or gateway." },
      },
      async run({ args }) {
        await unpause(asString(args.scope));
      },
    }),
    scenario: defineCommand({
      meta: { description: "Scenario utilities." },
      subCommands: {
        list: defineCommand({
          meta: { description: "List bundled scenario presets." },
          async run() {
            await listScenarios();
          },
        }),
      },
    }),
  },
});

export const main = async (argv = process.argv) => {
  try {
    const rawArgs = argv.slice(2);
    await runCommand(root, {
      rawArgs,
      showUsage: rawArgs.length === 0 || rawArgs.includes("--help") || rawArgs.includes("-h"),
    });
  } catch (error) {
    const message = formatCliError(error);
    if (message) {
      console.error(message);
    }
    process.exitCode = 1;
    await showResumeHint(argv).catch(() => undefined);
  }
};

if (import.meta.main) {
  await main();
}
