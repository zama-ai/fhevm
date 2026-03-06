import { describe, expect, test } from "bun:test";

import type { DotFhevmPaths } from "../config/dotfhevm";
import type { VersionGroup } from "../config/versions";
import { ExitCode, FhevmCliError } from "../errors";

import { runUnpauseCommand, toUnpauseError } from "./unpause";

const PATHS: DotFhevmPaths = {
  root: "/tmp/.fhevm",
  env: "/tmp/.fhevm/env",
  compose: "/tmp/.fhevm/compose",
  keys: "/tmp/.fhevm/keys",
  keysMinioSnapshot: "/tmp/.fhevm/keys/minio-snapshot",
  keysVolumeSnapshot: "/tmp/.fhevm/keys/volume-snapshot",
  logs: "/tmp/.fhevm/logs",
  stateFile: "/tmp/.fhevm/state.json",
  versionCache: "/tmp/.fhevm/version-cache.json",
};

const VERSIONS: Record<VersionGroup, string> = {
  coprocessor: "v1.0.0",
  "kms-connector": "v1.0.0",
  contracts: "v1.0.0",
  core: "v1.0.0",
  relayer: "v1.0.0",
  "test-suite": "v1.0.0",
};

describe("unpause command", () => {
  test("validates target values", async () => {
    await expect(
      runUnpauseCommand("bad-target", PATHS, {
        findExistingEnvFiles: () => new Map(),
        resolveAllVersions: async () => VERSIONS,
        buildVersionEnvVars: () => ({}),
        startAndWaitForServices: async () => [],
      }),
    ).rejects.toMatchObject({
      exitCode: ExitCode.CONFIG,
      message: expect.stringContaining("invalid target"),
    });
  });

  test("runs one-shot unpause service when env files exist", async () => {
    const started: string[][] = [];

    await runUnpauseCommand("host", PATHS, {
      findExistingEnvFiles: () => new Map([["host-sc", "/tmp/.fhevm/env/host-sc.env"]]),
      resolveAllVersions: async () => VERSIONS,
      buildVersionEnvVars: () => ({ HOST_VERSION: "v1.0.0" }),
      startAndWaitForServices: async (services) => {
        started.push(services.map((service) => service.name));
        return [];
      },
    });

    expect(started).toEqual([["host-sc-unpause"]]);
  });

  test("passes through typed CLI errors", () => {
    const error = new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "unpause",
      message: "bad target",
    });

    expect(toUnpauseError(error)).toBe(error);
  });

  test("maps unknown errors to docker failures", () => {
    const error = toUnpauseError(new Error("boom"));
    expect(error.exitCode).toBe(ExitCode.DOCKER);
    expect(error.step).toBe("unpause");
    expect(error.message).toBe("boom");
  });
});
