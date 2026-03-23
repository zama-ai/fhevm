/**
 * Loads, validates, and resolves coprocessor scenario definitions and their interaction with local overrides.
 */
import fs from "node:fs/promises";
import path from "node:path";

import YAML from "yaml";

import { PreflightError } from "../errors";
import {
  GROUP_SERVICE_SUFFIXES,
  MAX_COPROCESSOR_INSTANCES,
  REPO_ROOT,
  resolveServiceOverrides,
} from "../layout";
import type {
  CoprocessorInstanceSource,
  CoprocessorScenario,
  LocalOverride,
  OverrideGroup,
  ResolvedCoprocessorScenario,
  ResolvedCoprocessorScenarioInstance,
  ScenarioSummary,
} from "../types";

const COPROCESSOR_SCENARIO_KIND = "coprocessor-consensus";
const COPROCESSOR_SCENARIO_VERSION = 1;
const COPROCESSOR_SCENARIO_DIR = path.join(REPO_ROOT, "test-suite", "fhevm", "scenarios");
const SCENARIO_FILE = /\.ya?ml$/i;
const COPROCESSOR_ARG_TARGETS = new Set([
  "*",
  ...GROUP_SERVICE_SUFFIXES.coprocessor.filter((value) => !value.includes("migration")),
]);

/** Normalizes a scalar scenario value into a string. */
const normalizeScalar = (value: unknown, label: string) => {
  if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  throw new Error(`${label} must be a string, number, or boolean`);
};

/** Normalizes optional human-readable text fields from a scenario file. */
const normalizeOptionalText = (value: unknown, label: string) => {
  if (value === undefined) {
    return undefined;
  }
  const normalized = normalizeScalar(value, label).trim();
  return normalized.length ? normalized : undefined;
};

/** Normalizes scenario arg maps into string-array command fragments. */
const normalizeArgs = (args: Record<string, unknown> | undefined, label: string) =>
  Object.fromEntries(
    Object.entries(args ?? {}).map(([key, values]) => {
      if (!Array.isArray(values)) {
        throw new Error(`${label}.${key} must be an array`);
      }
      return [
        key,
        values.map((value, index) => normalizeScalar(value, `${label}.${key}[${index}]`)),
      ];
    }),
  );

/** Applies the default inherited source to omitted instance sources. */
const normalizeSource = (source: CoprocessorInstanceSource | undefined) =>
  source ?? { mode: "inherit" as const };

/** Validates that scenario arg targets reference supported coprocessor services. */
const validateInstanceArgs = (args: Record<string, string[]> | undefined, label: string) => {
  for (const key of Object.keys(args ?? {})) {
    if (!COPROCESSOR_ARG_TARGETS.has(key)) {
      throw new Error(`${label}: unknown arg target "${key}". Valid: ${[...COPROCESSOR_ARG_TARGETS].join(", ")}`);
    }
  }
};

/** Resolves a scenario reference into candidate on-disk YAML paths. */
const scenarioCandidatePaths = (value: string) => {
  const absolute = path.resolve(value);
  const explicit = value.includes("/") || value.includes("\\") || SCENARIO_FILE.test(value);
  return explicit
    ? [absolute]
    : [
        path.join(COPROCESSOR_SCENARIO_DIR, `${value}.yaml`),
        path.join(COPROCESSOR_SCENARIO_DIR, `${value}.yml`),
      ];
};

/** Returns the default single-instance inherited coprocessor scenario. */
export const defaultCoprocessorScenario = (): ResolvedCoprocessorScenario => ({
  version: COPROCESSOR_SCENARIO_VERSION,
  kind: COPROCESSOR_SCENARIO_KIND,
  origin: "default",
  name: "Default",
  description: "Single inherited coprocessor instance.",
  topology: { count: 1, threshold: 1 },
  instances: [{ index: 0, source: { mode: "inherit" }, env: {}, args: {} }],
});

