import { describe, expect, test } from "bun:test";

import {
  DRIFT_CLEANUP_SQL,
  DRIFT_INSTALL_SQL,
  driftDatabaseName,
  parseDriftInstanceIndex,
  parsePositiveInteger,
} from "./ciphertext-drift";
import { findDriftWarning } from "./ciphertext-drift-runner";

describe("ciphertext-drift", () => {
  test("driftDatabaseName maps primary and replica instances", () => {
    expect(driftDatabaseName(0)).toBe("coprocessor");
    expect(driftDatabaseName(1)).toBe("coprocessor_1");
    expect(driftDatabaseName(7)).toBe("coprocessor_7");
  });

  test("parseDriftInstanceIndex rejects non-integer input", () => {
    expect(parseDriftInstanceIndex("0")).toBe(0);
    expect(parseDriftInstanceIndex("3")).toBe(3);
    expect(() => parseDriftInstanceIndex("-1")).toThrow("instance index must be a non-negative integer");
    expect(() => parseDriftInstanceIndex("abc")).toThrow("instance index must be a non-negative integer");
  });

  test("parsePositiveInteger rejects zero and malformed values", () => {
    expect(parsePositiveInteger("2", "timeout")).toBe(2);
    expect(() => parsePositiveInteger("0", "timeout")).toThrow("timeout must be a positive integer");
    expect(() => parsePositiveInteger("2.5", "timeout")).toThrow("timeout must be a positive integer");
  });

  test("install and cleanup SQL keep the trigger lifecycle explicit", () => {
    expect(DRIFT_INSTALL_SQL).toContain("CREATE TABLE IF NOT EXISTS drift_injection_state");
    expect(DRIFT_INSTALL_SQL).toContain("CREATE TRIGGER ciphertext_drift_injector");
    expect(DRIFT_CLEANUP_SQL).toContain("DROP TRIGGER IF EXISTS ciphertext_drift_injector");
    expect(DRIFT_CLEANUP_SQL).toContain("DROP TABLE IF EXISTS drift_injection_state");
  });

  test("findDriftWarning prefers the injected handle but falls back to any drift warning", () => {
    const other = '{"message":"Drift detected: observed multiple digest variants for handle","handle":"0xaaaa"}';
    const exact = '{"message":"Drift detected: observed multiple digest variants for handle","handle":"0xbbbb"}';
    expect(findDriftWarning(`${other}\n${exact}`, "bbbb")).toEqual({
      handleHex: "bbbb",
      exact: true,
    });
    expect(findDriftWarning(other, "bbbb")).toEqual({
      handleHex: "aaaa",
      exact: false,
    });
  });
});
