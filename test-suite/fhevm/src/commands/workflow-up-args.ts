import fs from "node:fs/promises";

import { Effect } from "effect";
import YAML from "yaml";

import { PreflightError } from "../errors";
import { GROUP_SERVICE_SUFFIXES, resolveServiceOverrides } from "../layout";
import { parseLocalOverride } from "../options";
import { COPROCESSOR_SCENARIO_KIND, COPROCESSOR_SCENARIO_VERSION } from "../scenario";
import { OVERRIDE_GROUPS, type LocalOverride } from "../types";

type WorkflowImageMode = "registry" | "workspace";

const WORKFLOW_DEFAULT_SCENARIO = "./scenarios/two-of-two.yaml";

const normalizeWorkflowSelections = (value: string) => {
  const tokens = value
    .split(",")
    .map((token) => token.trim())
    .filter(Boolean);
  const startsSelection = (token: string) =>
    token === "all" ||
    token === "build" ||
    OVERRIDE_GROUPS.includes(token as (typeof OVERRIDE_GROUPS)[number]) ||
    token.includes(":");
  const selections: string[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    let token = tokens[index];
    if (!startsSelection(token)) {
      throw new Error(`Unsupported override ${token}`);
    }
    while (
      token.includes(":") &&
      index + 1 < tokens.length &&
      !startsSelection(tokens[index + 1])
    ) {
      index += 1;
      token = `${token},${tokens[index]}`;
    }
    selections.push(token);
  }
  return selections;
};

const overrideSuffixes = (override: LocalOverride) => {
  if (!override.services?.length) {
    return undefined;
  }
  const remaining = new Set(override.services);
  const suffixes: string[] = [];
  for (const suffix of GROUP_SERVICE_SUFFIXES[override.group]) {
    const resolved = resolveServiceOverrides(override.group, [suffix]);
    if (!resolved.every((service) => remaining.has(service))) {
      continue;
    }
    suffixes.push(suffix);
    for (const service of resolved) {
      remaining.delete(service);
    }
  }
  if (remaining.size) {
    throw new Error(`Failed to serialize override ${override.group}`);
  }
  return suffixes;
};

const coprocessorLocalServices = (overrides: LocalOverride[]) => {
  const local = overrides.filter((override) => override.group === "coprocessor");
  if (!local.length) {
    return undefined;
  }
  if (local.some((override) => !override.services?.length)) {
    return [];
  }
  return [...new Set(local.flatMap((override) => overrideSuffixes(override) ?? []))];
};

const overrideArg = (override: LocalOverride) =>
  !override.services?.length
    ? override.group
    : `${override.group}:${(overrideSuffixes(override) ?? []).join(",")}`;

const writeWorkflowScenario = (
  filePath: string,
  localServices: string[] | undefined,
) =>
  fs.writeFile(
    filePath,
    YAML.stringify({
      version: COPROCESSOR_SCENARIO_VERSION,
      kind: COPROCESSOR_SCENARIO_KIND,
      topology: { count: 2, threshold: 2 },
      instances: [
        {
          index: 1,
          source: { mode: "local" },
          ...(localServices?.length ? { localServices } : {}),
        },
      ],
    }),
  );

export const workflowUpArgs = (options: {
  imageMode: string;
  override: string | undefined;
  scenarioOut: string | undefined;
}) =>
  Effect.gen(function* () {
    if (!["registry", "workspace"].includes(options.imageMode)) {
      return yield* Effect.fail(
        new PreflightError({ message: "image_mode must be registry or workspace" }),
      );
    }
    const imageMode = options.imageMode as WorkflowImageMode;
    const override = options.override?.trim() ?? "";
    if (imageMode === "registry" && override) {
      return yield* Effect.fail(
        new PreflightError({ message: "override requires image_mode=workspace" }),
      );
    }
    if (imageMode === "workspace" && !override) {
      return yield* Effect.fail(
        new PreflightError({ message: "image_mode=workspace requires override" }),
      );
    }

    let forceModernRelayer = false;
    let scenarioPath = WORKFLOW_DEFAULT_SCENARIO;
    const args: string[] = ["--target", "latest-main"];
    if (imageMode === "workspace") {
      const selections = yield* Effect.try({
        try: () => normalizeWorkflowSelections(override),
        catch: (error) => new PreflightError({ message: (error as Error).message }),
      });
      if (!selections.length) {
        return yield* Effect.fail(
          new PreflightError({ message: "override did not contain any usable values" }),
        );
      }
      forceModernRelayer = selections.includes("all") || selections.includes("build");
      const overrides = yield* Effect.try({
        try: () =>
          selections
            .filter((selection) => selection !== "all" && selection !== "build")
            .flatMap(parseLocalOverride),
        catch: (error) => new PreflightError({ message: (error as Error).message }),
      });
      const localServices = coprocessorLocalServices(overrides);
      const coprocessorLocal = forceModernRelayer || localServices !== undefined;
      if (coprocessorLocal && !options.scenarioOut) {
        return yield* Effect.fail(
          new PreflightError({ message: "coprocessor workspace overrides require --scenario-out" }),
        );
      }
      if (coprocessorLocal) {
        yield* Effect.tryPromise({
          try: () => writeWorkflowScenario(options.scenarioOut!, localServices?.length ? localServices : undefined),
          catch: (error) =>
            new PreflightError({
              message: `Failed to write workflow scenario: ${error}`,
            }),
        });
        scenarioPath = options.scenarioOut!;
      }
      for (const resolved of overrides) {
        if (resolved.group === "coprocessor") {
          continue;
        }
        args.push("--override", overrideArg(resolved));
      }
      if (forceModernRelayer) {
        args.push("--build");
      }
    }
    args.push("--scenario", scenarioPath);
    console.log(JSON.stringify({ args, forceModernRelayer }, null, 2));
  });
