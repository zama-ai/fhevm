import { describe, expect, it } from "vitest";

import { parseArtifact } from "../src/cli/commands/verify-user-decrypt";

describe("parseArtifact", () => {
  it("accepts validation artifact schema v2", () => {
    expect(parseArtifact({ schemaVersion: 2 })).toEqual({ schemaVersion: 2 });
  });

  it("rejects legacy validation artifact schema v1", () => {
    expect(() => parseArtifact({ schemaVersion: 1 })).toThrow(
      "Invalid user-decrypt validation artifact",
    );
  });
});
