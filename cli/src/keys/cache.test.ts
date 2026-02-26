import { afterEach, describe, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { createDefaultConfig } from "../config/model";
import type { DotFhevmPaths } from "../config/dotfhevm";

import { __internal, checkKeyCache, clearKeyCache, restoreKeys, snapshotKeys } from "./cache";

function makePaths(root: string): DotFhevmPaths {
  return {
    root,
    env: join(root, "env"),
    compose: join(root, "compose"),
    keys: join(root, "keys"),
    keysMinioSnapshot: join(root, "keys", "minio-snapshot"),
    keysVolumeSnapshot: join(root, "keys", "volume-snapshot"),
    logs: join(root, "logs"),
    stateFile: join(root, "state.json"),
    versionCache: join(root, "version-cache.json"),
  };
}

function makeConfig() {
  return createDefaultConfig({
    deployer: { privateKey: "0x1", address: "0x1" },
    newOwner: { privateKey: "0x2", address: "0x2" },
    txSender: { privateKey: "0x3", address: "0x3" },
    coprocessors: [],
    kmsNodes: [],
    custodians: [],
    pausers: [],
  });
}

afterEach(() => {
  __internal.resetOpsForTests();
});

describe("key cache", () => {
  test("checkKeyCache reports incomplete snapshots", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-key-cache-empty-"));
    const paths = makePaths(root);

    try {
      await mkdir(paths.keysMinioSnapshot, { recursive: true });
      await mkdir(paths.keysVolumeSnapshot, { recursive: true });

      const state = await checkKeyCache(paths);
      expect(state.hasMinioSnapshot).toBe(false);
      expect(state.hasVolumeSnapshot).toBe(false);
      expect(state.isComplete).toBe(false);
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  test("checkKeyCache reports complete snapshots with metadata", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-key-cache-full-"));
    const paths = makePaths(root);

    try {
      await mkdir(paths.keysMinioSnapshot, { recursive: true });
      await mkdir(paths.keysVolumeSnapshot, { recursive: true });
      await Bun.write(join(paths.keysMinioSnapshot, "k1"), "a");
      await Bun.write(join(paths.keysVolumeSnapshot, "k2"), "b");
      await Bun.write(join(paths.keys, "snapshot-meta.json"), JSON.stringify({ date: "2026-01-01T00:00:00.000Z" }));

      const state = await checkKeyCache(paths);
      expect(state.hasMinioSnapshot).toBe(true);
      expect(state.hasVolumeSnapshot).toBe(true);
      expect(state.isComplete).toBe(true);
      expect(state.snapshotDate).toBe("2026-01-01T00:00:00.000Z");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  test("snapshotKeys snapshots minio and volume then writes metadata", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-key-cache-snapshot-"));
    const paths = makePaths(root);
    const calls: string[] = [];

    try {
      __internal.setOpsForTests({
        downloadBucket: async () => {
          calls.push("downloadBucket");
          return 7;
        },
        exportVolume: async () => {
          calls.push("exportVolume");
        },
      });

      await snapshotKeys(paths, makeConfig());

      expect(calls).toEqual(["downloadBucket", "exportVolume"]);
      const meta = JSON.parse(await Bun.file(join(paths.keys, "snapshot-meta.json")).text()) as Record<string, unknown>;
      expect(meta.minioObjectCount).toBe(7);
      expect(meta.volumeExported).toBe(true);
      expect(typeof meta.date).toBe("string");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  test("restoreKeys restores volume before uploading minio bucket", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-key-cache-restore-"));
    const paths = makePaths(root);
    const calls: string[] = [];

    try {
      await mkdir(paths.keysMinioSnapshot, { recursive: true });
      await mkdir(paths.keysVolumeSnapshot, { recursive: true });

      __internal.setOpsForTests({
        importVolume: async () => {
          calls.push("importVolume");
        },
        uploadBucket: async () => {
          calls.push("uploadBucket");
          return 0;
        },
      });

      await restoreKeys(paths, makeConfig());
      expect(calls).toEqual(["importVolume", "uploadBucket"]);
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  test("clearKeyCache removes keys directory", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-key-cache-clear-"));
    const paths = makePaths(root);

    try {
      await mkdir(paths.keysMinioSnapshot, { recursive: true });
      await Bun.write(join(paths.keysMinioSnapshot, "k1"), "x");

      await clearKeyCache(paths);
      expect(await Bun.file(paths.keys).exists()).toBe(false);
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });
});
