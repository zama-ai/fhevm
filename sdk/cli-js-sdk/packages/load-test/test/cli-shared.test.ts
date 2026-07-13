import { describe, expect, it } from "vitest";

import {
  parseBoundedInt,
  parseBoundedIntOrAuto,
  parseBoundedNonNegativeNumber,
  parseNonNegativeNumber,
} from "../src/cli/shared";

describe("parseBoundedInt", () => {
  it("accepts positive integers at or below the ceiling", () => {
    expect(parseBoundedInt("--connections", 1024)("1")).toBe(1);
    expect(parseBoundedInt("--connections", 1024)("1024")).toBe(1024);
  });

  it("rejects values above the ceiling with a clear, labeled message", () => {
    expect(() => parseBoundedInt("--connections", 1024)("1025")).toThrow(
      /--connections must be at most 1024, got "1025"/,
    );
  });

  it("still rejects non-positive or non-integer input", () => {
    expect(() => parseBoundedInt("--threads", 128)("0")).toThrow(/positive integer/);
    expect(() => parseBoundedInt("--threads", 128)("1.5")).toThrow(/positive integer/);
  });
});

describe("parseBoundedIntOrAuto", () => {
  it("passes through the literal \"auto\"", () => {
    expect(parseBoundedIntOrAuto("--encrypt-concurrency", 256)("auto")).toBe("auto");
  });

  it("bounds numeric values", () => {
    expect(parseBoundedIntOrAuto("--encrypt-concurrency", 256)("256")).toBe(256);
    expect(() => parseBoundedIntOrAuto("--encrypt-concurrency", 256)("257")).toThrow(
      /--encrypt-concurrency must be at most 256/,
    );
  });
});

describe("parseNonNegativeNumber", () => {
  it("accepts 0 and positive numbers", () => {
    expect(parseNonNegativeNumber("0")).toBe(0);
    expect(parseNonNegativeNumber("0.2")).toBe(0.2);
  });

  it("rejects negative numbers", () => {
    expect(() => parseNonNegativeNumber("-0.1")).toThrow(/non-negative number/);
  });
});

describe("parseBoundedNonNegativeNumber", () => {
  it("accepts values within [0, max]", () => {
    const parse = parseBoundedNonNegativeNumber("--max-error-rate-increase", 1);
    expect(parse("0")).toBe(0);
    expect(parse("1")).toBe(1);
    expect(parse("0.01")).toBe(0.01);
  });

  it("rejects values above max with a clear message", () => {
    expect(() => parseBoundedNonNegativeNumber("--max-error-rate-increase", 1)("1.5")).toThrow(
      /--max-error-rate-increase must be between 0 and 1, got "1.5"/,
    );
  });

  it("rejects negative values", () => {
    expect(() => parseBoundedNonNegativeNumber("--max-error-rate-increase", 1)("-0.1")).toThrow(
      /non-negative number/,
    );
  });
});
