import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, rm, stat, writeFile } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { ensureDotFhevm, getDotFhevmPaths } from "../config/dotfhevm";
import { __internal } from "../docker/services";
import { ExitCode, FhevmCliError } from "../errors";

import { runCleanCommand, toCleanError } from "./clean";

const tempRoots: string[] = [];

afterEach(async () => {
  __internal.resetDockerOpsForTests();
  await Promise.all(tempRoots.map((root) => rm(root, { recursive: true, force: true })));
  tempRoots.length = 0;
});

describe("clean command", () => {
  test("default clean removes docker volumes and filesystem state", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await writeFile(paths.stateFile, "{}", "utf8");
    await writeFile(paths.versionCache, "{}", "utf8");

    let receivedVolumes: boolean | undefined;
    __internal.setDockerOpsForTests({
      composeDown: async (options) => {
        receivedVolumes = options.volumes;
      },
    });

    const result = await runCleanCommand({ all: false, dryRun: false }, paths);

    expect(receivedVolumes).toBe(true);
    expect(result.failures).toHaveLength(0);
    expect(result.removed).toContain(paths.env);
    expect(result.removed).toContain(paths.stateFile);
    expect(await pathExists(paths.keys)).toBe(true);
  });

  test("clean --all removes the entire .fhevm tree", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    __internal.setDockerOpsForTests({
      composeDown: async () => {},
    });

    const result = await runCleanCommand({ all: true, dryRun: false }, paths);

    expect(result.failures).toHaveLength(0);
    expect(result.removed).toEqual([paths.root]);
    expect(await pathExists(paths.root)).toBe(false);
  });

  test("clean --dry-run skips docker cleanup and preserves files", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await writeFile(paths.stateFile, "{}", "utf8");

    let dockerCalls = 0;
    __internal.setDockerOpsForTests({
      composeDown: async () => {
        dockerCalls += 1;
      },
    });

    const result = await runCleanCommand({ all: false, dryRun: true }, paths);

    expect(dockerCalls).toBe(0);
    expect(result.failures).toHaveLength(0);
    expect(result.removed).toContain(paths.env);
    expect(result.removed).toContain(paths.stateFile);
    expect(await pathExists(paths.root)).toBe(true);
  });

  test("clean --dry-run --all previews full removal", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);

    const result = await runCleanCommand({ all: true, dryRun: true }, paths);

    expect(result.failures).toHaveLength(0);
    expect(result.removed).toEqual([paths.root]);
    expect(await pathExists(paths.root)).toBe(true);
  });

  test("continues with filesystem cleanup when docker cleanup fails", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await writeFile(paths.stateFile, "{}", "utf8");

    __internal.setDockerOpsForTests({
      composeDown: async () => {
        throw new FhevmCliError({
          exitCode: ExitCode.DOCKER,
          step: "clean",
          message: "docker compose down failed",
        });
      },
    });

    const result = await runCleanCommand({ all: false, dryRun: false }, paths);

    expect(result.failures).toHaveLength(1);
    expect(result.failures[0]?.phase).toBe("docker");
    expect(result.removed).toContain(paths.env);
    expect(await pathExists(paths.stateFile)).toBe(false);
    expect(toCleanError(result.failures).exitCode).toBe(ExitCode.DOCKER);
  });

  test("succeeds when .fhevm does not exist", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = getDotFhevmPaths(root);

    __internal.setDockerOpsForTests({
      composeDown: async () => {},
    });

    const result = await runCleanCommand({ all: false, dryRun: false }, paths);

    expect(result.failures).toHaveLength(0);
    expect(result.removed).toEqual([]);
  });
});

async function pathExists(path: string): Promise<boolean> {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}
