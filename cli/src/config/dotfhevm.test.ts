import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, rm, stat, writeFile } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { cleanDotFhevm, ensureDotFhevm, getDotFhevmPaths } from "./dotfhevm";

const tempRoots: string[] = [];

afterEach(async () => {
  await Promise.all(tempRoots.map((root) => rm(root, { recursive: true, force: true })));
  tempRoots.length = 0;
});

describe("dotfhevm", () => {
  test("ensureDotFhevm creates directories and is idempotent", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const first = await ensureDotFhevm(root);
    const second = await ensureDotFhevm(root);

    expect(second).toEqual(first);
    await expectExists(first.root);
    await expectExists(first.env);
    await expectExists(first.compose);
    await expectExists(first.keys);
    await expectExists(first.keysMinioSnapshot);
    await expectExists(first.keysVolumeSnapshot);
    await expectExists(first.logs);
  });

  test("getDotFhevmPaths resolves structure without creating dirs", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = getDotFhevmPaths(root);

    expect(paths.root).toBe(join(root, ".fhevm"));
    expect(await pathExists(paths.root)).toBe(false);
  });

  test("cleanDotFhevm keeps keys for standard clean", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await writeFile(paths.stateFile, "{}", "utf8");
    await writeFile(paths.versionCache, "{}", "utf8");

    const removed = await cleanDotFhevm(paths, { all: false, dryRun: false });

    expect(removed).toContain(paths.env);
    expect(removed).toContain(paths.compose);
    expect(removed).toContain(paths.logs);
    expect(removed).toContain(paths.stateFile);
    expect(removed).toContain(paths.versionCache);

    expect(await pathExists(paths.keys)).toBe(true);
    expect(await pathExists(paths.env)).toBe(false);
    expect(await pathExists(paths.stateFile)).toBe(false);
  });

  test("cleanDotFhevm --all removes entire tree", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await cleanDotFhevm(paths, { all: true, dryRun: false });

    expect(await pathExists(paths.root)).toBe(false);
  });

  test("cleanDotFhevm dry-run reports without deleting", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-cli-"));
    tempRoots.push(root);

    const paths = await ensureDotFhevm(root);
    await writeFile(paths.stateFile, "{}", "utf8");

    const planned = await cleanDotFhevm(paths, { all: true, dryRun: true });

    expect(planned).toContain(paths.root);
    expect(await pathExists(paths.root)).toBe(true);
    expect(await pathExists(paths.stateFile)).toBe(true);
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

async function expectExists(path: string): Promise<void> {
  expect(await pathExists(path)).toBe(true);
}
