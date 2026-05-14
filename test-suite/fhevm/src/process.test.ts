import { describe, expect, test } from "bun:test";

import { CommandError } from "./errors";
import { runStreaming } from "./utils/process";

describe("runStreaming", () => {
  test("preserves stderr in command failures", async () => {
    await expect(runStreaming(["sh", "-lc", "echo boom >&2; exit 7"])).rejects.toMatchObject({
      name: "CommandError",
      stderr: "boom",
    } satisfies Partial<CommandError>);
  });
});
