import { afterEach, describe, expect, test } from "bun:test";

import { buildCacheConfig, buildCacheEnvVars, __internal } from "./buildkit-cache";
import { __internal as detectInternal } from "./detect";

afterEach(() => {
  __internal.resetEnvReaderForTests();
  detectInternal.resetEnvReaderForTests();
});

describe("buildkit cache", () => {
  test("gha backend returns empty overrides when env is unset", () => {
    __internal.setEnvReaderForTests(() => undefined);

    expect(buildCacheEnvVars("gha")).toEqual({});
  });

  test("local backend keeps a single cache writer", () => {
    __internal.setEnvReaderForTests(() => undefined);

    const env = buildCacheEnvVars("local");
    expect(Object.keys(env)).toHaveLength(__internal.cacheEnvVars.length);
    expect(env.FHEVM_CACHE_FROM_COPROCESSOR).toBe("type=local,src=.buildx-cache/");
    expect(env.FHEVM_CACHE_TO_COPROCESSOR).toBe("type=local,dest=.buildx-cache/,mode=max");
    expect(env.FHEVM_CACHE_TO_GATEWAY_SC_DEPLOY).toBe("");
  });

  test("none backend disables all cache env vars", () => {
    __internal.setEnvReaderForTests(() => undefined);

    const env = buildCacheEnvVars("none");
    expect(Object.keys(env)).toHaveLength(__internal.cacheEnvVars.length);
    expect(new Set(Object.values(env))).toEqual(new Set([""]));
  });

  test("uses pre-set env vars unchanged", () => {
    __internal.setEnvReaderForTests((name) =>
      name === "FHEVM_CACHE_FROM_COPROCESSOR" ? "type=gha,scope=preconfigured" : undefined,
    );

    const env = buildCacheEnvVars("local");
    expect(env.FHEVM_CACHE_FROM_COPROCESSOR).toBe("type=gha,scope=preconfigured");
    expect(env.FHEVM_CACHE_TO_COPROCESSOR).toBe("type=local,dest=.buildx-cache/,mode=max");
  });

  test("buildCacheConfig selects backend from CI detection", () => {
    __internal.setEnvReaderForTests(() => undefined);
    detectInternal.setEnvReaderForTests((name) => (name === "CI" ? "true" : undefined));

    const config = buildCacheConfig();
    expect(config.backend).toBe("gha");
    expect(config.envVars).toEqual({});
  });

  test("buildCacheConfig no-cache forces none backend", () => {
    __internal.setEnvReaderForTests(() => undefined);
    detectInternal.setEnvReaderForTests((name) => (name === "CI" ? "true" : undefined));

    const config = buildCacheConfig({ noCache: true });
    expect(config.backend).toBe("none");
    expect(config.envVars.FHEVM_CACHE_FROM_COPROCESSOR).toBe("");
  });
});
