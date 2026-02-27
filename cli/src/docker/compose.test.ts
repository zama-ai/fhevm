import { describe, expect, test } from "bun:test";

import { ExitCode, FhevmCliError } from "../errors";

import {
  __internal,
  buildComposeArgs,
  composeDown,
  composeExecStreaming,
  composeUp,
  composeWait,
} from "./compose";

describe("docker compose wrapper", () => {
  test("builds base compose args in the expected order", () => {
    const args = buildComposeArgs({
      project: "fhevm",
      files: ["a.yml", "b.yml"],
      envFile: ".fhevm/env/coprocessor.env",
    });

    expect(args).toEqual([
      "docker",
      "compose",
      "-p",
      "fhevm",
      "-f",
      "a.yml",
      "-f",
      "b.yml",
      "--env-file",
      ".fhevm/env/coprocessor.env",
    ]);
  });

  test("builds up args with flags and services", () => {
    const args = __internal.buildUpArgs({
      project: "fhevm",
      files: ["stack.yml"],
      build: true,
      noCache: true,
      wait: true,
      waitTimeout: 30,
      services: ["foo", "bar"],
    });

    expect(args).toEqual([
      "docker",
      "compose",
      "-p",
      "fhevm",
      "-f",
      "stack.yml",
      "up",
      "-d",
      "--build",
      "--no-cache",
      "--wait",
      "--timeout",
      "30",
      "foo",
      "bar",
    ]);
  });

  test("builds logs args with json/no-color and service filters", () => {
    const args = __internal.buildLogsArgs({
      project: "fhevm",
      files: ["stack.yml"],
      follow: true,
      tail: 50,
      noColor: true,
      format: "json",
      services: ["coprocessor-tfhe-worker"],
    });

    expect(args).toEqual([
      "docker",
      "compose",
      "-p",
      "fhevm",
      "-f",
      "stack.yml",
      "logs",
      "-f",
      "--tail",
      "50",
      "--no-color",
      "--format",
      "json",
      "coprocessor-tfhe-worker",
    ]);
  });

  test("compose up failures surface as docker errors", async () => {
    await expect(
      composeUp({
        project: "fhevm",
        files: ["/definitely/missing/compose.yml"],
      }),
    ).rejects.toMatchObject({
      exitCode: ExitCode.DOCKER,
      name: FhevmCliError.name,
    });
  });

  test("compose down failures surface as docker errors", async () => {
    await expect(
      composeDown({
        project: "fhevm",
        files: ["/definitely/missing/compose.yml"],
      }),
    ).rejects.toMatchObject({
      exitCode: ExitCode.DOCKER,
    });
  });

  test("compose wait returns exit code for caller passthrough", async () => {
    const exitCode = await composeWait(["service"], {
      project: "fhevm",
      files: ["/definitely/missing/compose.yml"],
      timeoutMs: 50,
    });

    expect(typeof exitCode).toBe("number");
    expect(exitCode).not.toBe(0);
  });

  test("compose exec streaming returns raw exit code without throwing", async () => {
    const exitCode = await composeExecStreaming("missing-service", ["echo", "hello"], {
      project: "fhevm",
      files: ["/definitely/missing/compose.yml"],
    });

    expect(typeof exitCode).toBe("number");
    expect(exitCode).not.toBe(0);
  });
});
