import { afterEach, describe, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import { buildCacheConfig, __internal as cacheInternal } from "./buildkit-cache";
import { __internal as detectInternal } from "./detect";
import { __internal as jsonOutputInternal } from "./json-output";
import type { DotFhevmPaths } from "../config/dotfhevm";
import { createDefaultConfig } from "../config/model";
import { ExitCode, exitWithError } from "../errors";
import { __internal as keyCacheInternal, checkKeyCache, restoreKeys, snapshotKeys } from "../keys/cache";
import { createPipelineOutput } from "../pipeline/output";

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
  cacheInternal.resetEnvReaderForTests();
  detectInternal.resetEnvReaderForTests();
  keyCacheInternal.resetOpsForTests();
  jsonOutputInternal.resetWriterForTests();
});

describe("ci integration", () => {
  test("detect CI + build cache config flow", () => {
    cacheInternal.setEnvReaderForTests(() => undefined);
    detectInternal.setEnvReaderForTests((name) => (name === "GITHUB_ACTIONS" ? "true" : undefined));

    const config = buildCacheConfig();
    expect(config.backend).toBe("gha");
    expect(config.envVars).toEqual({});

    const none = buildCacheConfig({ noCache: true });
    expect(none.backend).toBe("none");
    expect(none.envVars.FHEVM_CACHE_FROM_COPROCESSOR).toBe("");
  });

  test("key cache lifecycle from snapshot to restore", async () => {
    const root = await mkdtemp(join(tmpdir(), "fhevm-ci-integration-keys-"));
    const paths = makePaths(root);
    const restoreCalls: string[] = [];

    try {
      const config = makeConfig();
      await mkdir(paths.keys, { recursive: true });

      expect((await checkKeyCache(paths)).isComplete).toBe(false);

      keyCacheInternal.setOpsForTests({
        downloadBucket: async (_endpoint, _bucket, dest) => {
          await mkdir(join(dest, "PUB"), { recursive: true });
          await Bun.write(join(dest, "PUB", "key"), "cached");
          return 1;
        },
        exportVolume: async (_volume, dest) => {
          await mkdir(dest, { recursive: true });
          await Bun.write(join(dest, "cache"), "ok");
        },
        importVolume: async () => {
          restoreCalls.push("import");
        },
        uploadBucket: async () => {
          restoreCalls.push("upload");
          return 1;
        },
      });

      await snapshotKeys(paths, config);
      expect((await checkKeyCache(paths)).isComplete).toBe(true);

      await restoreKeys(paths, config);
      expect(restoreCalls).toEqual(["import", "upload"]);
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  test("pipeline output json mode emits NDJSON events", () => {
    const lines: string[] = [];
    jsonOutputInternal.setWriterForTests((line) => {
      lines.push(line);
    });

    const output = createPipelineOutput("json");
    output.pipelineHeader(13);
    output.stepStart({ number: 1, displayName: "MinIO" }, 13);
    output.stepSuccess({ number: 1, displayName: "MinIO" }, 123);
    output.discoveryResult("MinIO IP", "172.18.0.2");
    output.pipelineFail(1, "boom");

    const events = lines.map((line) => JSON.parse(line) as Record<string, unknown>);
    expect(events.map((event) => event.type)).toEqual([
      "pipeline-start",
      "step-start",
      "step-success",
      "discovery",
      "pipeline-fail",
    ]);
  });

  test("exitWithError emits JSON in json mode", () => {
    const stderr: string[] = [];
    const originalError = console.error;
    const originalExit = process.exit;

    console.error = (...args: unknown[]) => {
      stderr.push(args.map((arg) => String(arg)).join(" "));
    };
    (process as { exit: (code?: number) => never }).exit = ((code?: number) => {
      throw new Error(`exit:${String(code)}`);
    }) as typeof process.exit;

    try {
      expect(() =>
        exitWithError(
          { exitCode: ExitCode.CONFIG, message: "bad config", service: "gateway-node" },
          { json: true },
        ),
      ).toThrow("exit:2");

      const parsed = JSON.parse(stderr[0] ?? "") as Record<string, unknown>;
      expect(parsed.error).toBe(true);
      expect(parsed.hint).toBe("fhevm-cli logs gateway-node");
    } finally {
      console.error = originalError;
      process.exit = originalExit;
    }
  });
});
