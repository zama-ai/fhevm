import { afterEach, describe, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { ENV_FILE_NAMES, type EnvFileName } from "./service-map";
import { findExistingEnvFiles } from "./env-loader";
import { composeEnvFilePath } from "./env-writer";

const tempRoots: string[] = [];

afterEach(async () => {
  await Promise.all(tempRoots.map((root) => rm(root, { recursive: true, force: true })));
  tempRoots.length = 0;
});

describe("env loader", () => {
  test("returns empty map when env directory does not exist", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const envFileByName = findExistingEnvFiles(join(root, ".fhevm", "env"));
    expect(envFileByName.size).toBe(0);
  });

  test("loads only existing env files", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const envDir = join(root, ".fhevm", "env");
    await mkdir(envDir, { recursive: true });
    await writeEnv(envDir, "coprocessor");
    await writeEnv(envDir, "kms-core");

    const envFileByName = findExistingEnvFiles(envDir);

    expect(envFileByName.size).toBe(2);
    expect(envFileByName.get("coprocessor")).toBe(composeEnvFilePath(envDir, "coprocessor"));
    expect(envFileByName.get("kms-core")).toBe(composeEnvFilePath(envDir, "kms-core"));
    expect(envFileByName.has("relayer")).toBe(false);
  });

  test("loads every supported env file name when present", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const envDir = join(root, ".fhevm", "env");
    await mkdir(envDir, { recursive: true });

    await Promise.all(ENV_FILE_NAMES.map((name) => writeEnv(envDir, name)));

    const envFileByName = findExistingEnvFiles(envDir);

    expect(envFileByName.size).toBe(ENV_FILE_NAMES.length);
    for (const name of ENV_FILE_NAMES) {
      expect(envFileByName.get(name)).toBe(composeEnvFilePath(envDir, name));
    }
  });
});

async function writeEnv(envDir: string, name: EnvFileName): Promise<void> {
  await writeFile(composeEnvFilePath(envDir, name), "KEY=value\n", "utf8");
}
