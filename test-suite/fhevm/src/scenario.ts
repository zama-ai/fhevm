import fs from "node:fs/promises";
import path from "node:path";

import { Effect } from "effect";
import YAML from "yaml";

import { PreflightError } from "./errors";
import {
  GROUP_BUILD_SERVICES,
  GROUP_SERVICE_SUFFIXES,
  MAX_COPROCESSOR_INSTANCES,
  REPO_ROOT,
  resolveServiceOverrides,
} from "./layout";
import type {
  CoprocessorInstanceSource,
  CoprocessorScenario,
  LocalOverride,
  ResolvedCoprocessorScenario,
  ResolvedCoprocessorScenarioInstance,
} from "./types";

const COPROCESSOR_SCENARIO_KIND = "coprocessor-consensus";
const COPROCESSOR_SCENARIO_VERSION = 1;
const COPROCESSOR_SCENARIO_DIR = path.join(
  REPO_ROOT,
  "test-suite",
  "fhevm",
  "scenarios",
);

const COPROCESSOR_ARG_TARGETS = new Set([
  "*",
  ...GROUP_SERVICE_SUFFIXES["coprocessor"].filter((value) => !value.includes("migration")),
]);

const normalizeScalar = (value: unknown, label: string) => {
  if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  throw new Error(`${label} must be a string, number, or boolean`);
};

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

const normalizeSource = (
  source: CoprocessorInstanceSource | undefined,
) => source ?? { mode: "inherit" as const };

const validateInstanceArgs = (
  args: Record<string, string[]> | undefined,
  label: string,
) => {
  for (const key of Object.keys(args ?? {})) {
    if (!COPROCESSOR_ARG_TARGETS.has(key)) {
      throw new Error(
        `${label}: unknown arg target "${key}". Valid: ${[...COPROCESSOR_ARG_TARGETS].join(", ")}`,
      );
    }
  }
};

export const defaultCoprocessorScenario = (): ResolvedCoprocessorScenario => ({
  version: COPROCESSOR_SCENARIO_VERSION,
  kind: COPROCESSOR_SCENARIO_KIND,
  origin: "default",
  topology: { count: 1, threshold: 1 },
  instances: [
    {
      index: 0,
      source: { mode: "inherit" },
      env: {},
      args: {},
    },
  ],
});

export const parseCoprocessorScenario = (
  text: string,
  sourceLabel = "scenario",
): CoprocessorScenario => {
  const parsed = YAML.parse(text) as Record<string, unknown> | null;
  if (!parsed || typeof parsed !== "object") {
    throw new Error(`${sourceLabel}: expected a YAML object`);
  }
  if (parsed.version !== COPROCESSOR_SCENARIO_VERSION) {
    throw new Error(
      `${sourceLabel}: expected version ${COPROCESSOR_SCENARIO_VERSION}`,
    );
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
      throw new Error(
        `${sourceLabel}: instances[${offset}].index must be between 0 and ${count - 1}`,
      );
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
          throw new Error(
            `${sourceLabel}: instances[${offset}].source.tag is required for registry mode`,
          );
        }
        normalizedSource = { mode, tag };
      } else {
        throw new Error(
          `${sourceLabel}: instances[${offset}].source.mode must be inherit, local, or registry`,
        );
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
    const normalizedArgs = normalizeArgs(
      args as Record<string, unknown> | undefined,
      `${sourceLabel}: instances[${offset}].args`,
    );
    validateInstanceArgs(normalizedArgs, `${sourceLabel}: instances[${offset}]`);
    const normalizedLocalServices = localServices
      ? resolveServiceOverrides(
          "coprocessor",
          localServices.map((value, serviceIndex) =>
            normalizeScalar(
              value,
              `${sourceLabel}: instances[${offset}].localServices[${serviceIndex}]`,
            ),
          ),
        )
      : undefined;
    if (normalizedLocalServices && normalizedSource?.mode !== "local") {
      throw new Error(
        `${sourceLabel}: instances[${offset}].localServices requires source.mode=local`,
      );
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
  return {
    version: COPROCESSOR_SCENARIO_VERSION,
    kind: COPROCESSOR_SCENARIO_KIND,
    topology: { count, threshold },
    instances,
  };
};

export const loadCoprocessorScenario = (
  filePath: string,
): Effect.Effect<CoprocessorScenario, PreflightError> =>
  Effect.tryPromise({
    try: async () => {
      const absolute = path.resolve(filePath);
      const text = await fs.readFile(absolute, "utf8");
      return parseCoprocessorScenario(text, absolute);
    },
    catch: (error) =>
      new PreflightError({
        message: error instanceof Error ? error.message : String(error),
      }),
  });

export const resolveScenarioFile = (
  filePath: string,
  input: CoprocessorScenario,
): ResolvedCoprocessorScenario => {
  const byIndex = new Map(
    (input.instances ?? []).map((instance) => [instance.index, instance] as const),
  );
  return {
    version: COPROCESSOR_SCENARIO_VERSION,
    kind: COPROCESSOR_SCENARIO_KIND,
    origin: "file",
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

const localCoprocessorOverride = (
  scenario: Pick<ResolvedCoprocessorScenario, "instances">,
): LocalOverride | undefined => {
  const localInstances = scenario.instances.filter(
    (instance) => instance.source.mode === "local",
  );
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
    return overrides.map((override) =>
      override.group === "coprocessor" ? { group: "coprocessor" } : override,
    );
  }
  const existingServices = existing.services ?? [];
  const localServices = local.services ?? [];
  return overrides.map((override) =>
    override.group === "coprocessor"
      ? {
          group: "coprocessor",
          services: [...new Set([...existingServices, ...localServices])],
        }
      : override,
  );
};

export const synthesizeOverrideScenario = (
  overrides: LocalOverride[],
): ResolvedCoprocessorScenario | undefined => {
  const hasCoprocessorOverride = overrides.some((item) => item.group === "coprocessor");
  if (!hasCoprocessorOverride) {
    return undefined;
  }
  return {
    version: COPROCESSOR_SCENARIO_VERSION,
    kind: COPROCESSOR_SCENARIO_KIND,
    origin: "override-shorthand",
    topology: { count: 1, threshold: 1 },
    instances: [
      {
        index: 0,
        source: { mode: "local" },
        env: {},
        args: {},
        localServices: mergeOverrideServices(overrides),
      },
    ],
  };
};

export const hasLocalCoprocessorInstance = (
  state: Pick<ResolvedCoprocessorScenario, "instances"> | { scenario: ResolvedCoprocessorScenario },
) =>
  ("scenario" in state ? state.scenario.instances : state.instances).some(
    (instance) => instance.source.mode === "local",
  );
