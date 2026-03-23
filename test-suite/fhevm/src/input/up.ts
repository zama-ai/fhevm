/**
 * Parses and validates `up`-family CLI flags into normalized stack input.
 */
import { resolveServiceOverrides } from "../layout";
import { OVERRIDE_GROUPS, STEP_NAMES, TARGETS, type LocalOverride, type OverrideGroup, type StepName, type VersionTarget } from "../types";
import { PreflightError } from "../errors";
import { asBool, asString, asStringList } from "./shared";

const ALL_OVERRIDES: LocalOverride[] = OVERRIDE_GROUPS.map((group) => ({ group }));

/** Parses a local override flag into normalized override selections. */
const parseLocalOverride = (value: string): LocalOverride[] => {
  if (value === "all") {
    return ALL_OVERRIDES;
  }
  const colonIdx = value.indexOf(":");
  if (colonIdx < 0) {
    if (!OVERRIDE_GROUPS.includes(value as OverrideGroup)) {
      throw new Error(`Unsupported override ${value}. Valid: all, ${OVERRIDE_GROUPS.join(", ")}`);
    }
    return [{ group: value as OverrideGroup }];
  }
  const group = value.slice(0, colonIdx);
  const rest = value.slice(colonIdx + 1);
  if (!OVERRIDE_GROUPS.includes(group as OverrideGroup)) {
    throw new Error(`Unsupported override group "${group}". Valid: ${OVERRIDE_GROUPS.join(", ")}`);
  }
  const parts = rest.split(",").map((part) => part.trim()).filter(Boolean);
  if (!parts.length) {
    throw new Error(`Expected at least one service name in override "${value}"`);
  }
  return [{ group: group as OverrideGroup, services: resolveServiceOverrides(group as OverrideGroup, parts) }];
};

/** Normalizes and validates `up` command arguments before stack execution. */
export const parseUpInput = (args: Record<string, unknown>) => {
  const target = asString(args.target);
  const sha = asString(args.sha);
  const fromStepRaw = asString(args["from-step"] ?? args.fromStep);
  const lockFile = asString(args["lock-file"] ?? args.lockFile);
  const scenarioPath = asString(args.scenario);
  const resume = asBool(args.resume);
  const dryRun = asBool(args["dry-run"] ?? args.dryRun);
  const reset = asBool(args.reset);
  const allowSchemaMismatch = asBool(args["allow-schema-mismatch"] ?? args.allowSchemaMismatch);
  const build = asBool(args.build);

  if (target && !TARGETS.includes(target as VersionTarget)) {
    throw new PreflightError(`Unsupported target ${target}. Valid: ${TARGETS.join(", ")}`);
  }
  const validTarget = (target ?? "latest-main") as VersionTarget;

  let fromStep: StepName | undefined;
  if (fromStepRaw) {
    if (!STEP_NAMES.includes(fromStepRaw as StepName)) {
      throw new PreflightError(`Unknown step ${fromStepRaw}. Valid: ${STEP_NAMES.join(", ")}`);
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
  const explicitOverrides = overrideValues.flatMap(parseLocalOverride);
  const overrides = [...(build ? ALL_OVERRIDES : []), ...explicitOverrides];

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
