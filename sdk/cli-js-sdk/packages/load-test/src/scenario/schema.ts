import { z } from "zod";

import { FLOWS } from "../relayer/types";
import { artifactSlugSchema } from "../shared/paths";

/**
 * Scenario model: flow mix, load shape, duration, and pass/fail thresholds.
 * Scenarios are data, not code — built-ins live in `builtin.ts` and custom
 * scenarios load from JSON files.
 */

export const flowKindSchema = z.enum(FLOWS);

export const flowMixSchema = z.object({
  flow: flowKindSchema,
  /** Relative weight in the arrival mix; weights need not sum to 1. */
  weight: z.number().positive(),
  /** Handles per decrypt request (combination size k for public-decrypt). */
  handlesPerRequest: z.number().int().min(1).max(16).default(1),
});

export const closedStageSchema = z.object({
  /** Active client loops during this stage. */
  vus: z.number().int().positive(),
  durationSec: z.number().positive(),
});

/** Load shapes. `constant`/`segments` are open model; `closed` fixes active clients. */
export const rateShapeSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("constant"),
    rps: z.number().positive(),
    durationSec: z.number().positive(),
  }),
  z.object({
    kind: z.literal("segments"),
    segments: z
      .array(
        z.object({
          fromRps: z.number().min(0),
          toRps: z.number().min(0),
          durationSec: z.number().positive(),
        }),
      )
      .min(1),
  }),
  /**
   * Closed-model backlog scenario: submit `count` requests as fast as
   * `maxRps` allows, then poll everything to completion.
   */
  z.object({
    kind: z.literal("burst"),
    count: z.number().int().positive(),
    maxRps: z.number().positive().optional(),
  }),
  z
    .object({
      kind: z.literal("closed"),
      /** Fixed active clients for steady closed-model runs. */
      vus: z.number().int().positive().optional(),
      /** Duration for fixed-VU closed runs. */
      durationSec: z.number().positive().optional(),
      /** VU stages for ramp/spike-style closed runs. */
      stages: z.array(closedStageSchema).min(1).optional(),
      /** Pause after one full request workflow before the VU starts another. */
      thinkTimeMs: z.number().min(0).default(0),
      /** Optional hard cap; required for single-use pools in closed scenarios. */
      maxIterations: z.number().int().positive().optional(),
    })
    .superRefine((shape, ctx) => {
      const hasSteady = shape.vus !== undefined || shape.durationSec !== undefined;
      if (shape.stages && hasSteady) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "closed shape must use either vus+durationSec or stages, not both",
        });
      }
      if (!shape.stages && (shape.vus === undefined || shape.durationSec === undefined)) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "closed shape requires vus and durationSec when stages are absent",
        });
      }
    }),
]);

export const flowThresholdSchema = z.object({
  e2eP95Ms: z.number().positive().optional(),
  e2eP99Ms: z.number().positive().optional(),
  maxErrorRate: z.number().min(0).max(1).optional(),
});

export const thresholdsSchema = z.object({
  /** Failed + timed-out requests over submitted, across all flows. */
  maxErrorRate: z.number().min(0).max(1).default(0.01),
  /** Decrypt results that did not match known plaintexts. Always strict. */
  maxVerifyFailures: z.number().int().min(0).default(0),
  perFlow: z.partialRecord(flowKindSchema, flowThresholdSchema).default({}),
});

export const saturationStopSchema = z.object({
  /** Stop a stepped ramp once queue depth grows for this many consecutive steps. */
  enabled: z.boolean().default(false),
  consecutiveSteps: z.number().int().min(1).default(2),
  /** Minimum queued-row growth per step considered "growing". */
  minQueueGrowth: z.number().int().min(1).default(10),
});

export const scenarioSchema = z
  .object({
    name: artifactSlugSchema,
    description: z.string().default(""),
    flows: z.array(flowMixSchema).min(1),
    shape: rateShapeSchema,
    /** Overall per-request deadline (submit → terminal status). */
    requestTimeoutSec: z.number().positive().default(600),
    /** Grace period after the last submission for outstanding pollers. */
    drainTimeoutSec: z.number().positive().default(900),
    thresholds: thresholdsSchema.default({
      maxErrorRate: 0.01,
      maxVerifyFailures: 0,
      perFlow: {},
    }),
    saturationStop: saturationStopSchema.default({
      enabled: false,
      consecutiveSteps: 2,
      minQueueGrowth: 10,
    }),
  })
  .superRefine((scenario, context) => {
    const seen = new Set<string>();
    for (const [index, mix] of scenario.flows.entries()) {
      if (seen.has(mix.flow)) {
        context.addIssue({
          code: z.ZodIssueCode.custom,
          path: ["flows", index, "flow"],
          message: `Duplicate flow "${mix.flow}"; each flow may appear only once per scenario`,
        });
      }
      seen.add(mix.flow);
    }
  });

export type FlowMix = z.infer<typeof flowMixSchema>;
export type RateShape = z.infer<typeof rateShapeSchema>;
export type Thresholds = z.infer<typeof thresholdsSchema>;
export type Scenario = z.infer<typeof scenarioSchema>;
export type ScenarioInput = z.input<typeof scenarioSchema>;

/**
 * Total submissions implied by a shape when a finite budget is known.
 * Closed-model scenarios without maxIterations are duration-bound, not
 * request-count-bound, so their final count is known only after execution.
 */
export const plannedRequestCount = (shape: RateShape): number | undefined => {
  switch (shape.kind) {
    case "constant":
      return Math.floor(shape.rps * shape.durationSec);
    case "segments":
      return shape.segments.reduce(
        (total, segment) =>
          total + Math.floor(((segment.fromRps + segment.toRps) / 2) * segment.durationSec),
        0,
      );
    case "burst":
      return shape.count;
    case "closed":
      return shape.maxIterations;
  }
};

/** Wall-clock duration of the submission phase, in seconds (undefined for burst). */
export const shapeDurationSec = (shape: RateShape): number | undefined => {
  switch (shape.kind) {
    case "constant":
      return shape.durationSec;
    case "segments":
      return shape.segments.reduce((total, segment) => total + segment.durationSec, 0);
    case "burst":
      return undefined;
    case "closed":
      return shape.stages
        ? shape.stages.reduce((total, stage) => total + stage.durationSec, 0)
        : shape.durationSec;
  }
};

export const shapeModel = (shape: RateShape): "open" | "closed" | "drain" => {
  switch (shape.kind) {
    case "constant":
    case "segments":
      return "open";
    case "closed":
      return "closed";
    case "burst":
      return "drain";
  }
};
