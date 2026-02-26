import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, rm, writeFile } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { ensureDotFhevm } from "../config/dotfhevm";
import { composeEnvFilePath } from "../config/env-writer";
import { getServiceByName, type ServiceDefinition } from "../config/service-map";
import { __internal } from "../docker/services";
import { ExitCode, FhevmCliError } from "../errors";

import {
  buildRestartHints,
  resolveRestartServices,
  runRestartCommand,
  toRestartError,
} from "./restart";

const tempRoots: string[] = [];

afterEach(async () => {
  __internal.resetDockerOpsForTests();
  await Promise.all(tempRoots.map((root) => rm(root, { recursive: true, force: true })));
  tempRoots.length = 0;
});

describe("restart command", () => {
  test("resolves an exact service name", () => {
    const services = resolveRestartServices("coprocessor-tfhe-worker");
    expect(services).toHaveLength(1);
    expect(services[0]?.name).toBe("coprocessor-tfhe-worker");
  });

  test("resolves local component shorthand", () => {
    const services = resolveRestartServices("tfhe-worker");
    expect(services.map((service) => service.name)).toEqual(["coprocessor-tfhe-worker"]);
  });

  test("resolves shorthand components to multiple services", () => {
    const services = resolveRestartServices("coprocessor");
    expect(services).toHaveLength(8);
  });

  test("rejects unknown service names", () => {
    expect(() => resolveRestartServices("unknown-service")).toThrowError(
      expect.objectContaining({ exitCode: ExitCode.CONFIG }),
    );
  });

  test("errors when required env files are missing", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);

    await expect(runRestartCommand("tfhe-worker", paths)).rejects.toMatchObject({
      exitCode: ExitCode.CONFIG,
      message: expect.stringContaining("run 'fhevm-cli up' first"),
    });
  });

  test("stops a resolved service with loaded env files", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await writeFile(composeEnvFilePath(paths.env, "coprocessor"), "A=1\n", "utf8");

    const stopped: string[][] = [];
    __internal.setDockerOpsForTests({
      composeStop: async (services) => {
        stopped.push([...services]);
      },
      composeStart: async () => {
        throw new Error("composeStart should not be called for restart command");
      },
      waitForAllReady: async () => {
        throw new Error("waitForAllReady should not be called for restart command");
      },
    });

    const result = await runRestartCommand("tfhe-worker", paths);

    expect(stopped).toEqual([["coprocessor-tfhe-worker"]]);
    expect(result.services.map((service) => service.name)).toEqual(["coprocessor-tfhe-worker"]);
    expect(result.hints.get("coprocessor-tfhe-worker")).toContain("cargo run --bin tfhe_worker");
  });

  test("does not produce local run hints for infra services", () => {
    const service = mustService("minio");
    const hints = buildRestartHints([service]);
    expect(hints.size).toBe(0);
  });

  test("passes through typed CLI errors", () => {
    const error = new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "restart",
      message: "invalid service",
    });

    expect(toRestartError(error)).toBe(error);
  });

  test("maps unknown errors to docker failures", () => {
    const error = toRestartError(new Error("boom"));
    expect(error.exitCode).toBe(ExitCode.DOCKER);
    expect(error.step).toBe("restart");
    expect(error.message).toBe("boom");
  });
});

function mustService(name: string): ServiceDefinition {
  const service = getServiceByName(name);
  if (!service) {
    throw new Error(`missing fixture service: ${name}`);
  }

  return service;
}
