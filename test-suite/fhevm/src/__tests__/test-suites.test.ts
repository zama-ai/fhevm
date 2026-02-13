import { describe, expect, it } from "bun:test";
import { TEST_SUITES, SUITE_NAMES } from "../test-suites.js";

describe("test-suites", () => {
  it("has all 13 named suites from the bash script", () => {
    const expected = [
      "input-proof",
      "input-proof-compute-decrypt",
      "user-decryption",
      "delegated-user-decryption",
      "public-decryption",
      "erc20",
      "public-decrypt-http-ebool",
      "public-decrypt-http-mixed",
      "operators",
      "random",
      "random-subset",
      "paused-host-contracts",
      "paused-gateway-contracts",
    ];
    expect(SUITE_NAMES.sort()).toEqual(expected.sort());
  });

  it("every suite has a non-empty grep pattern", () => {
    for (const [name, suite] of Object.entries(TEST_SUITES)) {
      expect(suite.grep.length).toBeGreaterThan(0);
    }
  });

  it("every suite has a non-empty label", () => {
    for (const [name, suite] of Object.entries(TEST_SUITES)) {
      expect(suite.label.length).toBeGreaterThan(0);
    }
  });

  it("no duplicate grep patterns", () => {
    const greps = Object.values(TEST_SUITES).map((s) => s.grep);
    const unique = new Set(greps);
    expect(unique.size).toBe(greps.length);
  });

  it("only operators suite has parallel flag", () => {
    for (const [name, suite] of Object.entries(TEST_SUITES)) {
      if (name === "operators") {
        expect(suite.parallel).toBe(true);
      } else {
        expect(suite.parallel).toBeUndefined();
      }
    }
  });

  it("SUITE_NAMES matches Object.keys(TEST_SUITES)", () => {
    expect(SUITE_NAMES.sort()).toEqual(Object.keys(TEST_SUITES).sort());
  });

  // Specific grep pattern spot-checks against the bash script
  it("input-proof grep matches bash", () => {
    expect(TEST_SUITES["input-proof"].grep).toBe("test user input uint64");
  });

  it("public-decryption grep matches bash", () => {
    expect(TEST_SUITES["public-decryption"].grep).toBe(
      "test async decrypt (uint.*|ebytes.* trivial|ebytes64 non-trivial|ebytes256 non-trivial with snapshot|addresses|several addresses)",
    );
  });

  it("operators grep matches bash", () => {
    expect(TEST_SUITES["operators"].grep).toBe("test operator|FHEVM manual operations");
  });

  it("random grep matches bash", () => {
    expect(TEST_SUITES["random"].grep).toBe(
      "generate and decrypt|generating rand in reverting sub-call|upper bound and decrypt",
    );
  });
});
