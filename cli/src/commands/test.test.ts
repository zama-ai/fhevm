import { describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";
import type { VersionGroup } from "../config/versions";
import { getServiceByName } from "../config/service-map";

import {
  buildHardhatArgs,
  mapTestExitCode,
  resolveTestPattern,
  runTestCommand,
  toTestError,
  validateTestType,
} from "./test";

const VERSIONS: Record<VersionGroup, string> = {
  coprocessor: "v1.0.0",
  "kms-connector": "v1.0.0",
  contracts: "v1.0.0",
  core: "v1.0.0",
  relayer: "v1.0.0",
  "test-suite": "v1.0.0",
};

describe("test command", () => {
  test("validates supported test types", () => {
    expect(() => validateTestType("input-proof")).not.toThrow();
    expect(() => validateTestType("invalid")).toThrowError(
      expect.objectContaining({ exitCode: ExitCode.CONFIG }),
    );
  });

  test("resolves pattern mapping and override", () => {
    expect(resolveTestPattern("erc20")).toBe("EncryptedERC20");
    expect(resolveTestPattern("erc20", "custom")).toBe("custom");
  });

  test("requires grep override for debug", () => {
    expect(() => resolveTestPattern("debug")).toThrowError(
      expect.objectContaining({ exitCode: ExitCode.CONFIG }),
    );
  });

  test("builds hardhat args with optional flags", () => {
    const args = buildHardhatArgs({
      pattern: "Rand",
      network: "staging",
      verbose: true,
      noHardhatCompile: true,
      randomOrder: true,
    });

    expect(args).toEqual([
      "npx",
      "hardhat",
      "test",
      "--parallel",
      "--verbose",
      "--no-hardhat-compile",
      "--grep",
      "Rand",
      "--network",
      "staging",
    ]);
  });

  test("maps non-zero runner exit codes to 10+", () => {
    expect(mapTestExitCode(0)).toBe(ExitCode.SUCCESS);
    expect(mapTestExitCode(1)).toBe(ExitCode.TEST_FAILURE);
    expect(mapTestExitCode(2)).toBe(11);
  });

  test("returns mapped exit code from streaming docker exec", async () => {
    const service = getServiceByName("test-suite-e2e-debug");
    if (!service) {
      throw new Error("missing fixture service: fhevm-test-suite-e2e-debug");
    }

    let inspectedContainer = "";

    const exitCode = await runTestCommand(
      {
        type: "input-proof",
        verbose: false,
        network: "staging",
        randomOrder: false,
        noHardhatCompile: false,
      },
      {
        getContainerState: async (containerName) => {
          inspectedContainer = containerName;
          return "running";
        },
        composeExecStreaming: async () => 1,
        resolveAllVersions: async () => VERSIONS,
        buildVersionEnvVars: () => ({ TEST_SUITE_VERSION: "v1.0.0" }),
      },
    );

    expect(inspectedContainer).toBe(service.containerName);
    expect(exitCode).toBe(ExitCode.TEST_FAILURE);
  });

  test("fails when test container is not running", async () => {
    await expect(
      runTestCommand(
        {
          type: "input-proof",
          verbose: false,
          network: "staging",
          randomOrder: false,
          noHardhatCompile: false,
        },
        {
          getContainerState: async () => "not-found",
          composeExecStreaming: async () => 0,
          resolveAllVersions: async () => VERSIONS,
          buildVersionEnvVars: () => ({ TEST_SUITE_VERSION: "v1.0.0" }),
        },
      ),
    ).rejects.toMatchObject({
      exitCode: ExitCode.CONFIG,
      message: expect.stringContaining("run 'fhevm-cli up' first"),
    });
  });

  test("maps unknown errors to docker test errors", () => {
    const converted = toTestError(new Error("boom"));

    expect(converted).toMatchObject({
      exitCode: ExitCode.DOCKER,
      step: "test",
      message: "boom",
    });
  });
});