/** Parses and validates a YAML coprocessor scenario file. */
export const parseCoprocessorScenario = (text: string, sourceLabel = "scenario"): CoprocessorScenario => {
  const parsed = YAML.parse(text) as Record<string, unknown> | null;
  if (!parsed || typeof parsed !== "object") {
    throw new Error(`${sourceLabel}: expected a YAML object`);
  }
  if (parsed.version !== COPROCESSOR_SCENARIO_VERSION) {
    throw new Error(`${sourceLabel}: expected version ${COPROCESSOR_SCENARIO_VERSION}`);
  }
  if (parsed.kind !== COPROCESSOR_SCENARIO_KIND) {
    throw new Error(`${sourceLabel}: expected kind ${COPROCESSOR_SCENARIO_KIND}`);
  }
  const topology = parsed.topology;
  if (!topology || typeof topology !== "object") {
    throw new Error(`${sourceLabel}: missing topology`);
  }
  const count = Number((topology as Record<string, unknown>).count);
  const threshold = Number((topology as Record<string, unknown>).threshold);
  if (!Number.isInteger(count) || count < 1) {
    throw new Error(`${sourceLabel}: topology.count must be >= 1`);
  }
  if (count > MAX_COPROCESSOR_INSTANCES) {
    throw new Error(`${sourceLabel}: topology.count must be <= ${MAX_COPROCESSOR_INSTANCES}`);
  }
  if (!Number.isInteger(threshold) || threshold < 1 || threshold > count) {
    throw new Error(`${sourceLabel}: topology.threshold must be between 1 and count`);
  }

  const rawInstances = parsed.instances;
  if (rawInstances !== undefined && !Array.isArray(rawInstances)) {
    throw new Error(`${sourceLabel}: instances must be an array`);
  }

  const seen = new Set<number>();
  const instances = (rawInstances ?? []).map((entry, offset) => {
    if (!entry || typeof entry !== "object") {
      throw new Error(`${sourceLabel}: instances[${offset}] must be an object`);
    }
    const instance = entry as Record<string, unknown>;
    const index = Number(instance.index);
    if (!Number.isInteger(index) || index < 0 || index >= count) {
      throw new Error(`${sourceLabel}: instances[${offset}].index must be between 0 and ${count - 1}`);
    }
    if (seen.has(index)) {
      throw new Error(`${sourceLabel}: duplicate instance index ${index}`);
    }
    seen.add(index);

    const source = instance.source;
    let normalizedSource:
      | { mode: "inherit" }
      | { mode: "local" }
      | { mode: "registry"; tag: string }
      | undefined;
    if (source !== undefined) {
      if (!source || typeof source !== "object") {
        throw new Error(`${sourceLabel}: instances[${offset}].source must be an object`);
      }
      const mode = String((source as Record<string, unknown>).mode ?? "");
      if (mode === "inherit" || mode === "local") {
        normalizedSource = { mode };
      } else if (mode === "registry") {
        const tag = String((source as Record<string, unknown>).tag ?? "");
        if (!tag) {
          throw new Error(`${sourceLabel}: instances[${offset}].source.tag is required for registry mode`);
        }
        normalizedSource = { mode, tag };
      } else {
        throw new Error(`${sourceLabel}: instances[${offset}].source.mode must be inherit, local, or registry`);
      }
    }

    const env = instance.env;
    if (env !== undefined && (!env || typeof env !== "object" || Array.isArray(env))) {
      throw new Error(`${sourceLabel}: instances[${offset}].env must be a map`);
    }
    const args = instance.args;
    if (args !== undefined && (!args || typeof args !== "object" || Array.isArray(args))) {
      throw new Error(`${sourceLabel}: instances[${offset}].args must be a map`);
    }
    const localServices = instance.localServices;
    if (localServices !== undefined && !Array.isArray(localServices)) {
      throw new Error(`${sourceLabel}: instances[${offset}].localServices must be an array`);
    }

    const normalizedArgs = normalizeArgs(args as Record<string, unknown> | undefined, `${sourceLabel}: instances[${offset}].args`);
    validateInstanceArgs(normalizedArgs, `${sourceLabel}: instances[${offset}]`);
    const normalizedLocalServices = localServices
      ? resolveServiceOverrides(
          "coprocessor",
          localServices.map((value, serviceIndex) =>
            normalizeScalar(value, `${sourceLabel}: instances[${offset}].localServices[${serviceIndex}]`),
          ),
        )
      : undefined;
    if (normalizedLocalServices && normalizedSource?.mode !== "local") {
      throw new Error(`${sourceLabel}: instances[${offset}].localServices requires source.mode=local`);
    }

    return {
      index,
      source: normalizedSource,
      env: Object.fromEntries(
        Object.entries((env as Record<string, unknown> | undefined) ?? {}).map(([key, value]) => [
          key,
          normalizeScalar(value, `${sourceLabel}: instances[${offset}].env.${key}`),
        ]),
      ),
      args: normalizedArgs,
      localServices: normalizedLocalServices,
    };
  });

  const multiChain = parsed.multiChain === true ? true : undefined;

  return {
    version: COPROCESSOR_SCENARIO_VERSION,
    kind: COPROCESSOR_SCENARIO_KIND,
    name: normalizeOptionalText(parsed.name, `${sourceLabel}: name`),
    description: normalizeOptionalText(parsed.description, `${sourceLabel}: description`),
    multiChain,
    topology: { count, threshold },
    instances,
  };
};

