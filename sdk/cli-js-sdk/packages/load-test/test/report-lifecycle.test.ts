import { describe, expect, it } from "vitest";

import { buildReport } from "../src/report/build";
import { renderMarkdownReport } from "../src/report/render-md";
import { scenarioSchema } from "../src/scenario/schema";

describe("run report lifecycle status", () => {
  it("persists interrupted status in the machine-readable report", () => {
    const scenario = scenarioSchema.parse({
      name: "interrupted",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: { kind: "burst", count: 1 },
    });
    const report = buildReport({
      scenario,
      network: "testnet",
      relayerUrl: "https://relayer.example",
      startedAt: "2026-01-01T00:00:00.000Z",
      endedAt: "2026-01-01T00:00:01.000Z",
      records: [],
      submitted: 0,
      completed: 0,
      abandoned: 0,
      poolExhausted: false,
      submissionDurationMs: 1,
      interrupted: true,
      targets: [],
    });

    expect(report.run.status).toBe("interrupted");
    expect(renderMarkdownReport(report)).toContain("**Status:** interrupted");
  });
});
