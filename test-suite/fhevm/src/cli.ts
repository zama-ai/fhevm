/**
 * cli.ts — Entry point for the fhevm CLI.
 *
 * Parses arguments, dispatches to the appropriate command handler,
 * provides the LiveLayer, and handles errors.
 */
import { Effect, Layer } from "effect";
import { parseArgs } from "node:util";

import { OVERRIDE_GROUPS, STEP_NAMES, TARGETS } from "./types";
import type {
  InstanceOverride,
  LocalOverride,
  OverrideGroup,
  StepName,
  Topology,
  UpOptions,
} from "./types";
import { resolveServiceOverrides } from "./layout";
import { LiveLayer } from "./services/layers";

import { up, upDryRun } from "./commands/up";
import { down } from "./commands/down";
import { clean } from "./commands/clean";
import { status } from "./commands/status";
import { logs } from "./commands/logs";
import { upgrade } from "./commands/upgrade";
import { test } from "./commands/test";
import { pause } from "./commands/pause";
import { unpause } from "./commands/unpause";
import { compatDefaults } from "./commands/compat-defaults";
import { StateManager } from "./services/StateManager";

// ---------------------------------------------------------------------------
// CLI Parsing helpers
// ---------------------------------------------------------------------------

const createTopology = (
  count: number,
  threshold?: number,
  instances?: Record<string, InstanceOverride>,
): Topology => ({
  count,
  threshold: threshold ?? count,
  instances: instances ?? {},
});

const ALL_OVERRIDES: LocalOverride[] = OVERRIDE_GROUPS.map((group) => ({
  group,
}));

export const parseLocalOverride = (value: string): LocalOverride[] => {
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
  const overrideGroup = group as OverrideGroup;
  const parts = rest
    .split(",")
    .map((part) => part.trim())
    .filter(Boolean);
  if (!parts.length) {
    throw new Error(
      `Expected at least one service name in override "${value}"`,
    );
  }
  return [
    {
      group: overrideGroup,
      services: resolveServiceOverrides(overrideGroup, parts),
    },
  ];
};

export const parseKeyValue = (value: string) => {
  const idx = value.indexOf("=");
  if (idx < 0) {
    throw new Error(`Expected KEY=VALUE, got ${value}`);
  }
  return [value.slice(0, idx), value.slice(idx + 1)] as const;
};

export const parseInstanceKey = (value: string) => {
  const idx = value.indexOf(":");
  if (idx < 0) {
    throw new Error(`Expected INDEX:VALUE, got ${value}`);
  }
  const index = Number(value.slice(0, idx));
  if (!Number.isInteger(index) || index < 0) {
    throw new Error(`Invalid instance index in ${value}`);
  }
  return [index, value.slice(idx + 1)] as const;
};

export const parseInstanceEnv = (values: string[]) => {
  const out: Record<string, InstanceOverride> = {};
  for (const value of values) {
    const [index, payload] = parseInstanceKey(value);
    const [key, envValue] = parseKeyValue(payload);
    const name = `coprocessor-${index}`;
    out[name] ??= { env: {}, args: {} };
    out[name].env[key] = envValue;
  }
  return out;
};

export const parseInstanceArgs = (values: string[]) => {
  const out: Record<string, InstanceOverride> = {};
  for (const value of values) {
    const [index, payload] = parseInstanceKey(value);
    const [service, arg] = parseKeyValue(payload);
    const name = `coprocessor-${index}`;
    out[name] ??= { env: {}, args: {} };
    out[name].args[service] ??= [];
    out[name].args[service].push(arg);
  }
  return out;
};

export const mergeInstanceOverrides = (
  ...items: Record<string, InstanceOverride>[]
) => {
  const out: Record<string, InstanceOverride> = {};
  for (const item of items) {
    for (const [name, override] of Object.entries(item)) {
      out[name] ??= { env: {}, args: {} };
      Object.assign(out[name].env, override.env);
      for (const [service, args] of Object.entries(override.args)) {
        out[name].args[service] = [
          ...(out[name].args[service] ?? []),
          ...args,
        ];
      }
    }
  }
  return out;
};

const ensureStep = (value?: string): StepName | undefined => {
  if (!value) {
    return undefined;
  }
  if (!STEP_NAMES.includes(value as StepName)) {
    throw new Error(`Unknown step ${value}`);
  }
  return value as StepName;
};

// ---------------------------------------------------------------------------
// parseCli
// ---------------------------------------------------------------------------

