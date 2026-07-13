import type { RequestLoadStage } from "../flows/types";
import type { FlowMix, RateShape } from "../scenario/schema";

/**
 * Arrival schedule for the open model: submission times are fixed up front
 * by the target rate, never by response latency, so late responses cannot
 * reduce the offered load.
 */

/** Yields arrival offsets in milliseconds from run start. */
export function* arrivalOffsetsMs(shape: RateShape): Generator<number> {
  switch (shape.kind) {
    case "constant": {
      const count = Math.floor(shape.rps * shape.durationSec);
      for (let i = 0; i < count; i += 1) yield (i / shape.rps) * 1000;
      return;
    }
    case "segments": {
      let baseMs = 0;
      for (const segment of shape.segments) {
        const { fromRps, toRps, durationSec } = segment;
        const count = Math.floor(((fromRps + toRps) / 2) * durationSec);
        // Cumulative arrivals inside the segment: N(t) = f*t + (g-f)*t^2/(2D).
        // Arrival n is the inverse; linear when the rate is flat.
        const a = (toRps - fromRps) / (2 * durationSec);
        for (let n = 0; n < count; n += 1) {
          const t =
            Math.abs(a) < 1e-12
              ? n / fromRps
              : (-fromRps + Math.sqrt(fromRps * fromRps + 4 * a * n)) / (2 * a);
          yield baseMs + t * 1000;
        }
        baseMs += durationSec * 1000;
      }
      return;
    }
    case "burst": {
      const spacingMs = shape.maxRps ? 1000 / shape.maxRps : 0;
      for (let i = 0; i < shape.count; i += 1) yield i * spacingMs;
      return;
    }
  }
}

/** End offsets (ms) of each shape segment; used for saturation feedback. */
export const segmentBoundariesMs = (shape: RateShape): number[] => {
  if (shape.kind !== "segments") return [];
  const boundaries: number[] = [];
  let baseMs = 0;
  for (const segment of shape.segments) {
    baseMs += segment.durationSec * 1000;
    boundaries.push(baseMs);
  }
  return boundaries;
};

export type ClosedStageOffsets = Readonly<{
  index: number;
  loadStage: RequestLoadStage;
  vus: number;
  startMs: number;
  endMs: number;
}>;

/** Closed-model VU stages: fixed active client loops over each time window. */
export const closedStageOffsetsMs = (shape: Extract<RateShape, { kind: "closed" }>): ClosedStageOffsets[] => {
  if (shape.stages) {
    let startMs = 0;
    return shape.stages.map((stage, index) => {
      const current = {
        index,
        loadStage: {
          index,
          label: `${stage.vus.toString()}vu`,
          model: "closed" as const,
          startOffsetMs: startMs,
          endOffsetMs: startMs + stage.durationSec * 1000,
          vus: stage.vus,
        },
        vus: stage.vus,
        startMs,
        endMs: startMs + stage.durationSec * 1000,
      };
      startMs = current.endMs;
      return current;
    });
  }
  return [
    {
      index: 0,
      loadStage: {
        index: 0,
        label: `${(shape.vus ?? 0).toString()}vu`,
        model: "closed",
        startOffsetMs: 0,
        endOffsetMs: (shape.durationSec ?? 0) * 1000,
        vus: shape.vus ?? 0,
      },
      vus: shape.vus ?? 0,
      startMs: 0,
      endMs: (shape.durationSec ?? 0) * 1000,
    },
  ];
};

export const openLoadStageForOffset = (
  shape: Exclude<RateShape, { kind: "closed" }>,
  offsetMs: number,
): RequestLoadStage => {
  switch (shape.kind) {
    case "constant":
      return {
        index: 0,
        label: `${shape.rps.toString()}rps`,
        model: "open",
        startOffsetMs: 0,
        endOffsetMs: shape.durationSec * 1000,
        targetRps: shape.rps,
      };
    case "burst":
      return {
        index: 0,
        label: shape.maxRps ? `burst-${shape.maxRps.toString()}rps` : "burst",
        model: "drain",
        startOffsetMs: 0,
        targetRps: shape.maxRps,
      };
    case "segments": {
      let startMs = 0;
      for (let index = 0; index < shape.segments.length; index += 1) {
        const segment = shape.segments[index];
        if (!segment) break;
        const endMs = startMs + segment.durationSec * 1000;
        if (offsetMs < endMs || index === shape.segments.length - 1) {
          const flat = segment.fromRps === segment.toRps;
          return {
            index,
            label: flat
              ? `${segment.fromRps.toString()}rps`
              : `${segment.fromRps.toString()}-${segment.toRps.toString()}rps`,
            model: "open",
            startOffsetMs: startMs,
            endOffsetMs: endMs,
            targetRps: flat ? segment.fromRps : undefined,
            fromRps: segment.fromRps,
            toRps: segment.toRps,
          };
        }
        startMs = endMs;
      }
      return { index: 0, label: "segment-0", model: "open" };
    }
  }
};

/**
 * Deterministic weighted flow interleaving (smooth weighted round-robin):
 * reproducible run-to-run, unlike random draws, while converging on the
 * requested ratio over any window.
 */
export const createFlowSequencer = (mix: readonly FlowMix[]): (() => FlowMix) => {
  const totalWeight = mix.reduce((total, entry) => total + entry.weight, 0);
  const credits = mix.map(() => 0);
  return () => {
    let best = 0;
    for (let i = 0; i < mix.length; i += 1) {
      const entry = mix[i];
      if (entry === undefined) continue;
      credits[i] = (credits[i] ?? 0) + entry.weight;
      if ((credits[i] ?? 0) > (credits[best] ?? 0)) best = i;
    }
    credits[best] = (credits[best] ?? 0) - totalWeight;
    const chosen = mix[best];
    if (!chosen) throw new Error("Flow mix is empty.");
    return chosen;
  };
};
