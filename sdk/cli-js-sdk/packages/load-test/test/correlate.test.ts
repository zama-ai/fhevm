import { describe, expect, it } from "vitest";

import type { StageRow } from "../src/collectors/stage-rows";
import type { RequestRecord } from "../src/flows/types";
import { buildCorrelation } from "../src/report/correlate";

const record = (over: Partial<RequestRecord>): RequestRecord => ({
  flow: "input-proof",
  index: 0,
  startedAtMs: 0,
  sentRequestId: "r",
  pollCount: 3,
  outcome: "succeeded",
  e2eLatencyMs: 9000,
  ...over,
});

const stageRow = (jobId: string, createdAt: string, completedAt: string): StageRow => ({
  flow: "input-proof",
  externalJobId: jobId,
  status: "completed",
  createdAt,
  completedAt,
  readinessAttemptCount: 0,
  broadcastAttemptCount: 1,
});

describe("buildCorrelation", () => {
  it("joins client and server e2e by job id and measures poll overhead", () => {
    const records = [
      record({ index: 0, jobId: "a", e2eLatencyMs: 9000 }),
      record({ index: 1, jobId: "b", e2eLatencyMs: 6000 }),
    ];
    const rows = [
      stageRow("a", "2026-06-15T00:00:00.000Z", "2026-06-15T00:00:07.000Z"), // server 7s
      stageRow("b", "2026-06-15T00:00:00.000Z", "2026-06-15T00:00:05.000Z"), // server 5s
    ];
    const [c] = buildCorrelation(records, rows);
    expect(c?.flow).toBe("input-proof");
    expect(c?.matched).toBe(2);
    expect(c?.serverE2e?.p50Ms).toBeGreaterThanOrEqual(5000);
    // overhead = client - server: 2000 and 1000.
    expect(c?.pollOverhead?.maxMs).toBeGreaterThanOrEqual(2000);
  });

  it("ignores unmatched, failed, and unfinished requests", () => {
    const records = [
      record({ index: 0, jobId: "a" }),
      record({ index: 1, jobId: "x" }), // no stage row
      record({ index: 2, jobId: "b", outcome: "failed" }),
    ];
    const rows = [
      stageRow("a", "2026-06-15T00:00:00.000Z", "2026-06-15T00:00:07.000Z"),
      stageRow("b", "2026-06-15T00:00:00.000Z", "2026-06-15T00:00:05.000Z"),
    ];
    const [c] = buildCorrelation(records, rows);
    expect(c?.matched).toBe(1);
  });

  it("returns nothing when no server timing is available", () => {
    expect(buildCorrelation([record({ jobId: "a" })], [])).toEqual([]);
  });

  it("clamps negative overhead (clock skew) to zero", () => {
    const records = [record({ jobId: "a", e2eLatencyMs: 4000 })];
    const rows = [stageRow("a", "2026-06-15T00:00:00.000Z", "2026-06-15T00:00:05.000Z")]; // server 5s > client 4s
    const [c] = buildCorrelation(records, rows);
    expect(c?.pollOverhead?.p50Ms).toBe(1); // clamped to ~0 (hist floor 1ms)
  });
});