/** Resolves a user-provided scenario name or path into a concrete file path. */
export const resolveScenarioReference = async (value: string) => {
  if (value === "list") {
    throw new PreflightError("`--scenario list` is not supported; run `fhevm-cli scenario list`");
  }
  for (const candidate of scenarioCandidatePaths(value)) {
    try {
      await fs.access(candidate);
      return candidate;
    } catch {
      // try next
    }
  }
  const named = !value.includes("/") && !value.includes("\\") && !SCENARIO_FILE.test(value);
  throw new PreflightError(
    named ? `Unknown scenario ${value}; run \`fhevm-cli scenario list\`` : `Scenario file not found: ${path.resolve(value)}`,
  );
};

/** Loads and parses a coprocessor scenario from disk. */
export const loadCoprocessorScenario = async (scenarioRef: string) => {
  try {
    const absolute = await resolveScenarioReference(scenarioRef);
    const text = await fs.readFile(absolute, "utf8");
    return parseCoprocessorScenario(text, absolute);
  } catch (error) {
    if (error instanceof PreflightError) {
      throw error;
    }
    throw new PreflightError(error instanceof Error ? error.message : String(error));
  }
};

/** Expands a parsed scenario into a fully populated resolved runtime scenario. */
export const resolveScenarioFile = (filePath: string, input: CoprocessorScenario): ResolvedCoprocessorScenario => {
  const byIndex = new Map((input.instances ?? []).map((instance) => [instance.index, instance] as const));
  return {
    version: COPROCESSOR_SCENARIO_VERSION,
    kind: COPROCESSOR_SCENARIO_KIND,
    origin: "file",
    name: input.name,
    description: input.description,
    multiChain: input.multiChain,
    sourcePath: path.resolve(filePath),
    topology: { ...input.topology },
    instances: Array.from({ length: input.topology.count }, (_, index) => {
      const instance = byIndex.get(index);
      return {
        index,
        source: normalizeSource(instance?.source),
        env: { ...(instance?.env ?? {}) },
        args: normalizeArgs(instance?.args, "scenario.args"),
        localServices: instance?.localServices,
      } satisfies ResolvedCoprocessorScenarioInstance;
    }),
  };
};

/** Merges explicit coprocessor override service lists into one deduplicated set. */
const mergeOverrideServices = (overrides: LocalOverride[]) => {
  const coprocessorOverrides = overrides.filter((item) => item.group === "coprocessor");
  if (!coprocessorOverrides.length) {
    return undefined;
  }
  if (coprocessorOverrides.some((item) => !item.services?.length)) {
    return undefined;
  }
  return [...new Set(coprocessorOverrides.flatMap((item) => item.services ?? []))];
};

/** Derives the coprocessor override implied by scenario-local instances. */
const localCoprocessorOverride = (scenario: Pick<ResolvedCoprocessorScenario, "instances">): LocalOverride | undefined => {
  const localInstances = scenario.instances.filter((instance) => instance.source.mode === "local");
  if (!localInstances.length) {
    return undefined;
  }
  if (localInstances.some((instance) => !instance.localServices?.length)) {
    return { group: "coprocessor" };
  }
  return {
    group: "coprocessor",
    services: [...new Set(localInstances.flatMap((instance) => instance.localServices ?? []))],
  };
};

