import { describe, expect, it, beforeEach, afterEach } from "bun:test";
import { tmpdir } from "os";
import { join } from "path";
import { mkdtempSync, existsSync, rmSync } from "fs";
import { computeCacheEnvVars } from "../cache.js";

describe("computeCacheEnvVars", () => {
  let tmpDir: string;

  beforeEach(() => {
    tmpDir = mkdtempSync(join(tmpdir(), "fhevm-cache-test-"));
  });

  afterEach(() => {
    rmSync(tmpDir, { recursive: true, force: true });
  });

  // We pass the cacheDir relative to FHEVM_ROOT, but computeCacheEnvVars
  // resolves from FHEVM_ROOT. For testing, we use the absolute path approach
  // by calling computeCacheEnvVars() with no args and checking key shapes.

  it("includes Docker BuildKit flags", () => {
    const env = computeCacheEnvVars();
    expect(env.DOCKER_BUILDKIT).toBe("1");
    expect(env.COMPOSE_DOCKER_CLI_BUILD).toBe("1");
    expect(env.BUILDX_NO_DEFAULT_ATTESTATIONS).toBe("1");
    expect(env.DOCKER_BUILD_PROVENANCE).toBe("false");
    expect(env.FHEVM_CARGO_PROFILE).toBe("local");
  });

  it("includes unified coprocessor cache", () => {
    const env = computeCacheEnvVars();
    expect(env.FHEVM_CACHE_FROM_COPROCESSOR).toBeDefined();
    expect(env.FHEVM_CACHE_TO_COPROCESSOR).toBeDefined();
    expect(env.FHEVM_CACHE_FROM_COPROCESSOR).toContain("type=local,src=");
    expect(env.FHEVM_CACHE_TO_COPROCESSOR).toContain("mode=max");
  });

  it("includes unified kms-connector cache", () => {
    const env = computeCacheEnvVars();
    expect(env.FHEVM_CACHE_FROM_KMS_CONNECTOR).toBeDefined();
    expect(env.FHEVM_CACHE_TO_KMS_CONNECTOR).toBeDefined();
  });

  it("includes individual caches for all expected services", () => {
    const env = computeCacheEnvVars();
    const expectedServices = [
      "GATEWAY_DEPLOY_MOCKED_ZAMA_OFT",
      "GATEWAY_SC_ADD_NETWORK",
      "GATEWAY_SC_ADD_PAUSERS",
      "GATEWAY_SC_DEPLOY",
      "GATEWAY_SC_PAUSE",
      "GATEWAY_SC_TRIGGER_CRSGEN",
      "GATEWAY_SC_TRIGGER_KEYGEN",
      "GATEWAY_SC_UNPAUSE",
      "GATEWAY_SET_RELAYER_MOCKED_PAYMENT",
      "HOST_SC_ADD_PAUSERS",
      "HOST_SC_DEPLOY",
      "HOST_SC_PAUSE",
      "HOST_SC_UNPAUSE",
      "KMS_CONNECTOR_DB_MIGRATION",
      "TEST_SUITE_E2E_DEBUG",
    ];

    for (const key of expectedServices) {
      expect(env[`FHEVM_CACHE_FROM_${key}`]).toBeDefined();
      expect(env[`FHEVM_CACHE_TO_${key}`]).toBeDefined();
    }
  });

  it("coprocessor cache dirs reference the same directory", () => {
    const env = computeCacheEnvVars();
    // Extract the path from "type=local,src=<path>"
    const fromPath = env.FHEVM_CACHE_FROM_COPROCESSOR.replace("type=local,src=", "");
    const toPath = env.FHEVM_CACHE_TO_COPROCESSOR.replace("type=local,dest=", "").replace(",mode=max", "");
    expect(fromPath).toBe(toPath);
  });

  it("every CACHE_FROM has a matching CACHE_TO", () => {
    const env = computeCacheEnvVars();
    const fromKeys = Object.keys(env).filter((k) => k.startsWith("FHEVM_CACHE_FROM_"));
    const toKeys = new Set(Object.keys(env).filter((k) => k.startsWith("FHEVM_CACHE_TO_")));

    for (const fromKey of fromKeys) {
      const suffix = fromKey.replace("FHEVM_CACHE_FROM_", "");
      expect(toKeys.has(`FHEVM_CACHE_TO_${suffix}`)).toBe(true);
    }
  });

  it("total cache var count: 5 flags + 2 unified*2 + 15 individual*2 = 39", () => {
    const env = computeCacheEnvVars();
    // 5 global flags + (2 unified + 15 individual) * 2 (FROM+TO) = 5 + 34 = 39
    expect(Object.keys(env)).toHaveLength(39);
  });
});