export const parseCli = (argv: string[]) => {
  const command = argv[2];
  const parsed = parseArgs({
    args: argv.slice(3),
    allowPositionals: true,
    allowNegative: true,
    options: {
      target: { type: "string", default: "latest-main" },
      sha: { type: "string" },
      override: { type: "string", multiple: true, default: [] },
      coprocessors: { type: "string", default: "1" },
      threshold: { type: "string" },
      "from-step": { type: "string" },
      "lock-file": { type: "string" },
      resume: { type: "boolean", default: false },
      "dry-run": { type: "boolean", default: false },
      images: { type: "boolean", default: false },
      grep: { type: "string" },
      network: { type: "string", default: "staging" },
      verbose: { type: "boolean", default: false },
      "instance-env": { type: "string", multiple: true, default: [] },
      "instance-arg": { type: "string", multiple: true, default: [] },
      "allow-schema-mismatch": { type: "boolean", default: false },
      reset: { type: "boolean", default: false },
      follow: { type: "boolean", default: true },
      parallel: { type: "boolean" },
    },
  });
  const target = parsed.values.target as string;
  if (!TARGETS.includes(target as (typeof TARGETS)[number])) {
    throw new Error(`Unsupported target ${target}`);
  }
  const sha = parsed.values.sha as string | undefined;
  if (target === "sha" && !sha) {
    throw new Error("--target sha requires --sha");
  }
  if (target !== "sha" && sha) {
    throw new Error("--sha requires --target sha");
  }
  if (sha && parsed.values["lock-file"]) {
    throw new Error("--sha cannot be used with --lock-file");
  }
  const count = Number(parsed.values.coprocessors);
  const threshold = parsed.values.threshold
    ? Number(parsed.values.threshold)
    : undefined;
  if (!Number.isInteger(count) || count < 1 || count > 5) {
    throw new Error("--coprocessors must be between 1 and 5");
  }
  if (
    threshold !== undefined &&
    (!Number.isInteger(threshold) || threshold < 1 || threshold > count)
  ) {
    throw new Error(
      "--threshold must be between 1 and --coprocessors",
    );
  }
  const overrides = (parsed.values.override as string[]).flatMap(
    parseLocalOverride,
  );
  const topology = createTopology(
    count,
    threshold,
    mergeInstanceOverrides(
      parseInstanceEnv(parsed.values["instance-env"] as string[]),
      parseInstanceArgs(parsed.values["instance-arg"] as string[]),
    ),
  );
  return {
    command,
    parsed,
    target: target as UpOptions["target"],
    sha,
    overrides,
    topology,
  };
};

// ---------------------------------------------------------------------------
// usage
// ---------------------------------------------------------------------------

const usage = () => {
  console.log(`Usage: fhevm-cli <command> [options]

Commands:
  up       start or resume the local stack
  deploy   alias for up
  down     stop stack containers
  clean    stop stack containers and delete .fhevm
  status   print state and running containers
  logs     follow container logs (--no-follow for one-shot tail)
  upgrade  rebuild and restart an active local runtime override
  pause    pause host or gateway contracts (host|gateway)
  unpause  unpause host or gateway contracts (host|gateway)
  test     run e2e tests in fhevm-test-suite-e2e-debug

up options:
  --target latest-main|latest-release|sha|devnet|testnet|mainnet
  --sha <git-sha>   required with --target sha
  --lock-file <path-to-bundle-json>
  --override <group[:svc1,svc2]>    repeated; groups: ${OVERRIDE_GROUPS.join(", ")}
  --allow-schema-mismatch          bypass latest-release shared-DB override guard
  --coprocessors <n>
  --threshold <t>
  --instance-env <idx:KEY=VALUE>
  --instance-arg <idx:service=--flag=value>
  --from-step <${STEP_NAMES.join("|")}>   requires --resume, except in --dry-run
  --resume
  --dry-run
  --reset                          re-resolve versions from GitHub (ignore cache)

clean options:
  --images  remove CLI-owned local override images too

logs options:
  --no-follow              print tail and exit (default: follow)

test options:
  --grep <pattern>         override the default test filter
  --parallel               run tests in parallel (auto for operators)
  --network <name>         hardhat network (default: staging)
  --verbose                pass --verbose to hardhat
`);
};

// ---------------------------------------------------------------------------
// main — parse, dispatch, provide LiveLayer, handle errors
// ---------------------------------------------------------------------------

