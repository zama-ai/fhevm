import { describe, expect, test } from "bun:test";

import { BuildError, formatCliError } from "./errors";

describe("formatCliError", () => {
  test("adds a Docker DNS hint for build failures caused by name resolution", () => {
    const message = formatCliError(
      new BuildError({
        component: "kms-connector",
        stderr:
          "error: failed to download from static.rust-lang.org: temporary failure in name resolution",
      }),
    );
    expect(message).toContain("kms-connector build failed");
    expect(message).toContain("Docker BuildKit could not resolve an external host");
  });

  test("keeps ordinary build failures concise", () => {
    const message = formatCliError(
      new BuildError({
        component: "coprocessor",
        stderr: "cargo build failed",
      }),
    );
    expect(message).toBe("coprocessor build failed\ncargo build failed");
  });
});