/** Merges explicit overrides with scenario-implied local coprocessor overrides. */
export const effectiveOverrides = (
  overrides: LocalOverride[],
  scenario: Pick<ResolvedCoprocessorScenario, "instances">,
): LocalOverride[] => {
  const local = localCoprocessorOverride(scenario);
  if (!local) {
    return overrides;
  }
  const existing = overrides.find((override) => override.group === "coprocessor");
  if (!existing) {
    return [...overrides, local];
  }
  if (!existing.services?.length || !local.services?.length) {
    return overrides.map((override) => (override.group === "coprocessor" ? { group: "coprocessor" } : override));
  }
  const existingServices = existing.services ?? [];
  const localServices = local.services ?? [];
  return overrides.map((override) =>
    override.group === "coprocessor"
      ? { group: "coprocessor", services: [...new Set([...existingServices, ...localServices])] }
      : override,
  );
};

/** Detects whether a scenario explicitly owns coprocessor image sourcing. */
const scenarioOwnsCoprocessorSource = (
  scenario: Pick<ResolvedCoprocessorScenario, "instances">,
  group: OverrideGroup,
) => group === "coprocessor" && scenario.instances.some((instance) => instance.source.mode !== "inherit");

/** Rejects override combinations that conflict with scenario-owned coprocessor source. */
export const assertScenarioOverrideCompatibility = (
  scenario: Pick<ResolvedCoprocessorScenario, "instances" | "sourcePath">,
  overrides: LocalOverride[],
) => {
  const conflicting = overrides.find((override) => scenarioOwnsCoprocessorSource(scenario, override.group));
  if (!conflicting) {
    return;
  }
  const target = conflicting.services?.length
    ? `${conflicting.group}:${conflicting.services.map((service) => service.replace(/^coprocessor-/, "")).join(",")}`
    : conflicting.group;
  throw new PreflightError(
    `--override ${target} conflicts with scenario-defined coprocessor source${scenario.sourcePath ? ` in ${scenario.sourcePath}` : ""}; remove the override or make the scenario inherit coprocessor source`,
  );
};

/** Synthesizes a local single-instance scenario from coprocessor override shorthand. */
export const synthesizeOverrideScenario = (overrides: LocalOverride[]): ResolvedCoprocessorScenario | undefined => {
  if (!overrides.some((item) => item.group === "coprocessor")) {
    return undefined;
  }
  return {
    version: COPROCESSOR_SCENARIO_VERSION,
    kind: COPROCESSOR_SCENARIO_KIND,
    origin: "override-shorthand",
    name: "Override Shorthand",
    description: "Single local coprocessor instance synthesized from --override coprocessor.",
    topology: { count: 1, threshold: 1 },
    instances: [{ index: 0, source: { mode: "local" }, env: {}, args: {}, localServices: mergeOverrideServices(overrides) }],
  };
};

/** Reports whether a resolved scenario includes any local coprocessor instances. */
export const hasLocalCoprocessorInstance = (
  state: Pick<ResolvedCoprocessorScenario, "instances"> | { scenario: ResolvedCoprocessorScenario },
) => ("scenario" in state ? state.scenario.instances : state.instances).some((instance) => instance.source.mode === "local");

/** Lists bundled scenarios with their summary metadata for CLI discovery. */
export const listScenarioSummaries = async (): Promise<ScenarioSummary[]> => {
  try {
    const entries = await fs.readdir(COPROCESSOR_SCENARIO_DIR, { withFileTypes: true });
    const files = entries
      .filter((entry) => entry.isFile() && SCENARIO_FILE.test(entry.name))
      .map((entry) => entry.name)
      .sort();
    return await Promise.all(
      files.map(async (fileName) => {
        const filePath = path.join(COPROCESSOR_SCENARIO_DIR, fileName);
        const parsed = parseCoprocessorScenario(await fs.readFile(filePath, "utf8"), filePath);
        return {
          key: fileName.replace(SCENARIO_FILE, ""),
          filePath,
          name: parsed.name,
          description: parsed.description,
        } satisfies ScenarioSummary;
      }),
    );
  } catch (error) {
    throw new PreflightError(error instanceof Error ? error.message : String(error));
  }
};
