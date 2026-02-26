import { afterEach, describe, expect, test } from "bun:test";
import { mkdtemp, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import type { DotFhevmPaths } from "../config/dotfhevm";

import { __internal, captureDiagnosticLogs } from "./diagnostics";

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

afterEach(() => {
  __internal.resetOpsForTests();
});

describe("ci diagnostics", () => {
  test("captures logs for running containers", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-ci-diagnostics-"));
    const paths = makePaths(root);
    const tails: number[] = [];

    try {
      __internal.setOpsForTests({
        listProjectContainers: async () => [
          { name: "fhevm-minio", service: "fhevm-minio", state: "running" },
          { name: "host-node", service: "host-node", state: "exited" },
          { name: "gateway-node", service: "gateway-node", state: "running" },
        ],
        getContainerLogs: async (_name, options) => {
          tails.push(options?.tail ?? 0);
          return "line1\nline2";
        },
      });

      const files = await captureDiagnosticLogs(paths, { tailLines: 42 });
      expect(files).toEqual([join(paths.logs, "fhevm-minio.log"), join(paths.logs, "gateway-node.log")]);
      expect(tails).toEqual([42, 42]);
      expect((await Bun.file(files[0] ?? "").text()).trim()).toBe("line1\nline2");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  test("returns empty list when no running containers", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-ci-diagnostics-empty-"));
    const paths = makePaths(root);

    try {
      __internal.setOpsForTests({
        listProjectContainers: async () => [],
      });

      expect(await captureDiagnosticLogs(paths)).toEqual([]);
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });
});
