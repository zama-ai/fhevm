import { afterEach, describe, expect, test } from "bun:test";

import { __internal } from "../docker/services";
import { ExitCode, FhevmCliError } from "../errors";
import type { ComposeDownOptions } from "../docker/types";

import { runDownCommand, toDownError } from "./down";

afterEach(() => {
  __internal.resetDockerOpsForTests();
});

describe("down command", () => {
  test("stops all services without removing volumes", async () => {
    let composeOptions: ComposeDownOptions | undefined;

    __internal.setDockerOpsForTests({
      composeDown: async (options) => {
        composeOptions = options;
      },
    });

    await runDownCommand();

    expect(composeOptions).toBeDefined();
    expect(composeOptions?.project).toBe("fhevm");
    expect(composeOptions?.volumes).toBeUndefined();
    expect(composeOptions?.files.length).toBeGreaterThan(0);
  });

  test("passes through typed CLI errors", () => {
    const error = new FhevmCliError({
      exitCode: ExitCode.DOCKER,
      step: "down",
      message: "docker failed",
    });

    expect(toDownError(error)).toBe(error);
  });

  test("maps unknown errors to docker failures", () => {
    const error = toDownError(new Error("boom"));

    expect(error.exitCode).toBe(ExitCode.DOCKER);
    expect(error.step).toBe("down");
    expect(error.message).toBe("boom");
  });
});
