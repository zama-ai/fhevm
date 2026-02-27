import { afterEach, describe, expect, test } from "bun:test";

import {
  __internal,
  emitJsonDiscovery,
  emitJsonEvent,
  emitJsonPipelineComplete,
  emitJsonPipelineFail,
  emitJsonPipelineStart,
  emitJsonStepFail,
  emitJsonStepSkipped,
  emitJsonStepStart,
  emitJsonStepSuccess,
} from "./json-output";

function parseSingle(lines: string[]): Record<string, unknown> {
  expect(lines).toHaveLength(1);
  expect(lines[0]?.endsWith("\n")).toBe(true);
  return JSON.parse(lines[0] ?? "") as Record<string, unknown>;
}

function expectTimestamp(value: unknown): void {
  expect(typeof value).toBe("string");
  expect(Number.isNaN(Date.parse(String(value)))).toBe(false);
}

afterEach(() => {
  __internal.resetWriterForTests();
});

describe("json output", () => {
  test("emits raw event", () => {
    const lines: string[] = [];
    __internal.setWriterForTests((line) => {
      lines.push(line);
    });

    emitJsonEvent({ type: "custom", value: 1 });

    const event = parseSingle(lines);
    expectTimestamp(event.timestamp);
    expect(event.type).toBe("custom");
    expect(event.value).toBe(1);
  });

  test("emits pipeline events", () => {
    const lines: string[] = [];
    __internal.setWriterForTests((line) => {
      lines.push(line);
    });

    emitJsonPipelineStart(13);
    emitJsonPipelineComplete(1234);
    emitJsonPipelineFail(4, "boom");

    const events = lines.map((line) => JSON.parse(line) as Record<string, unknown>);
    expect(events.map((event) => event.type)).toEqual(["pipeline-start", "pipeline-complete", "pipeline-fail"]);
    expect(events[0]?.totalSteps).toBe(13);
    expect(events[1]?.durationMs).toBe(1234);
    expect(events[2]?.failedStep).toBe(4);
    expect(events[2]?.error).toBe("boom");
    for (const event of events) {
      expectTimestamp(event.timestamp);
    }
  });

  test("emits step lifecycle and discovery events", () => {
    const lines: string[] = [];
    __internal.setWriterForTests((line) => {
      lines.push(line);
    });

    emitJsonStepStart(1, "MinIO", 13);
    emitJsonStepSuccess(1, "MinIO", 200);
    emitJsonStepFail(2, "KMS", "failed", 300);
    emitJsonStepSkipped(13, "Test Suite", "preserved from previous run");
    emitJsonDiscovery("MinIO IP", "172.18.0.2");

    const events = lines.map((line) => JSON.parse(line) as Record<string, unknown>);
    expect(events[0]?.type).toBe("step-start");
    expect(events[0]?.step).toBe(1);
    expect(events[0]?.name).toBe("MinIO");
    expect(events[0]?.totalSteps).toBe(13);

    expect(events[1]?.type).toBe("step-success");
    expect(events[1]?.durationMs).toBe(200);

    expect(events[2]?.type).toBe("step-fail");
    expect(events[2]?.error).toBe("failed");

    expect(events[3]?.type).toBe("step-skip");
    expect(events[3]?.reason).toBe("preserved from previous run");

    expect(events[4]?.type).toBe("discovery");
    expect(events[4]?.label).toBe("MinIO IP");
    expect(events[4]?.value).toBe("172.18.0.2");
  });
});
