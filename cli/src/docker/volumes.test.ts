import { afterEach, describe, expect, test } from "bun:test";

import { ExitCode } from "../errors";

import { __internal, exportVolume, importVolume, volumeExists } from "./volumes";

afterEach(() => {
  __internal.resetOpsForTests();
});

describe("docker volumes", () => {
  test("validates volume names", () => {
    expect(__internal.isValidVolumeName("fhevm_keys-cache")).toBe(true);
    expect(__internal.isValidVolumeName("bad name")).toBe(false);
    expect(__internal.isValidVolumeName("bad\"name")).toBe(false);
  });

  test("exportVolume runs docker copy command with absolute destination", async () => {
    const calls: string[][] = [];
    __internal.setOpsForTests({
      exec: async (command) => {
        calls.push(command);
        return { exitCode: 0, stdout: "", stderr: "" };
      },
    });

    await exportVolume("fhevm_keys-cache", ".fhevm/keys/volume-snapshot");

    expect(calls).toHaveLength(1);
    expect(calls[0]).toEqual([
      "docker",
      "run",
      "--rm",
      "-v",
      "fhevm_keys-cache:/source",
      "-v",
      `${process.cwd()}/.fhevm/keys/volume-snapshot:/dest`,
      "alpine",
      "sh",
      "-c",
      "cp -a /source/. /dest/",
    ]);
  });

  test("importVolume runs docker copy command with absolute source", async () => {
    const calls: string[][] = [];
    __internal.setOpsForTests({
      exec: async (command) => {
        calls.push(command);
        return { exitCode: 0, stdout: "", stderr: "" };
      },
    });

    await importVolume(".fhevm/keys/volume-snapshot", "fhevm_keys-cache");

    expect(calls).toHaveLength(1);
    expect(calls[0]).toEqual([
      "docker",
      "run",
      "--rm",
      "-v",
      `${process.cwd()}/.fhevm/keys/volume-snapshot:/source`,
      "-v",
      "fhevm_keys-cache:/dest",
      "alpine",
      "sh",
      "-c",
      "cp -a /source/. /dest/",
    ]);
  });

  test("volumeExists checks docker volume inspect exit code", async () => {
    __internal.setOpsForTests({
      exec: async () => ({ exitCode: 0, stdout: "[]", stderr: "" }),
    });
    expect(await volumeExists("fhevm_keys-cache")).toBe(true);

    __internal.setOpsForTests({
      exec: async () => ({ exitCode: 1, stdout: "", stderr: "missing" }),
    });
    expect(await volumeExists("fhevm_keys-cache")).toBe(false);
  });

  test("rejects invalid volume names", async () => {
    await expect(exportVolume("../bad", ".fhevm")).rejects.toMatchObject({
      exitCode: ExitCode.CONFIG,
    });
    await expect(importVolume(".fhevm", "bad name")).rejects.toMatchObject({
      exitCode: ExitCode.CONFIG,
    });
    await expect(volumeExists("bad$name")).rejects.toMatchObject({
      exitCode: ExitCode.CONFIG,
    });
  });
});
