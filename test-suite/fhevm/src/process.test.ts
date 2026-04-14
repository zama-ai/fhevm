import { describe, expect, test } from "bun:test";

import { CommandError } from "./errors";
import { runStreaming } from "./utils/process";

describe("runStreaming", () => {
  test("preserves a useful failure hint when stderr is inherited", async () => {
    await expect(runStreaming(["sh", "-lc", "echo boom >&2; exit 7"])).rejects.toMatchObject({
      name: "CommandError",
      stderr: "see output above",
    } satisfies Partial<CommandError>);
  });
});
