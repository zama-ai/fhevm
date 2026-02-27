import { afterEach, describe, expect, test } from "bun:test";

import { __internal, detectCI } from "./detect";

afterEach(() => {
  __internal.resetEnvReaderForTests();
});

describe("ci detect", () => {
  test("detects CI from CI=true", () => {
    __internal.setEnvReaderForTests((name) => (name === "CI" ? "true" : undefined));

    expect(detectCI()).toEqual({
      isCI: true,
      isGitHubActions: false,
      cacheType: "gha",
    });
  });

  test("detects CI from GITHUB_ACTIONS=true", () => {
    __internal.setEnvReaderForTests((name) => (name === "GITHUB_ACTIONS" ? "true" : undefined));

    expect(detectCI()).toEqual({
      isCI: true,
      isGitHubActions: true,
      cacheType: "gha",
    });
  });

  test("returns local backend outside CI", () => {
    __internal.setEnvReaderForTests(() => undefined);

    expect(detectCI()).toEqual({
      isCI: false,
      isGitHubActions: false,
      cacheType: "local",
    });
  });

  test("returns none backend when no-cache is set", () => {
    __internal.setEnvReaderForTests((name) => (name === "CI" ? "true" : undefined));

    expect(detectCI({ noCache: true })).toEqual({
      isCI: true,
      isGitHubActions: false,
      cacheType: "none",
    });
  });
});
