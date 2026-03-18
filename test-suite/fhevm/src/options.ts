/**
 * options.ts — Shared CLI option definitions for @effect/cli commands.
 *
 * Reusable Options/Args plus custom transform helpers
 * moved from the old parseArgs-based cli.ts.
 */
import { Args, Options } from "@effect/cli";
import { Option } from "effect";
import { OVERRIDE_GROUPS } from "./types";
import type {
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
const BUILD_OVERRIDES: LocalOverride[] = ALL_OVERRIDES.filter(({ group }) => group !== "coprocessor");

export const expandBuildOverrides = (scenarioPath?: string) =>
  scenarioPath ? BUILD_OVERRIDES : ALL_OVERRIDES;

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

// ---------------------------------------------------------------------------
// Shared @effect/cli Options
// ---------------------------------------------------------------------------

// NOTE: target and from-step use Options.text (NOT Options.choice) so that
// invalid values reach the handler and produce our custom error messages.
// Tests assert exact strings like "Unsupported target bogus".

export const targetOption = Options.text("target").pipe(
  Options.withDescription("Bundle source to boot: latest-main, latest-supported, sha, or a network target."),
  Options.withDefault("latest-main"),
);

export const shaOption = Options.text("sha").pipe(
  Options.withDescription("Commit SHA to resolve when --target sha is used."),
  Options.optional,
);

export const overrideOption = Options.text("override").pipe(
  Options.withDescription("Build selected workspace groups locally. Repeatable; supports <group> or group:service."),
  Options.repeated,
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => [])),
);

export const fromStepOption = Options.text("from-step").pipe(
  Options.withDescription("Start from a specific pipeline step when resuming or previewing."),
  Options.optional,
);

export const lockFileOption = Options.text("lock-file").pipe(
  Options.withDescription("Use an existing lock snapshot instead of resolving versions live."),
  Options.optional,
);

export const scenarioOption = Options.text("scenario").pipe(
  Options.withDescription("Path to a coprocessor consensus scenario file."),
  Options.optional,
);

export const resumeOption = Options.boolean("resume").pipe(
  Options.withDescription("Resume from persisted .fhevm state instead of starting fresh."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const dryRunOption = Options.boolean("dry-run").pipe(
  Options.withDescription("Print the resolved plan and stop before mutating state or containers."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const resetOption = Options.boolean("reset").pipe(
  Options.withDescription("Discard persisted state and regenerate the runtime from scratch."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const allowSchemaMismatchOption = Options.boolean("allow-schema-mismatch").pipe(
  Options.withDescription("Bypass schema-coupled local override safety checks."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const buildOption = Options.boolean("build").pipe(
  Options.withDescription("Build every workspace-owned group locally. On scenario runs, coprocessor remains scenario-driven."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const forceModernRelayerOption = Options.boolean("force-modern-relayer").pipe(
  Options.withDescription("Force modern relayer pins regardless of the selected component refs."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const imagesOption = Options.boolean("images").pipe(
  Options.withDescription("Also remove locally built images when cleaning the stack."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const noFollowOption = Options.boolean("no-follow").pipe(
  Options.withDescription("Print the recent logs and exit instead of following."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const grepOption = Options.text("grep").pipe(
  Options.withDescription("Custom grep pattern passed through to the e2e runner."),
  Options.optional,
);

export const networkOption = Options.text("network").pipe(
  Options.withDescription("Hardhat network passed to the test suite."),
  Options.withDefault("staging"),
);

export const verboseOption = Options.boolean("verbose").pipe(
  Options.withDescription("Enable verbose output from the underlying test command."),
  Options.optional,
  Options.map((value) => Option.getOrElse(value, () => false)),
);

export const parallelOption = Options.boolean("parallel").pipe(
  Options.withDescription("Run supported test suites in parallel."),
  Options.optional,
);

// ---------------------------------------------------------------------------
// Shared @effect/cli Args
// ---------------------------------------------------------------------------

export const serviceArg = Args.text({ name: "service" }).pipe(
  Args.withDescription("Container alias or service name to target."),
  Args.optional,
);

export const scopeArg = Args.text({ name: "scope" }).pipe(
  Args.withDescription("Pause target: host or gateway."),
);

export const groupArg = Args.text({ name: "group" }).pipe(
  Args.withDescription("Local override group to rebuild in-place."),
);

export const testNameArg = Args.text({ name: "test-name" }).pipe(
  Args.withDescription("Named test profile to run."),
  Args.optional,
);

export const compatVersionArg = Args.text({ name: "version" }).pipe(
  Args.withDescription("Component version or git ref to inspect for stack-era compatibility."),
  Args.repeated,
);
