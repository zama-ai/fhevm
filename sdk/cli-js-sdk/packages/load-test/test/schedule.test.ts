import { describe, expect, it } from "vitest";

import {
  arrivalOffsetsMs,
  closedStageOffsetsMs,
  createFlowSequencer,
  segmentBoundariesMs,
} from "../src/runner/schedule";
import type { RateShape } from "../src/scenario/schema";

describe("arrivalOffsetsMs", () => {
  it("spaces constant-rate arrivals uniformly", () => {
    const offsets = [...arrivalOffsetsMs({ kind: "constant", rps: 10, durationSec: 1 })];
    expect(offsets).toHaveLength(10);
    expect(offsets[0]).toBe(0);
    expect(offsets[1]).toBeCloseTo(100, 6);
    expect(offsets[9]).toBeCloseTo(900, 6);
  });

  it("produces the trapezoid count for a linear ramp", () => {
    const shape: RateShape = {
      kind: "segments",
      segments: [{ fromRps: 0, toRps: 10, durationSec: 10 }],
    };
    const offsets = [...arrivalOffsetsMs(shape)];
    // Average rate 5 rps over 10 s.
    expect(offsets).toHaveLength(50);
    // Monotonically non-decreasing within the window.
    for (let i = 1; i < offsets.length; i += 1) {
      expect(offsets[i]).toBeGreaterThanOrEqual(offsets[i - 1] ?? 0);
    }
    expect(offsets.at(-1)).toBeLessThanOrEqual(10_000);
  });

  it("concatenates stepped segments with correct bases", () => {
    const shape: RateShape = {
      kind: "segments",
      segments: [
        { fromRps: 1, toRps: 1, durationSec: 2 },
        { fromRps: 2, toRps: 2, durationSec: 2 },
      ],
    };
    const offsets = [...arrivalOffsetsMs(shape)];
    expect(offsets).toHaveLength(2 + 4);
    expect(offsets.slice(0, 2)).toEqual([0, 1000]);
    expect(offsets.slice(2)).toEqual([2000, 2500, 3000, 3500]);
  });

  it("spaces burst arrivals by the rate cap", () => {
    const offsets = [...arrivalOffsetsMs({ kind: "burst", count: 4, maxRps: 100 })];
    expect(offsets).toEqual([0, 10, 20, 30]);
  });

  it("fires bursts back-to-back without a cap", () => {
    const offsets = [...arrivalOffsetsMs({ kind: "burst", count: 3 })];
    expect(offsets).toEqual([0, 0, 0]);
  });
});

describe("segmentBoundariesMs", () => {
  it("returns cumulative segment ends", () => {
    expect(
      segmentBoundariesMs({
        kind: "segments",
        segments: [
          { fromRps: 1, toRps: 1, durationSec: 60 },
          { fromRps: 2, toRps: 2, durationSec: 30 },
        ],
      }),
    ).toEqual([60_000, 90_000]);
  });

  it("is empty for non-segment shapes", () => {
    expect(segmentBoundariesMs({ kind: "constant", rps: 1, durationSec: 60 })).toEqual([]);
  });
});

describe("closedStageOffsetsMs", () => {
  it("returns one fixed-VU window for steady closed shapes", () => {
    expect(
      closedStageOffsetsMs({ kind: "closed", vus: 5, durationSec: 60, thinkTimeMs: 0 }),
    ).toEqual([
      {
        index: 0,
        loadStage: {
          index: 0,
          label: "5vu",
          model: "closed",
          startOffsetMs: 0,
          endOffsetMs: 60_000,
          vus: 5,
        },
        vus: 5,
        startMs: 0,
        endMs: 60_000,
      },
    ]);
  });

  it("returns cumulative windows for staged closed shapes", () => {
    expect(
      closedStageOffsetsMs({
        kind: "closed",
        stages: [
          { vus: 2, durationSec: 30 },
          { vus: 4, durationSec: 15 },
        ],
        thinkTimeMs: 0,
      }),
    ).toEqual([
      {
        index: 0,
        loadStage: {
          index: 0,
          label: "2vu",
          model: "closed",
          startOffsetMs: 0,
          endOffsetMs: 30_000,
          vus: 2,
        },
        vus: 2,
        startMs: 0,
        endMs: 30_000,
      },
      {
        index: 1,
        loadStage: {
          index: 1,
          label: "4vu",
          model: "closed",
          startOffsetMs: 30_000,
          endOffsetMs: 45_000,
          vus: 4,
        },
        vus: 4,
        startMs: 30_000,
        endMs: 45_000,
      },
    ]);
  });
});

describe("createFlowSequencer", () => {
  it("respects weights over a window", () => {
    const next = createFlowSequencer([
      { flow: "input-proof", weight: 6, handlesPerRequest: 1 },
      { flow: "user-decrypt", weight: 3, handlesPerRequest: 1 },
      { flow: "public-decrypt", weight: 1, handlesPerRequest: 1 },
    ]);
    const counts = new Map<string, number>();
    for (let i = 0; i < 1000; i += 1) {
      const flow = next().flow;
      counts.set(flow, (counts.get(flow) ?? 0) + 1);
    }
    expect(counts.get("input-proof")).toBe(600);
    expect(counts.get("user-decrypt")).toBe(300);
    expect(counts.get("public-decrypt")).toBe(100);
  });

  it("interleaves smoothly rather than in blocks", () => {
    const next = createFlowSequencer([
      { flow: "input-proof", weight: 1, handlesPerRequest: 1 },
      { flow: "user-decrypt", weight: 1, handlesPerRequest: 1 },
    ]);
    const sequence = Array.from({ length: 4 }, () => next().flow);
    expect(new Set(sequence.slice(0, 2)).size).toBe(2);
  });
});
