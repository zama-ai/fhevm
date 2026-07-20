import { describe, expect, test } from "bun:test";

import { CommandError } from "./errors";
import { runStreaming, runWithHeartbeat } from "./utils/process";

describe("runStreaming", () => {
  test("preserves stderr in command failures", async () => {
    await expect(runStreaming(["sh", "-lc", "echo boom >&2; exit 7"])).rejects.toMatchObject({
      name: "CommandError",
      stderr: "boom",
    } satisfies Partial<CommandError>);
  });
});

describe("runWithHeartbeat", () => {
  test("preserves streamed output in command failures", async () => {
    await expect(
      runWithHeartbeat(["sh", "-lc", "echo reconstruction-failed; echo warning >&2; exit 7"], "test"),
    ).rejects.toMatchObject({
      name: "CommandError",
      stderr: "reconstruction-failed\nwarning",
    } satisfies Partial<CommandError>);
  });
});
