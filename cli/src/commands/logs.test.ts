import { describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";

import { buildLogsComposeOptions, parseTail, resolveLogServices, toLogsError } from "./logs";

describe("logs command", () => {
  test("resolves exact service names", () => {
    const services = resolveLogServices("coprocessor-tfhe-worker");
    expect(services.map((service) => service.name)).toEqual(["coprocessor-tfhe-worker"]);
  });

  test("resolves component aliases", () => {
    const services = resolveLogServices("coprocessor");
    expect(services.length).toBeGreaterThan(1);
    expect(services.map((service) => service.name)).toContain("coprocessor-tfhe-worker");
  });

  test("rejects unknown service names", () => {
    expect(() => resolveLogServices("unknown-service")).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
      }),
    );
  });

  test("applies default tail when not following", () => {
    expect(parseTail(undefined, false)).toBe(100);
  });

  test("omits default tail in follow mode", () => {
    expect(parseTail(undefined, true)).toBeUndefined();
  });

  test("validates tail value", () => {
    expect(() => parseTail("-1", false)).toThrowError(
      expect.objectContaining({
        exitCode: ExitCode.CONFIG,
      }),
    );
  });

  test("builds compose options for json output", () => {
    const options = buildLogsComposeOptions({
      service: "tfhe-worker",
      follow: false,
      json: true,
      tail: "25",
    });

    expect(options.services).toEqual(["coprocessor-tfhe-worker"]);
    expect(options.tail).toBe(25);
    expect(options.noColor).toBe(true);
    expect(options.format).toBe("json");
  });

  test("maps unknown errors to docker logs errors", () => {
    const converted = toLogsError(new Error("boom"));

    expect(converted).toMatchObject({
      exitCode: ExitCode.DOCKER,
      step: "logs",
      message: "boom",
    });
  });
});