export const main = async (argv = process.argv, layerOverride?: Layer.Layer<any, never, never>) => {
  const layer = layerOverride ?? LiveLayer;
  let command: string | undefined;
  try {
    const parsed = parseCli(argv);
    command = parsed.command === "deploy" ? "up" : parsed.command;
    const fromStep = ensureStep(
      parsed.parsed.values["from-step"] as string | undefined,
    );
    if (
      command === "up" &&
      fromStep &&
      !parsed.parsed.values.resume &&
      !parsed.parsed.values["dry-run"]
    ) {
      throw new Error("--from-step requires --resume or --dry-run");
    }

    let program: Effect.Effect<void, unknown, never>;

    switch (command) {
      case "up":
        if (parsed.parsed.values["dry-run"]) {
          program = upDryRun({
            target: parsed.target,
            sha: parsed.sha,
            overrides: parsed.overrides,
            topology: parsed.topology,
            fromStep,
            lockFile: parsed.parsed.values["lock-file"] as
              | string
              | undefined,
            allowSchemaMismatch:
              parsed.parsed.values["allow-schema-mismatch"] as boolean,
            reset: parsed.parsed.values.reset as boolean,
          }).pipe(Effect.provide(layer));
        } else {
          program = up({
            target: parsed.target,
            sha: parsed.sha,
            overrides: parsed.overrides,
            topology: parsed.topology,
            fromStep,
            lockFile: parsed.parsed.values["lock-file"] as
              | string
              | undefined,
            allowSchemaMismatch:
              parsed.parsed.values["allow-schema-mismatch"] as boolean,
            resume: parsed.parsed.values.resume as boolean,
            dryRun: parsed.parsed.values["dry-run"] as boolean,
            reset: parsed.parsed.values.reset as boolean,
          }).pipe(Effect.provide(layer));
        }
        break;
      case "down":
        program = down.pipe(Effect.provide(layer));
        break;
      case "clean":
        program = clean({
          images: parsed.parsed.values.images as boolean,
        }).pipe(Effect.provide(layer));
        break;
      case "status":
        program = status.pipe(Effect.provide(layer));
        break;
      case "logs":
        program = logs(parsed.parsed.positionals[0], {
          follow: parsed.parsed.values.follow as boolean,
        }).pipe(Effect.provide(layer));
        break;
      case "upgrade":
        program = upgrade(parsed.parsed.positionals[0]).pipe(
          Effect.provide(layer),
        );
        break;
      case "pause":
        program = pause(parsed.parsed.positionals[0]).pipe(
          Effect.provide(layer),
        );
        break;
      case "unpause":
        program = unpause(parsed.parsed.positionals[0]).pipe(
          Effect.provide(layer),
        );
        break;
      case "test":
        program = test(parsed.parsed.positionals[0], {
          grep: parsed.parsed.values.grep as string | undefined,
          network: parsed.parsed.values.network as string,
          verbose: parsed.parsed.values.verbose as boolean,
          parallel: parsed.parsed.values.parallel as boolean | undefined,
        }).pipe(Effect.provide(layer));
        break;
      case "compat-defaults":
        program = compatDefaults;
        break;
      case "doctor":
        throw new Error(
          "`doctor` was removed; use `fhevm-cli up --dry-run ...`",
        );
      case "help":
      case "--help":
      case "-h":
      case undefined:
        usage();
        return;
      default:
        throw new Error(`Unknown command ${parsed.command}`);
    }

    await Effect.runPromise(
      program.pipe(
        Effect.catchAll((error) => {
          if (error && typeof error === "object" && "message" in error) {
            console.error((error as { message: string }).message);
          } else {
            console.error(String(error));
          }
          process.exitCode = 1;
          return Effect.void;
        }),
      ),
    );

    // Show resume hint if 'up' failed
    if (command === "up" && process.exitCode === 1) {
      try {
        const hasState = await Effect.runPromise(
          Effect.gen(function* () {
            const stateManager = yield* StateManager;
            return yield* stateManager.load;
          }).pipe(Effect.provide(layer)),
        );
        if (hasState) {
          console.error(
            "Hint: run with --resume to continue, or without to start fresh.",
          );
        }
      } catch {
        // Ignore errors checking state for the hint
      }
    }
  } catch (error) {
    console.error((error as Error).message);
    process.exitCode = 1;
  }
};

export { main as default };

if (import.meta.main) {
  await main();
}
