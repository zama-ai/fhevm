import { z } from "zod";

import { flowKindSchema, scenarioSchema, type Scenario } from "./schema";

/** Overrides shared by standalone runs and suite entries. */
export const scenarioOverrideSchema = z.object({
  rps: z.number().positive().optional(),
  vus: z.number().int().positive().optional(),
  thinkTimeMs: z.number().min(0).optional(),
  durationSec: z.number().positive().optional(),
  count: z.number().int().positive().optional(),
  maxIterations: z.number().int().positive().optional(),
  flow: flowKindSchema.optional(),
}).strict();

export type ScenarioOverrides = Readonly<z.infer<typeof scenarioOverrideSchema>>;

const rejectUnsupported = (
  scenario: Scenario,
  overrides: ScenarioOverrides,
  allowed: readonly (keyof ScenarioOverrides)[],
): void => {
  const unsupported = Object.entries(overrides)
    .filter(([key, value]) => value !== undefined && !allowed.includes(key as keyof ScenarioOverrides))
    .map(([key]) => key);
  if (unsupported.length > 0) {
    throw new Error(
      `Scenario "${scenario.name}" uses a ${scenario.shape.kind} shape; unsupported override(s): ` +
        `${unsupported.join(", ")}.`,
    );
  }
};

/** Applies validated model-aware overrides to a fully resolved scenario. */
export const applyScenarioOverrides = (
  scenario: Scenario,
  input: ScenarioOverrides = {},
): Scenario => {
  const overrides = scenarioOverrideSchema.parse(input);
  if (overrides.flow !== undefined && scenario.flows.length !== 1) {
    throw new Error(
      `Scenario "${scenario.name}" has ${scenario.flows.length.toString()} flows; ` +
        "--flow can only override a single-flow scenario.",
    );
  }
  const flows = overrides.flow === undefined
    ? scenario.flows
    : [{ ...scenario.flows[0]!, flow: overrides.flow }];

  switch (scenario.shape.kind) {
    case "constant": {
      rejectUnsupported(scenario, overrides, ["rps", "durationSec", "flow"]);
      return scenarioSchema.parse({
        ...scenario,
        flows,
        shape: {
          ...scenario.shape,
          rps: overrides.rps ?? scenario.shape.rps,
          durationSec: overrides.durationSec ?? scenario.shape.durationSec,
        },
      });
    }
    case "segments": {
      rejectUnsupported(scenario, overrides, ["rps", "durationSec", "flow"]);
      const baseRate = scenario.shape.segments
        .flatMap((segment) => [segment.fromRps, segment.toRps])
        .find((rate) => rate > 0);
      if (overrides.rps !== undefined && baseRate === undefined) {
        throw new Error(
          `Scenario "${scenario.name}" has no positive segment rate to scale with --rps.`,
        );
      }
      const scale = overrides.rps === undefined ? 1 : overrides.rps / (baseRate ?? 1);
      return scenarioSchema.parse({
        ...scenario,
        flows,
        shape: {
          kind: "segments",
          segments: scenario.shape.segments.map((segment) => ({
            fromRps: segment.fromRps * scale,
            toRps: segment.toRps * scale,
            durationSec: overrides.durationSec ?? segment.durationSec,
          })),
        },
      });
    }
    case "burst": {
      rejectUnsupported(scenario, overrides, ["rps", "count", "flow"]);
      return scenarioSchema.parse({
        ...scenario,
        flows,
        shape: {
          ...scenario.shape,
          count: overrides.count ?? scenario.shape.count,
          maxRps: overrides.rps ?? scenario.shape.maxRps,
        },
      });
    }
    case "closed": {
      rejectUnsupported(scenario, overrides, [
        "vus", "thinkTimeMs", "durationSec", "maxIterations", "flow",
      ]);
      const closedShape = scenario.shape;
      const firstStageVus = closedShape.stages?.[0]?.vus;
      const shape = closedShape.stages
        ? {
            ...closedShape,
            stages: closedShape.stages.map((stage) => ({
              vus: overrides.vus === undefined
                ? stage.vus
                : Math.max(1, Math.round(stage.vus * overrides.vus / (firstStageVus ?? 1))),
              durationSec: overrides.durationSec ?? stage.durationSec,
            })),
            thinkTimeMs: overrides.thinkTimeMs ?? closedShape.thinkTimeMs,
            maxIterations: overrides.maxIterations ?? closedShape.maxIterations,
          }
        : {
            ...closedShape,
            vus: overrides.vus ?? closedShape.vus,
            durationSec: overrides.durationSec ?? closedShape.durationSec,
            thinkTimeMs: overrides.thinkTimeMs ?? closedShape.thinkTimeMs,
            maxIterations: overrides.maxIterations ?? closedShape.maxIterations,
          };
      return scenarioSchema.parse({ ...scenario, flows, shape });
    }
  }
};
