/**
 * options.ts — Shared CLI option definitions for @effect/cli commands.
 *
 * Reusable Options/Args plus custom transform helpers
 * moved from the old parseArgs-based cli.ts.
 */
import { Options, Args } from "@effect/cli";
import { OVERRIDE_GROUPS } from "./types";
import type {
  InstanceOverride,
  LocalOverride,
  OverrideGroup,
} from "./types";
import { resolveServiceOverrides } from "./layout";

// ---------------------------------------------------------------------------
// Custom transform helpers (moved from cli.ts, unchanged logic)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Shared @effect/cli Options
// ---------------------------------------------------------------------------

// NOTE: target and from-step use Options.text (NOT Options.choice) so that
// invalid values reach the handler and produce our custom error messages.
// Tests assert exact strings like "Unsupported target bogus".

export const targetOption = Options.text("target").pipe(
  Options.withDefault("latest-main"),
);

export const shaOption = Options.text("sha").pipe(Options.optional);

export const overrideOption = Options.text("override").pipe(
  Options.repeated,
);

export const coprocessorsOption = Options.text("coprocessors").pipe(
  Options.withDefault("1"),
);

export const thresholdOption = Options.text("threshold").pipe(
  Options.optional,
);

export const fromStepOption = Options.text("from-step").pipe(
  Options.optional,
);

export const lockFileOption = Options.text("lock-file").pipe(
  Options.optional,
);

export const resumeOption = Options.boolean("resume").pipe(
  Options.withDefault(false),
);

export const dryRunOption = Options.boolean("dry-run").pipe(
  Options.withDefault(false),
);

export const resetOption = Options.boolean("reset").pipe(
  Options.withDefault(false),
);

export const allowSchemaMismatchOption = Options.boolean("allow-schema-mismatch").pipe(
  Options.withDefault(false),
);

export const instanceEnvOption = Options.text("instance-env").pipe(
  Options.repeated,
);

export const instanceArgOption = Options.text("instance-arg").pipe(
  Options.repeated,
);

export const imagesOption = Options.boolean("images").pipe(
  Options.withDefault(false),
);

export const noFollowOption = Options.boolean("no-follow").pipe(
  Options.withDefault(false),
);

export const grepOption = Options.text("grep").pipe(Options.optional);

export const networkOption = Options.text("network").pipe(
  Options.withDefault("staging"),
);

export const verboseOption = Options.boolean("verbose").pipe(
  Options.withDefault(false),
);

export const parallelOption = Options.boolean("parallel").pipe(
  Options.optional,
);

// ---------------------------------------------------------------------------
// Shared @effect/cli Args
// ---------------------------------------------------------------------------

export const serviceArg = Args.text({ name: "service" }).pipe(Args.optional);

export const scopeArg = Args.text({ name: "scope" }).pipe(Args.optional);

export const groupArg = Args.text({ name: "group" }).pipe(Args.optional);

export const testNameArg = Args.text({ name: "test-name" }).pipe(Args.optional);
