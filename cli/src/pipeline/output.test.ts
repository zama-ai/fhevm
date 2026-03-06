import { afterAll, afterEach, beforeAll, describe, expect, test } from "bun:test";

import { __internal as jsonOutputInternal } from "../ci/json-output";
import type { BootState } from "./state";
import {
  createPipelineOutput,
  formatDuration,
  printDiscoveryResult,
  printParallelStepStart,
  printPipelineHeader,
  printPipelineSummary,
  printStepFail,
  printStepSkipped,
  printStepStart,
  printStepSuccess,
} from "./output";

const logs: string[] = [];
const originalLog = console.log;

beforeAll(() => {
  console.log = (...args: unknown[]) => {
    logs.push(args.join(" "));
  };
});

afterAll(() => {
  console.log = originalLog;
});

afterEach(() => {
  logs.length = 0;
});

describe("pipeline output", () => {
  test("formats durations", () => {
    expect(formatDuration(125)).toBe("0.1s");
    expect(formatDuration(2_450)).toBe("2.5s");
    expect(formatDuration(272_000)).toBe("4m 32s");
  });

  test("prints step lifecycle lines", () => {
    printPipelineHeader(13, { resume: true });
    printStepStart({ number: 1, displayName: "MinIO (S3 storage)" }, 13);
    printStepSuccess({ displayName: "MinIO (S3 storage)" }, 2_400);
    printDiscoveryResult("MinIO IP", "172.18.0.2");
    printStepFail({ displayName: "KMS Core" }, "boom", 4_100);
    printStepSkipped({ displayName: "Test Suite" }, "preserved from previous run");

    expect(logs[0]).toContain("fhEVM Boot Pipeline (13 steps)");
    expect(logs.some((line) => line.includes("Resuming from last failed step"))).toBe(true);
    expect(logs.some((line) => line.includes("[1/13] MinIO (S3 storage)..."))).toBe(true);
    expect(logs.some((line) => line.includes("✓ MinIO (S3 storage) (2.4s)"))).toBe(true);
    expect(logs.some((line) => line.includes("Discovered MinIO IP: 172.18.0.2"))).toBe(true);
    expect(logs.some((line) => line.includes("✗ KMS Core (4.1s)"))).toBe(true);
    expect(logs.some((line) => line.includes("Test Suite (skipped: preserved from previous run)"))).toBe(true);
  });

  test("prints parallel step header", () => {
    printParallelStepStart(
      [
        { number: 5, displayName: "Host Node (Anvil)" },
        { number: 6, displayName: "Gateway Node (Anvil)" },
      ],
      13,
    );

    expect(logs[0]).toContain("[5/13] Host Node (Anvil) + [6/13] Gateway Node (Anvil)...");
  });

  test("prints pipeline summary", () => {
    const state: BootState = {
      version: 1,
      startedAt: "2026-01-01T00:00:00.000Z",
      completedAt: "2026-01-01T00:04:32.000Z",
      lastStep: 13,
      lastStepName: "test-suite",
      status: "completed",
      runtime: {},
      steps: [],
    };

    printPipelineSummary(state);

    expect(logs[0]).toContain("Pipeline Complete (4m 32s)");
  });

  test("creates JSON pipeline output adapter", () => {
    const lines: string[] = [];
    jsonOutputInternal.setWriterForTests((line) => {
      lines.push(line);
    });

    try {
      const output = createPipelineOutput("json");
      const state: BootState = {
        version: 1,
        startedAt: "2026-01-01T00:00:00.000Z",
        completedAt: "2026-01-01T00:00:05.000Z",
        lastStep: 1,
        lastStepName: "minio",
        status: "completed",
        runtime: {},
        steps: [],
      };

      output.pipelineHeader(13);
      output.parallelStepStart(
        [
          { number: 5, displayName: "Host Node (Anvil)" },
          { number: 6, displayName: "Gateway Node (Anvil)" },
        ],
        13,
      );
      output.stepSuccess({ number: 5, displayName: "Host Node (Anvil)" }, 300);
      output.stepFail({ number: 6, displayName: "Gateway Node (Anvil)" }, "boom", 400);
      output.stepSkipped({ number: 13, displayName: "Test Suite" }, "preserved");
      output.discoveryResult("MinIO IP", "172.18.0.2");
      output.pipelineSummary(state);

      const events = lines.map((line) => JSON.parse(line) as Record<string, unknown>);
      expect(events.map((event) => event.type)).toEqual([
        "pipeline-start",
        "step-start",
        "step-start",
        "step-success",
        "step-fail",
        "step-skip",
        "discovery",
        "pipeline-complete",
      ]);
      expect(events[3]?.durationMs).toBe(300);
      expect(events[7]?.durationMs).toBe(5000);
    } finally {
      jsonOutputInternal.resetWriterForTests();
    }
  });
});
