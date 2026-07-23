import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";

import { Recorder } from "../src/runner/recorder";
import { JsonlWriter } from "../src/shared/jsonl";

const dirs: string[] = [];
afterEach(async () => {
  await Promise.all(dirs.splice(0).map((dir) => rm(dir, { recursive: true, force: true })));
});

describe("JSONL lifecycle", () => {
  it("surfaces a queued write failure again from durable close", async () => {
    const dir = await mkdtemp(join(tmpdir(), "jsonl-failure-"));
    dirs.push(dir);
    const writer = await JsonlWriter.open<{ value: number }>(join(dir, "records.jsonl"));
    const failure = new Error("disk full");
    const handle = (writer as unknown as { handle: { writeFile: typeof vi.fn } }).handle;
    handle.writeFile = vi.fn().mockRejectedValue(failure);
    await expect(writer.write({ value: 1 })).rejects.toBe(failure);
    await expect(writer.close()).rejects.toBe(failure);
  });

  it("attempts every recorder writer close and aggregates failures", async () => {
    const recorder = new Recorder();
    const closes = [
      vi.fn().mockRejectedValue(new Error("main failed")),
      vi.fn().mockResolvedValue(undefined),
      vi.fn().mockRejectedValue(new Error("candidate failed")),
    ];
    const target = recorder as unknown as Record<string, unknown>;
    target.writer = { close: closes[0] };
    target.relayerAWriter = { close: closes[1] };
    target.relayerBWriter = { close: closes[2] };
    const error = await recorder.close().catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(AggregateError);
    expect(closes.every((close) => close.mock.calls.length === 1)).toBe(true);
  });
});
