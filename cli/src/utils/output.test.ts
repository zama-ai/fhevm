import { describe, expect, test } from "bun:test";

import { fail, isCI, pass, warn } from "./output";

describe("output", () => {
  test("status helpers return non-empty strings", () => {
    expect(pass("Docker")).toBeString();
    expect(fail("Ports", "occupied")).toContain("Ports");
    expect(warn("Memory", "low")).toContain("Memory");
  });

  test("status helpers include plain prefixes in non-tty environments", () => {
    expect(pass("Docker")).toContain("PASS");
    expect(fail("Ports")).toContain("FAIL");
    expect(warn("Memory")).toContain("WARN");
  });

  test("isCI checks CI and GITHUB_ACTIONS", () => {
    const previousCI = process.env.CI;
    const previousActions = process.env.GITHUB_ACTIONS;

    try {
      process.env.CI = "true";
      process.env.GITHUB_ACTIONS = "";
      expect(isCI()).toBe(true);

      process.env.CI = "";
      process.env.GITHUB_ACTIONS = "true";
      expect(isCI()).toBe(true);

      process.env.CI = "";
      process.env.GITHUB_ACTIONS = "";
      expect(isCI()).toBe(false);
    } finally {
      process.env.CI = previousCI;
      process.env.GITHUB_ACTIONS = previousActions;
    }
  });
});
